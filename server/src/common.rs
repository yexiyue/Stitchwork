use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::{ColumnTrait, QueryFilter, Select};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: u64,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
    pub sort_by: Option<String>,
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
    pub search: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: Option<Vec<String>>,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    20
}
fn default_sort_order() -> String {
    "desc".to_string()
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

// ============ Date Range Filter Helpers ============

/// Parse date string to NaiveDateTime at start of day (00:00:00)
pub fn parse_start_date(date_str: &str) -> Option<NaiveDateTime> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
}

/// Parse date string to NaiveDateTime at end of day (23:59:59)
pub fn parse_end_date(date_str: &str) -> Option<NaiveDateTime> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .ok()
        .and_then(|d| d.and_hms_opt(23, 59, 59))
}

/// Apply date range filter to a Select query
///
/// # Example
/// ```ignore
/// let query = order::Entity::find();
/// let query = apply_date_filter(query, order::Column::ReceivedAt, params.start_date.as_deref(), params.end_date.as_deref());
/// ```
pub fn apply_date_filter<E, C>(
    mut query: Select<E>,
    column: C,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Select<E>
where
    E: sea_orm::EntityTrait,
    C: ColumnTrait + Copy,
{
    if let Some(start) = start_date.and_then(parse_start_date) {
        query = query.filter(column.gte(start));
    }
    if let Some(end) = end_date.and_then(parse_end_date) {
        query = query.filter(column.lte(end));
    }
    query
}

// ============ Ownership Verification Helpers ============

/// Trait for entities owned by a boss
pub trait OwnedByBoss {
    fn boss_id(&self) -> Uuid;

    /// Verify that the entity is owned by the given boss_id
    fn verify_owner(&self, boss_id: Uuid) -> Result<()> {
        if self.boss_id() != boss_id {
            Err(AppError::Forbidden)
        } else {
            Ok(())
        }
    }
}

/// Trait for entities owned by a user (customer belongs to boss via user_id)
pub trait OwnedByUser {
    fn user_id(&self) -> Uuid;

    fn verify_owner(&self, user_id: Uuid) -> Result<()> {
        if self.user_id() != user_id {
            Err(AppError::Forbidden)
        } else {
            Ok(())
        }
    }
}
