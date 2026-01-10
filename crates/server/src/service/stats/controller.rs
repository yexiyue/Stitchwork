use axum::extract::{Query, State};
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{
    CustomerContributionList, CustomerSummaryList, DailyOrderStatsList, DailyStatsList,
    GroupStatsList, MonthlyOrderStatsList, OrderOverview, OrderProgressList, OrderStats,
    OrderStatsParams, WorkerProductionList, WorkerStatsParams,
};
use crate::common::ApiResponse;
use entity::user::Role;
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

#[derive(TypedPath)]
#[typed_path("/stats/orders/overview")]
pub struct OrderOverviewPath;

#[derive(TypedPath)]
#[typed_path("/stats/orders/monthly")]
pub struct MonthlyOrderStatsPath;

#[derive(TypedPath)]
#[typed_path("/stats/orders/by-customer")]
pub struct CustomerContributionPath;

#[derive(TypedPath)]
#[typed_path("/stats/orders/progress")]
pub struct OrderProgressPath;

#[derive(TypedPath)]
#[typed_path("/stats/orders/daily")]
pub struct DailyOrderStatsPath;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(order_stats)
        .typed_get(customer_stats)
        .typed_get(worker_stats)
        .typed_get(daily_stats)
        .typed_get(stats_by_order)
        .typed_get(stats_by_process)
        .typed_get(order_overview)
        .typed_get(monthly_order_stats)
        .typed_get(customer_contribution)
        .typed_get(order_progress)
        .typed_get(daily_order_stats)
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

// ============ Order Stats ============

async fn order_overview(
    _: OrderOverviewPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<OrderStatsParams>,
) -> Result<ApiResponse<OrderOverview>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::order_overview(&state.db, claims.sub, params).await?,
    ))
}

async fn monthly_order_stats(
    _: MonthlyOrderStatsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<OrderStatsParams>,
) -> Result<ApiResponse<MonthlyOrderStatsList>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::monthly_order_stats(&state.db, claims.sub, params).await?,
    ))
}

async fn customer_contribution(
    _: CustomerContributionPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<OrderStatsParams>,
) -> Result<ApiResponse<CustomerContributionList>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::customer_contribution(&state.db, claims.sub, params).await?,
    ))
}

async fn order_progress(
    _: OrderProgressPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<OrderStatsParams>,
) -> Result<ApiResponse<OrderProgressList>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::order_progress(&state.db, claims.sub, params).await?,
    ))
}

async fn daily_order_stats(
    _: DailyOrderStatsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<OrderStatsParams>,
) -> Result<ApiResponse<DailyOrderStatsList>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::daily_order_stats(&state.db, claims.sub, params).await?,
    ))
}
