//! AI SDK Data Stream Protocol events
//!
//! Implements the [AI SDK Data Stream Protocol](https://sdk.vercel.ai/docs/ai-sdk-ui/stream-protocol)
//! event types for streaming responses to frontend clients.
//!
//! # Event Types
//!
//! The protocol supports the following event categories:
//!
//! ## Stream Lifecycle
//! - `start` - Initial message acknowledgment with message ID
//! - `finish` - Stream completion with final status
//! - `done` - Final marker (sent as `[DONE]`)
//! - `abort` - Stream terminated with reason
//! - `error` - Error occurred during streaming
//! - `message-metadata` - Message metadata update
//!
//! ## Text Content
//! - `text-start` - Beginning of a text block
//! - `text-delta` - Incremental text content
//! - `text-end` - End of a text block
//!
//! ## Reasoning/Thinking
//! - `reasoning-start` - Beginning of reasoning block
//! - `reasoning-delta` - Incremental reasoning content
//! - `reasoning-end` - End of reasoning block
//!
//! ## Tool Calls
//! - `tool-input-start` - Tool call started
//! - `tool-input-delta` - Incremental tool arguments
//! - `tool-input-available` - Complete tool call available
//! - `tool-input-error` - Tool input error
//! - `tool-output-available` - Tool execution result
//! - `tool-output-error` - Tool output error
//!
//! ## Sources & Files
//! - `source-url` - URL source reference
//! - `source-document` - Document source reference
//! - `file` - File attachment
//!
//! ## Steps & Custom Data
//! - `start-step` / `finish-step` - Step markers
//! - `data-{name}` - Custom data events
//!
//! # Examples
//!
//! ## Creating Events
//!
//! ```ignore
//! use rig_ai_sdk::AISdkEvent;
//!
//! // Start a message
//! let start = AISdkEvent::Start {
//!     message_id: "msg-123".to_string(),
//!     message_metadata: None,
//!     provider_metadata: None,
//! };
//!
//! // Send text
//! let delta = AISdkEvent::TextDelta {
//!     id: "text-456".to_string(),
//!     delta: "Hello, ".to_string(),
//!     provider_metadata: None,
//! };
//!
//! // Custom error
//! let error = AISdkEvent::error("Something went wrong");
//!
//! // Custom data
//! let usage = AISdkEvent::custom_data("usage", json!({"tokens": 100}));
//! ```

use serde::Serialize;
use serde_json::{Value, json};
use std::fmt::Display;

/// Provider metadata from the AI provider.
pub type ProviderMetadata = Value;

/// Message metadata.
pub type MessageMetadata = Value;

/// Finish reason for the stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishReason {
    /// Stream stopped naturally.
    Stop,
    /// Maximum token limit reached.
    Length,
    /// Model requested tool use.
    ToolCalls,
    /// Content filtered by safety filters.
    ContentFilter,
    /// Stream was stopped for unknown reasons.
    Unknown,
}

impl Display for FinishReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinishReason::Stop => write!(f, "stop"),
            FinishReason::Length => write!(f, "length"),
            FinishReason::ToolCalls => write!(f, "tool-calls"),
            FinishReason::ContentFilter => write!(f, "content-filter"),
            FinishReason::Unknown => write!(f, "unknown"),
        }
    }
}

