# rig + assistant-ui 前端工具调研文档

> 调研目标：如何在 rig (Rust AI Agent 框架) 中实现前端工具，配合 assistant-ui 的 `makeAssistantTool`

## 1. 背景

### 1.1 需求

在 StitchWork 项目中，需要实现**前端工具**（Frontend Tools）：
- LLM 可以调用这些工具
- 工具在**前端执行**（而不是后端）
- 支持用户交互（如表单填写、确认操作）
- 执行结果返回给 LLM 继续对话

### 1.2 当前技术栈

| 层级 | 技术 | 版本 |
| ---- | ---- | ---- |
| 前端 UI | @assistant-ui/react | v0.11.53 |
| 前端 AI SDK | @assistant-ui/react-ai-sdk | v1.1.20 |
| 后端 Agent | rig-core | v0.28.0 |
| 后端 MCP | rmcp | v0.12.0 |
| 流协议 | AI SDK Data Stream Protocol | - |

---

## 2. 技术调研

### 2.1 assistant-ui 前端工具机制

#### makeAssistantTool vs makeAssistantToolUI

| 函数 | 用途 | 执行位置 |
| ---- | ---- | -------- |
| `makeAssistantToolUI` | 仅渲染 UI，工具在后端执行 | 后端 |
| `makeAssistantTool` | 完整工具（定义 + 执行 + UI） | 前端 |

#### makeAssistantTool 使用示例

```tsx
import { makeAssistantTool, tool } from "@assistant-ui/react";
import { z } from "zod";

const WeatherTool = makeAssistantTool({
  toolName: "getWeather",
  description: "Get current weather for a location",
  parameters: z.object({
    location: z.string(),
    unit: z.enum(["celsius", "fahrenheit"]),
  }),
  // 前端执行逻辑
  execute: async ({ location, unit }) => {
    const weather = await fetchWeatherAPI(location, unit);
    return weather;
  },
  // 渲染 UI
  render: ({ args, result, status }) => {
    if (status.type === "running") {
      return <div>Checking weather in {args.location}...</div>;
    }
    return <WeatherCard result={result} />;
  },
});
```

#### 前端工具的数据流（AI SDK v5）

```text
1. 前端定义工具 (makeAssistantTool)
   ↓
2. AssistantChatTransport 将工具定义发送给后端 (request.tools)
   ↓
3. 后端使用 frontendTools(tools) 包装工具定义
   ↓
4. LLM 决定调用工具，返回 tool_call
   ↓
5. 后端返回 tool-input-available 事件 (providerExecuted: false)
   ↓
6. 前端接收事件，执行本地 execute 函数
   ↓
7. 前端发送 tool-output-available 或新请求包含 tool result
   ↓
8. 后端继续对话
```

#### 关键：providerExecuted 字段

```typescript
// AI SDK 事件格式
{
  type: "tool-input-available",
  toolCallId: "call_123",
  toolName: "getWeather",
  input: { location: "Beijing" },
  providerExecuted: false  // false = 前端执行，true = 后端已执行
}
```

### 2.2 rig 工具机制

#### Tool trait 定义

rig 的工具必须实现 `Tool` trait：

```rust
pub trait Tool: Send + Sync {
    const NAME: &'static str;
    type Error: std::error::Error + Send + Sync;
    type Args: for<'de> Deserialize<'de> + Send;
    type Output: Serialize + Send;

    // 工具定义（名称、描述、参数 schema）
    async fn definition(&self, prompt: String) -> ToolDefinition;

    // 工具执行
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error>;
}
```

#### 工具注册方式

```rust
// 1. 静态工具 - 编译时确定
let agent = client
    .agent("gpt-4")
    .tool(Adder)      // 实现了 Tool trait
    .tool(Subtract)
    .build();

// 2. 动态工具 - 从向量存储检索
let agent = client
    .agent("gpt-4")
    .dynamic_tools(2, vector_store_index, toolset)
    .build();

// 3. MCP 工具 - 通过 rmcp
let agent = client
    .agent("gpt-4")
    .rmcp_tools(tools.tools, mcp_client.peer().clone())
    .build();
```

#### 关键发现

1. **rig 没有 `tool_definition` 方法** - 不能只添加定义不添加实现
2. **Tool trait 必须实现 `call` 方法** - 必须有执行逻辑
3. **工具在编译时确定** - 不支持运行时动态添加任意工具
4. **MCP 工具是例外** - rmcp 支持运行时工具注册

