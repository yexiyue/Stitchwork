use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    http::header,
    response::{IntoResponse, Sse, sse::KeepAlive},
};

use axum_extra::routing::{RouterExt, TypedPath};

use crate::{
    AppState,
    chat::{AISdkChatRequest, ChatSession},
    error::Result,
    service::auth::Claims,
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
        .session_id
        .and_then(|s| uuid::Uuid::parse_str(&s).ok())
        .unwrap_or_else(uuid::Uuid::new_v4);

    let chat_session = app_state
        .session_manager
        .get_or_try_insert(session_id, || {
            ChatSession::new(
                &app_state.db,
                &app_state.rig_client,
                claims,
                req.tools.clone(),
            )
        })
        .await?;

    let (prompt, history) = rig_ai_sdk::extract_prompt_and_history(&req.messages)?;
    let stream = chat_session.chat(prompt, req.tools, history).await;

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
