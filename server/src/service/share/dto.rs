use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateShareRequest {
    pub title: String,
    pub order_ids: Vec<Uuid>,
    pub process_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateShareRequest {
    pub title: Option<String>,
    pub order_ids: Option<Vec<Uuid>>,
    pub process_ids: Option<Vec<Uuid>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicShareResponse {
    pub title: String,
    pub workshop_name: Option<String>,
    pub workshop_desc: Option<String>,
    pub avatar: Option<String>,
    pub orders: Vec<PublicOrderInfo>,
    pub processes: Vec<PublicProcessInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicOrderInfo {
    pub id: Uuid,
    pub product_name: String,
    pub description: Option<String>,
    pub images: Option<serde_json::Value>,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicProcessInfo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub piece_price: rust_decimal::Decimal,
    pub order_product_name: String,
}
