use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use tracing::error;

use super::{RequestCreateUser, ResponseOutUser};
use crate::database::users::{self, Entity as Users};
use crate::routes::users::ResponseUser;
use crate::utilities::app_error::AppError;
use crate::utilities::hash::verify_password;
use crate::utilities::jwt::create_token;
use crate::utilities::token_wrapper::TokenWrapper;

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseOutUser>, AppError> {
    let user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&db)
        .await
        .map_err(|error| {
            error!("Error in loging{:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "login error, please try later",
            )
        })?;
    if let Some(user) = user {
        if !verify_password(&request_user.password, &user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Incorrect username and/or password",
            ));
        }
        let token = create_token(jwt_secret.0, user.username.clone())?;
        let mut user = user.into_active_model();
        user.token = Set(Some(token));

        let user = user.save(&db).await.map_err(|error| {
            error!("Error in updating token to user{:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error in login please try later",
            )
        })?;

        let response = ResponseOutUser {
            data: ResponseUser {
                id: user.id.unwrap(),
                username: user.username.unwrap(),
                token: user.token.unwrap().unwrap(),
            },
        };
        Ok(Json(response))
    } else {
        error!("Error in login user ");
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Incorrect username and/or password",
        ))
    }
}
