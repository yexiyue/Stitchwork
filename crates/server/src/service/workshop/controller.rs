use axum::{
    extract::{Path, Query, State},
    routing::{delete, get},
    Extension, Router,
};
use std::sync::Arc;

use crate::common::{ApiResponse, ListData, QueryParams};
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::AppState;

use super::dto::{
    CreateWorkshopRequest, StaffResponse, UpdateWorkshopRequest, WorkshopResponse,
};
use super::service;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/workshop",
            get(get_workshop).post(create_workshop).put(update_workshop),
        )
        .route("/staff", get(get_staff_list))
        .route("/staff/{id}", delete(remove_staff))
}

async fn get_workshop(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Option<WorkshopResponse>>> {
    claims.require_boss()?;
    let ws = service::get_workshop(&state.db, claims.sub).await?;
    Ok(ApiResponse::ok(ws))
}

async fn create_workshop(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(req): AppJson<CreateWorkshopRequest>,
) -> Result<ApiResponse<WorkshopResponse>> {
    claims.require_boss()?;
    let ws = service::create_workshop(&state.db, claims.sub, req).await?;
    Ok(ApiResponse::ok(ws))
}

async fn update_workshop(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(req): AppJson<UpdateWorkshopRequest>,
) -> Result<ApiResponse<WorkshopResponse>> {
    claims.require_boss()?;
    let ws = service::update_workshop(&state.db, claims.sub, req).await?;
    Ok(ApiResponse::ok(ws))
}

async fn get_staff_list(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<QueryParams>,
) -> Result<ApiResponse<ListData<StaffResponse>>> {
    claims.require_boss()?;
    let data = service::get_staff_list(&state.db, claims.sub, params).await?;
    Ok(ApiResponse::ok(data))
}

async fn remove_staff(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Path(staff_id): Path<uuid::Uuid>,
) -> Result<ApiResponse<()>> {
    claims.require_boss()?;
    service::remove_staff(&state.db, claims.sub, staff_id).await?;
    Ok(ApiResponse::ok(()))
}
