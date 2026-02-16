use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

/// 创建工资单请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePayrollDto {
    /// 员工ID
    pub user_id: Uuid,
    /// 发放金额
    pub amount: Decimal,
    /// 关联的计件记录ID列表
    pub record_ids: Vec<Uuid>,
    /// 付款凭证图片URL
    pub payment_image: Option<String>,
    /// 备注
    pub note: Option<String>,
}

/// 更新工资单请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePayrollDto {
    /// 发放金额
    pub amount: Option<Decimal>,
    /// 付款凭证图片URL
    pub payment_image: Option<String>,
    /// 备注
    pub note: Option<String>,
}

/// 工资单关联的计件记录响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayrollRecordResponse {
    /// 计件记录ID
    pub id: Uuid,
    /// 计件数量
    pub quantity: i32,
    /// 计件金额（元）
    pub amount: Decimal,
    /// 记录时间
    pub recorded_at: chrono::DateTime<chrono::Utc>,
    /// 工序名称
    pub process_name: Option<String>,
    /// 订单/产品名称
    pub order_name: Option<String>,
    /// 订单图片URL列表
    pub order_images: Option<Json>,
    /// 计件单价
    pub piece_price: Option<Decimal>,
}

/// 工资单详情响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayrollDetailResponse {
    /// 工资单ID
    pub id: Uuid,
    /// 员工ID
    pub user_id: Uuid,
    /// 老板ID
    pub boss_id: Uuid,
    /// 发放金额（元）
    pub amount: Decimal,
    /// 付款凭证图片URL
    pub payment_image: Option<String>,
    /// 备注
    pub note: Option<String>,
    /// 发放时间
    pub paid_at: DateTime<Utc>,
    /// 关联的计件记录列表
    pub records: Vec<PayrollRecordResponse>,
}
