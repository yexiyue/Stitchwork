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
    pub state: Option<PartState>,
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
    pub state: Option<PartState>,
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
    pub filename: Option<String>,
    pub provider_metadata: Option<ProviderMetadata>,
}

/// Tool state as string literal (AI SDK 5.x format)
///
/// Represents the current state of a tool invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolState {
    /// Input is being streamed
    InputStreaming,
    /// Input is available
    InputAvailable,
    /// Output is available
    OutputAvailable,
    /// Output error
    OutputError,
}

/// Tool invocation part (AI SDK 5.x ToolUIPart format)
///
/// Modern tool call format with streaming state support and provider execution info.
/// The `type` field uses dynamic format: "tool-{toolName}"
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolPart {
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_name: String,
    pub state: ToolState,
    pub title: Option<String>,
    pub provider_executed: Option<bool>,
    pub call_provider_metadata: Option<ProviderMetadata>,
    pub preliminary: Option<bool>,
    pub input: Option<Value>,
    pub output: Option<Value>,
    pub raw_input: Option<Value>,
    pub error_text: Option<String>,
}

/// URL source reference
///
/// References a URL as a source for the response.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceUrlPart {
    pub source_id: String,
    pub url: String,
    pub title: Option<String>,
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
    pub filename: Option<String>,
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
/// { "type": "tool-get_weather", "toolCallId": "...", "state": "input-available" }
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
    /// Tool invocation (AI SDK 5.x ToolUIPart format)
    Tool(ToolPart),
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
/// source-url, source-document, step-start. Tool parts are handled separately
/// due to dynamic type field (tool-{NAME}).
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum UIMessagePartTagged {
    Text(TextPart),
    Reasoning(ReasoningPart),
    File(FilePart),
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

        // Check if this is a data-{name} or tool-{NAME} type
        if let Some(t) = raw.get("type").and_then(|v| v.as_str()) {
            if t.starts_with("data-") {
                let data_part = DataPart {
                    data_type: t.strip_prefix("data-").unwrap_or(t).to_string(),
                    id: raw.get("id").and_then(|v| v.as_str()).map(String::from),
                    data: raw.get("data").cloned().unwrap_or(Value::Null),
                };
                return Ok(UIMessagePart::Data(data_part));
            }

            // Handle tool-{NAME} type (AI SDK 5.x ToolUIPart format)
            if t.starts_with("tool-") {
                let tool_name = t.strip_prefix("tool-").unwrap_or(t).to_string();
                let mut tool_part: ToolPart =
                    serde_json::from_value(raw.clone()).map_err(serde::de::Error::custom)?;
                // Set tool_name from the type field
                tool_part.tool_name = tool_name;
                return Ok(UIMessagePart::Tool(tool_part));
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
                UIMessagePartTagged::SourceUrl(v) => UIMessagePart::SourceUrl(v),
                UIMessagePartTagged::SourceDocument(v) => UIMessagePart::SourceDocument(v),
                UIMessagePartTagged::StepStart => UIMessagePart::StepStart,
            }),
            Err(e) => Err(e),
        }
    }
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

    /// Returns `true` if this is a tool invocation.
    pub fn is_tool(&self) -> bool {
        matches!(self, UIMessagePart::Tool(_))
    }

    /// Gets the tool part if this is a Tool.
    pub fn as_tool(&self) -> Option<&ToolPart> {
        match self {
            UIMessagePart::Tool(p) => Some(p),
            _ => None,
        }
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

    /// Returns `true` if the message contains tool invocations.
    pub fn has_tool_calls(&self) -> bool {
        self.parts.iter().any(|p| p.is_tool())
    }

    /// Returns `true` if the message contains file attachments.
    pub fn has_files(&self) -> bool {
        self.parts.iter().any(|p| p.as_file().is_some())
    }
}
