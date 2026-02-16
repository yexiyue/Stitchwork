use axum::{
    extract::{Path, Query, State},
    routing::{delete, post},
    Extension, Router,
};
use sea_orm::EntityTrait;
use std::sync::Arc;
use uuid::Uuid;

use crate::common::{ApiResponse, ListData};
use crate::error::{AppError, Result};
use crate::service::auth::Claims;
use crate::AppState;
use entity::user::{self, Role};

use super::dto::{RegisterCodeQueryParams, RegisterCodeResponse};
use super::service;

/// 检查用户是否有权操作注册码（超管或 Boss）。
/// 返回 (is_super_admin,) 供 service 层判断行为。
async fn check_permission(db: &sea_orm::DbConn, claims: &Claims) -> Result<bool> {
    let user = user::Entity::find_by_id(claims.sub)
        .one(db)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if user.is_super_admin {
        return Ok(true);
    }

    if claims.role == Role::Boss {
        return Ok(false);
    }

    Err(AppError::Forbidden)
}

async fn create_code(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<RegisterCodeResponse>> {
    let is_super_admin = check_permission(&state.db, &claims).await?;
    let result = service::create_register_code(&state.db, claims.sub, is_super_admin).await?;
    Ok(ApiResponse::ok(result))
}

async fn list_codes(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Query(params): Query<RegisterCodeQueryParams>,
) -> Result<ApiResponse<ListData<RegisterCodeResponse>>> {
    let is_super_admin = check_permission(&state.db, &claims).await?;
    let result =
        service::list_register_codes(&state.db, claims.sub, is_super_admin, params).await?;
    Ok(ApiResponse::ok(result))
}

async fn delete_code(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>> {
    let is_super_admin = check_permission(&state.db, &claims).await?;
    service::delete_register_code(&state.db, claims.sub, is_super_admin, id).await?;
    Ok(ApiResponse::ok(()))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register-codes", post(create_code).get(list_codes))
        .route("/register-codes/{id}", delete(delete_code))
}
