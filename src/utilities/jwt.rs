use std::time::Duration;
use axum::http::StatusCode;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use super::app_error::AppError;


/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {    
    exp: usize,
    username: String
}


pub fn create_token(secret:String, username: String) -> Result<String, AppError> {    
    let now = chrono::Utc::now();
    let exp =  (now + Duration::from_secs(300)).timestamp() as usize;
    let claims = Claims{ exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());
    
    let token = encode(&token_header, &claims, &key)
    .map_err(|error| {
        eprintln!("Error creating token {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "please try again later")
    })?;
    Ok(token)
   }