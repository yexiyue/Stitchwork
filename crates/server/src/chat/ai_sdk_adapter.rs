use axum::response::sse::Event;
use serde::Serialize;
use serde_json::{json, Value};
use std::fmt::Display;
use uuid::Uuid;

/// AI SDK Data Stream Protocol 事件
/// 参考: https://sdk.vercel.ai/docs/ai-sdk-ui/stream-protocol
#[derive(Debug, Clone)]
pub enum AISdkEvent {
    /// 消息开始
    Start { message_id: Uuid },

    /// 文本块开始
    TextStart { id: Uuid },
    /// 文本增量
    TextDelta { id: Uuid, delta: String },
    /// 文本块结束
    TextEnd { id: Uuid },

    /// 推理块开始
    ReasoningStart { id: Uuid },
    /// 推理增量
    ReasoningDelta { id: Uuid, delta: String },
    /// 推理块结束
    ReasoningEnd { id: Uuid },

    /// URL 来源引用
    SourceUrl { source_id: String, url: String },
    /// 文档来源引用
    SourceDocument {
        source_id: String,
        media_type: String,
        title: String,
    },

    /// 文件
    File { url: String, media_type: String },

    /// 工具输入开始
    ToolInputStart {
        tool_call_id: String,
        tool_name: String,
    },
    /// 工具输入增量
    ToolInputDelta { tool_call_id: String, delta: String },
    /// 工具输入完成
    ToolInputAvailable {
        tool_call_id: String,
        tool_name: String,
        input: Value,
    },
    /// 工具输出
    ToolOutputAvailable { tool_call_id: String, output: Value },

    /// 自定义数据 (type: "data-{name}")
    CustomData { name: String, data: Value },

    /// 步骤开始
    StartStep,
    /// 步骤结束
    FinishStep,

    /// 消息完成
    Finish,
    /// 流中止
    Abort { reason: String },
    /// 错误
    Error { error_text: String },
    /// 流结束标记
    Done,
}

impl AISdkEvent {
    /// 生成错误事件
    pub fn error(error: impl Into<String>) -> AISdkEvent {
        AISdkEvent::Error {
            error_text: error.into(),
        }
    }

    /// 生成自定义数据事件
    pub fn custom_data<V: Serialize>(name: impl Into<String>, data: V) -> AISdkEvent {
        AISdkEvent::CustomData {
            name: name.into(),
            data: serde_json::to_value(data).unwrap_or_default(),
        }
    }
}

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

impl From<AISdkEvent> for Event {
    fn from(value: AISdkEvent) -> Self {
        Event::default().data(value.to_string())
    }
}

/// AI SDK 流式响应构建器
pub struct AISdkStreamBuilder {
    message_id: Uuid,
    text_id: Option<Uuid>,
    reasoning_id: Option<Uuid>,
}

impl AISdkStreamBuilder {
    pub fn new() -> Self {
        Self {
            message_id: Uuid::new_v4(),
            text_id: None,
            reasoning_id: None,
        }
    }

    /// 生成消息开始事件
    pub fn start(&self) -> AISdkEvent {
        AISdkEvent::Start {
            message_id: self.message_id,
        }
    }

    /// 开始新的文本块
    pub fn text_start(&mut self) -> AISdkEvent {
        let id = Uuid::new_v4();
        self.text_id = Some(id);
        AISdkEvent::TextStart { id }
    }

    /// 文本增量
    pub fn text_delta(&self, delta: impl Into<String>) -> Option<AISdkEvent> {
        self.text_id.map(|id| AISdkEvent::TextDelta {
            id,
            delta: delta.into(),
        })
    }

    /// 结束当前文本块
    pub fn text_end(&mut self) -> Option<AISdkEvent> {
        self.text_id.take().map(|id| AISdkEvent::TextEnd { id })
    }

    /// 开始新的推理块
    pub fn reasoning_start(&mut self) -> AISdkEvent {
        let id = Uuid::new_v4();
        self.reasoning_id = Some(id);
        AISdkEvent::ReasoningStart { id }
    }

    /// 推理增量
    pub fn reasoning_delta(&self, delta: impl Into<String>) -> Option<AISdkEvent> {
        self.reasoning_id.map(|id| AISdkEvent::ReasoningDelta {
            id,
            delta: delta.into(),
        })
    }

    /// 结束当前推理块
    pub fn reasoning_end(&mut self) -> Option<AISdkEvent> {
        self.reasoning_id
            .take()
            .map(|id| AISdkEvent::ReasoningEnd { id })
    }

    /// 生成完成事件
    pub fn finish(&self) -> AISdkEvent {
        AISdkEvent::Finish
    }

    /// 生成结束标记
    pub fn done(&self) -> AISdkEvent {
        AISdkEvent::Done
    }
}

impl Default for AISdkStreamBuilder {
    fn default() -> Self {
        Self::new()
    }
}
