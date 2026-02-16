use schemars::JsonSchema;
use serde::Deserialize;

/// 创建客户请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerDto {
    /// 客户名称
    pub name: String,
    /// 客户电话
    pub phone: Option<String>,
    /// 客户描述/备注
    pub description: Option<String>,
}

/// 更新客户请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerDto {
    /// 客户名称
    pub name: Option<String>,
    /// 客户电话
    pub phone: Option<String>,
    /// 客户描述/备注
    pub description: Option<String>,
}
