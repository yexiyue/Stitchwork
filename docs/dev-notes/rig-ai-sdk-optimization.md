# rig-ai-sdk 优化建议

> 基于 aisdk 源码分析的对比和优化建议

## rig-ai-sdk 定位

`rig-ai-sdk` 是一个**消息格式转换适配器**：

- **输入**：AI SDK UIMessage 格式 → Rig Message 格式
- **输出**：Rig MultiTurnStreamItem → AI SDK Data Stream Protocol 事件

**不是**：完整的 AI SDK（不需要工具定义、多 Provider 支持等）

---

## 当前优势

与 `aisdk` 的 Vercel UI Stream 实现对比，`rig-ai-sdk` 更完善：

| 特性 | rig-ai-sdk | aisdk |
|------|-----------|-------|
| 完整工具事件链 | ✅ 6种事件 | ❌ 3种 |
| 前端工具支持 | ✅ cancel_with_reason | ❌ 不支持 |
| 动态 type 解析 | ✅ `tool-{NAME}` | ❌ 不支持 |
| UIMessage 解析 | ✅ 完整 | ⚠️ 基础 |

---

## 优化建议

### 1. JSON 序列化性能优化

当前 `Display` 实现使用 `json!()` 宏动态构建，可以改用 serde 派生提升性能：

```rust
// 当前：运行时构建 JSON
impl Display for AISdkEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AISdkEvent::TextDelta { id, delta, provider_metadata } => {
                let mut obj = json!({"type": "text-delta", "id": id, "delta": delta});
                if let Some(metadata) = provider_metadata {
                    obj["providerMetadata"] = metadata.clone();
                }
                obj
            }
            // ...
        };
        write!(f, "{}", value)
    }
}

// 改进：编译时生成序列化代码
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum AISdkEventJson {
    #[serde(rename = "text-delta")]
    TextDelta {
        id: String,
        delta: String,
        #[serde(skip_serializing_if = "Option::is_none", rename = "providerMetadata")]
        provider_metadata: Option<ProviderMetadata>,
    },
    // ...
}
```

### 2. 添加 Step 跟踪

在 `AISdkStreamBuilder` 中跟踪当前步骤，便于多轮对话调试：

```rust
pub struct AISdkStreamBuilder {
    message_id: String,
    text_id: Option<String>,
    reasoning_id: Option<String>,
    current_step: usize,  // 新增
}

impl AISdkStreamBuilder {
    pub fn start_step(&mut self) -> AISdkEvent {
        self.current_step += 1;
        AISdkEvent::StartStep
    }
}
```

### 3. 前端工具状态标记

在 `ToolInputAvailable` 中添加 `state` 字段，让前端明确知道需要执行：

```rust
// adapter.rs 改进
fn convert_stream_item<R>(...) -> Vec<AISdkEvent> {
    // ...
    StreamedAssistantContent::ToolCall(tool_call) => {
        result.push(AISdkEvent::ToolInputAvailable {
            tool_call_id,
            tool_name: tool_call.function.name,
            input: tool_call.function.arguments,
            provider_executed: Some(false),  // 由前端执行
            // 可选：添加状态标记
            // state: Some("requires-action".to_string()),
            ..Default::default()
        });
    }
}
```

### 4. 便捷方法

在 `AISdkStreamBuilder` 添加常用操作的快捷方法：

```rust
impl AISdkStreamBuilder {
    /// 创建中止事件
    pub fn abort(&self, reason: impl Into<String>) -> AISdkEvent {
        AISdkEvent::Abort { reason: reason.into() }
    }

    /// 创建 usage 数据事件
    pub fn usage(&self, usage: impl Serialize) -> AISdkEvent {
        AISdkEvent::custom_data("usage", usage)
    }
}
```

---

## 不需要的功能

以下是 `aisdk` 的功能，**不适用于** `rig-ai-sdk`：

| 功能 | 原因 |
|------|------|
| `#[tool]` 宏 | 工具定义由 Rig 处理 |
| 多 Provider 支持 | Provider 由 Rig 处理 |
| LanguageModel trait | 不直接调用 LLM |
| 请求 Builder | 请求构建由业务层处理 |
| Hook 系统 | 可以在业务层实现 |

---

## 总结

`rig-ai-sdk` 当前实现已经满足需求，主要可优化方向：

1. **性能**：用 serde 派生替代 `json!()` 宏
2. **调试**：添加 step 跟踪
3. **易用性**：添加 `abort()`, `usage()` 便捷方法
4. **前端工具**：添加 `state` 字段标记（可选，当前 `cancel_with_reason` 方案也能工作）
