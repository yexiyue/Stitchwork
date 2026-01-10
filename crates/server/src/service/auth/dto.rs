use entity::user::Role;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::service::workshop::dto::WorkshopResponse;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[schemars(description = "用户名或手机号")]
    pub username: String,
    #[schemars(description = "密码")]
    pub password: String,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    pub id: Uuid,
    pub username: String,
    pub role: Role,
    pub display_name: Option<String>,
    pub phone: String,
    pub avatar: Option<String>,
    pub is_super_admin: bool,
    pub workshop: Option<WorkshopResponse>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
    pub user: LoginUser,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    #[schemars(description = "用户名")]
    pub username: String,
    #[schemars(description = "密码")]
    pub password: String,
    #[schemars(description = "手机号")]
    pub phone: String,
    #[schemars(description = "注册码（老板注册需要）")]
    pub register_code: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    #[schemars(description = "显示名称")]
    pub display_name: Option<String>,
    #[schemars(description = "手机号")]
    pub phone: Option<String>,
    #[schemars(description = "头像URL")]
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    #[schemars(description = "旧密码")]
    pub old_password: String,
    #[schemars(description = "新密码")]
    pub new_password: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterStaffRequest {
    #[schemars(description = "用户名")]
    pub username: String,
    #[schemars(description = "密码")]
    pub password: String,
    #[schemars(description = "手机号")]
    pub phone: String,
    #[schemars(description = "邀请码（员工注册需要）")]
    pub invite_code: String,
}
