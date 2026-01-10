use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProcessDto {
    #[schemars(description = "所属订单ID")]
    pub order_id: Uuid,
    #[schemars(description = "工序名称")]
    pub name: String,
    #[schemars(description = "工序描述")]
    pub description: Option<String>,
    #[schemars(description = "计件单价")]
    pub piece_price: Decimal,
}

#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProcessQueryParams {
    #[schemars(description = "按订单ID筛选")]
    pub order_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProcessDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub piece_price: Option<Decimal>,
}