#### rig Agent 执行流程（关键问题）

```text
Agent.stream_chat() 内部循环：
1. 发送 prompt 给 LLM
2. LLM 返回响应（可能包含 tool_call）
3. 如果有 tool_call：
   a. 自动执行 tool.call()
   b. 将结果发回 LLM
   c. 回到步骤 1 继续
4. 如果没有 tool_call：返回最终响应
```

**问题**：如果我们在 `tool.call()` 中返回"待执行"标记，rig 会把它当作真正的结果，继续循环，不会等待前端执行。

#### Prompt Hooks 机制（解决方案）

rig 提供了 **Prompt Hooks**，可以在 agent 循环的各个阶段插入自定义逻辑：

```rust
// Prompt Hooks 可以：
// - 访问 completion request, response, tool calls, tool responses
// - 使用 CancelSignal::cancel() 取消 agentic loop
// - 取消后返回 PromptError::PromptCancelled 和当前 chat history
```

**这意味着我们可以**：
1. 实现一个 Prompt Hook
2. 在检测到前端工具调用时，使用 `CancelSignal::cancel()` 中断循环
3. 返回 tool_call 信息给前端
4. 前端执行完成后，新请求带上 tool result，继续对话

### 2.3 当前项目实现分析

#### 现有代码结构

```text
crates/server/src/chat/
├── mod.rs           # ChatSession - 管理 agent 和 MCP
├── request.rs       # ForwardedTools 类型定义
└── session_manager.rs

crates/rig-ai-sdk/src/
├── adapter.rs       # rig stream → AI SDK 事件转换
├── convert.rs       # UIMessage → rig Message 转换
├── event.rs         # AISdkEvent 定义
└── message.rs       # UIMessage 类型
```

#### 当前 ForwardedTools 处理方式

```rust
// crates/server/src/chat/mod.rs
// 当前只是拼接到 system prompt，LLM 无法真正调用
let frontend_tools_prompt = if let Some(forwarded) = &forwarded_tools {
    let mut tools_str = String::from("你可以使用以下前端工具：\n\n");
    for (name, tool) in &forwarded.tools {
        tools_str.push_str(&format!("1. {} - {}\n", name, tool.description));
    }
    tools_str
} else {
    String::new()
};
```

**问题**：LLM 知道有这些工具，但无法通过标准 tool_call 调用它们。

---

## 3. 方案对比

### 方案 A：通过 MCP 实现前端工具代理

**思路**：创建一个特殊的 MCP 工具，当被调用时返回标记让前端执行。

```rust
// 创建前端工具的 MCP 服务
pub struct FrontendToolsMcp {
    tools: HashMap<String, ForwardedTool>,
}

#[tool_router]
impl FrontendToolsMcp {
    // 动态生成的工具方法
    pub async fn call_frontend_tool(
        &self,
        name: String,
        args: Value,
    ) -> Result<Json<FrontendPending>> {
        Ok(Json(FrontendPending {
            marker: "__FRONTEND_PENDING__",
            tool_name: name,
            args,
        }))
    }
}
```

**优点**：
- 利用现有 MCP 基础设施
- rig 原生支持 rmcp_tools

**缺点**：
- rmcp 的 `#[tool_router]` 宏是编译时的，不支持动态工具名
- 需要修改 rmcp 或绕过宏

### 方案 B：实现动态 Tool trait

**思路**：为每个前端工具动态创建实现 Tool trait 的结构体。

```rust
pub struct FrontendToolProxy {
    name: String,
    description: String,
    parameters: Value,
}

impl Tool for FrontendToolProxy {
    // 使用 name() 方法而不是 const NAME
    fn name(&self) -> &str { &self.name }

    async fn definition(&self, _: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name.clone(),
            description: self.description.clone(),
            parameters: self.parameters.clone(),
        }
    }

    async fn call(&self, args: Value) -> Result<Value, Error> {
        // 返回前端待执行标记
        Ok(json!({
            "__frontend_pending__": true,
            "tool_name": self.name,
            "args": args
        }))
    }
}
```

**问题**：rig 的 `Tool` trait 要求 `const NAME: &'static str`，不支持动态名称。

### 方案 C：修改适配层拦截 ToolCall（推荐）

**思路**：不在 rig 层面注册前端工具，而是：
1. 将前端工具定义直接注入到 LLM 请求中
2. 在适配层拦截 ToolCall，判断是否是前端工具
3. 如果是前端工具，发送 `providerExecuted: false` 事件

