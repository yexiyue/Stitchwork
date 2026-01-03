use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresignRequest {
    pub filename: String,
    pub content_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PresignResponse {
    pub upload_url: String,
    pub file_url: String,
}
