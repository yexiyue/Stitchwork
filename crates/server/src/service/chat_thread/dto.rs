use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateThreadDto {
    pub title: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMessageDto {
    pub id: String,
    pub parent_id: Option<String>,
    pub format: String,
    pub content: serde_json::Value,
}
