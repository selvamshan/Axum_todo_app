use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use tracing::error;

use crate::{database::{tasks, users::Model as UserModel}, utilities::app_error::AppError};

use super::{custom_extractor::ValidateCreateTask, ResponseDataTask, ResponseTask};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,    
    request_task: ValidateCreateTask
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let new_task = tasks::ActiveModel{       
        priority: Set(request_task.priority),
        title: Set(request_task.title.unwrap()),       
        description: Set(request_task.description),      
        user_id: Set(Some(user.id)),       
        ..Default::default()
    };
    let task = new_task.save(&db)
    .await
    .map_err(|error| {
        error!("Error in creating task {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Not able to create task")
    })?;
    
    Ok((
        StatusCode::CREATED,
        Json( ResponseDataTask{ 
            data :  ResponseTask 
            {
                id: task.id.unwrap(),
                title: task.title.unwrap(),
                desctription: task.description.unwrap(),
                priority: task.priority.unwrap(), 
                completed_at: task.completed_at.unwrap().map(|time| time.to_string()),
                user_id: task.user_id.unwrap()              
            } 
        })         
    ))
}