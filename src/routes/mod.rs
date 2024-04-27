use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

mod hello_world;
mod users;
use hello_world::hello;

use crate::middleware::require_auth::require_auth;
use crate::routes::users::create_users::create_user;
use crate::routes::users::login::login;
use crate::routes::users::logout::logout;
use crate::utilities::app_state::AppState;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(app_state.clone(),require_auth))
        .route("/", get(hello))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}
