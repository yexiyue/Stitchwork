use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Extension, Router,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::ApiResponse;
use crate::entity::share;
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::AppState;

use super::dto::{CreateShareRequest, PublicShareResponse, UpdateShareRequest};
use super::service;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/shares", post(create))
        .route("/shares", get(list))
        .route("/shares/{id}", put(update))
        .route("/shares/{id}", delete(delete_share))
}

pub fn public_router() -> Router<Arc<AppState>> {
    Router::new().route("/public/share/{token}", get(get_public))
}

async fn create(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(req): AppJson<CreateShareRequest>,
) -> Result<ApiResponse<share::Model>> {
    claims.require_boss()?;
    let share = service::create(&state.db, claims.sub, req).await?;
    Ok(ApiResponse::ok(share))
}

async fn list(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Vec<share::Model>>> {
    claims.require_boss()?;
    let shares = service::list(&state.db, claims.sub).await?;
    Ok(ApiResponse::ok(shares))
}

async fn update(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    AppJson(req): AppJson<UpdateShareRequest>,
) -> Result<ApiResponse<share::Model>> {
    claims.require_boss()?;
    let share = service::update(&state.db, claims.sub, id, req).await?;
    Ok(ApiResponse::ok(share))
}

async fn delete_share(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>> {
    claims.require_boss()?;
    service::delete(&state.db, claims.sub, id).await?;
    Ok(ApiResponse::ok(()))
}

async fn get_public(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Result<ApiResponse<PublicShareResponse>> {
    let res = service::get_public(&state.db, &token).await?;
    Ok(ApiResponse::ok(res))
}
