use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateShareRequest {
    #[schemars(description = "分享标题")]
    pub title: String,
    #[schemars(description = "分享描述")]
    pub description: Option<String>,
    #[schemars(description = "关联的订单ID列表")]
    pub order_ids: Vec<Uuid>,
    #[schemars(description = "关联的工序ID列表")]
    pub process_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateShareRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub order_ids: Option<Vec<Uuid>>,
    pub process_ids: Option<Vec<Uuid>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicShareResponse {
    pub title: String,
    pub description: Option<String>,
    pub workshop_name: Option<String>,
    pub workshop_desc: Option<String>,
    pub workshop_address: Option<String>,
    pub workshop_image: Option<String>,
    pub piece_unit: String,
    pub boss_phone: Option<String>,
    pub avatar: Option<String>,
    pub processes: Vec<PublicProcessInfo>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicProcessInfo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub piece_price: rust_decimal::Decimal,
    pub order_product_name: String,
    pub order_images: Vec<String>,
    pub remaining_quantity: i32,
}
