use axum::{
    extract::State,
    routing::{get, post, put},
    Extension, Router,
};
use std::sync::Arc;

use crate::common::ApiResponse;
use crate::error::{AppJson, Result};
use crate::AppState;

use super::dto::{
    ChangePasswordRequest, LoginRequest, LoginResponse, LoginUser, RegisterRequest,
    RegisterStaffRequest, UpdateProfileRequest,
};
use super::jwt::Claims;
use super::service;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/register-staff", post(register_staff))
}

pub fn protected_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/profile", get(get_profile).put(update_profile))
        .route("/password", put(change_password))
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
    let user_id = service::register(&state.db, &state.notifier, req).await?;
    Ok(ApiResponse::ok(serde_json::json!({ "userId": user_id })))
}

async fn register_staff(
    State(state): State<Arc<AppState>>,
    AppJson(req): AppJson<RegisterStaffRequest>,
) -> Result<ApiResponse<LoginResponse>> {
    let res = service::register_staff(&state.db, &state.invite_codes, &state.notifier, req).await?;
    Ok(ApiResponse::ok(res))
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

// 修改密码
async fn change_password(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(req): AppJson<ChangePasswordRequest>,
) -> Result<ApiResponse<()>> {
    service::change_password(&state.db, claims.sub, req).await?;
    Ok(ApiResponse::ok(()))
}
