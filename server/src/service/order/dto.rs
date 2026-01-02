use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::Value as Json;
use strum::{AsRefStr, EnumString};
use uuid::Uuid;

use crate::error::{AppError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, AsRefStr, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    pub fn parse(s: &str) -> Result<Self> {
        s.parse().map_err(|_| AppError::BadRequest(format!("Invalid status: {}", s)))
    }

    pub fn can_transition_to(&self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Processing)
                | (Self::Pending, Self::Cancelled)
                | (Self::Processing, Self::Completed)
                | (Self::Processing, Self::Cancelled)
                | (Self::Completed, Self::Delivered)
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderDto {
    pub customer_id: Uuid,
    pub product_name: String,
    pub description: Option<String>,
    pub images: Option<Json>,
    pub quantity: i32,
    pub unit_price: Decimal,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryParams {
    pub customer_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderStatusDto {
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderDto {
    pub product_name: Option<String>,
    pub description: Option<String>,
    pub images: Option<Json>,
    pub quantity: Option<i32>,
    pub unit_price: Option<Decimal>,
    pub status: Option<String>,
}
