use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::piece_record::{PieceRecordStatus, RecordedBy};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePieceRecordDto {
    #[schemars(description = "工序ID")]
    pub process_id: Uuid,
    #[schemars(description = "员工ID")]
    pub user_id: Uuid,
    #[schemars(description = "计件数量")]
    pub quantity: i32,
}

#[derive(Debug, Serialize, JsonSchema)]
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

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePieceRecordDto {
    #[schemars(description = "计件数量")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BatchApproveDto {
    #[schemars(description = "要批量审批的计件记录ID列表")]
    pub ids: Vec<Uuid>,
}
