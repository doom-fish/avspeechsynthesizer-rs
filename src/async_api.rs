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
use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream};
use std::convert::TryFrom;
use std::ffi::c_void;
use std::sync::{Mutex, PoisonError};

type SharedSender = Mutex<Option<AsyncStreamSender<SpeechSynthesisEvent>>>;
type SenderContext = CallbackContext<SharedSender>;

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
        /// Stores the AVSpeechSynthesis character range about to be spoken.
        character_range: TextRange,
        /// Stores the AVSpeechSynthesis utterance that owns the range.
        utterance: SpeechUtterance,
    },
    /// About to speak a marker (macOS 14+)
    WillSpeakMarker {
        /// Stores the AVSpeechSynthesis marker about to be spoken.
        marker: SpeechSynthesisMarker,
        /// Stores the AVSpeechSynthesis utterance that owns the marker.
        utterance: SpeechUtterance,
    },
}

/// Handle that closes the async event stream when dropped
struct SubscriptionHandle {
    bridge: *mut c_void,
    context: SenderContext,
}

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.bridge.is_null() {
            // SAFETY: `self.bridge` is a valid handle returned by `avs_synthesis_event_subscribe`
            // and is being freed exactly once (guaranteed by Drop semantics).
            unsafe { avs_synthesis_event_unsubscribe(self.bridge) };
        }
    }
}

unsafe impl Send for SubscriptionHandle {}
unsafe impl Sync for SubscriptionHandle {}

/// Async stream of speech synthesis events
///
/// Wraps [`BoundedAsyncStream`] to provide event stream iteration over speech synthesis
/// events. When this stream is dropped, the underlying subscription is automatically cleaned up.
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
        if capacity == 0 {
            return Err(crate::error::AvSpeechError::InvalidArgument(
                "event stream capacity must be greater than zero".to_string(),
            ));
        }
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let context = SenderContext::new(Mutex::new(Some(sender)));

        let bridge = unsafe {
            avs_synthesis_event_subscribe(
                synthesizer.as_raw(),
                event_callback,
                Some(close_callback),
                context.as_ptr(),
                Some(SenderContext::RETAIN),
                Some(SenderContext::RELEASE),
            )
        };

        if bridge.is_null() {
            return Err(crate::error::AvSpeechError::Unknown(
                "Failed to subscribe to synthesis events".to_string(),
            ));
        }

        Ok(Self {
            inner: stream,
            _handle: SubscriptionHandle { bridge, context },
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
unsafe extern "C" fn event_callback(kind: i32, payload: *mut c_void, ctx: *mut c_void) {
    let push = |sender: &SharedSender| {
        if payload.is_null() {
            return;
        }

        // SAFETY: `payload` is a valid C string pointer because it came from the Swift bridge
        // and the bridge guarantees it is null-terminated.
        let cstr = unsafe { std::ffi::CStr::from_ptr(payload.cast()) };
        let Ok(json_str) = cstr.to_str() else { return };

        if let Ok(event_payload) = serde_json::from_str::<EventPayload>(json_str) {
            if let Some(event) = event_payload.to_event(kind) {
                if let Some(sender) = sender
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .as_ref()
                {
                    sender.push(event);
                }
            }
        }
    };
    let _ = unsafe { SenderContext::with(ctx, "event_callback", push) };
}

unsafe extern "C" fn close_callback(ctx: *mut c_void) {
    let close = |sender: &SharedSender| {
        drop(sender.lock().unwrap_or_else(PoisonError::into_inner).take());
    };
    let _ = unsafe { SenderContext::with(ctx, "close_callback", close) };
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

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::{SpeechSynthesisEvent, SpeechSynthesisEventStream};
    use crate::error::AvSpeechError;
    use crate::synthesizer::test_support::{deliver_start, listener_counts};
    use crate::synthesizer::{SpeechEvent, SpeechSynthesizer};

    fn started_text(event: Option<SpeechSynthesisEvent>) -> String {
        match event {
            Some(SpeechSynthesisEvent::DidStart(utterance)) => utterance.speech_string().to_owned(),
            other => panic!("expected a DidStart event, got {other:?}"),
        }
    }

    #[test]
    fn a_stream_coexists_with_the_event_handler() {
        let mut synthesizer = SpeechSynthesizer::new().expect("synthesizer");
        let (tx, rx) = mpsc::channel();
        synthesizer.set_event_handler(move |event| {
            let _ = tx.send(event);
        });
        let stream = SpeechSynthesisEventStream::subscribe(&synthesizer, 8).expect("stream");
        assert_eq!(listener_counts(&synthesizer), (true, 1));

        deliver_start(&synthesizer, c"both");
        assert_eq!(started_text(stream.try_next()), "both");
        assert!(matches!(rx.try_recv(), Ok(SpeechEvent::DidStart(_))));

        drop(stream);
        assert_eq!(listener_counts(&synthesizer), (true, 0));
        deliver_start(&synthesizer, c"handler only");
        match rx.try_recv() {
            Ok(SpeechEvent::DidStart(utterance)) => {
                assert_eq!(utterance.speech_string(), "handler only");
            }
            other => panic!("expected the handler to keep receiving events, got {other:?}"),
        }
    }

    #[test]
    fn every_stream_receives_every_event() {
        let synthesizer = SpeechSynthesizer::new().expect("synthesizer");
        let first = SpeechSynthesisEventStream::subscribe(&synthesizer, 4).expect("first");
        let second = SpeechSynthesisEventStream::subscribe(&synthesizer, 4).expect("second");
        assert_eq!(listener_counts(&synthesizer), (false, 2));

        deliver_start(&synthesizer, c"shared");
        assert_eq!(started_text(first.try_next()), "shared");
        assert_eq!(started_text(second.try_next()), "shared");

        drop(first);
        deliver_start(&synthesizer, c"second only");
        assert_eq!(started_text(second.try_next()), "second only");
        assert_eq!(listener_counts(&synthesizer), (false, 1));
    }

    #[test]
    fn dropping_the_subscription_frees_the_sender() {
        let synthesizer = SpeechSynthesizer::new().expect("synthesizer");
        let SpeechSynthesisEventStream {
            inner,
            _handle: handle,
        } = SpeechSynthesisEventStream::subscribe(&synthesizer, 4).expect("stream");
        assert!(!inner.is_closed());

        drop(handle);
        assert!(inner.is_closed());
        assert_eq!(listener_counts(&synthesizer), (false, 0));
    }

    #[test]
    fn a_stream_ends_when_its_synthesizer_is_dropped() {
        let synthesizer = SpeechSynthesizer::new().expect("synthesizer");
        let mut stream = SpeechSynthesisEventStream::subscribe(&synthesizer, 4).expect("stream");
        deliver_start(&synthesizer, c"last");
        assert!(!stream.is_closed());

        drop(synthesizer);
        assert!(stream.is_closed());
        assert_eq!(started_text(stream.try_next()), "last");
        assert!(pollster::block_on(stream.next()).is_none());
    }

    #[test]
    fn zero_capacity_is_rejected() {
        let synthesizer = SpeechSynthesizer::new().expect("synthesizer");
        assert!(matches!(
            SpeechSynthesisEventStream::subscribe(&synthesizer, 0),
            Err(AvSpeechError::InvalidArgument(_))
        ));
        assert_eq!(listener_counts(&synthesizer), (false, 0));
    }
}
