use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;
use pingora::prelude::*;
use pingora::proxy::{http_proxy_service, ProxyHttp, Session};
use pingora::upstreams::peer::HttpPeer;
use pingora_limits::rate::Rate;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

/// 全局限流器 (leaky bucket)
/// 每秒 10 个请求，突发最多 30 个
static RATE_LIMITER: std::sync::LazyLock<Arc<Rate>> =
    std::sync::LazyLock::new(|| Arc::new(Rate::new(Duration::from_secs(1))));

/// 请求上下文
pub struct RequestCtx {
    /// 是否为静态文件请求（已在 request_filter 中处理）
    is_static: bool,
}

/// StitchWork 反向代理
///
/// 路由规则:
/// - /api/*   -> 转发到 Axum 后端
/// - /health  -> 转发到 Axum 后端
/// - /*       -> 静态文件服务 (SPA 模式)
pub struct StitchWorkProxy {
    /// 后端 API 服务器地址
    api_addr: String,
    /// 静态文件目录
    static_dir: PathBuf,
}

impl StitchWorkProxy {
    pub fn new(api_addr: &str, static_dir: PathBuf) -> Self {
        Self {
            api_addr: api_addr.to_string(),
            static_dir,
        }
    }

    /// 判断是否为 API 请求
    fn is_api_request(&self, path: &str) -> bool {
        path.starts_with("/api") || path == "/health"
    }

    /// 读取静态文件
    async fn read_static_file(&self, path: &str) -> Option<(Bytes, &'static str)> {
        // 规范化路径，防止目录遍历攻击
        let req_path = path.trim_start_matches('/');
        let file_path = if req_path.is_empty() || req_path == "/" {
            self.static_dir.join("index.html")
        } else {
            self.static_dir.join(req_path)
        };

        // 安全检查：确保路径在 static_dir 内
        let canonical = match file_path.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                // 文件不存在，SPA 模式返回 index.html
                return self.read_index_html().await;
            }
        };

        let static_canonical = match self.static_dir.canonicalize() {
            Ok(p) => p,
            Err(_) => return None,
        };

        if !canonical.starts_with(&static_canonical) {
            tracing::warn!("Path traversal attempt: {}", path);
            return None;
        }

        // 如果是目录，尝试读取 index.html
        if canonical.is_dir() {
            let index_path = canonical.join("index.html");
            if index_path.exists() {
                return self.read_file(&index_path).await;
            }
            return self.read_index_html().await;
        }

        self.read_file(&canonical).await
    }

    /// 读取文件内容
    async fn read_file(&self, path: &PathBuf) -> Option<(Bytes, &'static str)> {
        match tokio::fs::read(path).await {
            Ok(content) => {
                let mime = mime_guess::from_path(path)
                    .first_raw()
                    .unwrap_or("application/octet-stream");
                // 将 &str 转换为 'static 生命周期（因为 mime_guess 返回的是静态字符串）
                Some((Bytes::from(content), mime))
            }
            Err(e) => {
                tracing::debug!("Failed to read file {:?}: {}", path, e);
                None
            }
        }
    }

    /// 读取 index.html (SPA fallback)
    async fn read_index_html(&self) -> Option<(Bytes, &'static str)> {
        let index_path = self.static_dir.join("index.html");
        self.read_file(&index_path).await
    }
}

#[async_trait]
impl ProxyHttp for StitchWorkProxy {
    type CTX = RequestCtx;

    fn new_ctx(&self) -> Self::CTX {
        RequestCtx { is_static: false }
    }

    /// 请求过滤器 - 限流 + 静态文件处理
    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let path = session.req_header().uri.path();

        // 限流检查 (仅对 API 请求)
        if self.is_api_request(path) {
            let client_ip = session
                .client_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            // 每秒 10 个请求，突发最多 30 个
            let curr_rate = RATE_LIMITER.observe(&client_ip, 1);
            if curr_rate > 30 {
                tracing::warn!("Rate limit exceeded for {}: {} req/s", client_ip, curr_rate);

                let mut header =
                    pingora_http::ResponseHeader::build(StatusCode::TOO_MANY_REQUESTS, None)?;
                header.insert_header("Content-Type", "application/json")?;
                header.insert_header("Retry-After", "1")?;

                let body = Bytes::from(r#"{"code":429,"message":"Too many requests"}"#);
                header.insert_header("Content-Length", body.len().to_string())?;

                session
                    .write_response_header(Box::new(header), false)
                    .await?;
                session.write_response_body(Some(body), true).await?;

                return Ok(true); // 请求已处理
            }

            return Ok(false); // 继续代理流程
        }

        // 静态文件请求
        tracing::debug!("Static file request: {}", path);
        ctx.is_static = true;

        match self.read_static_file(path).await {
            Some((content, content_type)) => {
                // 构建响应头
                let mut header = pingora_http::ResponseHeader::build(StatusCode::OK, None)?;
                header.insert_header("Content-Type", content_type)?;
                header.insert_header("Content-Length", content.len().to_string())?;
                header.insert_header("Cache-Control", "public, max-age=31536000")?;

                // 对 HTML 文件不缓存（SPA 需要最新版本）
                if content_type == "text/html" {
                    header.insert_header("Cache-Control", "no-cache")?;
                }

                // 发送响应
                session
                    .write_response_header(Box::new(header), false)
                    .await?;
                session.write_response_body(Some(content), true).await?;

                Ok(true) // 请求已处理完毕
            }
            None => {
                // 404 Not Found
                let mut header = pingora_http::ResponseHeader::build(StatusCode::NOT_FOUND, None)?;
                header.insert_header("Content-Type", "text/plain")?;

                let body = Bytes::from("404 Not Found");
                header.insert_header("Content-Length", body.len().to_string())?;

                session
                    .write_response_header(Box::new(header), false)
                    .await?;
                session.write_response_body(Some(body), true).await?;

                Ok(true)
            }
        }
    }

    /// 选择上游服务器
    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        // 静态文件已在 request_filter 中处理
        if ctx.is_static {
            // 这里不应该被调用，但为了安全起见返回一个 peer
            return Err(Error::new(ErrorType::InternalError));
        }

        let path = session.req_header().uri.path();
        tracing::debug!("Proxying API request: {}", path);

        let peer = HttpPeer::new(&self.api_addr, false, String::new());
        Ok(Box::new(peer))
    }

    /// 上游请求过滤器 - 添加代理头
    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut pingora_http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        // 添加 X-Forwarded-* 头部
        upstream_request.insert_header("X-Forwarded-Proto", "http")?;

        // 转发客户端真实 IP
        if let Some(addr) = session.client_addr() {
            upstream_request.insert_header("X-Forwarded-For", addr.to_string())?;
            upstream_request.insert_header("X-Real-IP", addr.to_string())?;
        }

        Ok(())
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    // 配置
    let api_addr = std::env::var("API_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "./dist".to_string());

    let static_path = PathBuf::from(&static_dir);
    if !static_path.exists() {
        tracing::warn!("Static directory does not exist: {}", static_dir);
    }

    tracing::info!("Starting StitchWork Proxy");
    tracing::info!("  Listen: {}", listen_addr);
    tracing::info!("  API Backend: {}", api_addr);
    tracing::info!("  Static Dir: {}", static_dir);

    let proxy = StitchWorkProxy::new(&api_addr, static_path);
    let mut proxy_service = http_proxy_service(&server.configuration, proxy);
    proxy_service.add_tcp(&listen_addr);

    server.add_service(proxy_service);

    tracing::info!("Proxy server running on http://{}", listen_addr);
    server.run_forever();
}
