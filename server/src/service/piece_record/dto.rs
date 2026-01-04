use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::piece_record::{PieceRecordStatus, RecordedBy};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePieceRecordDto {
    pub process_id: Uuid,
    pub user_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PieceRecordResponse {
    pub id: Uuid,
    pub process_id: Uuid,
    pub user_id: Uuid,
    pub boss_id: Uuid,
    pub quantity: i32,
    pub amount: Decimal,
    pub status: PieceRecordStatus,
    pub recorded_by: RecordedBy,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    // 关联字段
    pub process_name: Option<String>,
    pub user_name: Option<String>,
    pub order_id: Option<Uuid>,
    pub order_name: Option<String>,
    pub order_image: Option<String>,
    pub piece_price: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePieceRecordDto {
    pub quantity: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchApproveDto {
    pub ids: Vec<Uuid>,
}
