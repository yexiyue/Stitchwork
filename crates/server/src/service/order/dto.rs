use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

use entity::order::{Model, OrderStatus};

/// 创建订单请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderDto {
    /// 客户ID
    pub customer_id: Uuid,
    /// 产品名称
    pub product_name: String,
    /// 订单描述
    pub description: Option<String>,
    /// 产品图片URL列表
    pub images: Option<Json>,
    /// 订单数量
    pub quantity: i32,
    /// 单价
    pub unit_price: Decimal,
}

/// 订单查询参数
#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryParams {
    /// 按客户ID筛选
    pub customer_id: Option<Uuid>,
}

/// 更新订单状态请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderStatusDto {
    /// 订单状态: pending/processing/completed/delivered/cancelled
    pub status: String,
}

/// 更新订单请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderDto {
    /// 产品名称
    pub product_name: Option<String>,
    /// 订单描述
    pub description: Option<String>,
    /// 产品图片URL列表
    pub images: Option<Json>,
    /// 订单数量
    pub quantity: Option<i32>,
    /// 单价
    pub unit_price: Option<Decimal>,
    /// 订单状态: pending/processing/completed/delivered/cancelled
    pub status: Option<String>,
}

/// 订单详情响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    /// 订单ID
    pub id: Uuid,
    /// 客户ID
    pub customer_id: Uuid,
    /// 老板ID
    pub boss_id: Uuid,
    /// 产品名称
    pub product_name: String,
    /// 订单描述
    pub description: Option<String>,
    /// 产品图片URL列表
    pub images: Option<Json>,
    /// 订单数量
    pub quantity: i32,
    /// 单价（员工不可见时为None）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Decimal>,
    /// 订单状态: pending/processing/completed/delivered/cancelled
    pub status: OrderStatus,
    /// 接单时间
    pub received_at: DateTime<Utc>,
    /// 出货时间
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
