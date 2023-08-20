use std::num::NonZeroU32;
use std::str::FromStr;
use argon2::{Algorithm, Argon2, AssociatedData, Params, ParamsBuilder, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::headers::{Authorization, HeaderMap};
use axum::headers::authorization::{Bearer, Credentials};
use axum::{Json, TypedHeader};
use axum_macros::debug_handler;
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use rand::Rng;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use snowflake::SnowflakeIdGenerator;
use uuid::Uuid;
use crate::db::Database;
use crate::err::{GlobError, GlobResult};
use crate::model::{UserData, UserId};
use crate::response::{AuthLoginRequest, AuthLoginResponse, AuthLoginStatus, AuthRegisterResponse, AuthRegisterStatus, AuthS0NextStep, AuthStatusResponse, GlobResponse};
use crate::state::{AppState, ConnectedClients, JwtSecret};

#[debug_handler]
pub async fn auth_status(
    State(jwt_secret): State<JwtSecret>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>
) -> Json<AuthStatusResponse> {
    let verified = verify_token(bearer.token(), &jwt_secret);
    return if let Ok(id) = verified {
        Json(AuthStatusResponse { next: AuthS0NextStep::Proceed { uid: id } })
    } else {
        Json(AuthStatusResponse { next: AuthS0NextStep::Login })
    }
}

#[debug_handler(state = AppState)]
pub async fn auth_login(
    State(AppState { database, connected_clients, jwt_secret, .. }): State<AppState>,
    Json(AuthLoginRequest { username, password }): Json<AuthLoginRequest>
) -> GlobResponse<AuthLoginResponse> {
    let user = database.users.find_one(doc! { "username": username }, None).await?;
    return if let Some(user) = user {
        let valid = verify_password(&password, &user.password);
        if !valid {
            Ok(Json(AuthLoginResponse {
                status: AuthLoginStatus::InvalidPassword
            }))
        } else {
            let jwt = generate_jwt(user.id, jwt_secret)?;
            connected_clients.insert(user.id);
            Ok(Json(AuthLoginResponse {
                status: AuthLoginStatus::LoggedIn {
                    token: jwt
                }
            }))
        }
    } else {
        Ok(Json(AuthLoginResponse {
            status: AuthLoginStatus::UserNotFound
        }))
    }
}

#[debug_handler(state = AppState)]
pub async fn auth_register(
    State(AppState { database, snowflakes, .. }): State<AppState>,
    Json(AuthLoginRequest { username, password }): Json<AuthLoginRequest>
) -> GlobResponse<AuthRegisterResponse> {
    let user = database.users.find_one(doc! { "username": username.clone() }, None).await?;
    return if let Some(_) = user {
        Ok(Json(AuthRegisterResponse { status: AuthRegisterStatus::UserExists }))
    } else {
        let hash = encode_password(&password)?;
        database.users.insert_one(UserData {
            username,
            password: hash,
            id: snowflakes.generate(),
            timestamp: Utc::now().timestamp() as u64,
            messages: Vec::new()
        }, None).await?;
        Ok(Json(AuthRegisterResponse { status: AuthRegisterStatus::Success }))
    }
}

fn encode_password(pwd: &str) -> GlobResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pwd.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(GlobError::PasswordError)
}

fn verify_password(pwd: &str, hash: &str) -> bool {
    return match PasswordHash::new(&hash) {
        Ok(parsed) => Argon2::default().verify_password(pwd.as_bytes(), &parsed).is_ok(),
        _ => false
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

fn generate_jwt(uid: i64, jwt_secret: String) -> GlobResult<String> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::hours(8)).timestamp() as usize;
    let claims = Claims {
        sub: uid.to_string(),
        iat,
        exp
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes())
    ).map_err(GlobError::from)
}

pub fn verify_token(jwt: &str, jwt_secret: &str) -> GlobResult<UserId> {
    let token = decode::<Claims>(&jwt, &DecodingKey::from_secret(jwt_secret.as_bytes()), &Validation::default())?;
    i64::from_str(&token.claims.sub).map_err(GlobError::from)
}