use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Extension, Json, Router,
};
use sea_orm::EntityTrait;
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::user;
use crate::error::{AppError, Result};
use crate::service::auth::Claims;
use crate::AppState;

use super::dto::{RegisterCodeResponse, UserListItem};
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

async fn create_register_code(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RegisterCodeResponse>> {
    require_super_admin(&state.db, claims.sub).await?;
    let result = service::create_register_code(&state.db).await?;
    Ok(Json(result))
}

async fn list_register_codes(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RegisterCodeResponse>>> {
    require_super_admin(&state.db, claims.sub).await?;
    let result = service::list_register_codes(&state.db).await?;
    Ok(Json(result))
}

async fn disable_register_code(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<()> {
    require_super_admin(&state.db, claims.sub).await?;
    service::disable_register_code(&state.db, id).await
}

async fn list_users(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<UserListItem>>> {
    require_super_admin(&state.db, claims.sub).await?;
    let result = service::list_users(&state.db).await?;
    Ok(Json(result))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/register-codes", post(create_register_code))
        .route("/admin/register-codes", get(list_register_codes))
        .route("/admin/register-codes/{id}", delete(disable_register_code))
        .route("/admin/users", get(list_users))
}
