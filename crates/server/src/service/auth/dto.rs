use entity::user::Role;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::service::workshop::dto::WorkshopResponse;

/// 登录请求参数
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// 用户名或手机号
    pub username: String,
    /// 密码
    pub password: String,
}

/// 登录用户信息
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
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
    /// 所属工坊信息
    pub workshop: Option<WorkshopResponse>,
}

/// 登录响应
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    /// JWT访问令牌
    pub token: String,
    /// 用户信息
    pub user: LoginUser,
}

/// 老板注册请求参数
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 手机号
    pub phone: String,
    /// 注册码（老板注册需要）
    pub register_code: String,
}

/// 更新用户资料请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    /// 显示名称
    pub display_name: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
}

/// 修改密码请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    /// 旧密码
    pub old_password: String,
    /// 新密码
    pub new_password: String,
}

/// 员工注册请求参数
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterStaffRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 手机号
    pub phone: String,
    /// 邀请码（员工注册需要）
    pub invite_code: String,
}
