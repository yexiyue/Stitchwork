//! AI SDK Data Stream Protocol adapter for rig
//!
//! Provides types and utilities to work with AI SDK's streaming protocol
//! in rig-based applications.

#![doc = include_str!("../README.md")]

#[cfg(feature = "axum-sse")]
pub mod adapter;
pub mod convert;
pub mod event;
pub mod message;
pub mod stream;

// Re-exports for convenience
#[cfg(feature = "axum-sse")]
pub use adapter::{adapt_rig_stream, adapt_rig_stream_sse};
pub use convert::{convert_message, convert_messages, extract_prompt_and_history};
pub use event::AISdkEvent;
pub use message::{UIMessage, UIMessagePart};
pub use stream::AISdkStreamBuilder;

/// Prelude for common imports
pub mod prelude {
    #[cfg(feature = "axum-sse")]
    pub use crate::adapter::{adapt_rig_stream, adapt_rig_stream_sse};
    pub use crate::convert::{convert_messages, extract_prompt_and_history};
    pub use crate::event::AISdkEvent;
    pub use crate::message::{UIMessage, UIMessagePart};
    pub use crate::stream::AISdkStreamBuilder;
}
