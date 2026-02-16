use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

/// 创建工序请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProcessDto {
    /// 所属订单ID
    pub order_id: Uuid,
    /// 工序名称
    pub name: String,
    /// 工序描述
    pub description: Option<String>,
    /// 计件单价
    pub piece_price: Decimal,
}

/// 工序查询参数
#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProcessQueryParams {
    /// 按订单ID筛选
    pub order_id: Option<Uuid>,
}

/// 更新工序请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProcessDto {
    /// 工序名称
    pub name: Option<String>,
    /// 工序描述
    pub description: Option<String>,
    /// 计件单价
    pub piece_price: Option<Decimal>,
}
