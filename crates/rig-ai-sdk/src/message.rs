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

use serde::{Deserialize, Deserializer, Serialize};
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
#[serde(rename_all = "lowercase")]
pub enum PartState {
    /// Content is currently being streamed
    Streaming,

    /// Content is complete
    #[default]
    Done,
}

// ============================================================================
// AI SDK UIMessage Part types
// ============================================================================

/// Text content part
///
/// Plain text content with optional streaming state and provider metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPart {
    pub text: String,
    #[serde(default)]
    pub state: Option<PartState>,
    #[serde(default)]
    pub provider_metadata: Option<ProviderMetadata>,
}

/// Reasoning/thinking part
///
/// Model reasoning or thinking block content (e.g., for o1-style models).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReasoningPart {
    /// Reasoning/thinking content
    pub text: String,
    #[serde(default)]
    pub state: Option<PartState>,
    #[serde(default)]
    pub provider_metadata: Option<ProviderMetadata>,
}

/// File attachment part (supports any media type)
///
/// Represents a file attachment with URL, media type, and optional filename.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePart {
    pub media_type: String,
    pub url: String,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub provider_metadata: Option<ProviderMetadata>,
}

/// Tool call (legacy format for compatibility)
///
/// Legacy tool call format without streaming state support.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallPart {
    pub tool_call_id: String,
    pub tool_name: String,
    pub args: Value,
    #[serde(default)]
    pub provider_metadata: Option<ProviderMetadata>,
}

/// Tool result
///
/// Result from a previous tool execution.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultPart {
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_name: Option<String>,
    pub result: Value,
}

/// Dynamic tool call (AI SDK 5.x format with state support)
///
/// Modern tool call format with streaming state support and provider execution info.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicToolPart {
    pub tool_name: String,
    pub tool_call_id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub provider_executed: bool,
    pub state: DynamicToolState,
    #[serde(default)]
    pub call_provider_metadata: Option<ProviderMetadata>,
    #[serde(default)]
    pub preliminary: bool,
}

/// URL source reference
///
/// References a URL as a source for the response.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceUrlPart {
    pub source_id: String,
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub provider_metadata: Option<ProviderMetadata>,
}

/// Document source reference
///
/// References a document as a source with media type and title.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDocumentPart {
    pub source_id: String,
    pub media_type: String,
    pub title: String,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub provider_metadata: Option<ProviderMetadata>,
}

/// Custom data part (supports dynamic `data-{name}` tags)
///
/// Arbitrary custom data attachment with a type name.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPart {
    /// Data type name (the `{name}` in `data-{name}`)
    pub data_type: String,
    #[serde(default)]
    pub id: Option<String>,
    pub data: Value,
}

/// AI SDK UIMessage part types.
///
/// A message consists of one or more parts, each representing a different
/// type of content (text, file, tool call, reasoning, etc.).
///
/// The format uses a `type` field to distinguish between different part types:
/// ```json
/// { "type": "text", "text": "Hello" }
/// { "type": "file", "mediaType": "image/png", "url": "..." }
/// { "type": "data-usage", "data": {...} }
/// ```
#[derive(Debug, Clone, Serialize)]
pub enum UIMessagePart {
    /// Text content
    Text(TextPart),
    /// Reasoning/thinking content
    Reasoning(ReasoningPart),
    /// File attachment
    File(FilePart),
    /// Legacy tool call
    ToolCall(ToolCallPart),
    /// Tool result
    ToolResult(ToolResultPart),
    /// Dynamic tool call
    DynamicTool(DynamicToolPart),
    /// URL source
    SourceUrl(SourceUrlPart),
    /// Document source
    SourceDocument(SourceDocumentPart),
    /// Step start marker
    StepStart,
    /// Dynamic data part (data-{name} pattern)
    Data(DataPart),
}

/// Helper struct for deserialization of UIMessagePart with type field
///
/// Uses `tag = "type"` to handle standard types: text, reasoning, file,
/// tool-call, tool-result, dynamic-tool, source-url, source-document, step-start
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum UIMessagePartTagged {
    Text(TextPart),
    Reasoning(ReasoningPart),
    File(FilePart),
    ToolCall(ToolCallPart),
    ToolResult(ToolResultPart),
    DynamicTool(DynamicToolPart),
    SourceUrl(SourceUrlPart),
    SourceDocument(SourceDocumentPart),
    StepStart,
}

