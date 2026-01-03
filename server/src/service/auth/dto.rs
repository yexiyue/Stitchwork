use crate::entity::user::Role;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::service::workshop::dto::WorkshopResponse;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    pub id: Uuid,
    pub username: String,
    pub role: Role,
    pub display_name: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub workshop: Option<WorkshopResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
    pub user: LoginUser,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
}
