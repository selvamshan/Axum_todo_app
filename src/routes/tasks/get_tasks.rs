use axum::extract::Path;
use axum::Json;
use tracing::error;
use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::users::Model as UserModel;
use crate::database::tasks::{self, Entity as Tasks};
use crate::routes::tasks::ResponseTask;
use crate:: utilities::app_error::AppError;

use super::{ResponseDataTasks, ResponseDataTask};

pub async fn get_all_tasks(
    State(db):State<DatabaseConnection>,
    Extension(user): Extension<UserModel>
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(user.id))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db)
        .await
        .map_err(|error| {
            error!("Error in fetching one task {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Internal server error"
            )
        })?
        .into_iter()
        .map(|db_task| ResponseTask{ 
            id: db_task.id, 
            title: db_task.title, 
            desctription: db_task.description, 
            priority: db_task.priority, 
            completed_at: db_task.completed_at.map(|time| time.to_string()), 
            user_id: db_task.user_id
        })
        .collect();
    Ok(Json(ResponseDataTasks{
        data:tasks
    }))
}

pub async fn get_one_task(
    State(db):State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    Path(task_id): Path<i32>
) -> Result<Json<ResponseDataTask>, AppError>  {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| {
            error!("Error in fetching all tasks {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Internal server error"
            )
        })?;

        let response = if let Some(task) = task {
            ResponseDataTask {
                data: ResponseTask {
                    id: task.id,
                    title: task.title,
                    desctription: task.description,
                    priority: task.priority,
                    completed_at: task.completed_at.map(|time| time.to_string()),
                    user_id: task.user_id,
                }
            }
        } else {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Could not find require task"));
        };

        Ok(Json(response))
}