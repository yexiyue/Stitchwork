use std::sync::Arc;

use axum::{http::request::Parts, Router};
use rmcp::{
    handler::server::{common::FromContextPart, tool::ToolCallContext},
    model::ErrorData,
    transport::{
        streamable_http_server::session::local::LocalSessionManager, StreamableHttpService,
    },
};
use sea_orm::DbConn;

use boss::BossMcp;
use staff::StaffMcp;

use crate::{error::AppError, service::auth, AppState};

pub mod boss;
pub mod staff;

/// 自定义 Claims extractor，可直接在 MCP 工具中使用
pub struct AuthClaims(pub auth::Claims);

impl<S> FromContextPart<ToolCallContext<'_, S>> for AuthClaims {
    fn from_context_part(context: &mut ToolCallContext<'_, S>) -> Result<Self, ErrorData> {
        // 从 Extensions 中获取 HTTP Parts
        let parts = context
            .request_context
            .extensions
            .get::<Parts>()
            .ok_or_else(|| AppError::Unauthorized)?;

        // 从 Parts 中提取 Claims
        let claims =
            auth::extract_claims_from_parts(parts).map_err(|e| -> ErrorData { e.into() })?;
        Ok(AuthClaims(claims))
    }
}

pub fn mcp_router(db: DbConn) -> Router<Arc<AppState>> {
    let database = db.clone();
    let boss_mcp_service = StreamableHttpService::new(
        move || Ok(BossMcp::new(database.clone())),
        LocalSessionManager::default().into(),
        Default::default(),
    );
    let staff_mcp_service = StreamableHttpService::new(
        move || Ok(StaffMcp::new(db.clone())),
        LocalSessionManager::default().into(),
        Default::default(),
    );
    Router::new()
        .nest_service("/mcp/boss", boss_mcp_service)
        .nest_service("/mcp/staff", staff_mcp_service)
}
