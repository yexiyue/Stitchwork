use axum::{extract::State, routing::{post, put}, Extension, Router};
use std::sync::Arc;

use crate::common::ApiResponse;
use crate::error::{AppJson, Result};
use crate::AppState;

use super::dto::{
    BindBossRequest, CreateStaffRequest, InviteCodeResponse, LoginRequest, LoginResponse,
    RegisterRequest, UpdateProfileRequest,
};
use super::jwt::Claims;
use super::service;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

pub fn protected_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/staff", post(create_staff))
        .route("/invite-code", post(generate_invite_code))
        .route("/bind-boss", post(bind_boss))
        .route("/profile", put(update_profile))
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

// 老板创建员工
async fn create_staff(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(req): AppJson<CreateStaffRequest>,
) -> Result<ApiResponse<serde_json::Value>> {
    claims.require_boss()?;
    let staff_id = service::create_staff(&state.db, claims.sub, req).await?;
    Ok(ApiResponse::ok(serde_json::json!({ "staffId": staff_id })))
}

// 老板生成邀请码
async fn generate_invite_code(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<InviteCodeResponse>> {
    claims.require_boss()?;
    let res = service::generate_invite_code(&state.invite_codes, claims.sub).await;
    Ok(ApiResponse::ok(res))
}

// 员工绑定老板
async fn bind_boss(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(req): AppJson<BindBossRequest>,
) -> Result<ApiResponse<()>> {
    service::bind_boss(&state.db, &state.invite_codes, claims.sub, req).await?;
    Ok(ApiResponse::ok(()))
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
