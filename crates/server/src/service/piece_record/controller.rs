use axum::extract::State;
use axum_extra::extract::Query;
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{BatchApproveDto, CreatePieceRecordDto, PieceRecordResponse, UpdatePieceRecordDto};
use crate::common::{ApiResponse, ListData, QueryParams};
use entity::piece_record::Model;
use entity::user::Role;
use entity::workshop;
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::service::notification::Notification;
use crate::AppState;

use super::service;
use sea_orm::DbConn;

/// 获取工坊的计件单位
async fn get_piece_unit(db: &DbConn, boss_id: Uuid) -> String {
    workshop::Entity::find()
        .filter(workshop::Column::OwnerId.eq(boss_id))
        .one(db)
        .await
        .ok()
        .flatten()
        .map(|w| w.piece_unit)
        .unwrap_or_else(|| "件".to_string())
}

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
    let record = service::create(&state.db, dto.clone(), &claims).await?;

    // 员工提交计件时通知老板
    if claims.role == Role::Staff {
        let response = service::get_one(&state.db, record.id, &claims).await?;
        let unit = get_piece_unit(&state.db, record.boss_id).await;
        state.notifier.send(
            record.boss_id,
            Notification::RecordSubmitted {
                user_name: response.user_name.unwrap_or_default(),
                process_name: response.process_name.unwrap_or_default(),
                quantity: record.quantity,
                unit,
            },
        );
    }

    Ok(ApiResponse::ok(record))
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

    // 先获取记录详情用于通知
    let response = service::get_one(&state.db, id, &claims).await?;
    let record = service::approve(&state.db, id, claims.sub).await?;

    // 通知员工审批通过
    let unit = get_piece_unit(&state.db, record.boss_id).await;
    state.notifier.send(
        record.user_id,
        Notification::RecordApproved {
            process_name: response.process_name.unwrap_or_default(),
            quantity: record.quantity,
            unit,
            amount: record.amount.to_string(),
        },
    );

    Ok(ApiResponse::ok(record))
}

async fn reject(
    PieceRecordRejectPath { id }: PieceRecordRejectPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;

    // 先获取记录详情用于通知
    let response = service::get_one(&state.db, id, &claims).await?;
    let record = service::reject(&state.db, id, claims.sub).await?;

    // 通知员工审批拒绝
    let unit = get_piece_unit(&state.db, record.boss_id).await;
    state.notifier.send(
        record.user_id,
        Notification::RecordRejected {
            process_name: response.process_name.unwrap_or_default(),
            quantity: record.quantity,
            unit,
        },
    );

    Ok(ApiResponse::ok(record))
}

async fn batch_approve(
    _: BatchApprovePath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<BatchApproveDto>,
) -> Result<ApiResponse<u64>> {
    claims.require_boss()?;

    // 先获取待处理记录用于通知
    let pending_records = service::get_pending_records(&state.db, &dto.ids, claims.sub).await?;

    let count = service::batch_approve(&state.db, dto.ids, claims.sub).await?;

    // 发送通知给每个员工
    let unit = get_piece_unit(&state.db, claims.sub).await;
    for record in pending_records {
        state.notifier.send(
            record.user_id,
            Notification::RecordApproved {
                process_name: record.process_name.unwrap_or_default(),
                quantity: record.quantity,
                unit: unit.clone(),
                amount: record.amount.to_string(),
            },
        );
    }

    Ok(ApiResponse::ok(count))
}

async fn batch_reject(
    _: BatchRejectPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<BatchApproveDto>,
) -> Result<ApiResponse<u64>> {
    claims.require_boss()?;

    // 先获取待处理记录用于通知
    let pending_records = service::get_pending_records(&state.db, &dto.ids, claims.sub).await?;

    let count = service::batch_reject(&state.db, dto.ids, claims.sub).await?;

    // 发送通知给每个员工
    let unit = get_piece_unit(&state.db, claims.sub).await;
    for record in pending_records {
        state.notifier.send(
            record.user_id,
            Notification::RecordRejected {
                process_name: record.process_name.unwrap_or_default(),
                quantity: record.quantity,
                unit: unit.clone(),
            },
        );
    }

    Ok(ApiResponse::ok(count))
}
