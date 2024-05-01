use axum::{async_trait, body::Body, extract::FromRequest, http::{Request, StatusCode}, Json};
use serde::Deserialize;
use  tracing::error;
use validator::Validate;

use crate::utilities::app_error::AppError;


#[derive(Deserialize, Validate, Debug)]
pub struct ValidateCreateTask {
    #[validate(length(min=1, max=1))]
    pub priority: Option<String>,   
    #[validate(required(message="missing title"))] 
    pub title: Option<String>,
    pub description: Option<String>
}


#[async_trait]
impl<S> FromRequest<S> for ValidateCreateTask
where
    Body: FromRequest<S>,
    S: Send + Sync,
{
    //type Rejection = Response;
    type Rejection = AppError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(task):Json<ValidateCreateTask> = Json::from_request(req, state)
            .await
           //.map_err(IntoResponse::into_response)?;
           .map_err(|err| {
            error!("Error in custom extrctor {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "interanl server error")
        })?;
        
        if let Err(errors) = task.validate() {
            error!("validation error: {}", errors);
            let field_errors = errors.field_errors();
            for (_, error) in field_errors {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST, 
                    error.first().unwrap().clone().message.unwrap().to_string()
                ));
            }
            
        }

        Ok(task)
    }
}
