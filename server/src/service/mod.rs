pub mod auth;
pub mod customer;
pub mod home;
pub mod notification;
pub mod order;
pub mod payroll;
pub mod piece_record;
pub mod process;
pub mod share;
pub mod stats;
pub mod upload;
pub mod workshop;

use axum::{middleware, Router};
use std::sync::Arc;

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    // 需要认证的路由
    let protected = Router::new()
        .merge(auth::protected_router())
        .merge(customer::router())
        .merge(home::router())
        .merge(order::router())
        .merge(process::router())
        .merge(piece_record::router())
        .merge(payroll::router())
        .merge(stats::router())
        .merge(share::router())
        .merge(upload::router())
        .merge(workshop::router())
        .layer(middleware::from_fn(auth::auth_middleware));

    Router::new().nest(
        "/api",
        Router::new()
            .merge(auth::router()) // 登录注册不需要认证
            .merge(share::public_router()) // 公开分享页面不需要认证
            .merge(notification::router()) // SSE 使用 query token 认证
            .merge(upload::public_router()) // 文件访问不需要认证
            .merge(protected),
    )
}
