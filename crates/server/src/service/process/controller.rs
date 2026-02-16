use axum::extract::{Query, State};
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{CreateProcessDto, ProcessQueryParams, UpdateProcessDto};
use crate::common::{ApiResponse, ListData, QueryParams};
use entity::process::Model;
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::AppState;

use super::service;

#[derive(TypedPath)]
#[typed_path("/processes")]
pub struct ProcessesPath;

#[derive(TypedPath, Deserialize)]
#[typed_path("/processes/{id}")]
pub struct ProcessPath {
    id: Uuid,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(list)
        .typed_post(create)
        .typed_get(get_one)
        .typed_put(update)
        .typed_delete(delete)
}

async fn list(
    _: ProcessesPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<QueryParams>,
    Query(filter): Query<ProcessQueryParams>,
) -> Result<ApiResponse<ListData<Model>>> {
    Ok(ApiResponse::ok(
        service::list(&state.db, params, filter, &claims).await?,
    ))
}

async fn create(
    _: ProcessesPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<CreateProcessDto>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(service::create(&state.db, dto).await?))
}

async fn get_one(
    ProcessPath { id }: ProcessPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Model>> {
    Ok(ApiResponse::ok(
        service::get_one(&state.db, id, &claims).await?,
    ))
}

async fn update(
    ProcessPath { id }: ProcessPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<UpdateProcessDto>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::update(&state.db, id, dto, claims.sub).await?,
    ))
}

async fn delete(
    ProcessPath { id }: ProcessPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<()>> {
    claims.require_boss()?;
    service::delete(&state.db, id, claims.sub).await?;
    Ok(ApiResponse::ok(()))
}
