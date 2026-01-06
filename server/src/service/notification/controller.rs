use axum::{
    extract::{Query, State},
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
    token: String,
}

/// SSE 事件流端点
/// GET /api/sse/events?token=<jwt>
async fn sse_events(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SseQuery>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    // 验证 token
    let claims = verify_token(&query.token).map_err(|_| AppError::Unauthorized)?;
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
