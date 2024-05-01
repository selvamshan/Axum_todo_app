use axum::http::StatusCode;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::error;

use super::app_error::AppError;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    username: String,
}

pub fn create_token(secret: String, username: String) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let exp = (now + Duration::from_secs(3600)).timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    let token = encode(&token_header, &claims, &key).map_err(|error| {
        error!("Error creating token {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "please try again later")
    })?;
    Ok(token)
}

pub fn validate_token(secret: &str, token: &str) -> Result<bool, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map_err(|error| {
            error!("token validation error: {:?}", error);
            match error.kind() {
                ErrorKind::InvalidToken | ErrorKind::InvalidSignature => {
                    AppError::new(StatusCode::UNAUTHORIZED, "Invalid Token")
                }
                ErrorKind::ExpiredSignature => {
                    AppError::new(StatusCode::UNAUTHORIZED, "Token got expired")
                }
                _ => {
                    error!("token validation error: {:?}", error);
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
                }
            }
        })
        .map(|_claim| true)
}
