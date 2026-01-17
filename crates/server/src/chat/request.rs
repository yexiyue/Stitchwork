//! AI SDK chat request type (business layer)
//!
//! This module defines the business-specific chat request type that includes
//! application-specific fields like knowledge base ID and reasoning settings.

use serde::Deserialize;
use rig_ai_sdk::UIMessage;

/// AI SDK chat request format
///
/// Represents a complete chat request from the frontend, including message history
/// and optional configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AISdkChatRequest {
    /// Session ID
    #[serde(default)]
    pub id: Option<String>,

    /// Message history (all messages in the conversation)
    pub messages: Vec<UIMessage>,

    /// Trigger type (e.g., "user-message", "regenerate")
    #[serde(default)]
    pub trigger: Option<String>,

    /// Message ID (used for regenerate operations)
    #[serde(default)]
    pub message_id: Option<String>,

    // --- Extension fields ---
    /// Model to use for the request
    #[serde(default)]
    pub model: Option<String>,

    /// Whether to enable model reasoning (o1-style thinking)
    #[serde(default)]
    pub enable_reasoning: bool,
}
