use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_s3::{Client, Config};

pub struct S3Config {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub public_url: String, // 公开访问的 URL 基础路径
}

impl S3Config {
    /// 从环境变量创建配置
    pub fn from_env() -> Option<Self> {
        let endpoint = std::env::var("S3_ENDPOINT").ok()?;
        let bucket = std::env::var("S3_BUCKET").ok()?;
        let public_url = std::env::var("S3_PUBLIC_URL")
            .unwrap_or_else(|_| format!("{}/{}", endpoint, bucket));
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
            .force_path_style(true) // Supabase 需要 path style
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
}
