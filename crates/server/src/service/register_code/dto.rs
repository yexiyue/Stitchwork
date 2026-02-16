use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 注册码详情响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCodeResponse {
    pub id: Uuid,
    /// 6位数字注册码
    pub code: String,
    /// 创建者 ID
    pub created_by: Option<Uuid>,
    /// 关联工坊 ID
    pub workshop_id: Option<Uuid>,
    /// 使用者 ID
    pub used_by: Option<Uuid>,
    /// 使用时间
    pub used_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 使用者用户名（批量加载）
    pub used_by_username: Option<String>,
}

/// 注册码查询参数
#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCodeQueryParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    20
}
