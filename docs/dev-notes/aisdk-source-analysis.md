# aisdk Rust Crate 源码分析

> 分析日期：2026-01-23
> 源码位置：`D:\workspace\aisdk`
> Crate 版本：v0.3.0

## 概述

[aisdk](https://crates.io/crates/aisdk) 是一个 Rust 实现的 AI SDK，风格类似于 Vercel 的 JavaScript AI SDK。它提供了统一的接口与多个 LLM 提供商交互，支持流式文本生成和工具调用。

## 项目结构

```text
aisdk/src/
├── core/
│   ├── language_model/
│   │   ├── mod.rs           # LanguageModel trait, LanguageModelOptions, Usage
│   │   ├── stream_text.rs   # stream_text() 流式生成实现
│   │   ├── generate_text.rs # generate_text() 非流式生成
│   │   └── request.rs       # LanguageModelRequest 请求构建
│   ├── tools.rs             # Tool, ToolExecute, ToolList, ToolCallInfo
│   ├── messages.rs          # Message, SystemMessage, UserMessage, AssistantMessage
│   ├── capabilities.rs      # 模型能力定义
│   ├── client.rs            # HTTP 客户端
│   ├── provider.rs          # Provider trait
│   └── utils.rs             # 工具函数
├── integrations/
│   ├── mod.rs
│   ├── axum.rs              # Axum SSE 集成
│   └── vercel_aisdk_ui.rs   # Vercel AI SDK UI Stream 协议
├── providers/
│   ├── openai/              # OpenAI 实现
│   ├── anthropic/           # Anthropic/Claude 实现
│   ├── google/              # Google Gemini 实现
│   ├── deepseek/            # DeepSeek 实现
│   ├── groq/                # Groq 实现
│   ├── amazon_bedrock/      # AWS Bedrock 实现
│   ├── openrouter/          # OpenRouter 实现
│   ├── togetherai/          # Together AI 实现
│   ├── vercel/              # Vercel AI 实现
│   └── xai/                 # xAI 实现
├── extensions.rs            # Provider 特定扩展
├── prompt.rs                # Prompt 工具
├── error.rs                 # 错误类型
└── lib.rs                   # 库入口
```

## 核心组件

### 1. LanguageModel Trait

定义了所有语言模型必须实现的核心接口：

```rust
// core/language_model/mod.rs
#[async_trait]
pub trait LanguageModel: Send + Sync + std::fmt::Debug + Clone + 'static {
    /// 返回模型名称
    fn name(&self) -> String;

    /// 非流式文本生成
    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse>;

    /// 流式文本生成
    async fn stream_text(
        &mut self,
        options: LanguageModelOptions
    ) -> Result<ProviderStream>;
}
```

### 2. Tool 定义

工具通过 `Tool` struct 定义，包含名称、描述、JSON Schema 和执行函数：

```rust
// core/tools.rs
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Schema,        // schemars::Schema
    pub execute: ToolExecute,        // 执行函数包装
}

pub struct ToolExecute {
    inner: Arc<ToolFn>,  // Box<dyn Fn(Value) -> Result<String, String>>
}

// 使用 #[tool] 宏简化定义
#[tool]
/// Adds two numbers together.
pub fn sum(a: u8, b: u8) -> Tool {
    Ok(format!("{}", a + b))
}
```

### 3. Message 类型

支持完整的对话消息类型：

```rust
// core/messages.rs
pub enum Message {
    System(SystemMessage),
    User(UserMessage),
    Assistant(AssistantMessage),
    Tool(ToolResultInfo),      // 工具执行结果
    Developer(String),         // 开发者消息
}

pub struct AssistantMessage {
    pub content: LanguageModelResponseContentType,
    pub usage: Option<Usage>,
}

pub enum LanguageModelResponseContentType {
    Text(String),
    ToolCall(ToolCallInfo),
    Reasoning { content: String, extensions: Extensions },
    NotSupported(String),
}
```

### 4. LanguageModelOptions

请求配置，包含所有生成参数和 Hook：

```rust
// core/language_model/mod.rs
pub struct LanguageModelOptions {
    pub system: Option<String>,
    pub schema: Option<Schema>,
    pub seed: Option<u32>,
    pub temperature: Option<u32>,        // 0-100
    pub top_p: Option<u32>,
    pub top_k: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub reasoning_effort: Option<ReasoningEffort>,

    // Hooks
    pub stop_when: Option<StopWhenHook>,
    pub on_step_start: Option<OnStepStartHook>,
    pub on_step_finish: Option<OnStepFinishHook>,

    // Internal
    pub(crate) tools: Option<ToolList>,
    pub(crate) messages: Vec<TaggedMessage>,
    pub(crate) current_step_id: usize,
    pub(crate) stop_reason: Option<StopReason>,
}
```

## 流式生成实现

### stream_text() 核心逻辑

```rust
// core/language_model/stream_text.rs
impl<M: LanguageModel> LanguageModelRequest<M> {
    pub async fn stream_text(&mut self) -> Result<StreamTextResponse> {
        let (tx, stream) = LanguageModelStream::new();
        let _ = tx.send(LanguageModelStreamChunkType::Start);

        tokio::spawn(async move {
            loop {
                options.current_step_id += 1;

                // Step 开始 Hook
                if let Some(hook) = options.on_step_start.clone() {
                    hook(&mut options);
                }

                // 调用模型
                let mut response = model.stream_text(options.clone()).await?;

                while let Some(chunk) = response.next().await {
                    match chunk {
                        LanguageModelStreamChunk::Done(final_msg) => {
                            match final_msg.content {
                                LanguageModelResponseContentType::Text(_) => {
                                    // 添加助手消息，标记完成
                                    options.stop_reason = Some(StopReason::Finish);
                                }
                                LanguageModelResponseContentType::ToolCall(ref tool_info) => {
                                    // 添加工具调用消息
                                    options.messages.push(...);
                                    // ⚠️ 关键：直接执行工具
                                    options.handle_tool_call(tool_info).await;
                                }
                            }

                            // Step 完成 Hook
                            if let Some(hook) = options.on_step_finish.clone() {
                                hook(&options);
                            }

                            // 检查 stop_when Hook
                            if let Some(hook) = &options.stop_when && hook(&options) {
                                options.stop_reason = Some(StopReason::Hook);
                                break;
                            }
                        }
                        LanguageModelStreamChunk::Delta(delta) => {
                            // 转发增量到输出流
                            tx.send(delta.clone());
                        }
                    }
                }

                // 检查是否应该停止
                if options.stop_reason.is_some() {
                    break;
                }
            }
        });

        Ok(StreamTextResponse { stream, options })
    }
}
```

### 工具执行

工具在服务端自动执行，没有暂停等待机制：

```rust
// core/language_model/mod.rs:279
impl LanguageModelOptions {
    pub(crate) async fn handle_tool_call(&mut self, input: &ToolCallInfo) -> &mut Self {
        if let Some(tools) = &self.tools {
            // 在 tokio 任务中执行工具
            let tool_result_task = tools.execute(input.clone()).await;
            let tool_result = tool_result_task.await
                .map_err(|err| Error::ToolCallError(...))?;

            // 构建工具结果
            let mut tool_output_info = ToolResultInfo::new(&input.tool.name);
            let output = match tool_result {
                Ok(result) => serde_json::Value::String(result),
                Err(err) => serde_json::Value::String(format!("Error: {}", err)),
            };
            tool_output_info.output(output);
            tool_output_info.id(&input.tool.id);

            // 添加到消息历史
            self.messages.push(TaggedMessage::new(
                self.current_step_id,
                Message::Tool(tool_output_info),
            ));
        }
        self
    }
}
```

## Vercel AI SDK UI 集成

### VercelUIStream 事件类型

```rust
// integrations/vercel_aisdk_ui.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum VercelUIStream {
    #[serde(rename = "text-start")]
    TextStart { id: String, provider_metadata: Option<Value> },

    #[serde(rename = "text-delta")]
    TextDelta { id: String, delta: String, provider_metadata: Option<Value> },

    #[serde(rename = "text-end")]
    TextEnd { id: String, provider_metadata: Option<Value> },

    #[serde(rename = "reasoning-start")]
    ReasoningStart { id: String, provider_metadata: Option<Value> },

    #[serde(rename = "reasoning-delta")]
    ReasoningDelta { id: String, delta: String, provider_metadata: Option<Value> },

    #[serde(rename = "reasoning-end")]
    ReasoningEnd { id: String, provider_metadata: Option<Value> },

    #[serde(rename = "tool-call-start")]
    ToolCallStart {
        id: String,
        tool_call_id: String,
        tool_name: String,
        provider_metadata: Option<Value>
    },

    #[serde(rename = "tool-call-delta")]
    ToolCallDelta {
        id: String,
        tool_call_id: String,
        delta: String,
        provider_metadata: Option<Value>
    },

    #[serde(rename = "tool-call-end")]
    ToolCallEnd {
        id: String,
        tool_call_id: String,
        result: Value,  // ⚠️ 包含执行结果
        provider_metadata: Option<Value>
    },

    #[serde(rename = "error")]
    Error { error_text: String },

    #[serde(rename = "not-supported")]
    NotSupported { error_text: String },
}
```

### Axum SSE 集成

```rust
// integrations/axum.rs
impl StreamTextResponse {
    pub fn to_axum_vercel_ui_stream(self) -> VercelUIStreamBuilder<Self, AxumSseResponse> {
        VercelUIStreamBuilder::new(self, |context, options| {
            let ui_stream = context.into_vercel_ui_stream(options);
            let boxed_stream = Box::pin(ui_stream.map(|res| {
                res.map(|ui_chunk| {
                    let json = serde_json::to_string(&ui_chunk).unwrap();
                    axum::response::sse::Event::default().data(json)
                })
                .map_err(|e| axum::Error::new(e))
            }));
            axum::response::Sse::new(boxed_stream).keep_alive(
                axum::response::sse::KeepAlive::new()
                    .interval(Duration::from_secs(15))
            )
        })
    }
}

// 使用示例
async fn chat_handler(req: VercelUIRequest) -> impl IntoResponse {
    let messages = Message::from_vercel_ui_message(&req.messages);

    LanguageModelRequest::builder()
        .model(OpenAI::gpt_4())
        .messages(messages)
        .build()
        .stream_text()
        .await?
        .to_axum_vercel_ui_stream()
        .send_reasoning()
        .build()
}
```

### 消息转换

```rust
// integrations/vercel_aisdk_ui.rs
impl Message {
    pub fn from_vercel_ui_message(ui_messages: &[VercelUIMessage]) -> Messages {
        ui_messages.iter().filter_map(|msg| {
            let content = msg.parts.iter()
                .filter(|part| part.part_type == "text")
                .map(|part| part.text.clone())
                .collect::<Vec<_>>()
                .join("");

            match msg.role.as_str() {
                "system" => Some(Message::System(content.into())),
                "user" => Some(Message::User(content.into())),
                "assistant" => Some(Message::Assistant(content.into())),
                _ => None,
            }
        }).collect()
    }
}
```

## 支持的 Provider

| Provider | 模型示例 | 特性 |
|----------|---------|------|
| OpenAI | gpt-4, gpt-3.5-turbo | 完整支持 |
| Anthropic | claude-3-opus, claude-3-sonnet | 支持 reasoning |
| Google | gemini-pro, gemini-ultra | 支持多模态 |
| DeepSeek | deepseek-chat, deepseek-coder | 代码生成 |
| Groq | llama-2-70b, mixtral-8x7b | 高速推理 |
| Amazon Bedrock | claude, titan | AWS 托管 |
| OpenRouter | 多模型路由 | 统一接口 |
| Together AI | 开源模型 | 高性价比 |
| Vercel | 边缘推理 | 低延迟 |
| xAI | grok | X 平台 |

## 限制与问题

### 1. 不支持前端工具

`aisdk` 的工具执行模型是完全服务端的：

- `handle_tool_call()` 在收到 tool call 后立即执行
- 没有暂停流等待外部输入的机制
- 没有 `requires-action` 状态或等待前端返回结果的 API

### 2. 缺少完整的 Data Stream Protocol 事件

与 Vercel AI SDK 的完整协议相比，缺少：

- `tool-input-start` / `tool-input-available`（前端工具等待状态）
- `tool-output-available`（区分服务端 vs 前端执行）
- `step-start` / `step-finish`（步骤边界）

### 3. ToolCallEnd 包含 result

```rust
ToolCallEnd {
    tool_call_id: String,
    result: Value,  // 已经包含执行结果
}
```

这表明工具已在服务端执行完成，无法支持"发送 tool call → 等待前端执行 → 接收结果"的流程。

## 与当前项目的适用性

| 需求 | aisdk 支持 | 备注 |
|------|-----------|------|
| 流式文本生成 | ✅ | 完整支持 |
| 服务端工具执行 | ✅ | 完整支持 |
| 多 Provider | ✅ | 10+ 个 |
| Axum 集成 | ✅ | 开箱即用 |
| 前端工具（human-in-the-loop）| ❌ | 不支持 |
| 工具结果注入 | ❌ | 不支持 |
| 暂停/恢复流 | ❌ | 不支持 |

## 结论

`aisdk` 是一个功能完善的 Rust AI SDK，适用于：

- 纯服务端工具执行场景
- 需要支持多个 LLM Provider 的项目
- 需要与 Vercel AI SDK 前端兼容的流式响应

**不适用于**需要前端工具执行（human-in-the-loop）的场景，如 StitchWork 当前的 `create_piece_record` 工具。

如需支持前端工具，需要：

1. Fork 并修改 `aisdk` 添加暂停/恢复机制
2. 自行实现完整的 Data Stream Protocol
3. 或继续使用当前的 `cancel_with_reason` + `sendAutomaticallyWhen` 变通方案
