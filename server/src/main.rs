use axum::{routing::get, Router};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod entity;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Entity First: 同步数据库 schema
    db.get_schema_registry("stitchwork_server::entity::*")
        .sync(&db)
        .await
        .expect("Failed to sync schema");

    let state = Arc::new(AppState { db });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(routes::customer::router())
        .merge(routes::worker::router())
        .merge(routes::order::router())
        .merge(routes::process::router())
        .merge(routes::piece_record::router())
        .merge(routes::payroll::router())
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
