use aws_sdk_s3::presigning::PresigningConfig;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Extension, Router,
};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

use crate::common::ApiResponse;
use crate::error::{AppError, AppJson, Result};
use crate::AppState;

use super::dto::{PresignRequest, PresignResponse};
use crate::service::auth::Claims;

/// 需要认证的路由
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/upload/presign", post(presign))
}

/// 公开路由（文件访问）
pub fn public_router() -> Router<Arc<AppState>> {
    Router::new().route("/files/{*key}", get(redirect_to_file))
}

async fn presign(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<Claims>,
    AppJson(req): AppJson<PresignRequest>,
) -> Result<ApiResponse<PresignResponse>> {
    let s3 = state
        .s3
        .as_ref()
        .ok_or(AppError::Internal("S3 not configured".to_string()))?;

    let ext = req.filename.rsplit('.').next().unwrap_or("jpg");
    let key = format!("uploads/{}.{}", Uuid::new_v4(), ext);

    let presigning_config =
        PresigningConfig::expires_in(Duration::from_secs(900)).expect("valid presigning config");

    let presigned = s3
        .client
        .put_object()
        .bucket(&s3.bucket)
        .key(&key)
        .content_type(&req.content_type)
        .presigned(presigning_config)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(ApiResponse::ok(PresignResponse {
        upload_url: presigned.uri().to_string(),
        key,
    }))
}

/// 重定向到文件（生成预签名 URL）
async fn redirect_to_file(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let s3 = match state.s3.as_ref() {
        Some(s3) => s3,
        None => return (StatusCode::SERVICE_UNAVAILABLE, "S3 not configured").into_response(),
    };

    match s3.get_presigned_url(&key, 3600).await {
        Ok(url) => Redirect::temporary(&url).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