```text
流程：
1. 前端发送 tools 定义
2. 后端将工具定义注入到 LLM 请求（通过 additional_tools 或修改 request）
3. LLM 返回 ToolCall
4. 适配层检查 tool_name 是否在 frontend_tools 列表中
5. 如果是：发送 tool-input-available (providerExecuted: false)
6. 如果不是：正常执行后端工具
```

**实现要点**：

```rust
// adapter.rs
fn convert_stream_item(
    events: &mut AISdkStreamBuilder,
    tool_names: &mut HashMap<String, String>,
    frontend_tools: &HashSet<String>,  // 新增：前端工具名称集合
    item: MultiTurnStreamItem,
) -> Vec<AISdkEvent> {
    match item {
        // 当 LLM 发出 ToolCall 时
        StreamedAssistantContent::ToolCall(tool_call) => {
            let is_frontend = frontend_tools.contains(&tool_call.function.name);

            result.push(AISdkEvent::ToolInputAvailable {
                tool_call_id: tool_call.id,
                tool_name: tool_call.function.name,
                input: tool_call.function.arguments,
                provider_executed: Some(!is_frontend),  // 前端工具 = false
                ..
            });

            if is_frontend {
                // 前端工具：不执行，等待前端返回结果
                return result;
            }
            // 后端工具：继续执行...
        }
    }
}
```

**优点**：
- 最小改动，不修改 rig 核心逻辑
- 利用现有的消息转换机制
- 与 AI SDK 协议完全兼容

**缺点**：
- 需要找到方法将工具定义注入到 LLM 请求中
- 可能需要修改 rig 的 agent 调用方式

### 方案 D：使用 Prompt Hooks 中断循环（推荐）

**思路**：利用 rig 的 Prompt Hooks 机制，在检测到前端工具调用时中断 agent 循环。

```rust
use rig::agent::{PromptHook, CancelSignal};

pub struct FrontendToolHook {
    frontend_tools: HashSet<String>,
    pending_tool_call: Arc<Mutex<Option<ToolCallInfo>>>,
}

impl PromptHook for FrontendToolHook {
    // 在 tool call 执行前检查
    fn on_tool_call(&self, tool_call: &ToolCall, cancel: &CancelSignal) {
        if self.frontend_tools.contains(&tool_call.function.name) {
            // 保存 tool call 信息
            *self.pending_tool_call.lock().unwrap() = Some(ToolCallInfo {
                id: tool_call.id.clone(),
                name: tool_call.function.name.clone(),
                arguments: tool_call.function.arguments.clone(),
            });
            // 取消循环，不执行后端工具
            cancel.cancel();
        }
    }
}
```

**执行流程**：

```text
1. 前端发送消息 + 工具定义
2. 后端创建 agent，注册 FrontendToolHook
3. Agent 调用 LLM
4. LLM 返回 tool_call (create-record)
5. FrontendToolHook.on_tool_call() 检测到是前端工具
6. 调用 cancel.cancel() 中断循环
7. 返回 PromptError::PromptCancelled + chat_history
8. 后端发送 tool-input-available (providerExecuted: false) 给前端
9. 前端执行工具，用户交互
10. 前端发送新请求，包含 tool_result
11. 后端继续对话（chat_history 包含之前的 tool_call + 新的 tool_result）
```

**优点**：

- 利用 rig 原生机制，不需要 hack
- 完美中断循环，不会产生错误的工具结果
- 保留完整的 chat history

**缺点**：

- 需要研究 Prompt Hooks 的具体 API
- 每次前端工具调用都需要新请求

---

## 4. 推荐方案

### 综合考虑，推荐方案 D（Prompt Hooks）

#### 核心思路

1. **前端**：使用 `makeAssistantTool` 定义工具
2. **传输**：`AssistantChatTransport` 将工具定义发送到后端 `request.tools`
3. **后端**：
   - 将前端工具定义注册为 MCP 工具（让 LLM 知道有这些工具）
   - 实现 `FrontendToolHook`，在检测到前端工具调用时中断循环
4. **适配层**：检测到循环被取消，发送 `tool-input-available (providerExecuted: false)`
5. **前端**：接收事件，执行本地 `execute` 函数，发送结果继续对话

#### 关键实现点

##### 4.1 后端：实现 FrontendToolHook

