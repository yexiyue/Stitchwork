//! AI SDK UIMessage to rig::message::Message conversion
//!
//! Converts frontend AI SDK UIMessage format to rig backend Message format.
//! Supports AI SDK 5.x with streaming states, dynamic tools, and rich file types.
//!
//! # Overview
//!
//! This module provides conversion functions to transform AI SDK messages into rig's
//! internal message format. The conversion supports:
//!
//! - Text, reasoning, and image content
//! - Legacy tool calls and AI SDK 5.x dynamic tools
//! - Tool results (including errors)
//! - Multi-part messages
//!
//! # Conversion Mapping
//!
//! | AI SDK Part | Rig Content |
//! |--------------|-------------|
//! | `Text` | `UserContent::Text` / `AssistantContent::Text` |
//! | `Reasoning` | `AssistantContent::Reasoning` |
//! | `File` (image) | `UserContent::Image` |
//! | `ToolCall` | `AssistantContent::ToolCall` |
//! | `DynamicTool` (InputAvailable) | `AssistantContent::ToolCall` |
//! | `DynamicTool` (OutputAvailable) | `UserContent::ToolResult` |
//! | `DynamicTool` (OutputError) | `UserContent::ToolResult` |
//! | `ToolResult` | `UserContent::ToolResult` |
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```ignore
//! use rig_ai_sdk::{convert_messages, ChatRequest};
//!
//! let messages = convert_messages(&request.messages)?;
//! ```
//!
//! ## Extracting Prompt and History
//!
//! ```ignore
//! use rig_ai_sdk::extract_prompt_and_history;
//!
//! let (prompt, history) = extract_prompt_and_history(&request)?;
//! // prompt = last message
//! // history = all previous messages
//! ```

use anyhow::{Result, anyhow};
use rig::OneOrMany;
use rig::message::{
    AssistantContent, DocumentSourceKind, Image, ImageMediaType, Message, ToolCall, ToolFunction,
    ToolResult, ToolResultContent, UserContent,
};
use serde_json::Value;

use crate::message::{ToolState, UIMessage, UIMessagePart};

/// Extracts the prompt (last message) and history (previous messages) from a message list.
///
/// This is a convenience function that splits the message list into the final prompt
/// and the conversation history.
///
/// # Errors
///
/// Returns an error if the message list is empty.
///
/// # Example
///
/// ```ignore
/// use rig_ai_sdk::extract_prompt_and_history;
///
/// let (prompt, history) = extract_prompt_and_history(&messages)?;
/// assert_eq!(history.len(), 2);  // First two messages
/// ```
pub fn extract_prompt_and_history(messages: &[UIMessage]) -> Result<(Message, Vec<Message>)> {
    let messages = convert_messages(messages)?;
    let (prompt, history) = messages
        .split_last()
        .ok_or_else(|| anyhow!("Message list is empty"))?;
    Ok((prompt.clone(), history.to_vec()))
}

/// Converts multiple [`UIMessage`]s to [`Message`]s.
///
/// Returns an error if any message conversion fails.
pub fn convert_messages(messages: &[UIMessage]) -> Result<Vec<Message>> {
    messages.iter().map(convert_message).collect()
}

/// Converts a single [`UIMessage`] to a [`Message`].
///
/// The conversion depends on the message role:
///
/// - `"user"` or `"system"` → [`Message::User`]
/// - `"assistant"` → [`Message::Assistant`]
///
/// # Errors
///
/// Returns an error if the role is not supported.
pub fn convert_message(msg: &UIMessage) -> Result<Message> {
    match msg.role.as_str() {
        "user" | "system" => {
            let contents: Vec<_> = msg.parts.iter().filter_map(to_user_content).collect();
            Ok(Message::User {
                content: OneOrMany::many(contents)
                    .unwrap_or_else(|_| OneOrMany::one(UserContent::text(""))),
            })
        }
        "assistant" => {
            let contents: Vec<_> = msg.parts.iter().filter_map(to_assistant_content).collect();
            Ok(Message::Assistant {
                // Don't use the frontend ID here - OpenAI manages its own message IDs
                // using the 'msg_' prefix format internally
                id: None,
                content: OneOrMany::many(contents)
                    .unwrap_or_else(|_| OneOrMany::one(AssistantContent::text(""))),
            })
        }
        role => Err(anyhow!("Unsupported message role: {}", role)),
    }
}

