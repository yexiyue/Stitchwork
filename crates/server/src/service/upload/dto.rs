use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PresignRequest {
    #[schemars(description = "文件名")]
    pub filename: String,
    #[schemars(description = "文件MIME类型")]
    pub content_type: String,
    #[schemars(description = "Blake3哈希值，用于秒传去重（base64 URL-safe编码）")]
    pub hash: Option<String>,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PresignResponse {
    #[schemars(description = "预签名上传URL，如果文件已存在则为None")]
    pub upload_url: Option<String>,
    #[schemars(description = "文件存储key")]
    pub key: String,
    #[schemars(description = "文件是否已存在（秒传）")]
    pub exists: bool,
}
