use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use tracing::error;

use crate::utilities::hash::hash_password;
use crate::utilities::token_wrapper::TokenWrapper;
// use crate::database::prelude::*;
use crate::{database::users, utilities::jwt::create_token};
use crate::utilities::app_error::AppError;
use super::{RequestCreateUser, ResponseOutUser, ResponseUser};


pub async fn create_user(
    State(db):State<DatabaseConnection>, 
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,     
) -> Result<Json<ResponseOutUser>, AppError>  {
    let mut new_user = users::ActiveModel{..Default::default()};
    new_user.username = Set(request_user.username.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.token = Set(Some(create_token(jwt_secret.0, request_user.username)?));

    let user = new_user.save(&db)
        .await
        .map_err(|error| {
            let error_message = error.to_string();
            if error_message.contains("duplicate key value violates unique constraint") {
                AppError::new(
                    StatusCode::BAD_REQUEST, 
                    "username already taken, try again with different username" )
            } else {
                error!("Error creating user{:?}", error_message);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    "something went wrong, Please try again" 
                )
            }            
            
        })?;        
    
    let response = ResponseOutUser{ 
        data:  ResponseUser{
            id: user.id.unwrap(),
            username: user.username.unwrap(),
            token: user.token.unwrap().unwrap()
        }
    };
    Ok(Json(response))
}