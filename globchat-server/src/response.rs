use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::err::{GlobError, GlobResult};

//########## /auth/status
#[derive(Serialize)]
pub struct AuthStatusResponse {
    pub next: AuthS0NextStep
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthS0NextStep {
    Proceed { uid: Uuid },
    Login,
}

//########## /auth/login
#[derive(Deserialize, Debug)]
pub struct AuthLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthLoginResponse {
    pub status: AuthLoginStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthLoginStatus {
    LoggedIn { token: String },
    UserNotFound,
    InvalidPassword,
}

//########## /auth/register
#[derive(Serialize)]
pub struct AuthRegisterResponse {
    pub status: AuthRegisterStatus
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthRegisterStatus {
    Success,
    UserExists
}

// util
pub type GlobResponse<T> = GlobResult<Json<T>>;

#[derive(Serialize)]
struct Success<T> {
    success: bool,
    #[serde(flatten)]
    value: T
}

#[derive(Serialize)]
struct Failure {
    error: String,
}

impl IntoResponse for GlobError {
    fn into_response(self) -> Response {
        (self.code(), Json(Success { success: false, value: Failure { error: self.to_string() } })).into_response()
    }
}