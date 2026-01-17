//! Rig stream to AI SDK event adapter
//!
//! This module provides adapters that convert rig's [`MultiTurnStreamItem`] stream
//! into AI SDK [`AISdkEvent`] format compatible with assistant-ui's Data Stream Protocol.
//!
//! # Overview
//!
//! The adapter transforms rig's internal streaming format into the AI SDK's event-based
//! protocol, which supports:
//!
//! - Text streaming with `text-delta` events
//! - Tool call streaming with `tool-input-*` and `tool-output-*` events
//! - Reasoning/model thinking with `reasoning-*` events
//! - Usage statistics via custom data events
//!
//! # Usage
//!
//! ```ignore
//! use rig_ai_sdk::adapt_rig_stream;
//! use rig::agent::Agent;
//!
//! async fn chat(agent: &Agent, prompt: String) -> impl Stream<Item = Result<AISdkEvent, Error>> {
//!     let rig_stream = agent.stream_chat(prompt, history).await;
//!     adapt_rig_stream(rig_stream)
//! }
//! ```

use crate::event::AISdkEvent;
use crate::stream::AISdkStreamBuilder;
use async_stream::stream;
use futures::{Stream, StreamExt};
use rig::agent::MultiTurnStreamItem;
use rig::streaming::{StreamedAssistantContent, StreamedUserContent, ToolCallDeltaContent};
use std::collections::HashMap;
use std::fmt::Display;

/// Adapts a rig multi-turn stream into an Axum SSE event stream.
///
/// This is a convenience function that combines [`adapt_rig_stream`] with
/// Axum SSE Event conversion. Use this when you want to return an SSE
/// stream directly from an Axum handler.
///
/// The resulting stream yields [`axum::response::sse::Event`] items that can
/// be directly wrapped in [`axum::response::sse::Sse`] for HTTP responses.
/// Error events are automatically converted to `error` type SSE events.
///
/// # Type Parameters
///
/// - `S`: The input stream type producing `Result<MultiTurnStreamItem<R>, E>`
/// - `R`: The response type from the agent
/// - `E`: The error type, must implement [`Display`] for error messages
///
/// # Example
///
/// ```ignore
/// use rig_ai_sdk::adapt_rig_stream_sse;
/// use axum::response::sse::{Event, Sse};
///
/// async fn chat_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
///     let rig_stream = agent.stream_chat(prompt, history).await;
///     Sse::new(adapt_rig_stream_sse(rig_stream))
/// }
/// ```
pub fn adapt_rig_stream_sse<S, R, E>(
    rig_stream: S,
) -> impl Stream<Item = Result<axum::response::sse::Event, std::convert::Infallible>>
where
    S: Stream<Item = Result<MultiTurnStreamItem<R>, E>> + Unpin,
    E: Display,
{
    adapt_rig_stream(rig_stream).map(|event| {
        Ok(axum::response::sse::Event::from(
            event.unwrap_or_else(|e| AISdkEvent::error(e.to_string())),
        ))
    })
}

/// Adapts a rig multi-turn stream into an AI SDK event stream.
///
/// This function converts rig's internal streaming format into the AI SDK
/// Data Stream Protocol format, suitable for frontend clients like assistant-ui.
///
/// # Event Flow
///
/// The adapter emits events in following order:
///
/// 1. `data-start` - Initial connection acknowledgment
/// 2. Streaming content events (text-delta, tool-input-*, reasoning-*, etc.)
/// 3. `data-finish` - Final status and metadata
/// 4. `data-done` - Stream completion
///
/// # Type Parameters
///
/// - `S`: The input stream type producing `Result<MultiTurnStreamItem<R>, E>`
/// - `R`: The response type from the agent
/// - `E`: The error type, must implement [`Display`] for error messages
///
/// # Example
///
/// ```ignore
/// use rig_ai_sdk::adapt_rig_stream;
/// use futures::StreamExt;
///
/// let rig_stream = agent.stream_chat(prompt, history).await;
/// let mut ai_sdk_stream = adapt_rig_stream(rig_stream);
///
/// while let Some(event) = ai_sdk_stream.next().await {
///     match event? {
///         AISdkEvent::TextDelta { id, delta, .. } => println!("{}: {}", id, delta),
///         AISdkEvent::ToolInputStart { tool_name, .. } => {
///             println!("Calling tool: {}", tool_name);
///         }
///         _ => {}
///     }
/// }
/// ```
pub fn adapt_rig_stream<S, R, E>(rig_stream: S) -> impl Stream<Item = Result<AISdkEvent, E>>
where
    S: Stream<Item = Result<MultiTurnStreamItem<R>, E>> + Unpin,
    E: Display,
{
    stream! {
        let mut events = AISdkStreamBuilder::new();
        let mut tool_names: HashMap<String, String> = HashMap::new();
        let mut rig_stream = rig_stream;

        // Start event
        yield Ok(events.start());

        while let Some(msg) = rig_stream.next().await {
            let msg = match msg {
                Ok(m) => m,
                Err(e) => {
                    yield Ok(AISdkEvent::error(e.to_string()));
                    break;
                }
            };

            for event in convert_stream_item(&mut events, &mut tool_names, msg) {
                yield Ok(event);
            }
        }

        // Finish events
        yield Ok(events.finish());
        yield Ok(events.done());
    }
}

