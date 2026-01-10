use axum::{extract::Request, http::header::AUTHORIZATION, middleware::Next, response::Response};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;
use entity::user::Role;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid, // user_id
    pub role: Role,
    pub exp: usize,
}

impl Claims {
    pub fn require_boss(&self) -> Result<(), AppError> {
        if self.role != Role::Boss {
            return Err(AppError::Forbidden);
        }
        Ok(())
    }
}

fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET 环境变量必须设置")
        .into_bytes()
}

pub fn create_token(user_id: Uuid, role: Role) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        role,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&get_secret()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&get_secret()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

/// 从 HTTP Parts 中提取 Claims（用于 MCP 工具）
pub fn extract_claims_from_parts(parts: &axum::http::request::Parts) -> Result<Claims, AppError> {
    let auth_header = parts
        .headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(AppError::Unauthorized),
    };

    verify_token(token).map_err(|_| AppError::Unauthorized)
}

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(AppError::Unauthorized),
    };

    match verify_token(token) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(AppError::Unauthorized),
    }
}
