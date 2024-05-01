use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use tracing::error;
use axum::{extract::State, Extension};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::database::users::Model as UserModel;
use crate::database::tasks::{self, Entity as Tasks};
use crate::utilities::app_error::AppError;

use super::RequestTask;

pub async fn mark_completed(
    State(db): State<DatabaseConnection>,
    Extension(user):Extension<UserModel>,
    Path(task_id): Path<i32>
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
    .filter(tasks::Column::UserId.eq(user.id))
    .one(&db)
    .await
    .map_err(|error| {
        error!("Error in fetching task for update {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Internal server error"
        )
    })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };
    let now = Utc::now();
    task.deleted_at = Set(Some(now.into()));

    task.save(&db)
    .await
    .map_err( |error| {
        error!("Error in marking the task deleted {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "error while updating task as completd"
        )
    })?;

    Ok(())
}

pub async fn mark_uncompleted(
    State(db): State<DatabaseConnection>,
    Extension(user):Extension<UserModel>,
    Path(task_id): Path<i32>,  
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
    .filter(tasks::Column::UserId.eq(user.id))
    .one(&db)
    .await
    .map_err(|error| {
        error!("Error in fetching task for update {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Internal server error"
        )
    })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };
    
    task.deleted_at = Set(None);

    task.save(&db)
    .await
    .map_err( |error| {
        error!("Error in marking the task deleted {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "error while updating task as uncompletd"
        )
    })?;

    Ok(())
}


pub async fn update_task(
    State(db): State<DatabaseConnection>,
    Extension(user):Extension<UserModel>,
    Path(task_id): Path<i32>,
    Json(request_task): Json<RequestTask>
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
    .filter(tasks::Column::UserId.eq(user.id))
    .one(&db)
    .await
    .map_err(|error| {
        error!("Error in fetching task for update {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Internal server error"
        )
    })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };
    
    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    };

    if let Some(description) = request_task.description {
        task.description = Set(description);
    };

    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    };

    if let Some(title) = request_task.title {
        task.title = Set(title);
    };
  

    task.save(&db)
    .await
    .map_err( |error| {
        error!("Error in marking the task deleted {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "error while updating task as uncompletd"
        )
    })?;

    Ok(())
}