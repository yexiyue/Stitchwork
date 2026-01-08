use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkshopRequest {
    pub name: String,
    pub desc: Option<String>,
    pub address: Option<String>,
    pub image: Option<String>,
    pub piece_unit: Option<String>,
    pub business_label: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkshopRequest {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub address: Option<String>,
    pub image: Option<String>,
    pub piece_unit: Option<String>,
    pub business_label: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteCodeResponse {
    pub code: String,
    pub expires_at: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindWorkshopRequest {
    pub invite_code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
}
