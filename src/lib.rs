use axum::Router;
use dotenvy_macro::dotenv;
use sea_orm::Database;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod database;
mod middleware;
mod routes;
mod utilities;
use crate::utilities::app_state::AppState;
use crate::utilities::token_wrapper::TokenWrapper;

pub async fn run() {
    let database_url = dotenv!("DATABASE_URL");
    let secret = dotenv!("JWT_SECRET").to_owned();
    //info!("{}", database_url);
    let database = Database::connect(database_url).await.unwrap();
    let app_state = AppState {
        database,
        jwt_secret: TokenWrapper(secret),
    };

    tracing_subscriber::fmt()
        .without_time() //For early local development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new().merge(routes::create_router(app_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
