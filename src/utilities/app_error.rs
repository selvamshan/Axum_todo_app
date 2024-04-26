use axum::http::StatusCode;
use axum::response::{Response,IntoResponse};
use axum::Json;
use serde::Serialize;



pub struct AppError {
    code: StatusCode,
    message: String,
}


impl AppError {
    pub fn new(code:StatusCode, message: impl Into<String>) -> Self{
        Self {
            code,
            message:message.into(),
        }
    }
}

#[derive(Serialize)]
pub struct ResponeError {
    error_message: String
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.code,
            Json({
                ResponeError{error_message: self.message}
            }),
        )
        .into_response()
    }
}