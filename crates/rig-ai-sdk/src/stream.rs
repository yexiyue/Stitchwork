//! AI SDK stream builder utilities
//!
//! Provides a builder for constructing AI SDK streaming responses
//! with proper state management for text and reasoning blocks.
//!
//! # Overview
//!
//! The [`AISdkStreamBuilder`] manages of state of streaming content blocks.
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

use crate::event::{AISdkEvent, FinishReason, MessageMetadata, ProviderMetadata};
use uuid::Uuid;

/// Builder for constructing AI SDK streaming responses.
///
/// Manages of state of text and reasoning blocks, ensuring proper
/// event sequencing according to the AI SDK Data Stream Protocol.
///
/// # State Management
///
/// - `message_id`: Fixed at creation, used for `start` event
/// - `text_id`: Created when starting a text block, cleared on `text_end`
/// - `reasoning_id`: Created when starting reasoning, cleared on `reasoning_end`
#[derive(Debug)]
pub struct AISdkStreamBuilder {
    message_id: String,
    text_id: Option<String>,
    reasoning_id: Option<String>,
}

impl AISdkStreamBuilder {
    /// Creates a new stream builder with a fresh message ID.
    pub fn new() -> Self {
        Self {
            message_id: Uuid::new_v4().to_string(),
            text_id: None,
            reasoning_id: None,
        }
    }

    /// Generates a message start event with builder's message ID.
    ///
    /// This should be first event sent.
    pub fn start(&self) -> AISdkEvent {
        AISdkEvent::Start {
            message_id: self.message_id.clone(),
            message_metadata: None,
            provider_metadata: None,
        }
    }

    /// Generates a message start event with custom metadata.
    ///
    /// This should be first event sent.
    pub fn start_with_metadata(
        &self,
        message_metadata: Option<MessageMetadata>,
        provider_metadata: Option<ProviderMetadata>,
    ) -> AISdkEvent {
        AISdkEvent::Start {
            message_id: self.message_id.clone(),
            message_metadata,
            provider_metadata,
        }
    }

    /// Starts a new text block.
    ///
    /// Generates a unique ID for this text block and stores it for subsequent
    /// `text_delta` calls. Returns `None` if a text block is already active.
    pub fn text_start(&mut self) -> Option<AISdkEvent> {
        self.text_start_with_metadata(None)
    }

