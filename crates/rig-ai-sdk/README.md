# rig-ai-sdk

AI SDK Data Stream Protocol adapter for [rig-core](https://github.com/zebp/rig-core).

This crate provides types and utilities to work with AI SDK's streaming protocol
in rig-based applications, enabling seamless integration between frontend AI SDK clients
(like [assistant-ui](https://sdk.vercel.ai/docs/ai-sdk-ui)) and rig-powered backends.

## Overview

The AI SDK Data Stream Protocol is a standardized format for streaming AI responses.
This crate implements the protocol for use with the rig agent framework:

- **Event types** - Complete set of events for streaming responses
- **Message types** - Frontend message format parsing
- **Conversion utilities** - Transform AI SDK messages to rig format
- **Stream adapters** - Convert rig streams to AI SDK events
- **Axum SSE support** - Direct integration with Axum handlers (optional feature)

## Features

| Feature | Default | Description |
|----------|-----------|-------------|
| `axum-sse` | No | Enables SSE stream conversion for Axum handlers |

## Installation

```toml
[dependencies]
rig-ai-sdk = { version = "0.1", features = ["axum-sse"] }
```

## Quick Start

### Basic Streaming

```rust
use rig_ai_sdk::{AISdkStreamBuilder, AISdkEvent};
use futures::StreamExt;

let mut builder = AISdkStreamBuilder::new();
let mut events = vec![builder.start()];

// Stream text
for chunk in ["Hello", ", ", "world", "!"] {
    events.push(builder.text_delta(chunk).unwrap());
}
events.push(builder.text_end().unwrap());
events.push(builder.finish());
events.push(builder.done());
```

### Axum SSE Handler

```rust
use axum::response::sse::{Event, Sse};
use rig_ai_sdk::adapt_rig_stream_sse;

async fn chat_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rig_stream = agent.stream_chat(prompt, history).await;
    Sse::new(adapt_rig_stream_sse(rig_stream))
}
```

### Converting Frontend Messages

```rust
use rig_ai_sdk::{UIMessage, extract_prompt_and_history};

let messages: Vec<UIMessage> = serde_json::from_str(&body)?;
let (prompt, history) = extract_prompt_and_history(&messages)?;
// Now use with rig agent
let response = agent.run_chat(prompt, history).await?;
```

## Modules

### [`event`](src/event.rs)

Defines [`AISdkEvent`] enum representing all protocol event types:

- **Lifecycle**: `start`, `finish`, `done`, `abort`, `error`
- **Text**: `text-start`, `text-delta`, `text-end`
- **Reasoning**: `reasoning-start`, `reasoning-delta`, `reasoning-end`
- **Tools**: `tool-input-start`, `tool-input-delta`, `tool-input-available`, `tool-output-available`
- **Sources**: `source-url`, `source-document`, `file`
- **Custom**: `data-{name}` events for any custom data

### [`message`](src/message.rs)

Types for receiving AI SDK messages from the frontend:

- [`UIMessage`] - Individual message with role and parts
- [`UIMessagePart`] - Message parts (text, tool, file, reasoning, etc.)
- [`PartState`] - Streaming state (`streaming` or `done`)
- [`DynamicToolState`] - Tool lifecycle states (5.x format)

### [`convert`](src/convert.rs)

Conversion utilities between AI SDK and rig formats:

- [`convert_messages`] - Batch convert UI messages to rig messages
- [`convert_message`] - Convert a single UI message
- [`extract_prompt_and_history`] - Split messages into prompt and history

### [`stream`](src/stream.rs)

[`AISdkStreamBuilder`] for constructing streaming responses with proper state management:

```rust
let mut builder = AISdkStreamBuilder::new();

// Proper event sequencing
builder.start();           // Initialize
builder.text_start();       // Begin text block
builder.text_delta("...");  // Stream content
builder.text_end();         // End text block
builder.finish();           // Complete stream
builder.done();             // Final marker
```

### [`adapter`](src/adapter.rs) (requires `axum-sse`)

Stream adapters for integrating rig with Axum:

- [`adapt_rig_stream`] - Convert rig stream to AI SDK events
- [`adapt_rig_stream_sse`] - Direct Axum SSE conversion

## Event Flow

A typical AI SDK stream follows this pattern:

```text
[start]
  ↓
[text-start] → [text-delta]* → [text-end]
  ↓
[reasoning-start] → [reasoning-delta]* → [reasoning-end]
  ↓
[tool-input-start] → [tool-input-delta]* → [tool-input-available]
  ↓
[tool-output-available]
  ↓
[finish] → [done]
```

## Conversion Mapping

| AI SDK Part | Rig Content |
|--------------|--------------|
| `Text` | `UserContent::Text` / `AssistantContent::Text` |
| `Reasoning` | `AssistantContent::Reasoning` |
| `File` (image) | `UserContent::Image` |
| `ToolCall` | `AssistantContent::ToolCall` |
| `DynamicTool` (InputAvailable) | `AssistantContent::ToolCall` |
| `DynamicTool` (OutputAvailable) | `UserContent::ToolResult` |
| `DynamicTool` (OutputError) | `UserContent::ToolResult` |
| `ToolResult` | `UserContent::ToolResult` |

## References

- [AI SDK Stream Protocol](https://sdk.vercel.ai/docs/ai-sdk-ui/stream-protocol)
- [AI SDK Transport](https://ai-sdk.dev/docs/ai-sdk-ui/transport)
- [assistant-ui](https://sdk.vercel.ai/docs/ai-sdk-ui)
- [rig-core](https://github.com/zebp/rig-core)

## License

MIT
