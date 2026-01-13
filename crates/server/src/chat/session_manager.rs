use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;
use uuid::Uuid;

use crate::chat::ChatSession;

#[derive(Clone)]
pub struct SessionManager {
    sessions: Cache<Uuid, Arc<ChatSession>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Cache::builder()
                .time_to_idle(Duration::from_secs(30 * 60)) // 30 分钟无访问自动过期
                .build(),
        }
    }

    pub async fn get_or_try_insert<F, Fut>(&self, id: Uuid, init: F) -> Result<Arc<ChatSession>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<ChatSession>>,
    {
        self.sessions
            .try_get_with(id, async { Ok(Arc::new(init().await?)) })
            .await
            .map_err(|e: Arc<anyhow::Error>| anyhow::anyhow!("{}", e))
    }
}
