use axum::extract::{Query, State};
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{CustomerSummaryList, OrderStats, WorkerProductionList, WorkerStatsParams};
use crate::common::ApiResponse;
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

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(order_stats)
        .typed_get(customer_stats)
        .typed_get(worker_stats)
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
