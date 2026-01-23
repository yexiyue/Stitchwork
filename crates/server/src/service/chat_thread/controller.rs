use axum::extract::{Query, State};
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::service;
use crate::AppState;
use crate::common::{ApiResponse, ListData, QueryParams};
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::service::chat_thread::dto::UpdateThreadDto;
use entity::chat_message::Model as MessageModel;
use entity::chat_thread::Model as ThreadModel;

#[derive(Debug, TypedPath)]
#[typed_path("/threads")]
pub struct ThreadsPath;

#[derive(Debug, TypedPath, Deserialize)]
#[typed_path("/threads/{id}")]
pub struct ThreadPath {
    id: Uuid,
}

#[derive(Debug, TypedPath, Deserialize)]
#[typed_path("/threads/{thread_id}/messages")]
pub struct MessagesPath {
    thread_id: Uuid,
}

async fn list(
    _: ThreadsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<QueryParams>,
) -> Result<ApiResponse<ListData<ThreadModel>>> {
    Ok(ApiResponse::ok(
        service::list(&state.db, params, claims.sub).await?,
    ))
}

async fn create(
    _: ThreadsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<ThreadModel>> {
    Ok(ApiResponse::ok(
        service::create(&state.db, claims.sub).await?,
    ))
}

async fn get_one(
    ThreadPath { id }: ThreadPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<ThreadModel>> {
    Ok(ApiResponse::ok(
        service::get_one(&state.db, id, claims.sub).await?,
    ))
}
async fn update(
    ThreadPath { id }: ThreadPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<UpdateThreadDto>,
) -> Result<ApiResponse<ThreadModel>> {
    Ok(ApiResponse::ok(
        service::update(&state.db, id, claims.sub, dto).await?,
    ))
}

async fn delete(
    ThreadPath { id }: ThreadPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<()>> {
    service::delete(&state.db, id, claims.sub).await?;
    Ok(ApiResponse::ok(()))
}

async fn list_messages(
    MessagesPath { thread_id }: MessagesPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Vec<MessageModel>>> {
    Ok(ApiResponse::ok(
        service::list_messages(&state.db, thread_id, claims.sub).await?,
    ))
}

async fn add_message(
    MessagesPath { thread_id }: MessagesPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(message): AppJson<serde_json::Value>,
) -> Result<ApiResponse<MessageModel>> {
    Ok(ApiResponse::ok(
        service::add_message(&state.db, thread_id, claims.sub, message).await?,
    ))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(get_one)
        .typed_get(list)
        .typed_post(create)
        .typed_put(update)
        .typed_delete(delete)
        .typed_get(list_messages)
        .typed_post(add_message)
}
