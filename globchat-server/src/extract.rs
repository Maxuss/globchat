use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use mongodb::bson::doc;
use crate::err::GlobError;
use crate::model::{UserData};
use crate::routes::auth::verify_token;
use crate::state::AppState;

#[derive(Debug)]
pub struct Authenticated(pub UserData);

#[async_trait]
impl FromRequestParts<AppState> for Authenticated {
    type Rejection = GlobError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        if let Some(v) = parts.headers.get(AUTHORIZATION).cloned() {
            let str = v.to_str()?;
            let token = str.replace("Bearer", "");
            let token = token.trim();
            if token.is_empty() {
                return Err(GlobError::Unauthenticated)
            }
            let user_id = verify_token(token, &state.jwt_secret)?;
            let user = state.database.users.find_one(doc! { "id": user_id }, None).await?;
            return if let Some(user) = user {
                Ok(Authenticated(user))
            } else {
                Err(GlobError::Unauthenticated)
            }
        }
        Err(GlobError::BadRequest("Authorization header not provided!"))
    }
}