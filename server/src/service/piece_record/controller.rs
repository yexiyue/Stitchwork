use axum::extract::State;
use axum_extra::extract::Query;
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{BatchApproveDto, CreatePieceRecordDto, PieceRecordResponse, UpdatePieceRecordDto};
use crate::common::{ApiResponse, ListData, QueryParams};
use crate::entity::piece_record::Model;
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::AppState;

use super::service;

#[derive(TypedPath)]
#[typed_path("/piece-records")]
pub struct PieceRecordsPath;

#[derive(TypedPath, Deserialize)]
#[typed_path("/piece-records/{id}")]
pub struct PieceRecordPath {
    id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/piece-records/{id}/approve")]
pub struct PieceRecordApprovePath {
    id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/piece-records/{id}/reject")]
pub struct PieceRecordRejectPath {
    id: Uuid,
}

#[derive(TypedPath)]
#[typed_path("/piece-records/batch-approve")]
pub struct BatchApprovePath;

#[derive(TypedPath)]
#[typed_path("/piece-records/batch-reject")]
pub struct BatchRejectPath;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(list)
        .typed_post(create)
        .typed_get(get_one)
        .typed_put(update)
        .typed_delete(delete)
        .typed_post(approve)
        .typed_post(reject)
        .typed_post(batch_approve)
        .typed_post(batch_reject)
}

async fn list(
    _: PieceRecordsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<QueryParams>,
) -> Result<ApiResponse<ListData<PieceRecordResponse>>> {
    Ok(ApiResponse::ok(
        service::list(&state.db, params, &claims).await?,
    ))
}

async fn create(
    _: PieceRecordsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<CreatePieceRecordDto>,
) -> Result<ApiResponse<Model>> {
    Ok(ApiResponse::ok(
        service::create(&state.db, dto, &claims).await?,
    ))
}

async fn get_one(
    PieceRecordPath { id }: PieceRecordPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<PieceRecordResponse>> {
    Ok(ApiResponse::ok(
        service::get_one(&state.db, id, &claims).await?,
    ))
}

async fn update(
    PieceRecordPath { id }: PieceRecordPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<UpdatePieceRecordDto>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::update(&state.db, id, dto, claims.sub).await?,
    ))
}

async fn delete(
    PieceRecordPath { id }: PieceRecordPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<()>> {
    claims.require_boss()?;
    service::delete(&state.db, id, claims.sub).await?;
    Ok(ApiResponse::ok(()))
}

async fn approve(
    PieceRecordApprovePath { id }: PieceRecordApprovePath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::approve(&state.db, id, claims.sub).await?,
    ))
}

async fn reject(
    PieceRecordRejectPath { id }: PieceRecordRejectPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::reject(&state.db, id, claims.sub).await?,
    ))
}

async fn batch_approve(
    _: BatchApprovePath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<BatchApproveDto>,
) -> Result<ApiResponse<u64>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::batch_approve(&state.db, dto.ids, claims.sub).await?,
    ))
}

async fn batch_reject(
    _: BatchRejectPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<BatchApproveDto>,
) -> Result<ApiResponse<u64>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::batch_reject(&state.db, dto.ids, claims.sub).await?,
    ))
}
