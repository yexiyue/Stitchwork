use axum::{routing::get, Router};
use sea_orm::{Database, DbConn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

mod common;
mod entity;
mod error;
mod s3;
mod service;

pub use s3::S3Client;
pub use service::notification::{Notification, Notifier, SharedNotifier};

/// 邀请码存储: code -> (workshop_id, expires_at)
pub type InviteCodes = RwLock<HashMap<String, (Uuid, i64)>>;

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
    pub invite_codes: Arc<InviteCodes>,
    pub s3: Option<S3Client>,
    pub notifier: SharedNotifier,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = sea_orm::ConnectOptions::new(&database_url);
    opt.sqlx_logging(true);
    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to database");

    // Entity First: 同步数据库 schema
    db.get_schema_registry("stitchwork_server::entity::*")
        .sync(&db)
        .await
        .expect("Failed to sync schema");

    // S3 客户端（可选）
    let s3 = s3::S3Config::from_env().map(|c| S3Client::new(&c));
    if s3.is_some() {
        tracing::info!("S3 client initialized");
    }

    let state = Arc::new(AppState {
        db,
        invite_codes: Arc::new(RwLock::new(HashMap::new())),
        s3,
        notifier: Arc::new(Notifier::new()),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(service::routes())
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
