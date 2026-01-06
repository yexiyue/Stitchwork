use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::{Client, Config};
use std::time::Duration;

pub struct S3Config {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub public_url: String, // 公开访问的 URL 基础路径
}

impl S3Config {
    /// 从环境变量创建配置（通用 S3 兼容存储）
    ///
    /// 环境变量:
    /// - `S3_ACCESS_KEY_ID`: 访问密钥 ID（必需）
    /// - `S3_ACCESS_KEY_SECRET`: 访问密钥 Secret（必需）
    /// - `S3_ENDPOINT`: S3 服务端点 URL（必需），如 `https://s3.amazonaws.com`
    /// - `S3_REGION`: 区域标识（可选，默认 `us-east-1`）
    /// - `S3_BUCKET`: 存储桶名称（必需）
    /// - `S3_PUBLIC_URL`: 公开访问基础 URL（可选，默认 `{endpoint}/{bucket}`）
    pub fn from_env() -> Option<Self> {
        let endpoint = std::env::var("S3_ENDPOINT").ok()?;
        let bucket = std::env::var("S3_BUCKET").ok()?;
        let public_url =
            std::env::var("S3_PUBLIC_URL").unwrap_or_else(|_| format!("{}/{}", endpoint, bucket));
        Some(Self {
            access_key_id: std::env::var("S3_ACCESS_KEY_ID").ok()?,
            access_key_secret: std::env::var("S3_ACCESS_KEY_SECRET").ok()?,
            endpoint,
            region: std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            bucket,
            public_url,
        })
    }

    /// 创建 S3 客户端
    pub fn build_client(&self) -> Client {
        let credentials = Credentials::new(
            &self.access_key_id,
            &self.access_key_secret,
            None,
            None,
            "s3-client",
        );

        let config = Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(&self.endpoint)
            .region(Region::new(self.region.clone()))
            .credentials_provider(SharedCredentialsProvider::new(credentials))
            .force_path_style(false) // 阿里云 OSS 使用 virtual-hosted style
            .build();

        Client::from_conf(config)
    }
}

#[derive(Clone)]
pub struct S3Client {
    pub client: Client,
    pub bucket: String,
    pub public_url: String,
}

impl S3Client {
    pub fn new(config: &S3Config) -> Self {
        Self {
            client: config.build_client(),
            bucket: config.bucket.clone(),
            public_url: config.public_url.clone(),
        }
    }

    /// 生成预签名的 GET URL，用于访问私有对象
    pub async fn get_presigned_url(&self, key: &str, expires_in_secs: u64) -> Result<String, String> {
        let presigning_config = PresigningConfig::expires_in(Duration::from_secs(expires_in_secs))
            .map_err(|e| e.to_string())?;

        let presigned = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| e.to_string())?;

        Ok(presigned.uri().to_string())
    }
}
