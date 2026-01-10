use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkshopResponse {
    pub id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    pub address: Option<String>,
    pub image: Option<String>,
    pub piece_unit: String,
    pub business_label: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkshopRequest {
    #[schemars(description = "工坊名称")]
    pub name: String,
    #[schemars(description = "工坊描述")]
    pub desc: Option<String>,
    #[schemars(description = "工坊地址")]
    pub address: Option<String>,
    #[schemars(description = "工坊图片URL")]
    pub image: Option<String>,
    #[schemars(description = "计件单位，如：件、个")]
    pub piece_unit: Option<String>,
    #[schemars(description = "业务标签")]
    pub business_label: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkshopRequest {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub address: Option<String>,
    pub image: Option<String>,
    pub piece_unit: Option<String>,
    pub business_label: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InviteCodeResponse {
    pub code: String,
    pub expires_at: i64,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BindWorkshopRequest {
    #[schemars(description = "邀请码")]
    pub invite_code: String,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StaffResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
}
