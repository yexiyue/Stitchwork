use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::Value as Json;
use uuid::Uuid;

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
