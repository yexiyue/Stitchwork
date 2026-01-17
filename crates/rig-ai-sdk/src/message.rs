//! AI SDK UIMessage type definitions
//!
//! Implements the AI SDK UIMessage format for receiving messages from frontend clients
//! like assistant-ui.
//!
//! # Overview
//!
//! This module provides types for deserializing AI SDK messages from frontend clients.
//! The format supports:
//!
//! - Rich text content with streaming states
//! - Multi-modal messages (text, images, files)
//! - Tool calls and tool results (both legacy and AI SDK 5.x formats)
//! - Reasoning/model thinking blocks
//! - Source references (URLs, documents)
//! - Custom data attachments
//!
//! Reference: [AI SDK Transport](https://ai-sdk.dev/docs/ai-sdk-ui/transport)
//!
//! # Examples
//!
//! ```ignore
//! use rig_ai_sdk::{UIMessage, UIMessagePart};
//!
//! let msg = UIMessage {
//!     id: "msg-1".to_string(),
//!     role: "user".to_string(),
//!     parts: vec![
//!         UIMessagePart::Text {
//!             text: "Hello!".to_string(),
//!             state: None,
//!             provider_metadata: None,
//!         }
//!     ],
//!     metadata: None,
//! };
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// Shared types
// ============================================================================

/// Provider metadata from AI SDK (simplified version).
///
/// Contains optional metadata fields that may be included by the AI provider.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct ProviderMetadata {
    /// Model ID being used
    pub model_id: Option<String>,

    /// Request ID from the provider
    pub request_id: Option<String>,

    /// Timestamp of the request
    pub timestamp: Option<String>,

    /// Additional provider-specific metadata fields
    #[serde(flatten)]
    pub extra: Value,
}

/// Streaming state for message parts.
///
/// Indicates whether content is being streamed or is complete.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Default)]
pub enum PartState {
    /// Content is currently being streamed
    #[serde(rename = "streaming")]
    Streaming,

    /// Content is complete
    #[serde(rename = "done")]
    #[default]
    Done,
}

// ============================================================================
// AI SDK UIMessage Part types
// ============================================================================

