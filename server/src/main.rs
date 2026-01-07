use axum::{routing::get, Router};
use sea_orm::{ActiveModelTrait, ColumnTrait, Database, DbConn, EntityTrait, QueryFilter, Set};
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

/// 初始化超级管理员（如果不存在）
async fn init_super_admin(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
    use entity::user::{self, Role};

    // 检查是否已有超管
    let existing = user::Entity::find()
        .filter(user::Column::IsSuperAdmin.eq(true))
        .one(db)
        .await?;

    if existing.is_some() {
        return Ok(());
    }

    // 从环境变量读取超管配置
    let username = match std::env::var("ADMIN_USERNAME") {
        Ok(v) if !v.is_empty() => v,
        _ => return Ok(()), // 未配置则跳过
    };
    let password = std::env::var("ADMIN_PASSWORD")
        .expect("ADMIN_PASSWORD must be set when ADMIN_USERNAME is configured");
    let phone = std::env::var("ADMIN_PHONE")
        .expect("ADMIN_PHONE must be set when ADMIN_USERNAME is configured");

    let password_hash = service::auth::hash_password(&password)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    let admin = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(username.clone()),
        password_hash: Set(password_hash),
        role: Set(Role::Boss), // 超管也是 Boss 角色
        display_name: Set(Some("超级管理员".to_string())),
        phone: Set(phone),
        avatar: Set(None),
        workshop_id: Set(None),
        is_super_admin: Set(true),
        created_at: Set(chrono::Utc::now()),
    };

    admin.insert(db).await?;
    tracing::info!("Super admin '{}' created", username);

    Ok(())
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

    // 初始化超级管理员
    if let Err(e) = init_super_admin(&db).await {
        tracing::error!("Failed to init super admin: {}", e);
    }

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
