//! Async API for `AVSpeechSynthesizer`
//!
//! This module provides async stream wrappers for speech synthesis events when the `async` feature is enabled.
//! The async API is **executor-agnostic** and works with any async runtime (Tokio, async-std, smol, etc.).
//!
//! ## Available Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`SpeechSynthesisEventStream`] | Async stream of speech synthesis events |
//!
//! ## Event Types
//!
//! All speech synthesis events are emitted as a single [`SpeechSynthesisEvent`] enum:
//! - `DidStart` — synthesis started speaking an utterance
//! - `DidFinish` — synthesis finished speaking an utterance
//! - `DidPause` — synthesis paused
//! - `DidContinue` — synthesis continued after pause
//! - `DidCancel` — synthesis was cancelled
//! - `WillSpeakRange` — about to speak a character range
//! - `WillSpeakMarker` — about to speak a marker (macOS 14+)
//!
//! ## Examples
//!
//! ```rust,no_run
//! use avspeechsynthesizer::prelude::*;
//! use avspeechsynthesizer::async_api::{SpeechSynthesisEventStream, SpeechSynthesisEvent};
//!
//! let synthesizer = SpeechSynthesizer::new()?;
//! let mut events = SpeechSynthesisEventStream::subscribe(&synthesizer, 16)?;
//!
//! let utterance = SpeechUtterance::new("Hello, world!");
//! synthesizer.speak(&utterance)?;
//!
//! // Listen for events with pollster
//! pollster::block_on(async {
//!     while let Some(event) = events.next().await {
//!         match event {
//!             SpeechSynthesisEvent::DidFinish(_) => break,
//!             SpeechSynthesisEvent::WillSpeakRange { character_range, .. } => {
//!                 println!("Speaking range: {:?}", character_range);
//!             }
//!             _ => {}
//!         }
//!     }
//! });
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::ffi::synthesizer::{avs_synthesis_event_subscribe, avs_synthesis_event_unsubscribe};
use crate::marker::{SpeechSynthesisMarker, TextRange};
use crate::utterance::SpeechUtterance;
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream};
use std::convert::TryFrom;
use std::ffi::c_void;

/// A speech synthesis event emitted from the [`SpeechSynthesisEventStream`]
#[derive(Debug, Clone)]
pub enum SpeechSynthesisEvent {
    /// Synthesis started speaking an utterance
    DidStart(SpeechUtterance),
    /// Synthesis finished speaking an utterance
    DidFinish(SpeechUtterance),
    /// Synthesis paused
    DidPause(SpeechUtterance),
    /// Synthesis continued after pause
    DidContinue(SpeechUtterance),
    /// Synthesis was cancelled
    DidCancel(SpeechUtterance),
    /// About to speak a character range
    WillSpeakRange {
        character_range: TextRange,
        utterance: SpeechUtterance,
    },
    /// About to speak a marker (macOS 14+)
    WillSpeakMarker {
        marker: SpeechSynthesisMarker,
        utterance: SpeechUtterance,
    },
}

/// Handle that closes the async event stream when dropped
struct SubscriptionHandle(*mut c_void);

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { avs_synthesis_event_unsubscribe(self.0) };
        }
    }
}

unsafe impl Send for SubscriptionHandle {}
unsafe impl Sync for SubscriptionHandle {}

/// Async stream of speech synthesis events
///
/// Wraps [`BoundedAsyncStream`](doom_fish_utils::stream::BoundedAsyncStream) to provide
/// event stream iteration over speech synthesis events. When this stream is dropped,
/// the underlying subscription is automatically cleaned up.
pub struct SpeechSynthesisEventStream {
    inner: BoundedAsyncStream<SpeechSynthesisEvent>,
    _handle: SubscriptionHandle,
}

impl SpeechSynthesisEventStream {
    /// Subscribe to speech synthesis events
    ///
    /// # Arguments
    ///
    /// * `synthesizer` - The synthesizer to subscribe to
    /// * `capacity` - Size of the event buffer. When full, oldest events are dropped.
    ///   Use larger capacity (e.g. 16, 32) to avoid losing events.
    ///
    /// # Returns
    ///
    /// A new event stream, or an error if subscription fails.
    pub fn subscribe(
        synthesizer: &crate::synthesizer::SpeechSynthesizer,
        capacity: usize,
    ) -> Result<Self, crate::error::AvSpeechError> {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));

        let handle = unsafe {
            avs_synthesis_event_subscribe(
                synthesizer.as_raw(),
                event_callback,
                sender_ptr.cast(),
            )
        };

        if handle.is_null() {
            // Clean up the boxed sender if subscription failed
            unsafe { drop(Box::from_raw(sender_ptr)) };
            return Err(crate::error::AvSpeechError::Unknown(
                "Failed to subscribe to synthesis events".to_string(),
            ));
        }

        Ok(Self {
            inner: stream,
            _handle: SubscriptionHandle(handle),
        })
    }

    /// Get the next event asynchronously
    ///
    /// Returns a future that resolves to the next event, or `None` if the stream is closed.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> doom_fish_utils::stream::NextItem<'_, SpeechSynthesisEvent> {
        self.inner.next()
    }

    /// Try to get the next event without waiting
    ///
    /// Returns `None` if the buffer is empty.
    #[must_use]
    pub fn try_next(&self) -> Option<SpeechSynthesisEvent> {
        self.inner.try_next()
    }

    /// Get the number of currently buffered events
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Clear all buffered events without closing the stream
    pub fn clear_buffer(&self) {
        self.inner.clear_buffer();
    }

    /// Check if the stream has been closed
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

impl std::fmt::Debug for SpeechSynthesisEventStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpeechSynthesisEventStream")
            .field("buffered", &self.buffered_count())
            .field("is_closed", &self.is_closed())
            .finish_non_exhaustive()
    }
}

// Event callback from Swift
extern "C" fn event_callback(kind: i32, payload: *mut c_void, ctx: *mut c_void) {
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<SpeechSynthesisEvent>>() };

    if payload.is_null() {
        return; // Encoding error, skip
    }

    // Parse the JSON payload
    let cstr = unsafe { std::ffi::CStr::from_ptr(payload.cast()) };
    let Ok(json_str) = cstr.to_str() else { return };

    if let Ok(payload) = serde_json::from_str::<EventPayload>(json_str) {
        if let Some(event) = payload.to_event(kind) {
            sender.push(event);
        }
    }
}

#[derive(serde::Deserialize)]
struct EventPayload {
    utterance: crate::utterance::UtterancePayload,
    #[serde(rename = "characterRange")]
    character_range: Option<TextRange>,
    marker: Option<crate::marker::MarkerPayload>,
}

impl EventPayload {
    fn to_event(&self, kind: i32) -> Option<SpeechSynthesisEvent> {
        let utterance = SpeechUtterance::try_from(self.utterance.clone()).ok()?;

        Some(match kind {
            0 => SpeechSynthesisEvent::DidStart(utterance),
            1 => SpeechSynthesisEvent::DidFinish(utterance),
            2 => SpeechSynthesisEvent::DidPause(utterance),
            3 => SpeechSynthesisEvent::DidContinue(utterance),
            4 => SpeechSynthesisEvent::DidCancel(utterance),
            5 => SpeechSynthesisEvent::WillSpeakRange {
                character_range: self.character_range.unwrap_or_default(),
                utterance,
            },
            6 => {
                let marker = self.marker.as_ref()?;
                SpeechSynthesisEvent::WillSpeakMarker {
                    marker: marker.clone().into(),
                    utterance,
                }
            }
            _ => return None,
        })
    }
}


