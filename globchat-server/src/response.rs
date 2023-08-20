use serde::{Deserialize, Serialize};
use uuid::Uuid;

//########## /auth/status
#[derive(Serialize)]
pub struct AuthStatusResponse {
    pub next: AuthS0NextStep
}

#[derive(Serialize)]
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
pub enum AuthRegisterStatus {
    Success,
    UserExists
}