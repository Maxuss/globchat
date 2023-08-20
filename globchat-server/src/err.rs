use std::num::ParseIntError;
use axum::http::header::ToStrError;
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
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Failed to convert header value to string: {0}")]
    StringError(#[from] ToStrError),
    #[error("You are unauthorized to access this endpoint. Login first.")]
    Unauthenticated,
    #[error("Your user does not exist. For some reason.")]
    InvalidUser,
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("Failed to parse a snowflake: {0}")]
    ParseError(#[from] ParseIntError),

    #[error("User with this ID was not found")]
    UserNotFound,
    #[error("Channel with this ID was not found")]
    ChannelNotFound,
    #[error("Message with this ID was not found")]
    MessageNotFound,
}

impl GlobError {
    pub fn code(&self) -> StatusCode {
        match self {
            GlobError::PasswordError(_) => StatusCode::BAD_REQUEST,
            GlobError::JwtError(_) => StatusCode::BAD_REQUEST,
            GlobError::UuidError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GlobError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GlobError::StringError(_) => StatusCode::BAD_REQUEST,
            GlobError::Unauthenticated => StatusCode::UNAUTHORIZED,
            GlobError::InvalidUser => StatusCode::BAD_REQUEST,
            GlobError::BadRequest(_) => StatusCode::BAD_REQUEST,
            GlobError::UserNotFound | GlobError::ChannelNotFound | GlobError::MessageNotFound => StatusCode::NOT_FOUND,
            GlobError::ParseError(_) => StatusCode::BAD_REQUEST
        }
    }
}