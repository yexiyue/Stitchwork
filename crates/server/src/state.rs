//! 应用状态定义

use std::collections::HashMap;
use std::sync::Arc;

use sea_orm::DbConn;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::chat::session_manager::SessionManager;
use crate::s3::S3Client;
use crate::service::notification::Notifier;

pub use rig::providers::anthropic;

/// 邀请码存储: code -> (workshop_id, expires_at)
pub type InviteCodes = RwLock<HashMap<String, (Uuid, i64)>>;

/// 应用状态，通过 Arc 共享，内部字段无需额外 Arc 包装
pub struct AppState {
    pub db: DbConn,
    pub invite_codes: InviteCodes,
    pub s3: Option<S3Client>,
    pub notifier: Notifier,
    pub rig_client: Arc<anthropic::Client>,
    pub session_manager: SessionManager,
}

impl AppState {
    pub fn new(
        db: DbConn,
        s3: Option<S3Client>,
        rig_client: anthropic::Client,
    ) -> Arc<Self> {
        Arc::new(Self {
            db,
            invite_codes: RwLock::new(HashMap::new()),
            s3,
            notifier: Notifier::new(),
            rig_client: Arc::new(rig_client),
            session_manager: SessionManager::default(),
        })
    }
}
