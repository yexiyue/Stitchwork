use std::sync::Arc;

use axum::{
    extract::State,
    http::header,
    response::{sse::KeepAlive, IntoResponse, Sse},
    Extension, Json, Router,
};

use axum_extra::routing::{RouterExt, TypedPath};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{chat::ChatSession, error::Result, service::auth::Claims, AppState};

#[derive(TypedPath)]
#[typed_path("/chat")]
pub struct ChatPath;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    message: String,
    id: Uuid,
}

pub async fn chat(
    _: ChatPath,
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<ChatRequest>,
) -> Result<impl IntoResponse> {
    let chat_session = app_state
        .session_manager
        .get_or_try_insert(req.id, || {
            ChatSession::new(&app_state.db, &app_state.rig_client, claims)
        })
        .await?;

    let stream = chat_session.chat(req.message.into(), vec![]).await;

    Ok((
        [(
            header::HeaderName::from_static("x-vercel-ai-ui-message-stream"),
            header::HeaderValue::from_static("v1"),
        )],
        Sse::new(stream).keep_alive(KeepAlive::default()),
    ))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().typed_post(chat).typed_get(chat)
}
