use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::{body::Body, extract::Request, middleware::Next};
use tracing::error;
// use axum_extra::TypedHeader;
// use headers::Authorization;
// use headers::authorization::Bearer;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::users;
use crate::database::users::Entity as Users;
use crate::utilities::app_error::AppError;
use crate::utilities::jwt::validate_token;
use crate::utilities::token_wrapper::TokenWrapper;

pub async fn require_auth(
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    headers: HeaderMap,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = if let Some(token) = headers.get("x-auth-token") {
        token
            .to_owned()
            .to_str()
            .map_err(|error| {
                error!("Error in extract token from header: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
            })?
            .to_owned()
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Missing authentication token",
        ));
    };
    validate_token(&token_secret.0, &token)?;
    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(token.clone()))
        .one(&db)
        .await
        .map_err(|error| {
            error!("Error in finding user for token{:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        })? {
        user
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized, please loging again",
        ));
    };

   
    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
