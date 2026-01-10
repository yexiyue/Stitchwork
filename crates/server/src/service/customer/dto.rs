use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerDto {
    #[schemars(description = "客户名称")]
    pub name: String,
    #[schemars(description = "客户电话")]
    pub phone: Option<String>,
    #[schemars(description = "客户描述/备注")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerDto {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
}
