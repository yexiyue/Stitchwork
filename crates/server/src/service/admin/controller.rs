use axum::{
    extract::{Query, State},
    routing::get,
    Extension, Router,
};
use sea_orm::EntityTrait;
use std::sync::Arc;
use uuid::Uuid;

use crate::common::{ApiResponse, ListData};
use crate::error::{AppError, Result};
use crate::service::auth::Claims;
use crate::AppState;
use entity::user;

use super::dto::{AdminQueryParams, AdminStats, UserListItem};
use super::service;

async fn require_super_admin(db: &sea_orm::DbConn, user_id: Uuid) -> Result<()> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !user.is_super_admin {
        return Err(AppError::Forbidden);
    }

    Ok(())
}

async fn get_stats(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<AdminStats>> {
    require_super_admin(&state.db, claims.sub).await?;
    let result = service::get_stats(&state.db).await?;
    Ok(ApiResponse::ok(result))
}

async fn list_users(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Query(params): Query<AdminQueryParams>,
) -> Result<ApiResponse<ListData<UserListItem>>> {
    require_super_admin(&state.db, claims.sub).await?;
    let result = service::list_users(&state.db, params).await?;
    Ok(ApiResponse::ok(result))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/stats", get(get_stats))
        .route("/admin/users", get(list_users))
}
