use axum::http::StatusCode;
use thiserror::Error;

pub type GlobResult<T> = Result<T, GlobError>;

#[derive(Debug, Error)]
pub enum GlobError {
    #[error("An error occurred while operating with a password: {0}")]
    PasswordError(argon2::password_hash::Error),
    #[error("An error occurred while operating with a JWT: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("An error occurred while parsing a UUID: {0}")]
    UuidError(#[from] uuid::Error),
    #[error("An error occurred within database: {0}")]
    DatabaseError(#[from] mongodb::error::Error)
}

impl GlobError {
    pub fn code(&self) -> StatusCode {
        match self {
            GlobError::PasswordError(_) => StatusCode::BAD_REQUEST,
            GlobError::JwtError(_) => StatusCode::BAD_REQUEST,
            GlobError::UuidError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GlobError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}