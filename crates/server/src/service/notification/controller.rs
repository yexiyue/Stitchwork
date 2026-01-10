use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Router,
};
use futures::stream::Stream;
use serde::Deserialize;
use std::{convert::Infallible, sync::Arc, time::Duration};

use crate::{error::AppError, service::auth::verify_token, AppState};

use super::Notification;

#[derive(Debug, Deserialize)]
pub struct SseQuery {
    token: Option<String>,
}

/// SSE 事件流端点
/// GET /api/sse/events
/// 认证方式（优先级从高到低）：
/// 1. Authorization: Bearer <token>
/// 2. ?token=<jwt>
async fn sse_events(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<SseQuery>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    // 优先从 Authorization header 获取 token
    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .or(query.token)
        .ok_or(AppError::Unauthorized)?;

    // 验证 token
    let claims = verify_token(&token).map_err(|_| AppError::Unauthorized)?;
    let user_id = claims.sub;

    // 订阅通知
    let mut rx = state.notifier.subscribe(user_id);

    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(notification) => {
                    if let Ok(json) = serde_json::to_string(&NotificationPayload::from(notification)) {
                        yield Ok(Event::default().data(json));
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    // 消息积压，跳过
                    continue;
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    };

    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("ping"),
    ))
}

/// 发送给前端的通知 payload
#[derive(serde::Serialize)]
struct NotificationPayload {
    #[serde(flatten)]
    notification: Notification,
    title: String,
    body: String,
}

impl From<Notification> for NotificationPayload {
    fn from(notification: Notification) -> Self {
        let title = notification.title().to_string();
        let body = notification.body();
        Self {
            notification,
            title,
            body,
        }
    }
}

/// SSE 路由 - 不需要认证中间件（token 通过 query 验证）
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/sse/events", get(sse_events))
}
