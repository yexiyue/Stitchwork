use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

use entity::order::{Model, OrderStatus};

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderDto {
    #[schemars(description = "客户ID")]
    pub customer_id: Uuid,
    #[schemars(description = "产品名称")]
    pub product_name: String,
    #[schemars(description = "订单描述")]
    pub description: Option<String>,
    #[schemars(description = "产品图片URL列表")]
    pub images: Option<Json>,
    #[schemars(description = "订单数量")]
    pub quantity: i32,
    #[schemars(description = "单价")]
    pub unit_price: Decimal,
}

#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryParams {
    #[schemars(description = "按客户ID筛选")]
    pub customer_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderStatusDto {
    #[schemars(description = "订单状态: pending/processing/completed/delivered/cancelled")]
    pub status: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderDto {
    pub product_name: Option<String>,
    pub description: Option<String>,
    pub images: Option<Json>,
    pub quantity: Option<i32>,
    pub unit_price: Option<Decimal>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub boss_id: Uuid,
    pub product_name: String,
    pub description: Option<String>,
    pub images: Option<Json>,
    pub quantity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Decimal>,
    pub status: OrderStatus,
    pub received_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
}

impl OrderResponse {
    pub fn from_model(model: Model, hide_price: bool) -> Self {
        Self {
            id: model.id,
            customer_id: model.customer_id,
            boss_id: model.boss_id,
            product_name: model.product_name,
            description: model.description,
            images: model.images,
            quantity: model.quantity,
            unit_price: if hide_price {
                None
            } else {
                Some(model.unit_price)
            },
            status: model.status,
            received_at: model.received_at,
            delivered_at: model.delivered_at,
        }
    }
}
