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
//! - `tool-output-available` - Tool execution result
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
//! use uuid::Uuid;
//!
//! // Start a message
//! let start = AISdkEvent::Start { message_id: Uuid::new_v4() };
//!
//! // Send text
//! let delta = AISdkEvent::TextDelta {
//!     id: Uuid::new_v4(),
//!     delta: "Hello, ".to_string(),
//! };
//!
//! // Custom error
//! let error = AISdkEvent::error("Something went wrong");
//!
//! // Custom data
//! let usage = AISdkEvent::custom_data("usage", json!({"tokens": 100}));
//! ```

use serde::Serialize;
use serde_json::{json, Value};
use std::fmt::Display;
use uuid::Uuid;

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
    Start { message_id: Uuid },

    /// Text block start
    ///
    /// Marks the beginning of a new text content block.
    TextStart { id: Uuid },

    /// Text delta
    ///
    /// Incremental text content for streaming responses.
    TextDelta { id: Uuid, delta: String },

    /// Text block end
    ///
    /// Marks the end of the current text block.
    TextEnd { id: Uuid },

    /// Reasoning block start
    ///
    /// Marks the beginning of a reasoning/thinking block (e.g., for o1-style models).
    ReasoningStart { id: Uuid },

    /// Reasoning delta
    ///
    /// Incremental reasoning content for streaming model thinking.
    ReasoningDelta { id: Uuid, delta: String },

    /// Reasoning block end
    ///
    /// Marks the end of the current reasoning block.
    ReasoningEnd { id: Uuid },

    /// URL source reference
    ///
    /// References a URL as a source for the response content.
    SourceUrl { source_id: String, url: String },

    /// Document source reference
    ///
    /// References a document as a source with media type and title.
    SourceDocument {
        source_id: String,
        media_type: String,
        title: String,
    },

    /// File attachment
    ///
    /// Represents a file with URL and media type.
    File { url: String, media_type: String },

    /// Tool input start
    ///
    /// Indicates the start of a tool call streaming.
    ToolInputStart {
        tool_call_id: String,
        tool_name: String,
    },

    /// Tool input delta
    ///
    /// Incremental tool arguments during streaming.
    ToolInputDelta { tool_call_id: String, delta: String },

    /// Tool input available
    ///
    /// Complete tool call with full arguments available.
    ToolInputAvailable {
        tool_call_id: String,
        tool_name: String,
        input: Value,
    },

    /// Tool output available
    ///
    /// Result from tool execution.
    ToolOutputAvailable { tool_call_id: String, output: Value },

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
    Finish,

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
            AISdkEvent::Start { message_id } => {
                json!({"type": "start", "messageId": message_id.to_string()})
            }
            AISdkEvent::TextStart { id } => {
                json!({"type": "text-start", "id": id.to_string()})
            }
            AISdkEvent::TextDelta { id, delta } => {
                json!({"type": "text-delta", "id": id.to_string(), "delta": delta})
            }
            AISdkEvent::TextEnd { id } => {
                json!({"type": "text-end", "id": id.to_string()})
            }
            AISdkEvent::ReasoningStart { id } => {
                json!({"type": "reasoning-start", "id": id.to_string()})
            }
            AISdkEvent::ReasoningDelta { id, delta } => {
                json!({"type": "reasoning-delta", "id": id.to_string(), "delta": delta})
            }
            AISdkEvent::ReasoningEnd { id } => {
                json!({"type": "reasoning-end", "id": id.to_string()})
            }
            AISdkEvent::SourceUrl { source_id, url } => {
                json!({"type": "source-url", "sourceId": source_id, "url": url})
            }
            AISdkEvent::SourceDocument {
                source_id,
                media_type,
                title,
            } => {
                json!({"type": "source-document", "sourceId": source_id, "mediaType": media_type, "title": title})
            }
            AISdkEvent::File { url, media_type } => {
                json!({"type": "file", "url": url, "mediaType": media_type})
            }
            AISdkEvent::ToolInputStart {
                tool_call_id,
                tool_name,
            } => {
                json!({"type": "tool-input-start", "toolCallId": tool_call_id, "toolName": tool_name})
            }
            AISdkEvent::ToolInputDelta {
                tool_call_id,
                delta,
            } => {
                json!({"type": "tool-input-delta", "toolCallId": tool_call_id, "inputTextDelta": delta})
            }
            AISdkEvent::ToolInputAvailable {
                tool_call_id,
                tool_name,
                input,
            } => {
                json!({"type": "tool-input-available", "toolCallId": tool_call_id, "toolName": tool_name, "input": input})
            }
            AISdkEvent::ToolOutputAvailable {
                tool_call_id,
                output,
            } => {
                json!({"type": "tool-output-available", "toolCallId": tool_call_id, "output": output})
            }
            AISdkEvent::CustomData { name, data } => {
                json!({"type": format!("data-{}", name), "data": data})
            }
            AISdkEvent::StartStep => json!({"type": "start-step"}),
            AISdkEvent::FinishStep => json!({"type": "finish-step"}),
            AISdkEvent::Finish => json!({"type": "finish"}),
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
