use axum::extract::{Query, State};
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{CreatePayrollDto, UpdatePayrollDto};
use crate::common::{ApiResponse, ListData, QueryParams};
use crate::entity::payroll::Model;
use crate::entity::user::Role;
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::AppState;

use super::service;

#[derive(TypedPath)]
#[typed_path("/payrolls")]
pub struct PayrollsPath;

#[derive(TypedPath, Deserialize)]
#[typed_path("/payrolls/{id}")]
pub struct PayrollPath {
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
    _: PayrollsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<QueryParams>,
) -> Result<ApiResponse<ListData<Model>>> {
    let user_id = if claims.role == Role::Staff { Some(claims.sub) } else { None };
    Ok(ApiResponse::ok(service::list(&state.db, params, user_id).await?))
}

async fn create(
    _: PayrollsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<CreatePayrollDto>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(service::create(&state.db, dto).await?))
}

async fn get_one(
    PayrollPath { id }: PayrollPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Model>> {
    let user_id = if claims.role == Role::Staff { Some(claims.sub) } else { None };
    Ok(ApiResponse::ok(service::get_one(&state.db, id, user_id).await?))
}

async fn update(
    PayrollPath { id }: PayrollPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<UpdatePayrollDto>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(service::update(&state.db, id, dto).await?))
}

async fn delete(
    PayrollPath { id }: PayrollPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<()>> {
    claims.require_boss()?;
    service::delete(&state.db, id).await?;
    Ok(ApiResponse::ok(()))
}