```rust
use rig::agent::{PromptHook, CancelSignal};
use std::sync::{Arc, Mutex};

pub struct FrontendToolHook {
    /// 前端工具名称集合
    frontend_tools: HashSet<String>,
    /// 被中断的 tool call 信息（用于返回给前端）
    pending_call: Arc<Mutex<Option<PendingToolCall>>>,
}

impl PromptHook for FrontendToolHook {
    fn on_tool_call(&self, tool_call: &ToolCall, cancel: &CancelSignal) {
        if self.frontend_tools.contains(&tool_call.function.name) {
            // 保存 tool call 信息
            let mut pending = self.pending_call.lock().unwrap();
            *pending = Some(PendingToolCall {
                tool_call_id: tool_call.id.clone(),
                tool_name: tool_call.function.name.clone(),
                input: tool_call.function.arguments.clone(),
            });
            // 中断 agent 循环
            cancel.cancel();
        }
    }
}
```

##### 4.2 后端：ChatSession 集成 Hook

```rust
impl ChatSession {
    pub async fn chat(&self, prompt: Message, history: Vec<Message>)
        -> impl Stream<Item = Result<Event, Infallible>>
    {
        let hook = FrontendToolHook::new(&self.frontend_tools);
        let pending_call = hook.pending_call.clone();

        // 使用 hook 执行
        let result = self.agent
            .prompt(prompt)
            .with_hook(hook)
            .stream_chat(history)
            .await;

        // 检查是否有被中断的前端工具调用
        if let Some(pending) = pending_call.lock().unwrap().take() {
            // 发送 tool-input-available 事件
            return stream_with_pending_tool(pending);
        }

        adapt_rig_stream_sse(result)
    }
}
```

##### 4.3 适配层：处理 PromptCancelled 错误

```rust
// 当收到 PromptError::PromptCancelled 时
// 不当作错误，而是检查是否有 pending tool call
// 如果有，发送 tool-input-available 事件
```

##### 4.4 前端：完善 CreateRecordTool

```tsx
export const CreateRecordTool = makeAssistantTool({
  toolName: "create-record",
  parameters: CreateRecordSchema,
  description: "创建计件记录",

  // 前端执行逻辑
  execute: async (args) => {
    // 这里不会自动执行，而是等待用户在 render 中交互
    // 可以直接调用 API 或返回用户输入
  },

  // 渲染交互 UI
  render: ({ args, status, result, addResult }) => {
    if (status?.type === "running") {
      return (
        <RecordCreationForm
          initialValues={args}
          onSubmit={async (values) => {
            // 调用 API 创建记录
            const result = await createPieceRecord(values);
            // 将结果返回给 assistant-ui
            addResult(result);
          }}
          onCancel={() => {
            addResult({ success: false, message: "用户取消" });
          }}
        />
      );
    }

    if (result) {
      return <RecordCreationResult result={result} />;
    }

    return null;
  },
});
```

---

## 5. 源码调研结果

### 5.1 StreamingPromptHook trait API

rig 提供了 `StreamingPromptHook` trait（位于 `rig-core/src/agent/prompt_request/streaming.rs`）：

```rust
pub trait StreamingPromptHook<M>: Clone + Send + Sync
where
    M: CompletionModel,
{
    /// 发送 prompt 给模型前调用
    fn on_completion_call(
        &self,
        prompt: &Message,
        history: &[Message],
        cancel_sig: CancelSignal,
    ) -> impl Future<Output = ()> + Send { async {} }

    /// 收到文本增量时调用
    fn on_text_delta(
        &self,
        text_delta: &str,
        aggregated_text: &str,
        cancel_sig: CancelSignal,
    ) -> impl Future<Output = ()> + Send { async {} }

    /// 收到工具调用增量时调用
    fn on_tool_call_delta(
        &self,
        tool_call_id: &str,
        tool_name: Option<&str>,
        tool_call_delta: &str,
        cancel_sig: CancelSignal,
    ) -> impl Future<Output = ()> + Send { async {} }

    /// 工具执行前调用 ✅ 关键：可在此拦截前端工具
    fn on_tool_call(
        &self,
        tool_name: &str,
        tool_call_id: Option<String>,
        args: &str,
        cancel_sig: CancelSignal,
    ) -> impl Future<Output = ()> + Send { async {} }

    /// 工具执行后调用
    fn on_tool_result(
        &self,
        tool_name: &str,
        tool_call_id: Option<String>,
        args: &str,
        result: &str,
        cancel_sig: CancelSignal,
    ) -> impl Future<Output = ()> + Send { async {} }

    /// 流式响应完成后调用
    fn on_stream_completion_response_finish(
        &self,
        prompt: &Message,
        response: &<M as CompletionModel>::StreamingResponse,
        cancel_sig: CancelSignal,
    ) -> impl Future<Output = ()> + Send { async {} }
}
```

