use axum::{
    extract::State,
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;

use crate::common::ApiResponse;
use crate::error::{AppJson, Result};
use crate::AppState;

use super::dto::{LoginRequest, LoginResponse, LoginUser, RegisterRequest, UpdateProfileRequest};
use super::jwt::Claims;
use super::service;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

pub fn protected_router() -> Router<Arc<AppState>> {
    Router::new().route("/profile", get(get_profile).put(update_profile))
}

async fn login(
    State(state): State<Arc<AppState>>,
    AppJson(req): AppJson<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>> {
    let res = service::login(&state.db, req).await?;
    Ok(ApiResponse::ok(res))
}

async fn register(
    State(state): State<Arc<AppState>>,
    AppJson(req): AppJson<RegisterRequest>,
) -> Result<ApiResponse<serde_json::Value>> {
    let user_id = service::register(&state.db, req).await?;
    Ok(ApiResponse::ok(serde_json::json!({ "userId": user_id })))
}

// 更新个人信息
async fn update_profile(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(req): AppJson<UpdateProfileRequest>,
) -> Result<ApiResponse<()>> {
    service::update_profile(&state.db, claims.sub, req).await?;
    Ok(ApiResponse::ok(()))
}

// 获取个人信息
async fn get_profile(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<LoginUser>> {
    let user = service::get_profile(&state.db, claims.sub).await?;
    Ok(ApiResponse::ok(user))
}