    /// Starts a new text block with provider metadata.
    ///
    /// Generates a unique ID for this text block and stores it for subsequent
    /// `text_delta` calls. Returns `None` if a text block is already active.
    pub fn text_start_with_metadata(
        &mut self,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Option<AISdkEvent> {
        // If text_id is already Some, a text block has been started
        if self.text_id.is_some() {
            return None;
        }
        let id = Uuid::new_v4().to_string();
        self.text_id = Some(id.clone());
        Some(AISdkEvent::TextStart { id, provider_metadata })
    }

    /// Generates a text delta event if a text block is active.
    ///
    /// Returns `None` if no text block has been started.
    pub fn text_delta(&self, delta: impl Into<String>) -> Option<AISdkEvent> {
        self.text_delta_with_metadata(delta, None)
    }

    /// Generates a text delta event with provider metadata if a text block is active.
    ///
    /// Returns `None` if no text block has been started.
    pub fn text_delta_with_metadata(
        &self,
        delta: impl Into<String>,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Option<AISdkEvent> {
        self.text_id.clone().map(|id| AISdkEvent::TextDelta {
            id,
            delta: delta.into(),
            provider_metadata,
        })
    }

    /// Ends of current text block if one is active.
    ///
    /// Returns `Some(event)` if a text block was active, `None` otherwise.
    pub fn text_end(&mut self) -> Option<AISdkEvent> {
        self.text_end_with_metadata(None)
    }

    /// Ends of current text block with provider metadata if one is active.
    ///
    /// Returns `Some(event)` if a text block was active, `None` otherwise.
    pub fn text_end_with_metadata(
        &mut self,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Option<AISdkEvent> {
        self.text_id.take().map(|id| AISdkEvent::TextEnd { id, provider_metadata })
    }

    /// Starts a new reasoning block.
    ///
    /// Generates a unique ID for this reasoning block and stores it for subsequent
    /// `reasoning_delta` calls. If a reasoning block is already active, it will
    /// be replaced.
    pub fn reasoning_start(&mut self, _reasoning_id: Option<String>) -> AISdkEvent {
        self.reasoning_start_with_metadata(None)
    }

    /// Starts a new reasoning block with provider metadata.
    ///
    /// Generates a unique ID for this reasoning block and stores it for subsequent
    /// `reasoning_delta` calls. If a reasoning block is already active, it will
    /// be replaced.
    pub fn reasoning_start_with_metadata(
        &mut self,
        provider_metadata: Option<ProviderMetadata>,
    ) -> AISdkEvent {
        let id = Uuid::new_v4().to_string();
        self.reasoning_id = Some(id.clone());
        AISdkEvent::ReasoningStart { id, provider_metadata }
    }

    /// Generates a reasoning delta event if a reasoning block is active.
    ///
    /// Returns `None` if no reasoning block has been started.
    pub fn reasoning_delta(
        &self,
        delta: impl Into<String>,
        reasoning_id: Option<String>,
    ) -> Option<AISdkEvent> {
        self.reasoning_delta_with_metadata(delta, reasoning_id, None)
    }

    /// Generates a reasoning delta event with provider metadata if a reasoning block is active.
    ///
    /// Returns `None` if no reasoning block has been started.
    pub fn reasoning_delta_with_metadata(
        &self,
        delta: impl Into<String>,
        reasoning_id: Option<String>,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Option<AISdkEvent> {
        reasoning_id
            .or_else(|| self.reasoning_id.as_ref().map(|id| id.to_string()))
            .map(|id| AISdkEvent::ReasoningDelta {
                id,
                delta: delta.into(),
                provider_metadata,
            })
    }

    /// Ends of current reasoning block if one is active.
    ///
    /// Returns `Some(event)` if a reasoning block was active, `None` otherwise.
    pub fn reasoning_end(&mut self) -> Option<AISdkEvent> {
        self.reasoning_end_with_metadata(None)
    }

    /// Ends of current reasoning block with provider metadata if one is active.
    ///
    /// Returns `Some(event)` if a reasoning block was active, `None` otherwise.
    pub fn reasoning_end_with_metadata(
        &mut self,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Option<AISdkEvent> {
        self.reasoning_id
            .take()
            .map(|id| AISdkEvent::ReasoningEnd { id, provider_metadata })
    }

    /// Generates a stream finish event.
    ///
    /// Indicates that stream has completed successfully.
    pub fn finish(&self) -> AISdkEvent {
        AISdkEvent::Finish {
            finish_reason: None,
            message_metadata: None,
        }
    }

    /// Generates a stream finish event with finish reason.
    ///
    /// Indicates that stream has completed successfully with the specified reason.
    pub fn finish_with_reason(&self, finish_reason: FinishReason) -> AISdkEvent {
        AISdkEvent::Finish {
            finish_reason: Some(finish_reason),
            message_metadata: None,
        }
    }

    /// Generates a stream finish event with metadata.
    ///
    /// Indicates that stream has completed successfully.
    pub fn finish_with_metadata(
        &self,
        finish_reason: Option<FinishReason>,
        message_metadata: Option<MessageMetadata>,
    ) -> AISdkEvent {
        AISdkEvent::Finish {
            finish_reason,
            message_metadata,
        }
    }

    /// Generates a stream done marker.
    ///
    /// This is final event sent after all content events.
    pub fn done(&self) -> AISdkEvent {
        AISdkEvent::Done
    }
}

impl Default for AISdkStreamBuilder {
    fn default() -> Self {
        Self::new()
    }
}
