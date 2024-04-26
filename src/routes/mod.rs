use axum::Router;
use axum::routing::{get, post};


mod hello_world;
mod users;
use hello_world::hello;

use crate::utilities::app_state::AppState;
use crate::routes::users::create_users::create_user;
use crate::routes::users::login::login;


pub fn create_router(app_state:AppState) -> Router {
   
    Router::new()
    .route("/", get(hello))
    .route("/api/v1/users", post(create_user))
    .route("/api/v1/users/login", post(login))
    .with_state(app_state)
}