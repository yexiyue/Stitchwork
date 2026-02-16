use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::user::Role;

/// 注册码详情响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCodeResponse {
    /// 注册码ID
    pub id: Uuid,
    /// 注册码（8位）
    pub code: String,
    /// 是否启用
    pub is_active: bool,
    /// 使用者ID
    pub used_by: Option<Uuid>,
    /// 使用时间
    pub used_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 使用者用户名
    pub used_by_username: Option<String>,
}

/// 管理后台统计数据
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminStats {
    // 用户统计
    /// 用户总数
    pub total_users: i64,
    /// 老板数量
    pub boss_count: i64,
    /// 员工数量
    pub staff_count: i64,
    /// 今日新增用户数
    pub today_new_users: i64,
    /// 本周新增用户数
    pub week_new_users: i64,
    /// 本月新增用户数
    pub month_new_users: i64,

    // 工坊统计
    /// 工坊总数
    pub total_workshops: i64,
    /// 活跃工坊数
    pub active_workshops: i64,

    // 注册码统计
    /// 注册码总数
    pub total_codes: i64,
    /// 已使用注册码数
    pub used_codes: i64,
    /// 可用注册码数
    pub available_codes: i64,
    /// 已禁用注册码数
    pub disabled_codes: i64,

    // 平台活跃度
    /// 今日订单数
    pub today_orders: i64,
    /// 本月订单数
    pub month_orders: i64,
    /// 今日计件记录数
    pub today_records: i64,
    /// 本月计件记录数
    pub month_records: i64,
}

/// 管理后台查询参数
#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminQueryParams {
    /// 页码，从1开始
    #[serde(default = "default_page")]
    pub page: u64,
    /// 每页数量
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    20
}

/// 用户列表项
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserListItem {
    /// 用户ID
    pub id: Uuid,
    /// 用户名
    pub username: String,
    /// 用户角色: boss/staff
    pub role: Role,
    /// 显示名称
    pub display_name: Option<String>,
    /// 手机号
    pub phone: String,
    /// 头像URL
    pub avatar: Option<String>,
    /// 是否为超级管理员
    pub is_super_admin: bool,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}
