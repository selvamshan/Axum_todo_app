use super::app_error::AppError;
use axum::http::StatusCode;
use bcrypt::{hash, verify};
use tracing::error;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, 12).map_err(|error| {
        eprintln!("Error hashing passowrd {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error secreting password",
        )
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|error| {
        error!("Password verification faided {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was problem in verifing password",
        )
    })
}
