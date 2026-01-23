use axum::{routing::get, Router};
use sea_orm::Database;
use stitchwork_server::{
    anthropic, init_super_admin, s3::S3Config, service, AppState, S3Client,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();
    dotenvy::dotenv().ok();

    // 数据库连接
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = sea_orm::ConnectOptions::new(&database_url);
    opt.sqlx_logging(true);
    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to database");

    // Entity First: 同步数据库 schema
    db.get_schema_registry("entity::*")
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

    // AI 客户端
    let rig_client = anthropic::Client::builder()
        .base_url(&std::env::var("RIG_BASE_URL").expect("RIG_BASE_URL must be set"))
        .api_key(&std::env::var("RIG_API_KEY").expect("RIG_API_KEY must be set"))
        .build()
        .unwrap();

    // 应用状态
    let state = AppState::new(db, s3, rig_client);

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 路由
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(service::routes())
        .layer(cors)
        .with_state(state);

    // 启动服务
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