impl Serialize for FinishReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// AI SDK Data Stream Protocol events.
///
/// This enum represents all event types in the AI SDK Data Stream Protocol.
/// Events are serialized to JSON when converted to SSE format.
///
/// Reference: [AI SDK Stream Protocol](https://sdk.vercel.ai/docs/ai-sdk-ui/stream-protocol)
#[derive(Debug, Clone)]
pub enum AISdkEvent {
    /// Message start event
    ///
    /// Sent first to initialize the stream with a unique message ID.
    Start {
        message_id: String,
        message_metadata: Option<MessageMetadata>,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Text block start
    ///
    /// Marks the beginning of a new text content block.
    TextStart {
        id: String,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Text delta
    ///
    /// Incremental text content for streaming responses.
    TextDelta {
        id: String,
        delta: String,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Text block end
    ///
    /// Marks the end of the current text block.
    TextEnd {
        id: String,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Reasoning block start
    ///
    /// Marks the beginning of a reasoning/thinking block (e.g., for o1-style models).
    ReasoningStart {
        id: String,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Reasoning delta
    ///
    /// Incremental reasoning content for streaming model thinking.
    ReasoningDelta {
        id: String,
        delta: String,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Reasoning block end
    ///
    /// Marks the end of the current reasoning block.
    ReasoningEnd {
        id: String,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// URL source reference
    ///
    /// References a URL as a source for the response content.
    SourceUrl {
        source_id: String,
        url: String,
        title: Option<String>,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Document source reference
    ///
    /// References a document as a source with media type and title.
    SourceDocument {
        source_id: String,
        media_type: String,
        title: String,
        filename: Option<String>,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// File attachment
    ///
    /// Represents a file with URL and media type.
    File {
        url: String,
        media_type: String,
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Tool input start
    ///
    /// Indicates the start of a tool call streaming.
    ToolInputStart {
        tool_call_id: String,
        tool_name: String,
        provider_executed: Option<bool>,
        provider_metadata: Option<ProviderMetadata>,
        dynamic: Option<bool>,
    },

    /// Tool input delta
    ///
    /// Incremental tool arguments during streaming.
    ToolInputDelta {
        tool_call_id: String,
        delta: String,
    },

    /// Tool input available
    ///
    /// Complete tool call with full arguments available.
    ToolInputAvailable {
        tool_call_id: String,
        tool_name: String,
        input: Value,
        provider_executed: Option<bool>,
        provider_metadata: Option<ProviderMetadata>,
        dynamic: Option<bool>,
    },

    /// Tool input error
    ///
    /// Error occurred during tool input processing.
    ToolInputError {
        tool_call_id: String,
        tool_name: String,
        input: Value,
        error_text: String,
        provider_executed: Option<bool>,
        provider_metadata: Option<ProviderMetadata>,
        dynamic: Option<bool>,
    },

    /// Tool output available
    ///
    /// Result from tool execution.
    ToolOutputAvailable {
        tool_call_id: String,
        output: Value,
        provider_executed: Option<bool>,
        dynamic: Option<bool>,
        preliminary: Option<bool>,
    },

    /// Tool output error
    ///
    /// Error occurred during tool output processing.
    ToolOutputError {
        tool_call_id: String,
        error_text: String,
        provider_executed: Option<bool>,
        dynamic: Option<bool>,
    },

    /// Message metadata update
    ///
    /// Updates the metadata for the current message.
    MessageMetadata {
        message_metadata: MessageMetadata,
    },

    /// Custom data event
    ///
    /// Generic data event with custom type name (serialized as `data-{name}`).
    CustomData { name: String, data: Value },

    /// Step start marker
    ///
    /// Marks the beginning of a processing step.
    StartStep,

    /// Step finish marker
    ///
    /// Marks the completion of a processing step.
    FinishStep,

    /// Stream finish
    ///
    /// Indicates the stream has finished successfully.
    Finish {
        finish_reason: Option<FinishReason>,
        message_metadata: Option<MessageMetadata>,
    },

    /// Stream abort
    ///
    /// Indicates the stream was terminated with a reason.
    Abort { reason: String },

    /// Error event
    ///
    /// Indicates an error occurred during streaming.
    Error { error_text: String },

    /// Stream done marker
    ///
    /// Final marker sent after all events. Serialized as `[DONE]`.
    Done,
}

impl AISdkEvent {
    /// Creates an error event from any type that can be converted to a string.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rig_ai_sdk::AISdkEvent;
    ///
    /// let error = AISdkEvent::error("Something went wrong");
    /// ```
    pub fn error(error: impl Into<String>) -> AISdkEvent {
        AISdkEvent::Error {
            error_text: error.into(),
        }
    }

    /// Creates a custom data event.
    ///
    /// The data will be serialized to JSON and the event type will be `data-{name}`.
    ///
    /// # Type Parameters
    ///
    /// - `V`: Any type that implements [`Serialize`]
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rig_ai_sdk::AISdkEvent;
    /// use serde_json::json;
    ///
    /// let usage = AISdkEvent::custom_data("usage", json!({
    ///     "prompt_tokens": 100,
    ///     "completion_tokens": 50
    /// }));
    /// ```
    pub fn custom_data<V: Serialize>(name: impl Into<String>, data: V) -> AISdkEvent {
        AISdkEvent::CustomData {
            name: name.into(),
            data: serde_json::to_value(data).unwrap_or_default(),
        }
    }
}

/// Formats the event as a JSON string (or `[DONE]` for the `Done` variant).
///
/// This is used when converting events to SSE format.
impl Display for AISdkEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AISdkEvent::Start {
                message_id,
                message_metadata,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "start", "messageId": message_id});
                if let Some(metadata) = message_metadata {
                    obj["messageMetadata"] = metadata.clone();
                }
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::TextStart {
                id,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "text-start", "id": id});
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::TextDelta {
                id,
                delta,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "text-delta", "id": id, "delta": delta});
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::TextEnd {
                id,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "text-end", "id": id});
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::ReasoningStart {
                id,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "reasoning-start", "id": id});
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::ReasoningDelta {
                id,
                delta,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "reasoning-delta", "id": id, "delta": delta});
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::ReasoningEnd {
                id,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "reasoning-end", "id": id});
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::SourceUrl {
                source_id,
                url,
                title,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "source-url", "sourceId": source_id, "url": url});
                if let Some(t) = title {
                    obj["title"] = json!(t);
                }
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::SourceDocument {
                source_id,
                media_type,
                title,
                filename,
                provider_metadata,
            } => {
                let mut obj = json!({
                    "type": "source-document",
                    "sourceId": source_id,
                    "mediaType": media_type,
                    "title": title
                });
                if let Some(fn_name) = filename {
                    obj["filename"] = json!(fn_name);
                }
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::File {
                url,
                media_type,
                provider_metadata,
            } => {
                let mut obj = json!({"type": "file", "url": url, "mediaType": media_type});
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::ToolInputStart {
                tool_call_id,
                tool_name,
                provider_executed,
                provider_metadata,
                dynamic,
            } => {
                let mut obj = json!({
                    "type": "tool-input-start",
                    "toolCallId": tool_call_id,
                    "toolName": tool_name
                });
                if let Some(executed) = provider_executed {
                    obj["providerExecuted"] = json!(executed);
                }
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                if let Some(d) = dynamic {
                    obj["dynamic"] = json!(d);
                }
                obj
            }
            AISdkEvent::ToolInputDelta {
                tool_call_id,
                delta,
            } => {
                json!({
                    "type": "tool-input-delta",
                    "toolCallId": tool_call_id,
                    "inputTextDelta": delta
                })
            }
            AISdkEvent::ToolInputAvailable {
                tool_call_id,
                tool_name,
                input,
                provider_executed,
                provider_metadata,
                dynamic,
            } => {
                let mut obj = json!({
                    "type": "tool-input-available",
                    "toolCallId": tool_call_id,
                    "toolName": tool_name,
                    "input": input
                });
                if let Some(executed) = provider_executed {
                    obj["providerExecuted"] = json!(executed);
                }
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                if let Some(d) = dynamic {
                    obj["dynamic"] = json!(d);
                }
                obj
            }
            AISdkEvent::ToolInputError {
                tool_call_id,
                tool_name,
                input,
                error_text,
                provider_executed,
                provider_metadata,
                dynamic,
            } => {
                let mut obj = json!({
                    "type": "tool-input-error",
                    "toolCallId": tool_call_id,
                    "toolName": tool_name,
                    "input": input,
                    "errorText": error_text
                });
                if let Some(executed) = provider_executed {
                    obj["providerExecuted"] = json!(executed);
                }
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                if let Some(d) = dynamic {
                    obj["dynamic"] = json!(d);
                }
                obj
            }
            AISdkEvent::ToolOutputAvailable {
                tool_call_id,
                output,
                provider_executed,
                dynamic,
                preliminary,
            } => {
                let mut obj = json!({
                    "type": "tool-output-available",
                    "toolCallId": tool_call_id,
                    "output": output
                });
                if let Some(executed) = provider_executed {
                    obj["providerExecuted"] = json!(executed);
                }
                if let Some(d) = dynamic {
                    obj["dynamic"] = json!(d);
                }
                if let Some(p) = preliminary {
                    obj["preliminary"] = json!(p);
                }
                obj
            }
            AISdkEvent::ToolOutputError {
                tool_call_id,
                error_text,
                provider_executed,
                dynamic,
            } => {
                let mut obj = json!({
                    "type": "tool-output-error",
                    "toolCallId": tool_call_id,
                    "errorText": error_text
                });
                if let Some(executed) = provider_executed {
                    obj["providerExecuted"] = json!(executed);
                }
                if let Some(d) = dynamic {
                    obj["dynamic"] = json!(d);
                }
                obj
            }
            AISdkEvent::MessageMetadata { message_metadata } => {
                json!({"type": "message-metadata", "messageMetadata": message_metadata})
            }
            AISdkEvent::CustomData { name, data } => {
                json!({"type": format!("data-{}", name), "data": data})
            }
            AISdkEvent::StartStep => json!({"type": "start-step"}),
            AISdkEvent::FinishStep => json!({"type": "finish-step"}),
            AISdkEvent::Finish {
                finish_reason,
                message_metadata,
            } => {
                let mut obj = json!({"type": "finish"});
                if let Some(reason) = finish_reason {
                    obj["finishReason"] = json!(reason);
                }
                if let Some(metadata) = message_metadata {
                    obj["messageMetadata"] = metadata.clone();
                }
                obj
            }
            AISdkEvent::Abort { reason } => json!({"type": "abort", "reason": reason}),
            AISdkEvent::Error { error_text } => json!({"type": "error", "errorText": error_text}),
            AISdkEvent::Done => return write!(f, "[DONE]"),
        };
        write!(f, "{}", value)
    }
}

/// Converts an [`AISdkEvent`] to an Axum SSE event.
///
/// This implementation is available with the `axum-sse` feature.
#[cfg(feature = "axum-sse")]
impl From<AISdkEvent> for axum::response::sse::Event {
    fn from(value: AISdkEvent) -> Self {
        axum::response::sse::Event::default().data(value.to_string())
    }
}
