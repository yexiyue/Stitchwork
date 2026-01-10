use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::user::Role;

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCodeResponse {
    pub id: Uuid,
    pub code: String,
    pub is_active: bool,
    pub used_by: Option<Uuid>,
    pub used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub used_by_username: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminStats {
    // 用户统计
    pub total_users: i64,
    pub boss_count: i64,
    pub staff_count: i64,
    pub today_new_users: i64,
    pub week_new_users: i64,
    pub month_new_users: i64,

    // 工坊统计
    pub total_workshops: i64,
    pub active_workshops: i64,

    // 注册码统计
    pub total_codes: i64,
    pub used_codes: i64,
    pub available_codes: i64,
    pub disabled_codes: i64,

    // 平台活跃度
    pub today_orders: i64,
    pub month_orders: i64,
    pub today_records: i64,
    pub month_records: i64,
}

#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminQueryParams {
    #[schemars(description = "页码，从1开始")]
    #[serde(default = "default_page")]
    pub page: u64,
    #[schemars(description = "每页数量")]
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    20
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserListItem {
    pub id: Uuid,
    pub username: String,
    pub role: Role,
    pub display_name: Option<String>,
    pub phone: String,
    pub avatar: Option<String>,
    pub is_super_admin: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
