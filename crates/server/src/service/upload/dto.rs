use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 预签名上传请求
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PresignRequest {
    /// 文件名
    pub filename: String,
    /// 文件MIME类型
    pub content_type: String,
    /// Blake3哈希值，用于秒传去重（base64 URL-safe编码）
    pub hash: Option<String>,
}

/// 预签名上传响应
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PresignResponse {
    /// 预签名上传URL，如果文件已存在则为None
    pub upload_url: Option<String>,
    /// 文件存储key
    pub key: String,
    /// 文件是否已存在（秒传）
    pub exists: bool,
}
