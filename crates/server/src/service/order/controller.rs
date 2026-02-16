use axum::extract::State;
use axum::{Extension, Router};
use axum_extra::extract::Query;
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::dto::{CreateOrderDto, OrderQueryParams, OrderResponse, UpdateOrderDto, UpdateOrderStatusDto};
use crate::common::{ApiResponse, ListData, QueryParams};
use entity::order::Model;
use entity::user::Role;
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::AppState;

use super::service;

#[derive(TypedPath)]
#[typed_path("/orders")]
pub struct OrdersPath;

#[derive(TypedPath, Deserialize)]
#[typed_path("/orders/{id}")]
pub struct OrderPath {
    id: Uuid,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/orders/{id}/status")]
pub struct OrderStatusPath {
    id: Uuid,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(list)
        .typed_post(create)
        .typed_get(get_one)
        .typed_put(update)
        .typed_delete(delete)
        .typed_patch(update_status)
}

async fn list(
    _: OrdersPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<QueryParams>,
    Query(filter): Query<OrderQueryParams>,
) -> Result<ApiResponse<ListData<OrderResponse>>> {
    let data = service::list(&state.db, params, filter, &claims).await?;
    let hide_price = claims.role == Role::Staff;
    Ok(ApiResponse::ok(ListData {
        list: data.list.into_iter().map(|m| OrderResponse::from_model(m, hide_price)).collect(),
        total: data.total,
    }))
}

async fn create(
    _: OrdersPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<CreateOrderDto>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(service::create(&state.db, dto).await?))
}

async fn get_one(
    OrderPath { id }: OrderPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<OrderResponse>> {
    let model = service::get_one(&state.db, id, &claims).await?;
    let hide_price = claims.role == Role::Staff;
    Ok(ApiResponse::ok(OrderResponse::from_model(model, hide_price)))
}

async fn update(
    OrderPath { id }: OrderPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<UpdateOrderDto>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::update(&state.db, id, dto, claims.sub).await?,
    ))
}

async fn delete(
    OrderPath { id }: OrderPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<()>> {
    claims.require_boss()?;
    service::delete(&state.db, id, claims.sub).await?;
    Ok(ApiResponse::ok(()))
}

async fn update_status(
    OrderStatusPath { id }: OrderStatusPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<UpdateOrderStatusDto>,
) -> Result<ApiResponse<Model>> {
    claims.require_boss()?;
    Ok(ApiResponse::ok(
        service::update_status(&state.db, id, dto, claims.sub).await?,
    ))
}
