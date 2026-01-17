use std::sync::Arc;

use axum::{
    extract::State,
    http::header,
    response::{sse::KeepAlive, IntoResponse, Sse},
    Extension, Json, Router,
};

use axum_extra::routing::{RouterExt, TypedPath};

use crate::{
    chat::{AISdkChatRequest, ChatSession},
    error::Result,
    service::auth::Claims,
    AppState,
};

#[derive(TypedPath)]
#[typed_path("/chat")]
pub struct ChatPath;

pub async fn chat(
    _: ChatPath,
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<AISdkChatRequest>,
) -> Result<impl IntoResponse> {
    let session_id = req
        .id
        .and_then(|s| uuid::Uuid::parse_str(&s).ok())
        .unwrap_or_else(uuid::Uuid::new_v4);

    let chat_session = app_state
        .session_manager
        .get_or_try_insert(session_id, || {
            ChatSession::new(&app_state.db, &app_state.rig_client, claims)
        })
        .await?;

    let (prompt, history) = rig_ai_sdk::extract_prompt_and_history(&req.messages)?;
    let stream = chat_session.chat(prompt, history).await;

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
