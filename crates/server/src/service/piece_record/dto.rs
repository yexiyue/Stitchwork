use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::piece_record::{PieceRecordStatus, RecordedBy};

/// 创建计件记录请求
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePieceRecordDto {
    /// 工序ID
    pub process_id: Uuid,
    /// 员工ID
    pub user_id: Uuid,
    /// 计件数量
    pub quantity: i32,
}

/// 计件记录详情响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PieceRecordResponse {
    /// 记录ID
    pub id: Uuid,
    /// 工序ID
    pub process_id: Uuid,
    /// 员工ID
    pub user_id: Uuid,
    /// 老板ID
    pub boss_id: Uuid,
    /// 计件数量
    pub quantity: i32,
    /// 计件金额（元）
    pub amount: Decimal,
    /// 记录状态: pending/approved/rejected/settled
    pub status: PieceRecordStatus,
    /// 记录来源: staff/boss
    pub recorded_by: RecordedBy,
    /// 记录时间
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    /// 工序名称（关联字段）
    pub process_name: Option<String>,
    /// 员工姓名（关联字段）
    pub user_name: Option<String>,
    /// 订单ID（关联字段）
    pub order_id: Option<Uuid>,
    /// 订单/产品名称（关联字段）
    pub order_name: Option<String>,
    /// 订单图片URL（关联字段）
    pub order_image: Option<String>,
    /// 计件单价（关联字段）
    pub piece_price: Option<Decimal>,
}

/// 更新计件记录请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePieceRecordDto {
    /// 计件数量
    pub quantity: Option<i32>,
}

/// 批量审批请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BatchApproveDto {
    /// 要批量审批的计件记录ID列表
    pub ids: Vec<Uuid>,
}
