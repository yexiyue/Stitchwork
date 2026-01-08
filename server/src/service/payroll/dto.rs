use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePayrollDto {
    pub user_id: Uuid,
    pub amount: Decimal,
    pub record_ids: Vec<Uuid>,
    pub payment_image: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePayrollDto {
    pub amount: Option<Decimal>,
    pub payment_image: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayrollRecordResponse {
    pub id: Uuid,
    pub quantity: i32,
    pub amount: Decimal,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    pub process_name: Option<String>,
    pub order_name: Option<String>,
    pub order_images: Option<Json>,
    pub piece_price: Option<Decimal>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayrollDetailResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub boss_id: Uuid,
    pub amount: Decimal,
    pub payment_image: Option<String>,
    pub note: Option<String>,
    pub paid_at: DateTime<Utc>,
    pub records: Vec<PayrollRecordResponse>,
}