/// Converts a single [`MultiTurnStreamItem`] into zero or more [`AISdkEvent`]s.
///
/// This internal function handles of core conversion logic from rig's stream
/// items to AI SDK events. The mapping is as follows:
///
/// | Rig Item | AI SDK Events |
/// |----------|---------------|
/// | `ToolResult` | `tool-output-available` |
/// | `Text` | `text-delta` (with optional `reasoning-end` and `text-start`) |
/// | `ToolCall` | `tool-input-available` |
/// | `ToolCallDelta::Name` | `tool-input-start` |
/// | `ToolCallDelta::Delta` | `tool-input-delta` |
/// | `Reasoning` | `reasoning-start`, `reasoning-delta` |
/// | `ReasoningDelta` | `reasoning-delta` |
/// | `Final` | `text-end` |
/// | `FinalResponse` | Custom `usage` data event |
///
/// # Parameters
///
/// - `events`: Mutable reference to the stream builder for tracking state
/// - `tool_names`: Mutable map for tracking tool call IDs to names
/// - `item`: The rig stream item to convert
///
/// # Returns
///
/// A vector of AI SDK events (may be empty if item is ignored)
fn convert_stream_item<R>(
    events: &mut AISdkStreamBuilder,
    tool_names: &mut HashMap<String, String>,
    item: MultiTurnStreamItem<R>,
) -> Vec<AISdkEvent> {
    let mut result = Vec::new();

    match item {
        MultiTurnStreamItem::StreamUserItem(StreamedUserContent::ToolResult(tool_result)) => {
            let tool_call_id = tool_result
                .call_id
                .as_ref()
                .unwrap_or(&tool_result.id)
                .clone();
            result.push(AISdkEvent::ToolOutputAvailable {
                tool_call_id,
                output: serde_json::to_value(&tool_result.content).unwrap_or_default(),
                provider_executed: None,
                dynamic: None,
                preliminary: None,
            });
        }
        MultiTurnStreamItem::StreamAssistantItem(assistant) => match assistant {
            StreamedAssistantContent::Text(text) => {
                // If coming from reasoning, end reasoning and start text
                if let Some(reasoning_end) = events.reasoning_end() {
                    result.push(reasoning_end);
                }

                if let Some(text_start) = events.text_start() {
                    result.push(text_start);
                }

                if let Some(text_delta) = events.text_delta(text.text) {
                    result.push(text_delta);
                }
            }
            StreamedAssistantContent::ToolCall(tool_call) => {
                let tool_call_id = tool_call.call_id.as_ref().unwrap_or(&tool_call.id).clone();
                result.push(AISdkEvent::ToolInputAvailable {
                    tool_call_id,
                    tool_name: tool_call.function.name,
                    input: tool_call.function.arguments,
                    provider_executed: None,
                    provider_metadata: None,
                    dynamic: None,
                });
            }
            StreamedAssistantContent::ToolCallDelta { id, content } => match content {
                ToolCallDeltaContent::Name(name) => {
                    tool_names.insert(id.clone(), name.clone());
                    result.push(AISdkEvent::ToolInputStart {
                        tool_call_id: id,
                        tool_name: name,
                        provider_executed: None,
                        provider_metadata: None,
                        dynamic: None,
                    });
                }
                ToolCallDeltaContent::Delta(delta) => {
                    result.push(AISdkEvent::ToolInputDelta {
                        tool_call_id: id,
                        delta,
                    });
                }
            },
            StreamedAssistantContent::Reasoning(reasoning) => {
                result.push(events.reasoning_start(reasoning.id));
                for item in &reasoning.reasoning {
                    if let Some(delta) = events.reasoning_delta(item, None) {
                        result.push(delta);
                    }
                }
            }
            StreamedAssistantContent::ReasoningDelta { reasoning, id } => {
                if let Some(reasoning_delta) = events.reasoning_delta(reasoning, id) {
                    result.push(reasoning_delta);
                }
            }
            StreamedAssistantContent::Final(_) => {
                if let Some(text_end) = events.text_end() {
                    result.push(text_end);
                }
            }
        },
        MultiTurnStreamItem::FinalResponse(final_response) => {
            result.push(AISdkEvent::custom_data("usage", final_response.usage()));
        }
        _ => {}
    }

    result
}
