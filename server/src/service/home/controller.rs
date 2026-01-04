use axum::extract::State;
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Serialize;
use std::sync::Arc;

use super::dto::{ActivityList, BossOverview, StaffOverview};
use crate::common::ApiResponse;
use crate::entity::user::Role;
use crate::error::Result;
use crate::service::auth::Claims;
use crate::AppState;

use super::service;

#[derive(TypedPath)]
#[typed_path("/home/overview")]
pub struct OverviewPath;

#[derive(TypedPath)]
#[typed_path("/home/activities")]
pub struct ActivitiesPath;

/// Overview response - can be boss or staff
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum OverviewResponse {
    Boss(BossOverview),
    Staff(StaffOverview),
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(overview)
        .typed_get(activities)
}

async fn overview(
    _: OverviewPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<OverviewResponse>> {
    let response = if claims.role == Role::Boss {
        OverviewResponse::Boss(service::boss_overview(&state.db, claims.sub).await?)
    } else {
        OverviewResponse::Staff(service::staff_overview(&state.db, claims.sub).await?)
    };
    Ok(ApiResponse::ok(response))
}

async fn activities(
    _: ActivitiesPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<ActivityList>> {
    let list = if claims.role == Role::Boss {
        service::boss_activities(&state.db, claims.sub).await?
    } else {
        service::staff_activities(&state.db, claims.sub).await?
    };
    Ok(ApiResponse::ok(list))
}