impl<'de> Deserialize<'de> for UIMessagePart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = serde_json::Value::deserialize(deserializer)?;

        // Check if this is a data-{name} type
        if let Some(t) = raw.get("type").and_then(|v| v.as_str()) {
            if t.starts_with("data-") {
                let data_part = DataPart {
                    data_type: t.strip_prefix("data-").unwrap_or(t).to_string(),
                    id: raw.get("id").and_then(|v| v.as_str()).map(String::from),
                    data: raw.get("data").cloned().unwrap_or(Value::Null),
                };
                return Ok(UIMessagePart::Data(data_part));
            }
        }

        // Otherwise, try tagged deserialization
        let tagged: Result<UIMessagePartTagged, _> =
            serde_json::from_value(raw.clone()).map_err(serde::de::Error::custom);

        match tagged {
            Ok(tagged) => Ok(match tagged {
                UIMessagePartTagged::Text(v) => UIMessagePart::Text(v),
                UIMessagePartTagged::Reasoning(v) => UIMessagePart::Reasoning(v),
                UIMessagePartTagged::File(v) => UIMessagePart::File(v),
                UIMessagePartTagged::ToolCall(v) => UIMessagePart::ToolCall(v),
                UIMessagePartTagged::ToolResult(v) => UIMessagePart::ToolResult(v),
                UIMessagePartTagged::DynamicTool(v) => UIMessagePart::DynamicTool(v),
                UIMessagePartTagged::SourceUrl(v) => UIMessagePart::SourceUrl(v),
                UIMessagePartTagged::SourceDocument(v) => UIMessagePart::SourceDocument(v),
                UIMessagePartTagged::StepStart => UIMessagePart::StepStart,
            }),
            Err(e) => Err(e),
        }
    }
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
    InputAvailable { input: Value },

    /// Output is available
    ///
    /// The tool has been executed and output is ready.
    OutputAvailable { input: Value, output: Value },

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
            UIMessagePart::Text(p) => Some(&p.text),
            _ => None,
        }
    }

    /// Returns `true` if this is a `Text` part.
    pub fn is_text(&self) -> bool {
        matches!(self, UIMessagePart::Text(_))
    }

    /// Returns `true` if this is a `Reasoning` part.
    pub fn is_reasoning(&self) -> bool {
        matches!(self, UIMessagePart::Reasoning(_))
    }

    /// Returns `true` if this is a tool call (including `DynamicTool`).
    pub fn is_tool_call(&self) -> bool {
        matches!(
            self,
            UIMessagePart::ToolCall(_) | UIMessagePart::DynamicTool(_)
        )
    }

    /// Returns `true` if this is a tool result.
    pub fn is_tool_result(&self) -> bool {
        matches!(self, UIMessagePart::ToolResult(_))
    }

    /// Returns `true` if this is a `Data` part.
    pub fn is_data(&self) -> bool {
        matches!(self, UIMessagePart::Data(_))
    }

    /// Gets the data content if this is a `Data` part.
    pub fn as_data(&self) -> Option<&DataPart> {
        match self {
            UIMessagePart::Data(p) => Some(p),
            _ => None,
        }
    }

    /// Gets the file content if this is a `File` part.
    ///
    /// Returns a tuple of `(media_type, url, optional_filename)`.
    pub fn as_file(&self) -> Option<(&str, &str, Option<&String>)> {
        match self {
            UIMessagePart::File(p) => Some((&p.media_type, &p.url, p.filename.as_ref())),
            _ => None,
        }
    }

    /// Gets the streaming state if applicable.
    ///
    /// Returns `Some(state)` for `Text` and `Reasoning` parts, `None` otherwise.
    pub fn state(&self) -> Option<PartState> {
        match self {
            UIMessagePart::Text(p) => p.state,
            UIMessagePart::Reasoning(p) => p.state,
            _ => None,
        }
    }

    /// Parses the media type into a [`MediaType`] category.
    ///
    /// Returns `Some(MediaType)` for `File` and `SourceDocument` parts, `None` otherwise.
    pub fn media_type_kind(&self) -> Option<MediaType> {
        match self {
            UIMessagePart::File(p) => {
                if p.media_type.starts_with("image/") {
                    Some(MediaType::Image)
                } else if p.media_type.starts_with("audio/") {
                    Some(MediaType::Audio)
                } else if p.media_type.starts_with("video/") {
                    Some(MediaType::Video)
                } else if matches!(
                    p.media_type.as_str(),
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
            UIMessagePart::SourceDocument(p) => {
                if p.media_type.starts_with("image/") {
                    Some(MediaType::Image)
                } else if p.media_type.starts_with("audio/") {
                    Some(MediaType::Audio)
                } else if p.media_type.starts_with("video/") {
                    Some(MediaType::Video)
                } else if matches!(
                    p.media_type.as_str(),
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
        self.parts
            .iter()
            .any(|p| p.state() == Some(PartState::Streaming))
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