### 5.2 关键发现

1. **ToolCall 事件先于 hook 调用**：rig 先 `yield ToolCall` 事件，然后才调用 `hook.on_tool_call()`
2. **CancelSignal 可中断循环**：调用 `cancel_sig.cancel()` 后返回 `PromptError::prompt_cancelled`，包含 `chat_history`
3. **ToolDyn trait 支持动态工具**：可以实现 `ToolDyn` trait 来注册运行时工具定义

### 5.3 前端工具注册方案：FrontendToolProxy

实现 `ToolDyn` trait 来注册前端工具定义：

```rust
use rig::tool::{ToolDefinition, ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
use serde_json::Value;

/// 前端工具代理 - 只提供定义，不执行（由 Hook 拦截）
pub struct FrontendToolProxy {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

impl FrontendToolProxy {
    pub fn new(name: String, description: String, parameters: Value) -> Self {
        Self { name, description, parameters }
    }

    /// 从 ForwardedTools 批量创建
    pub fn from_forwarded(tools: &ForwardedTools) -> Vec<Box<dyn ToolDyn>> {
        tools.tools.iter().map(|(name, tool)| {
            Box::new(Self::new(
                name.clone(),
                tool.description.clone(),
                tool.parameters.clone(),
            )) as Box<dyn ToolDyn>
        }).collect()
    }
}

impl ToolDyn for FrontendToolProxy {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn definition<'a>(&'a self, _prompt: String) -> WasmBoxedFuture<'a, ToolDefinition> {
        Box::pin(async move {
            ToolDefinition {
                name: self.name.clone(),
                description: self.description.clone(),
                parameters: self.parameters.clone(),
            }
        })
    }

    fn call<'a>(&'a self, _args: String) -> WasmBoxedFuture<'a, Result<String, ToolError>> {
        // 不会被调用，FrontendToolHook 会在调用前拦截
        Box::pin(async move {
            Err(ToolError::ToolCallError(Box::new(
                std::io::Error::new(std::io::ErrorKind::Other, "Frontend tool should not be called on backend")
            )))
        })
    }
}
```

## 6. 待验证问题

| 问题 | 状态 | 优先级 |
| ---- | ---- | ------ |
| ~~rig PromptHook trait 的具体 API 和方法签名？~~ | ✅ 已确认 | - |
| ~~PromptHook 中如何访问 tool_call 信息？~~ | ✅ 通过 on_tool_call 参数 | - |
| ~~CancelSignal 取消后，chat_history 如何获取？~~ | ✅ PromptCancelled 包含 | - |
| ~~如何将前端工具定义注册为 rig 可调用的工具？~~ | ✅ FrontendToolProxy | - |
| ~~rig 是否支持 stream_chat 配合 PromptHook？~~ | ✅ StreamingPromptRequest.with_hook() | - |
| assistant-ui 的 frontendTools 函数内部实现原理？ | 已了解 | - |

---

## 7. 下一步行动

1. ~~**研究 rig PromptHook 源码**~~：✅ 已确认 StreamingPromptHook API
2. ~~**研究 rig 工具注册机制**~~：✅ 已确认 ToolDyn trait + AgentBuilder.tools()
3. **实现 FrontendToolProxy**：创建 `crates/server/src/chat/frontend_tool.rs`
4. **实现 FrontendToolHook**：在 `on_tool_call` 中拦截前端工具
5. **修改适配层**：处理 `PromptCancelled` 错误，发送 `providerExecuted: false`
6. **前端集成测试**：验证 `makeAssistantTool` 完整流程

---

## 8. 参考资料

- [assistant-ui Generative UI Guide](https://www.assistant-ui.com/docs/guides/ToolUI)
- [assistant-ui Tools Guide](https://www.assistant-ui.com/docs/guides/Tools)
- [rig Documentation](https://docs.rig.rs/)
- [AI SDK Data Stream Protocol](https://sdk.vercel.ai/docs/ai-sdk-ui/stream-protocol)
