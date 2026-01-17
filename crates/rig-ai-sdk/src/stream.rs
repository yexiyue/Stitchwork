//! AI SDK stream builder utilities
//!
//! Provides a builder for constructing AI SDK streaming responses
//! with proper state management for text and reasoning blocks.
//!
//! # Overview
//!
//! The [`AISdkStreamBuilder`] manages the state of streaming content blocks.
//! It ensures proper sequencing of events:
//!
//! - `start` → `text-start` → `text-delta`* → `text-end` → `finish` → `done`
//! - `start` → `reasoning-start` → `reasoning-delta`* → `reasoning-end`
//!   → `text-start` → `text-delta`* → `text-end` → `finish` → `done`
//!
//! # Examples
//!
//! ## Basic Text Streaming
//!
//! ```ignore
//! use rig_ai_sdk::AISdkStreamBuilder;
//!
//! let mut builder = AISdkStreamBuilder::new();
//!
//! // Start stream
//! let events = vec![builder.start()];
//!
//! // Stream text
//! for chunk in ["Hello", ", ", "world", "!"] {
//!     events.push(builder.text_delta(chunk));
//! }
//! events.push(builder.text_end());
//! ```
//!
//! ## With Reasoning
//!
//! ```ignore
//! use rig_ai_sdk::AISdkStreamBuilder;
//!
//! let mut builder = AISdkStreamBuilder::new();
//!
//! // Reasoning first
//! events.push(builder.reasoning_start());
//! for chunk in ["Let me", " think..."] {
//!     events.push(builder.reasoning_delta(chunk));
//! }
//! if let Some(end) = builder.reasoning_end() {
//!     events.push(end);
//! }
//!
//! // Then text
//! events.push(builder.text_start());
//! for chunk in ["The answer", " is 42."] {
//!     events.push(builder.text_delta(chunk));
//! }
//! ```

use crate::event::AISdkEvent;
use uuid::Uuid;

/// Builder for constructing AI SDK streaming responses.
///
/// Manages the state of text and reasoning blocks, ensuring proper
/// event sequencing according to the AI SDK Data Stream Protocol.
///
/// # State Management
///
/// - `message_id`: Fixed at creation, used for the `start` event
/// - `text_id`: Created when starting a text block, cleared on `text_end`
/// - `reasoning_id`: Created when starting reasoning, cleared on `reasoning_end`
#[derive(Debug)]
pub struct AISdkStreamBuilder {
    message_id: Uuid,
    text_id: Option<Uuid>,
    reasoning_id: Option<Uuid>,
}

impl AISdkStreamBuilder {
    /// Creates a new stream builder with a fresh message ID.
    pub fn new() -> Self {
        Self {
            message_id: Uuid::new_v4(),
            text_id: None,
            reasoning_id: None,
        }
    }

    /// Generates a message start event with the builder's message ID.
    ///
    /// This should be the first event sent.
    pub fn start(&self) -> AISdkEvent {
        AISdkEvent::Start {
            message_id: self.message_id,
        }
    }

    /// Starts a new text block.
    ///
    /// Generates a unique ID for this text block and stores it for subsequent
    /// `text_delta` calls. Returns `None` if a text block is already active.
    pub fn text_start(&mut self) -> AISdkEvent {
        let id = Uuid::new_v4();
        self.text_id = Some(id);
        AISdkEvent::TextStart { id }
    }

    /// Generates a text delta event if a text block is active.
    ///
    /// Returns `None` if no text block has been started.
    pub fn text_delta(&self, delta: impl Into<String>) -> Option<AISdkEvent> {
        self.text_id.map(|id| AISdkEvent::TextDelta {
            id,
            delta: delta.into(),
        })
    }

    /// Ends the current text block if one is active.
    ///
    /// Returns `Some(event)` if a text block was active, `None` otherwise.
    pub fn text_end(&mut self) -> Option<AISdkEvent> {
        self.text_id.take().map(|id| AISdkEvent::TextEnd { id })
    }

    /// Starts a new reasoning block.
    ///
    /// Generates a unique ID for this reasoning block and stores it for subsequent
    /// `reasoning_delta` calls. If a reasoning block is already active, it will
    /// be replaced.
    pub fn reasoning_start(&mut self) -> AISdkEvent {
        let id = Uuid::new_v4();
        self.reasoning_id = Some(id);
        AISdkEvent::ReasoningStart { id }
    }

    /// Generates a reasoning delta event if a reasoning block is active.
    ///
    /// Returns `None` if no reasoning block has been started.
    pub fn reasoning_delta(&self, delta: impl Into<String>) -> Option<AISdkEvent> {
        self.reasoning_id.map(|id| AISdkEvent::ReasoningDelta {
            id,
            delta: delta.into(),
        })
    }

    /// Ends the current reasoning block if one is active.
    ///
    /// Returns `Some(event)` if a reasoning block was active, `None` otherwise.
    pub fn reasoning_end(&mut self) -> Option<AISdkEvent> {
        self.reasoning_id
            .take()
            .map(|id| AISdkEvent::ReasoningEnd { id })
    }

    /// Generates a stream finish event.
    ///
    /// Indicates that the stream has completed successfully.
    pub fn finish(&self) -> AISdkEvent {
        AISdkEvent::Finish
    }

    /// Generates a stream done marker.
    ///
    /// This is the final event sent after all content events.
    pub fn done(&self) -> AISdkEvent {
        AISdkEvent::Done
    }
}

impl Default for AISdkStreamBuilder {
    fn default() -> Self {
        Self::new()
    }
}
