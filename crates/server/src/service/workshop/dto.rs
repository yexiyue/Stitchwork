use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 工坊信息响应
#[derive(Debug, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkshopResponse {
    /// 工坊ID
    pub id: Uuid,
    /// 工坊名称
    pub name: String,
    /// 工坊描述
    pub desc: Option<String>,
    /// 工坊地址
    pub address: Option<String>,
    /// 工坊图片URL
    pub image: Option<String>,
    /// 计件单位，如：件、个
    pub piece_unit: String,
    /// 业务标签
    pub business_label: String,
}

/// 创建工坊请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkshopRequest {
    /// 工坊名称
    pub name: String,
    /// 工坊描述
    pub desc: Option<String>,
    /// 工坊地址
    pub address: Option<String>,
    /// 工坊图片URL
    pub image: Option<String>,
    /// 计件单位，如：件、个
    pub piece_unit: Option<String>,
    /// 业务标签
    pub business_label: Option<String>,
}

/// 更新工坊请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkshopRequest {
    /// 工坊名称
    pub name: Option<String>,
    /// 工坊描述
    pub desc: Option<String>,
    /// 工坊地址
    pub address: Option<String>,
    /// 工坊图片URL
    pub image: Option<String>,
    /// 计件单位，如：件、个
    pub piece_unit: Option<String>,
    /// 业务标签
    pub business_label: Option<String>,
}

/// 邀请码响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InviteCodeResponse {
    /// 邀请码
    pub code: String,
    /// 过期时间戳（秒）
    pub expires_at: i64,
}

/// 绑定工坊请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BindWorkshopRequest {
    /// 邀请码
    pub invite_code: String,
}

/// 员工信息响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StaffResponse {
    /// 员工ID
    pub id: Uuid,
    /// 用户名
    pub username: String,
    /// 显示名称
    pub display_name: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
}
