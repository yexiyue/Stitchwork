use axum::{routing::get, Router};
use rig::providers::openai;
use sea_orm::Database;
use std::collections::HashMap;
use std::sync::Arc;
use stitchwork_server::{
    chat::session_manager::SessionManager, init_super_admin, s3::S3Config, service, AppState,
    Notifier, S3Client,
};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();
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

    // 初始化超级管理员
    if let Err(e) = init_super_admin(&db).await {
        tracing::error!("Failed to init super admin: {}", e);
    }

    // S3 客户端（可选）
    let s3 = S3Config::from_env().map(|c| S3Client::new(&c));
    if s3.is_some() {
        tracing::info!("S3 client initialized");
    }

    let rig_client = openai::Client::builder()
        // .http_client(client)
        .base_url(&std::env::var("RIG_BASE_URL").expect("RIG_BASE_URL must be set"))
        .api_key(&std::env::var("RIG_API_KEY").expect("RIG_API_KEY must be set"))
        .build()
        .unwrap();

    // let embedding_model = rig_client.embedding_model(openai::TEXT_EMBEDDING_3_SMALL);

    // let knowledge_base = KnowledgeBase::new("./docs", embedding_model).await.unwrap();

    let state = Arc::new(AppState {
        db: db.clone(),
        invite_codes: Arc::new(RwLock::new(HashMap::new())),
        s3,
        notifier: Arc::new(Notifier::new()),
        rig_client: Arc::new(rig_client),
        session_manager: SessionManager::default(), // knowledge_base,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 注意: 限流已移至 Pingora 代理层

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(service::routes())
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
