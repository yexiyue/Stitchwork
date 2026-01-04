use axum::extract::{Query, State};
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{
    CustomerSummaryList, DailyStatsList, GroupStatsList, OrderStats, WorkerProductionList,
    WorkerStatsParams,
};
use crate::common::ApiResponse;
use crate::entity::user::Role;
use crate::error::Result;
use crate::service::auth::Claims;
use crate::AppState;

use super::service;

#[derive(TypedPath, Deserialize)]
#[typed_path("/orders/{id}/stats")]
pub struct OrderStatsPath {
    id: Uuid,
}

#[derive(TypedPath)]
#[typed_path("/stats/customers")]
pub struct CustomerStatsPath;

#[derive(TypedPath)]
#[typed_path("/stats/workers")]
pub struct WorkerStatsPath;

#[derive(TypedPath)]
#[typed_path("/stats/daily")]
pub struct DailyStatsPath;

#[derive(TypedPath)]
#[typed_path("/stats/by-order")]
pub struct StatsByOrderPath;

#[derive(TypedPath)]
#[typed_path("/stats/by-process")]
pub struct StatsByProcessPath;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(order_stats)
        .typed_get(customer_stats)
        .typed_get(worker_stats)
        .typed_get(daily_stats)
        .typed_get(stats_by_order)
        .typed_get(stats_by_process)
}

async fn order_stats(
    OrderStatsPath { id }: OrderStatsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<OrderStats>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::order_stats(&state.db, id, claims.sub).await?,
    ))
}

async fn customer_stats(
    _: CustomerStatsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<CustomerSummaryList>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::customer_summary(&state.db, claims.sub).await?,
    ))
}

async fn worker_stats(
    _: WorkerStatsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<WorkerStatsParams>,
) -> Result<ApiResponse<WorkerProductionList>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::worker_production(&state.db, claims.sub, params).await?,
    ))
}

async fn daily_stats(
    _: DailyStatsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<WorkerStatsParams>,
) -> Result<ApiResponse<DailyStatsList>> {
    // For staff, filter by their own user_id; for boss, filter by boss_id
    let (boss_id, user_id) = if claims.role == Role::Staff {
        (None, Some(claims.sub))
    } else {
        (Some(claims.sub), None)
    };
    Ok(ApiResponse::ok(
        service::daily_stats(&state.db, boss_id, user_id, params).await?,
    ))
}

async fn stats_by_order(
    _: StatsByOrderPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<WorkerStatsParams>,
) -> Result<ApiResponse<GroupStatsList>> {
    let (boss_id, user_id) = if claims.role == Role::Staff {
        (None, Some(claims.sub))
    } else {
        (Some(claims.sub), None)
    };
    Ok(ApiResponse::ok(
        service::stats_by_order(&state.db, boss_id, user_id, params).await?,
    ))
}

async fn stats_by_process(
    _: StatsByProcessPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<WorkerStatsParams>,
) -> Result<ApiResponse<GroupStatsList>> {
    let (boss_id, user_id) = if claims.role == Role::Staff {
        (None, Some(claims.sub))
    } else {
        (Some(claims.sub), None)
    };
    Ok(ApiResponse::ok(
        service::stats_by_process(&state.db, boss_id, user_id, params).await?,
    ))
}
