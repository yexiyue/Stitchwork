use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rmcp::model::IntoContents;
use thiserror::Error;

use crate::common::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Error(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, 400, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, 401, "Unauthorized".to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg.clone()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, 403, "Forbidden".to_string()),
            AppError::Database(err) => {
                tracing::error!(error = %err, "Database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    500,
                    "Database operation failed".to_string(),
                )
            }
            AppError::Internal(msg) => {
                tracing::error!(error = %msg, "Internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    500,
                    "An internal error occurred".to_string(),
                )
            }
            AppError::Error(err) => {
                tracing::error!(error = %err, "Err");
                (StatusCode::BAD_REQUEST, 400, err.to_string())
            }
        };

        let body = ApiResponse {
            code,
            message,
            data: (),
        };

        (status, Json(body)).into_response()
    }
}

impl IntoContents for AppError {
    fn into_contents(self) -> Vec<rmcp::model::Content> {
        vec![rmcp::model::Content::text(self.to_string())]
    }
}

impl From<AppError> for rmcp::model::ErrorData {
    fn from(e: AppError) -> Self {
        let code = match &e {
            AppError::BadRequest(_) => rmcp::model::ErrorCode::INVALID_PARAMS,
            AppError::Unauthorized => rmcp::model::ErrorCode::INVALID_REQUEST,
            AppError::Forbidden => rmcp::model::ErrorCode::INVALID_REQUEST,
            AppError::NotFound(_) => rmcp::model::ErrorCode::INVALID_PARAMS,
            _ => rmcp::model::ErrorCode::INTERNAL_ERROR,
        };
        rmcp::model::ErrorData::new(code, e.to_string(), None::<serde_json::Value>)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

/// 自定义 JSON 提取器
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        let message = match &rejection {
            JsonRejection::JsonDataError(err) => format!("Invalid JSON data: {}", err.body_text()),
            JsonRejection::JsonSyntaxError(err) => {
                format!("JSON syntax error: {}", err.body_text())
            }
            JsonRejection::MissingJsonContentType(_) => {
                "Missing Content-Type: application/json".to_string()
            }
            _ => "Invalid JSON request".to_string(),
        };
        AppError::BadRequest(message)
    }
}