/// AI SDK UIMessage part types.
///
/// A message consists of one or more parts, each representing a different
/// type of content (text, file, tool call, reasoning, etc.).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UIMessagePart {
    /// Text content part
    ///
    /// Plain text content with optional streaming state and provider metadata.
    Text {
        text: String,
        #[serde(default)]
        state: Option<PartState>,
        #[serde(default)]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Reasoning/thinking part
    ///
    /// Model reasoning or thinking block content (e.g., for o1-style models).
    Reasoning {
        text: String,
        #[serde(default)]
        state: Option<PartState>,
        #[serde(default)]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// File attachment part (supports any media type)
    ///
    /// Represents a file attachment with URL, media type, and optional filename.
    File {
        #[serde(rename = "mediaType")]
        media_type: String,
        url: String,
        #[serde(default)]
        filename: Option<String>,
        #[serde(default)]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Tool call (legacy format for compatibility)
    ///
    /// Legacy tool call format without streaming state support.
    #[serde(rename = "tool-call")]
    ToolCall {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        #[serde(rename = "toolName")]
        tool_name: String,
        args: Value,
        #[serde(default)]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Tool result
    ///
    /// Result from a previous tool execution.
    #[serde(rename = "tool-result")]
    ToolResult {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        #[serde(rename = "toolName", default)]
        tool_name: Option<String>,
        result: Value,
    },

    /// Dynamic tool call (AI SDK 5.x format with state support)
    ///
    /// Modern tool call format with streaming state support and provider execution info.
    #[serde(rename = "dynamic-tool")]
    DynamicTool {
        /// Tool name
        #[serde(rename = "toolName")]
        tool_name: String,

        /// Tool call ID
        #[serde(rename = "toolCallId")]
        tool_call_id: String,

        /// Optional display title for the tool call
        #[serde(default)]
        title: Option<String>,

        /// Whether the tool was executed by the provider
        #[serde(rename = "providerExecuted", default)]
        provider_executed: bool,

        /// Current state of the tool call
        state: DynamicToolState,

        /// Metadata about the provider call
        #[serde(rename = "callProviderMetadata", default)]
        call_provider_metadata: Option<ProviderMetadata>,

        /// Whether this is a preliminary result
        #[serde(default)]
        preliminary: bool,
    },

    /// URL source reference
    ///
    /// References a URL as a source for the response.
    #[serde(rename = "source-url")]
    SourceUrl {
        #[serde(rename = "sourceId")]
        source_id: String,
        url: String,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Document source reference
    ///
    /// References a document as a source with media type and title.
    #[serde(rename = "source-document")]
    SourceDocument {
        #[serde(rename = "sourceId")]
        source_id: String,
        #[serde(rename = "mediaType")]
        media_type: String,
        title: String,
        #[serde(default)]
        filename: Option<String>,
        #[serde(default)]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Step start marker
    ///
    /// Marks the beginning of a processing step.
    #[serde(rename = "step-start")]
    StepStart,

    /// Custom data part
    ///
    /// Arbitrary custom data attachment with a type name.
    #[serde(rename = "data")]
    Data {
        /// Data type name
        #[serde(rename = "type")]
        data_type: String,

        /// Optional data ID
        #[serde(default)]
        id: Option<String>,

        /// Data content
        data: Value,
    },
}

/// Dynamic tool state for AI SDK 5.x
///
/// Represents the current state of a dynamic tool call through its lifecycle.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum DynamicToolState {
    /// Input is being streamed
    ///
    /// The tool call is receiving input incrementally. The `input` field may
    /// contain partial data.
    InputStreaming {
        #[serde(default)]
        input: Option<Value>,
    },

    /// Input is available
    ///
    /// The complete tool input is now available.
    InputAvailable {
        input: Value,
    },

    /// Output is available
    ///
    /// The tool has been executed and output is ready.
    OutputAvailable {
        input: Value,
        output: Value,
    },

    /// Output error
    ///
    /// The tool execution resulted in an error.
    OutputError {
        #[serde(default)]
        input: Option<Value>,
        #[serde(rename = "errorText")]
        error_text: String,
    },
}

/// Media type classification
///
/// Categorizes media types into broader categories for easier handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    /// Image media type
    Image,

    /// Audio media type
    Audio,

    /// Video media type
    Video,

    /// Document media type
    Document,

    /// Other/unknown media type
    Other,
}

impl UIMessagePart {
    /// Gets the text content if this is a `Text` part.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rig_ai_sdk::UIMessagePart;
    ///
    /// let part = UIMessagePart::Text {
    ///     text: "Hello".to_string(),
    ///     state: None,
    ///     provider_metadata: None,
    /// };
    /// assert_eq!(part.as_text(), Some("Hello"));
    /// ```
    pub fn as_text(&self) -> Option<&str> {
        match self {
            UIMessagePart::Text { text, .. } => Some(text),
            _ => None,
        }
    }

    /// Returns `true` if this is a `Text` part.
    pub fn is_text(&self) -> bool {
        matches!(self, UIMessagePart::Text { .. })
    }

    /// Returns `true` if this is a `Reasoning` part.
    pub fn is_reasoning(&self) -> bool {
        matches!(self, UIMessagePart::Reasoning { .. })
    }

    /// Returns `true` if this is a tool call (including `DynamicTool`).
    pub fn is_tool_call(&self) -> bool {
        matches!(self, UIMessagePart::ToolCall { .. } | UIMessagePart::DynamicTool { .. })
    }

    /// Returns `true` if this is a tool result.
    pub fn is_tool_result(&self) -> bool {
        matches!(self, UIMessagePart::ToolResult { .. })
    }

    /// Gets the file content if this is a `File` part.
    ///
    /// Returns a tuple of `(media_type, url, optional_filename)`.
    pub fn as_file(&self) -> Option<(&str, &str, Option<&String>)> {
        match self {
            UIMessagePart::File {
                media_type,
                url,
                filename,
                ..
            } => Some((media_type, url, filename.as_ref())),
            _ => None,
        }
    }

    /// Gets the streaming state if applicable.
    ///
    /// Returns `Some(state)` for `Text` and `Reasoning` parts, `None` otherwise.
    pub fn state(&self) -> Option<PartState> {
        match self {
            UIMessagePart::Text { state, .. } | UIMessagePart::Reasoning { state, .. } => *state,
            _ => None,
        }
    }

    /// Parses the media type into a [`MediaType`] category.
    ///
    /// Returns `Some(MediaType)` for `File` and `SourceDocument` parts, `None` otherwise.
    pub fn media_type_kind(&self) -> Option<MediaType> {
        match self {
            UIMessagePart::File { media_type, .. }
            | UIMessagePart::SourceDocument { media_type, .. } => {
                if media_type.starts_with("image/") {
                    Some(MediaType::Image)
                } else if media_type.starts_with("audio/") {
                    Some(MediaType::Audio)
                } else if media_type.starts_with("video/") {
                    Some(MediaType::Video)
                } else if matches!(
                    media_type.as_str(),
                    "application/pdf"
                        | "application/msword"
                        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                        | "text/plain"
                        | "text/csv"
                        | "application/json"
                ) {
                    Some(MediaType::Document)
                } else {
                    Some(MediaType::Other)
                }
            }
            _ => None,
        }
    }
}

// ============================================================================
// AI SDK UIMessage
// ============================================================================

/// AI SDK UIMessage format
///
/// Represents a message in the AI SDK format with role, parts, and metadata.
///
/// Reference: [AI SDK Transport](https://ai-sdk.dev/docs/ai-sdk-ui/transport)
#[derive(Debug, Clone, Deserialize)]
pub struct UIMessage {
    /// Unique message ID
    pub id: String,

    /// Message role: "user", "assistant", or "system"
    pub role: String,

    /// Optional message metadata
    #[serde(default)]
    pub metadata: Option<Value>,

    /// Message parts (content items)
    pub parts: Vec<UIMessagePart>,
}

impl UIMessage {
    /// Gets the concatenated text content from all `Text` parts.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rig_ai_sdk::UIMessage;
    ///
    /// let msg = UIMessage {
    ///     id: "1".to_string(),
    ///     role: "user".to_string(),
    ///     parts: vec![
    ///         UIMessagePart::Text { text: "Hello, ".to_string(), state: None, provider_metadata: None },
    ///         UIMessagePart::Text { text: "world!".to_string(), state: None, provider_metadata: None },
    ///     ],
    ///     metadata: None,
    /// };
    /// assert_eq!(msg.text(), "Hello, world!");
    /// ```
    pub fn text(&self) -> String {
        self.parts
            .iter()
            .filter_map(|p| p.as_text())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Returns `true` if the message role is "user".
    pub fn is_user(&self) -> bool {
        self.role == "user"
    }

    /// Returns `true` if the message role is "assistant".
    pub fn is_assistant(&self) -> bool {
        self.role == "assistant"
    }

    /// Returns `true` if the message role is "system".
    pub fn is_system(&self) -> bool {
        self.role == "system"
    }

    /// Gets all parts matching the given predicate.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rig_ai_sdk::UIMessage;
    ///
    /// let tool_parts = msg.get_parts_by_type(|p| p.is_tool_call());
    /// ```
    pub fn get_parts_by_type<F>(&self, predicate: F) -> Vec<&UIMessagePart>
    where
        F: Fn(&UIMessagePart) -> bool,
    {
        self.parts.iter().filter(|p| predicate(p)).collect()
    }

    /// Returns `true` if the message contains streaming content.
    ///
    /// Checks if any part has `PartState::Streaming`.
    pub fn has_streaming_content(&self) -> bool {
        self.parts.iter().any(|p| p.state() == Some(PartState::Streaming))
    }

    /// Returns `true` if the message contains tool calls.
    pub fn has_tool_calls(&self) -> bool {
        self.parts.iter().any(|p| p.is_tool_call())
    }

    /// Returns `true` if the message contains tool results.
    pub fn has_tool_results(&self) -> bool {
        self.parts.iter().any(|p| p.is_tool_result())
    }

    /// Returns `true` if the message contains file attachments.
    pub fn has_files(&self) -> bool {
        self.parts.iter().any(|p| p.as_file().is_some())
    }
}
