//! AI SDK chat request type (business layer)
//!
//! This module defines the business-specific chat request type that includes
//! application-specific fields like knowledge base ID and reasoning settings.

use std::ops::Deref;

use rig_ai_sdk::UIMessage;
use serde::{Deserialize, Serialize};

/// Forwarded tool definition from frontend
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardedTool {
    /// Tool description
    pub description: String,

    /// JSON Schema for tool parameters
    pub parameters: serde_json::Value,
}

/// Collection of forwarded tools from frontend
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForwardedTools {
    #[serde(flatten)]
    pub tools: std::collections::HashMap<String, ForwardedTool>,
}

impl Deref for ForwardedTools {
    type Target = std::collections::HashMap<String, ForwardedTool>;

    fn deref(&self) -> &Self::Target {
        &self.tools
    }
}

/// AI SDK chat request format
///
/// Represents a complete chat request from the frontend, including message history
/// and optional configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AISdkChatRequest {
    /// Session ID
    #[serde(default)]
    pub session_id: Option<String>,

    /// Message history (all messages in the conversation)
    pub messages: Vec<UIMessage>,

    /// Trigger type (e.g., "user-message", "regenerate")
    #[serde(default)]
    pub trigger: Option<String>,

    /// Message ID (used for regenerate operations)
    #[serde(default)]
    pub message_id: Option<String>,

    /// Forwarded tools from frontend (for interactive tools like forms, selections)
    #[serde(default)]
    pub tools: Option<ForwardedTools>,

    // --- Extension fields ---
    /// Model to use for the request
    #[serde(default)]
    pub model: Option<String>,

    /// Whether to enable model reasoning (o1-style thinking)
    #[serde(default)]
    pub enable_reasoning: bool,
}
