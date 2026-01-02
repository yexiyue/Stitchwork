use axum::{routing::get, Router};
use std::sync::Arc;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/api/payroll", get(list))
}

async fn list() -> &'static str {
    "[]"
}
