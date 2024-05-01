use axum::middleware;
use axum::routing::{get, patch, post, put};
use axum::Router;

mod hello_world;
mod users;
mod tasks;

use hello_world::hello;
use crate::middleware::require_auth::require_auth;
use crate::routes::users::create_users::create_user;
use crate::routes::users::login::login;
use crate::routes::users::logout::logout;
use crate::routes::tasks::create_task::create_task;
use crate::routes::tasks::get_tasks::{get_all_tasks, get_one_task};
use crate::routes::tasks::update_tasks::{mark_completed, mark_uncompleted, update_task};
use crate::utilities::app_state::AppState;



pub fn create_router(app_state: AppState) -> Router {
    Router::new()
       .route("/api/v1/tasks/:task_id/uncompleted", put(mark_uncompleted))
        .route("/api/v1/tasks/:task_id/completed", put(mark_completed))
        .route("/api/v1/tasks/:task_id", patch(update_task))
        .route("/api/v1/tasks/:task_id", get(get_one_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/users/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(app_state.clone(),require_auth))
        .route("/", get(hello))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}
