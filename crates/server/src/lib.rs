//! StitchWork Server - 服装加工流程管理系统后端

// 模块声明
pub mod bootstrap;
pub mod chat;
pub mod common;
pub mod error;
pub mod mcp;
pub mod s3;
pub mod service;
pub mod state;
pub mod traits;

// Re-exports
pub use bootstrap::init_super_admin;
pub use s3::S3Client;
pub use service::notification::{Notification, Notifier};
pub use state::{anthropic, AppState, InviteCodes};
