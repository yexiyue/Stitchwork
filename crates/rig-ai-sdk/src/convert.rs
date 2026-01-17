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
    AssistantContent, DocumentSourceKind, Image, ImageMediaType, Message, Reasoning, ToolCall,
    ToolFunction, ToolResult, ToolResultContent, UserContent,
};
use serde_json::Value;

use crate::message::{DynamicToolState, UIMessage, UIMessagePart};

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
                id: Some(msg.id.clone()),
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
/// - `ToolResult` → `UserContent::ToolResult`
/// - `DynamicTool` (OutputAvailable/Error) → `UserContent::ToolResult`
/// - `File` (image only) → `UserContent::Image`
fn to_user_content(part: &UIMessagePart) -> Option<UserContent> {
    match part {
        // Text part
        UIMessagePart::Text { text, .. } => Some(UserContent::text(text)),

        // Tool result
        UIMessagePart::ToolResult {
            tool_call_id,
            result,
            ..
        } => Some(UserContent::ToolResult(ToolResult {
            id: tool_call_id.clone(),
            call_id: Some(tool_call_id.clone()),
            content: OneOrMany::one(ToolResultContent::text(json_to_string(result))),
        })),

        // DynamicTool tool result (OutputAvailable and OutputError)
        UIMessagePart::DynamicTool {
            tool_call_id,
            state,
            ..
        } => {
            // Only convert completed dynamic-tools with output to tool result
            match state {
                DynamicToolState::OutputAvailable { input: _, output } => {
                    Some(UserContent::ToolResult(ToolResult {
                        id: tool_call_id.clone(),
                        call_id: Some(tool_call_id.clone()),
                        content: OneOrMany::one(ToolResultContent::text(json_to_string(output))),
                    }))
                }
                DynamicToolState::OutputError {
                    input: _,
                    error_text,
                } => {
                    // Use error text as tool result
                    Some(UserContent::ToolResult(ToolResult {
                        id: tool_call_id.clone(),
                        call_id: Some(tool_call_id.clone()),
                        content: OneOrMany::one(ToolResultContent::text(error_text.clone())),
                    }))
                }
                _ => None, // InputStreaming and InputAvailable don't convert to UserContent
            }
        }

        // Image file
        UIMessagePart::File {
            media_type, url, ..
        } => {
            if media_type.starts_with("image/") {
                Some(UserContent::Image(Image {
                    data: DocumentSourceKind::Url(url.clone()),
                    media_type: parse_image_media_type(media_type),
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
/// - `Reasoning` → `AssistantContent::Reasoning`
/// - `ToolCall` → `AssistantContent::ToolCall`
/// - `DynamicTool` (with input) → `AssistantContent::ToolCall`
fn to_assistant_content(part: &UIMessagePart) -> Option<AssistantContent> {
    match part {
        // Text part
        UIMessagePart::Text { text, .. } => Some(AssistantContent::text(text)),

        // Reasoning part
        UIMessagePart::Reasoning { text, .. } => {
            Some(AssistantContent::Reasoning(Reasoning::new(text)))
        }

        // Legacy tool call format
        UIMessagePart::ToolCall {
            tool_call_id,
            tool_name,
            args,
            ..
        } => Some(AssistantContent::ToolCall(
            ToolCall::new(
                tool_call_id.clone(),
                ToolFunction {
                    name: tool_name.clone(),
                    arguments: args.clone(),
                },
            )
            .with_call_id(tool_call_id.clone()),
        )),

        // DynamicTool tool call
        UIMessagePart::DynamicTool {
            tool_call_id,
            tool_name,
            state,
            ..
        } => {
            // Only convert to ToolCall when input is available
            match state {
                DynamicToolState::InputAvailable { input }
                | DynamicToolState::OutputAvailable { input, .. } => {
                    Some(AssistantContent::ToolCall(
                        ToolCall::new(
                            tool_call_id.clone(),
                            ToolFunction {
                                name: tool_name.clone(),
                                arguments: input.clone(),
                            },
                        )
                        .with_call_id(tool_call_id.clone()),
                    ))
                }
                DynamicToolState::InputStreaming { input } => {
                    // If there's partial input, create ToolCall anyway
                    input.as_ref().map(|i| {
                        AssistantContent::ToolCall(
                            ToolCall::new(
                                tool_call_id.clone(),
                                ToolFunction {
                                    name: tool_name.clone(),
                                    arguments: i.clone(),
                                },
                            )
                            .with_call_id(tool_call_id.clone()),
                        )
                    })
                }
                _ => None, // OutputError state doesn't convert to ToolCall
            }
        }

        // Other types not currently supported as AssistantContent
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::PartState;
    use serde_json::json;

    #[test]
    fn test_convert_user_text_message() {
        let msg = UIMessage {
            id: "1".to_string(),
            role: "user".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::Text {
                text: "Hello".to_string(),
                state: None,
                provider_metadata: None,
            }],
        };

        let result = convert_message(&msg).unwrap();

        println!("{:?}", result);
        match result {
            Message::User { content } => {
                assert_eq!(content.iter().count(), 1);
            }
            _ => panic!("Expected User message"),
        }
    }

    #[test]
    fn test_convert_assistant_text_message() {
        let msg = UIMessage {
            id: "2".to_string(),
            role: "assistant".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::Text {
                text: "Hi there!".to_string(),
                state: Some(PartState::Done),
                provider_metadata: None,
            }],
        };

        let result = convert_message(&msg).unwrap();
        println!("{:?}", result);
        match result {
            Message::Assistant { id, content } => {
                assert_eq!(id, Some("2".to_string()));
                assert_eq!(content.iter().count(), 1);
            }
            _ => panic!("Expected Assistant message"),
        }
    }

    #[test]
    fn test_convert_tool_call_message() {
        let msg = UIMessage {
            id: "3".to_string(),
            role: "assistant".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::ToolCall {
                tool_call_id: "call_123".to_string(),
                tool_name: "get_weather".to_string(),
                args: json!({"city": "北京"}),
                provider_metadata: None,
            }],
        };

        let result = convert_message(&msg).unwrap();
        println!("{:?}", result);
        match result {
            Message::Assistant { content, .. } => {
                let first = content.iter().next().unwrap();
                match first {
                    AssistantContent::ToolCall(tc) => {
                        assert_eq!(tc.function.name, "get_weather");
                    }
                    _ => panic!("Expected ToolCall"),
                }
            }
            _ => panic!("Expected Assistant message"),
        }
    }

    #[test]
    fn test_convert_dynamic_tool_message() {
        let msg = UIMessage {
            id: "4".to_string(),
            role: "assistant".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::DynamicTool {
                tool_name: "search".to_string(),
                tool_call_id: "call_456".to_string(),
                title: None,
                provider_executed: true,
                state: DynamicToolState::InputAvailable {
                    input: json!({"query": "Rust"}),
                },
                call_provider_metadata: None,
                preliminary: false,
            }],
        };

        let result = convert_message(&msg).unwrap();
        println!("{:?}", result);
        match result {
            Message::Assistant { content, .. } => {
                let first = content.iter().next().unwrap();
                match first {
                    AssistantContent::ToolCall(tc) => {
                        assert_eq!(tc.function.name, "search");
                    }
                    _ => panic!("Expected ToolCall"),
                }
            }
            _ => panic!("Expected Assistant message"),
        }
    }

    #[test]
    fn test_convert_tool_result_message() {
        let msg = UIMessage {
            id: "5".to_string(),
            role: "user".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::ToolResult {
                tool_call_id: "call_123".to_string(),
                tool_name: Some("get_weather".to_string()),
                result: json!({"temp": 25, "weather": "晴"}),
            }],
        };

        let result = convert_message(&msg).unwrap();
        println!("{:?}", result);
        match result {
            Message::User { content } => {
                let first = content.iter().next().unwrap();
                match first {
                    UserContent::ToolResult(tr) => {
                        assert_eq!(tr.id, "call_123");
                    }
                    _ => panic!("Expected ToolResult"),
                }
            }
            _ => panic!("Expected User message"),
        }
    }

    #[test]
    fn test_convert_dynamic_tool_result_message() {
        let msg = UIMessage {
            id: "6".to_string(),
            role: "user".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::DynamicTool {
                tool_name: "search".to_string(),
                tool_call_id: "call_789".to_string(),
                title: None,
                provider_executed: true,
                state: DynamicToolState::OutputAvailable {
                    input: json!({"query": "Rust"}),
                    output: json!({"results": ["Rust", "Cargo"]}),
                },
                call_provider_metadata: None,
                preliminary: false,
            }],
        };

        let result = convert_message(&msg).unwrap();
        println!("{:?}", result);
        match result {
            Message::User { content } => {
                let first = content.iter().next().unwrap();
                match first {
                    UserContent::ToolResult(tr) => {
                        assert_eq!(tr.id, "call_789");
                    }
                    _ => panic!("Expected ToolResult"),
                }
            }
            _ => panic!("Expected User message"),
        }
    }

    #[test]
    fn test_convert_image_file() {
        let msg = UIMessage {
            id: "7".to_string(),
            role: "user".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::File {
                media_type: "image/png".to_string(),
                url: "https://example.com/image.png".to_string(),
                filename: Some("screenshot.png".to_string()),
                provider_metadata: None,
            }],
        };

        let result = convert_message(&msg).unwrap();
        match result {
            Message::User { content } => {
                let first = content.iter().next().unwrap();
                match first {
                    UserContent::Image(img) => {
                        assert_eq!(img.media_type, Some(ImageMediaType::PNG));
                    }
                    _ => panic!("Expected Image"),
                }
            }
            _ => panic!("Expected User message"),
        }
    }

    #[test]
    fn test_convert_reasoning_part() {
        let msg = UIMessage {
            id: "8".to_string(),
            role: "assistant".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::Reasoning {
                text: "Let me think about this...".to_string(),
                state: Some(PartState::Done),
                provider_metadata: None,
            }],
        };

        let result = convert_message(&msg).unwrap();
        match result {
            Message::Assistant { content, .. } => {
                let first = content.iter().next().unwrap();
                match first {
                    AssistantContent::Reasoning(r) => {
                        // Reasoning 结构有 reasoning 字段 (Vec<String>) 而不是 text
                        assert!(r.reasoning.iter().any(|s| s.contains("think")));
                    }
                    _ => panic!("Expected Reasoning"),
                }
            }
            _ => panic!("Expected Assistant message"),
        }
    }

    #[test]
    fn test_dynamic_tool_streaming_with_empty_input() {
        let msg = UIMessage {
            id: "9".to_string(),
            role: "user".to_string(),
            metadata: None,
            parts: vec![UIMessagePart::DynamicTool {
                tool_name: "search".to_string(),
                tool_call_id: "call_999".to_string(),
                title: None,
                provider_executed: true,
                state: DynamicToolState::InputStreaming { input: None },
                call_provider_metadata: None,
                preliminary: false,
            }],
        };

        let result = convert_message(&msg).unwrap();
        match result {
            Message::User { content } => {
                // InputStreaming 状态没有输入时不产生转换内容，会返回空文本
                let first = content.iter().next().unwrap();
                match first {
                    UserContent::Text(text) => {
                        assert!(text.text.is_empty());
                    }
                    _ => panic!("Expected empty Text content"),
                }
            }
            _ => panic!("Expected User message"),
        }
    }

    /// 测试用户消息：文本 + 图片
    #[test]
    fn test_multi_part_user_text_and_image() {
        let msg = UIMessage {
            id: "10".to_string(),
            role: "user".to_string(),
            metadata: None,
            parts: vec![
                UIMessagePart::Text {
                    text: "请查看这张图片".to_string(),
                    state: None,
                    provider_metadata: None,
                },
                UIMessagePart::File {
                    media_type: "image/jpeg".to_string(),
                    url: "https://example.com/photo.jpg".to_string(),
                    filename: Some("photo.jpg".to_string()),
                    provider_metadata: None,
                },
            ],
        };

        let result = convert_message(&msg).unwrap();
        match result {
            Message::User { content } => {
                let contents: Vec<_> = content.iter().collect();
                assert_eq!(contents.len(), 2);
                // 第一个应该是文本
                match &contents[0] {
                    UserContent::Text(t) => {
                        assert_eq!(t.text, "请查看这张图片");
                    }
                    _ => panic!("Expected Text first"),
                }
                // 第二个应该是图片
                match &contents[1] {
                    UserContent::Image(img) => {
                        assert_eq!(img.media_type, Some(ImageMediaType::JPEG));
                    }
                    _ => panic!("Expected Image second"),
                }
            }
            _ => panic!("Expected User message"),
        }
    }

    /// 测试助手消息：文本 + 工具调用
    #[test]
    fn test_multi_part_assistant_text_and_tool_call() {
        let msg = UIMessage {
            id: "11".to_string(),
            role: "assistant".to_string(),
            metadata: None,
            parts: vec![
                UIMessagePart::Text {
                    text: "我来帮你查询天气".to_string(),
                    state: Some(PartState::Done),
                    provider_metadata: None,
                },
                UIMessagePart::DynamicTool {
                    tool_name: "get_weather".to_string(),
                    tool_call_id: "call_111".to_string(),
                    title: None,
                    provider_executed: true,
                    state: DynamicToolState::InputAvailable {
                        input: json!({"city": "北京"}),
                    },
                    call_provider_metadata: None,
                    preliminary: false,
                },
            ],
        };

        let result = convert_message(&msg).unwrap();
        match result {
            Message::Assistant { content, id } => {
                assert_eq!(id, Some("11".to_string()));
                let contents: Vec<_> = content.iter().collect();
                assert_eq!(contents.len(), 2);
                match &contents[0] {
                    AssistantContent::Text(t) => {
                        assert_eq!(t.text, "我来帮你查询天气");
                    }
                    _ => panic!("Expected Text first"),
                }
                match &contents[1] {
                    AssistantContent::ToolCall(tc) => {
                        assert_eq!(tc.function.name, "get_weather");
                    }
                    _ => panic!("Expected ToolCall second"),
                }
            }
            _ => panic!("Expected Assistant message"),
        }
    }

    /// 测试用户消息：工具结果 + 文本
    #[test]
    fn test_multi_part_user_tool_result_and_text() {
        let msg = UIMessage {
            id: "12".to_string(),
            role: "user".to_string(),
            metadata: None,
            parts: vec![
                UIMessagePart::DynamicTool {
                    tool_name: "search".to_string(),
                    tool_call_id: "call_222".to_string(),
                    title: None,
                    provider_executed: true,
                    state: DynamicToolState::OutputAvailable {
                        input: json!({"query": "Rust"}),
                        output: json!({"results": ["Rust Programming Language"]}),
                    },
                    call_provider_metadata: None,
                    preliminary: false,
                },
                UIMessagePart::Text {
                    text: "搜索结果怎么样？".to_string(),
                    state: None,
                    provider_metadata: None,
                },
            ],
        };

        let result = convert_message(&msg).unwrap();
        match result {
            Message::User { content } => {
                let contents: Vec<_> = content.iter().collect();
                assert_eq!(contents.len(), 2);
                // 第一个是工具结果
                match &contents[0] {
                    UserContent::ToolResult(tr) => {
                        assert_eq!(tr.id, "call_222");
                    }
                    _ => panic!("Expected ToolResult first"),
                }
                // 第二个是文本
                match &contents[1] {
                    UserContent::Text(t) => {
                        assert_eq!(t.text, "搜索结果怎么样？");
                    }
                    _ => panic!("Expected Text second"),
                }
            }
            _ => panic!("Expected User message"),
        }
    }

    /// 测试助手消息：推理 + 文本 + 工具调用
    #[test]
    fn test_multi_part_assistant_reasoning_text_and_tool() {
        let msg = UIMessage {
            id: "13".to_string(),
            role: "assistant".to_string(),
            metadata: None,
            parts: vec![
                UIMessagePart::Reasoning {
                    text: "需要分析用户的需求".to_string(),
                    state: Some(PartState::Done),
                    provider_metadata: None,
                },
                UIMessagePart::Text {
                    text: "我理解了，让我来处理".to_string(),
                    state: Some(PartState::Done),
                    provider_metadata: None,
                },
                UIMessagePart::ToolCall {
                    tool_call_id: "call_333".to_string(),
                    tool_name: "calculate".to_string(),
                    args: json!({"expression": "1+1"}),
                    provider_metadata: None,
                },
            ],
        };

        let result = convert_message(&msg).unwrap();
        match result {
            Message::Assistant { content, .. } => {
                let contents: Vec<_> = content.iter().collect();
                assert_eq!(contents.len(), 3);
                match &contents[0] {
                    AssistantContent::Reasoning(r) => {
                        assert!(r.reasoning.iter().any(|s| s.contains("用户的需求")));
                    }
                    _ => panic!("Expected Reasoning first"),
                }
                match &contents[1] {
                    AssistantContent::Text(t) => {
                        assert_eq!(t.text, "我理解了，让我来处理");
                    }
                    _ => panic!("Expected Text second"),
                }
                match &contents[2] {
                    AssistantContent::ToolCall(tc) => {
                        assert_eq!(tc.function.name, "calculate");
                    }
                    _ => panic!("Expected ToolCall third"),
                }
            }
            _ => panic!("Expected Assistant message"),
        }
    }

    /// 测试批量消息转换
    #[test]
    fn test_convert_multiple_messages() {
        let messages = vec![
            UIMessage {
                id: "14".to_string(),
                role: "user".to_string(),
                metadata: None,
                parts: vec![UIMessagePart::Text {
                    text: "你好".to_string(),
                    state: None,
                    provider_metadata: None,
                }],
            },
            UIMessage {
                id: "15".to_string(),
                role: "assistant".to_string(),
                metadata: None,
                parts: vec![UIMessagePart::Text {
                    text: "你好！".to_string(),
                    state: Some(PartState::Done),
                    provider_metadata: None,
                }],
            },
            UIMessage {
                id: "16".to_string(),
                role: "user".to_string(),
                metadata: None,
                parts: vec![UIMessagePart::Text {
                    text: "再见".to_string(),
                    state: None,
                    provider_metadata: None,
                }],
            },
        ];

        let result = convert_messages(&messages).unwrap();
        assert_eq!(result.len(), 3);

        // 验证第一条
        match &result[0] {
            Message::User { content } => {
                assert_eq!(content.iter().count(), 1);
            }
            _ => panic!("Expected User message at index 0"),
        }

        // 验证第二条
        match &result[1] {
            Message::Assistant { id, content } => {
                assert_eq!(id, &Some("15".to_string()));
                assert_eq!(content.iter().count(), 1);
            }
            _ => panic!("Expected Assistant message at index 1"),
        }

        // 验证第三条
        match &result[2] {
            Message::User { content } => {
                assert_eq!(content.iter().count(), 1);
            }
            _ => panic!("Expected User message at index 2"),
        }
    }
}
