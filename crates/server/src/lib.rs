use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod chat;
pub mod common;
pub mod error;
pub mod mcp;
pub mod s3;
pub mod service;
pub mod traits;

pub use rig::providers::anthropic;
pub use s3::S3Client;
pub use service::notification::{Notification, Notifier, SharedNotifier};

use crate::chat::session_manager::SessionManager;

/// 邀请码存储: code -> (workshop_id, expires_at)
pub type InviteCodes = RwLock<HashMap<String, (Uuid, i64)>>;

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
    pub invite_codes: Arc<InviteCodes>,
    pub s3: Option<S3Client>,
    pub notifier: SharedNotifier,
    pub rig_client: Arc<anthropic::Client>,
    pub session_manager: SessionManager,
    // pub knowledge_base: chat::knowledge_base::KnowledgeBase,
}

/// 初始化超级管理员（如果不存在）
pub async fn init_super_admin(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
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
