use std::{convert::Infallible, sync::Arc};

use anyhow::anyhow;
use axum::{
    Extension, Json, Router,
    extract::State,
    http::header,
    response::{
        IntoResponse, Sse,
        sse::{Event, KeepAlive},
    },
};
use axum_extra::routing::{RouterExt, TypedPath};
use futures::{Stream, StreamExt};
use rig::{
    client::CompletionClient,
    completion::{CompletionModel, CompletionRequestBuilder, GetTokenUsage},
    streaming::StreamedAssistantContent,
};
use rig_ai_sdk::UIMessage;
use sea_orm::DbConn;
use serde::Deserialize;

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

#[derive(TypedPath)]
#[typed_path("/chat/generate-title")]
pub struct GeneratedNamePath;

#[derive(Deserialize)]
pub struct GenerateTitleRequest {
    pub thread_id: String,
    pub messages: Vec<UIMessage>,
}

pub async fn generate_title(
    _: GeneratedNamePath,
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<GenerateTitleRequest>,
) -> Result<impl IntoResponse> {
    let thread_id =
        uuid::Uuid::parse_str(&req.thread_id).map_err(|_| anyhow!("Invalid thread_id"))?;

    let model = app_state.rig_client.completion_model("glm-4.7");
    let (prompt, history) = rig_ai_sdk::extract_prompt_and_history(&req.messages)?;

    let request = CompletionRequestBuilder::new(model.clone(), prompt)
        .preamble("你是一个标题生成助手。根据用户的消息内容，生成一个简短的标题（不超过20个字）。只输出标题，不要有任何其他内容。".into())
        .max_tokens(100)
        .messages(history)
        .build();

    // 使用流式 API
    let rig_stream = model.stream(request).await.map_err(|e| anyhow!(e))?;

    // 转换为 UI Message Stream Protocol 格式，并在完成后保存标题
    let stream = ui_message_stream_from_rig_with_save(
        rig_stream,
        app_state.db.clone(),
        thread_id,
        claims.sub,
    );

    Ok((
        [(header::CONTENT_TYPE, "text/event-stream; charset=utf-8")],
        Sse::new(stream).keep_alive(KeepAlive::default()),
    ))
}

#[derive(TypedPath)]
#[typed_path("/chat/suggestion")]
pub struct SuggestionPath;

#[derive(Debug, Deserialize)]
pub struct SuggestionRequest {
    messages: Vec<UIMessage>,
}

pub async fn suggestion(
    _: SuggestionPath,
    State(app_state): State<Arc<AppState>>,
    Extension(_): Extension<Claims>,
    Json(req): Json<SuggestionRequest>,
) -> Result<impl IntoResponse> {
    let model = app_state.rig_client.completion_model("glm-4.6");
    let (prompt, history) = rig_ai_sdk::extract_prompt_and_history(&req.messages)?;

    let request = CompletionRequestBuilder::new(model.clone(), prompt)
        .preamble(r#"你是一个智能助手，负责根据对话上下文生成用户可能想问的后续问题建议。

要求：
1. 生成 2-3 个简短的问题建议，每个不超过 20 字
2. 问题之间用换行分隔
3. 问题要与当前对话主题相关且有实际价值
4. 只输出问题列表，不要有任何其他内容

示例输出格式：
如何查看本月工资明细？
最近有哪些待处理的订单？
怎么录入今天的计件记录？"#.into())
        .max_tokens(100)
        .messages(history)
        .build();
    let mut rig_stream = model.stream(request).await.map_err(|e| anyhow!(e))?;

    let stream = async_stream::stream! {
        while let Some(res) = rig_stream.next().await {
            match res {
                Ok(StreamedAssistantContent::Text(text)) => {
                    yield Ok::<_, Infallible>(Event::default().data(text.text));
                }
                Err(e) => {
                    yield Ok(Event::default().data(format!("[ERROR] {}", e)));
                    break;
                }
                _ => {}
            }
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

/// 将 rig 流式响应转换为 UI Message Stream Protocol 格式，并在完成后保存标题
/// 格式: data: {json}\n\n
/// 结束: data: [DONE]\n\n
fn ui_message_stream_from_rig_with_save<R>(
    rig_stream: rig::streaming::StreamingCompletionResponse<R>,
    db: DbConn,
    thread_id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> impl Stream<Item = std::result::Result<Event, Infallible>>
where
    R: Clone + Unpin + GetTokenUsage + Send + 'static,
{
    async_stream::stream! {
        let mut stream = rig_stream;
        let message_id = uuid::Uuid::new_v4().to_string();
        let mut title_text = String::new();

        // 发送 start 事件
        let start = serde_json::json!({ "type": "start", "messageId": message_id });
        yield Ok(Event::default().data(start.to_string()));

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(StreamedAssistantContent::Text(text)) => {
                    // 累积标题文本
                    title_text.push_str(&text.text);

                    // UI Message Stream: text-delta
                    let data = serde_json::json!({
                        "type": "text-delta",
                        "textDelta": text.text
                    });
                    yield Ok(Event::default().data(data.to_string()));
                }
                Ok(StreamedAssistantContent::Final(_)) => {
                    // 保存标题到数据库
                    let title = title_text.trim().to_string();
                    if !title.is_empty() {
                        let update_dto = super::chat_thread::dto::UpdateThreadDto {
                            title: Some(title),
                            archived: None,
                        };
                        if let Err(e) = super::chat_thread::service::update(&db, thread_id, user_id, update_dto).await {
                            tracing::error!("Failed to save title: {}", e);
                        }
                    }

                    // UI Message Stream: finish
                    let data = serde_json::json!({
                        "type": "finish",
                        "finishReason": "stop",
                        "usage": { "promptTokens": 0, "completionTokens": 0 }
                    });
                    yield Ok(Event::default().data(data.to_string()));
                }
                Ok(_) => {
                    // 忽略其他类型（ToolCall, Reasoning 等）
                }
                Err(e) => {
                    // UI Message Stream: error
                    let data = serde_json::json!({
                        "type": "error",
                        "errorText": e.to_string()
                    });
                    yield Ok(Event::default().data(data.to_string()));
                }
            }
        }

        // 发送 [DONE] 标记
        yield Ok(Event::default().data("[DONE]"));
    }
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_post(chat)
        .typed_post(generate_title)
        .typed_post(suggestion)
}
