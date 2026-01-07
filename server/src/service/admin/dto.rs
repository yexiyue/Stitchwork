use serde::Serialize;
use uuid::Uuid;

use crate::entity::user::Role;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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