/// Converts a [`UIMessagePart`] to a [`UserContent`] if applicable.
///
/// Returns `None` for parts that don't map to user content:
///
/// - `Text` → `UserContent::Text`
/// - `Tool` (OutputAvailable/Error) → `UserContent::ToolResult`
/// - `File` (image only) → `UserContent::Image`
fn to_user_content(part: &UIMessagePart) -> Option<UserContent> {
    match part {
        // Text part
        UIMessagePart::Text(p) => Some(UserContent::text(p.text.clone())),

        // Tool result (only OutputAvailable and OutputError convert to UserContent)
        UIMessagePart::Tool(p) => {
            match p.state {
                ToolState::OutputAvailable => {
                    if let Some(output) = &p.output {
                        Some(UserContent::ToolResult(ToolResult {
                            id: p.tool_call_id.clone(),
                            call_id: Some(p.tool_call_id.clone()),
                            content: OneOrMany::one(ToolResultContent::text(json_to_string(output))),
                        }))
                    } else {
                        None
                    }
                }
                ToolState::OutputError => {
                    if let Some(error_text) = &p.error_text {
                        Some(UserContent::ToolResult(ToolResult {
                            id: p.tool_call_id.clone(),
                            call_id: Some(p.tool_call_id.clone()),
                            content: OneOrMany::one(ToolResultContent::text(error_text.clone())),
                        }))
                    } else {
                        None
                    }
                }
                // InputStreaming and InputAvailable don't convert to UserContent
                _ => None,
            }
        }

        // Image file
        UIMessagePart::File(p) => {
            if p.media_type.starts_with("image/") {
                Some(UserContent::Image(Image {
                    data: DocumentSourceKind::Url(p.url.clone()),
                    media_type: parse_image_media_type(&p.media_type),
                    detail: None,
                    additional_params: None,
                }))
            } else {
                // Skip non-image files (rig currently doesn't support other file types)
                None
            }
        }

        // Other types not currently supported as UserContent
        // Can be extended here: SourceUrl, SourceDocument, Data, StepStart
        _ => None,
    }
}

/// Converts a [`UIMessagePart`] to an [`AssistantContent`] if applicable.
///
/// Returns `None` for parts that don't map to assistant content:
///
/// - `Text` → `AssistantContent::Text`
/// - `Tool` (with input) → `AssistantContent::ToolCall`
///
/// # Important
///
/// **Reasoning and Data parts are filtered out** when converting assistant messages
/// because:
/// - Reasoning is AI-generated output and should not be sent back to the AI
/// - OpenAI requires OpenAI-generated IDs for reasoning items
/// - Data parts are metadata (e.g., usage info) and not actual content
fn to_assistant_content(part: &UIMessagePart) -> Option<AssistantContent> {
    match part {
        // Text part
        UIMessagePart::Text(p) => Some(AssistantContent::text(p.text.clone())),

        // Tool call (InputAvailable, OutputAvailable, or InputStreaming with input)
        UIMessagePart::Tool(p) => {
            // Extract tool name from type field (already set during deserialization)
            let tool_name = p.tool_name.clone();

            match p.state {
                ToolState::InputAvailable | ToolState::OutputAvailable => {
                    if let Some(input) = &p.input {
                        Some(AssistantContent::ToolCall(
                            ToolCall::new(
                                p.tool_call_id.clone(),
                                ToolFunction {
                                    name: tool_name,
                                    arguments: input.clone(),
                                },
                            )
                            .with_call_id(p.tool_call_id.clone()),
                        ))
                    } else {
                        None
                    }
                }
                ToolState::InputStreaming => {
                    p.input.as_ref().map(|i| {
                        AssistantContent::ToolCall(
                            ToolCall::new(
                                p.tool_call_id.clone(),
                                ToolFunction {
                                    name: tool_name,
                                    arguments: i.clone(),
                                },
                            )
                            .with_call_id(p.tool_call_id.clone()),
                        )
                    })
                }
                ToolState::OutputError => None,
            }
        }

        // Filter out: Reasoning (AI-generated, should not be sent back to AI),
        // Data (metadata like usage info), SourceUrl, SourceDocument, StepStart
        _ => None,
    }
}

/// Converts a JSON [`Value`] to a string.
///
/// If the value is already a string, returns it directly.
/// Otherwise, serializes it to JSON.
fn json_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

/// Parses an image MIME type string to an [`ImageMediaType`].
///
/// Returns `Some(ImageMediaType)` for supported types, `None` otherwise.
/// Note that BMP and TIFF are not supported by rig.
fn parse_image_media_type(media_type: &str) -> Option<ImageMediaType> {
    match media_type {
        "image/jpeg" | "image/jpg" => Some(ImageMediaType::JPEG),
        "image/png" => Some(ImageMediaType::PNG),
        "image/gif" => Some(ImageMediaType::GIF),
        "image/webp" => Some(ImageMediaType::WEBP),
        "image/heic" => Some(ImageMediaType::HEIC),
        "image/heif" => Some(ImageMediaType::HEIF),
        "image/svg+xml" => Some(ImageMediaType::SVG),
        // BMP and TIFF are not supported by rig, return None
        _ => None,
    }
}

// ============================================================================
// Tests
// ============================================================================
