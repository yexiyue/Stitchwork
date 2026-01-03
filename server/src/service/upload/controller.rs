use aws_sdk_s3::presigning::PresigningConfig;
use axum::{extract::State, routing::post, Extension, Router};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

use crate::common::ApiResponse;
use crate::error::{AppError, AppJson, Result};
use crate::AppState;

use super::dto::{PresignRequest, PresignResponse};
use crate::service::auth::Claims;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/upload/presign", post(presign))
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

    // 生成唯一文件名
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

    let file_url = format!("{}/{}", s3.public_url, key);

    Ok(ApiResponse::ok(PresignResponse {
        upload_url: presigned.uri().to_string(),
        file_url,
    }))
}
