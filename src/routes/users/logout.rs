use axum::{extract::State, http::StatusCode, Extension};
use tracing::error;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::{database::users::Model, utilities::app_error::AppError};

pub async fn logout(
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>
)-> Result<(), AppError>{
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&db)
    .await
    .map_err(|error| {
        error!("Logout error in removing token {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
    })?;

    Ok(())
}


