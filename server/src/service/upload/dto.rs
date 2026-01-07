use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresignRequest {
    pub filename: String,
    pub content_type: String,
    /// Blake3 hash for deduplication (base64 URL-safe)
    pub hash: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PresignResponse {
    /// Presigned upload URL (None if file already exists)
    pub upload_url: Option<String>,
    /// Storage key for the file
    pub key: String,
    /// Whether the file already exists (instant upload)
    pub exists: bool,
}
