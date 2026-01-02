use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProcessDto {
    pub order_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub piece_price: Decimal,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProcessQueryParams {
    pub order_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProcessDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub piece_price: Option<Decimal>,
}
