use core::ffi::{c_char, c_void, CStr};
use core::ptr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;

use serde::Deserialize;

use crate::buffer_callback::{
    CollectedBufferWriteEventKind, CollectedBufferWritePayload, SpeechAudioBuffer,
};
use crate::error::AvSpeechError;
use crate::ffi;
use crate::marker::{MarkerPayload, SpeechSynthesisMarker, TextRange};
use crate::private::{
    error_from_status, json_cstring, parse_json_ptr, string_from_ptr, to_cstring,
};
use crate::utterance::{SpeechUtterance, UtterancePayload};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the AVSpeechSynthesis pause or stop boundary.
pub enum SpeechBoundary {
    /// Represents the immediate AVSpeechSynthesis boundary.
    Immediate,
    /// Represents the AVSpeechSynthesis word boundary.
    Word,
}

impl SpeechBoundary {
    #[must_use]
    /// Returns the raw AVSpeechSynthesis boundary value.
    pub const fn as_raw(self) -> i32 {
        match self {
            Self::Immediate => 0,
            Self::Word => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents an AVSpeechSynthesis delegate event.
pub enum SpeechEvent {
    /// Reports that AVSpeechSynthesis started an utterance.
    DidStart(SpeechUtterance),
    /// Reports that AVSpeechSynthesis finished an utterance.
    DidFinish(SpeechUtterance),
    /// Reports that AVSpeechSynthesis paused an utterance.
    DidPause(SpeechUtterance),
    /// Reports that AVSpeechSynthesis resumed an utterance.
    DidContinue(SpeechUtterance),
    /// Reports that AVSpeechSynthesis canceled an utterance.
    DidCancel(SpeechUtterance),
    /// Reports that AVSpeechSynthesis will speak a character range.
    WillSpeakRangeOfSpeechString {
        /// Stores the AVSpeechSynthesis character range about to be spoken.
        character_range: TextRange,
        /// Stores the AVSpeechSynthesis utterance that owns the range.
        utterance: SpeechUtterance,
    },
    /// Reports that AVSpeechSynthesis will speak a marker.
    WillSpeakMarker {
        /// Stores the AVSpeechSynthesis marker about to be spoken.
        marker: SpeechSynthesisMarker,
        /// Stores the AVSpeechSynthesis utterance that owns the marker.
        utterance: SpeechUtterance,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents an AVSpeechSynthesis utterance written to disk.
pub struct WrittenAudioFile {
    path: PathBuf,
    markers: Vec<SpeechSynthesisMarker>,
}

impl WrittenAudioFile {
    #[must_use]
    /// Returns the output path produced by AVSpeechSynthesis.
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis markers collected while writing the file.
    pub fn markers(&self) -> &[SpeechSynthesisMarker] {
        &self.markers
    }
}

type EventHandler = Box<dyn Fn(SpeechEvent) + Send + Sync + 'static>;
type BufferHandler = Box<dyn FnMut(SpeechAudioBuffer) + Send + 'static>;
type MarkerHandler = Box<dyn FnMut(Vec<SpeechSynthesisMarker>) + Send + 'static>;

#[derive(Default)]
struct EventHandlerBox {
    handler: RwLock<Option<EventHandler>>,
}

impl EventHandlerBox {
    fn read_handler(&self) -> RwLockReadGuard<'_, Option<EventHandler>> {
        match self.handler.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn write_handler(&self) -> RwLockWriteGuard<'_, Option<EventHandler>> {
        match self.handler.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn replace<F>(&self, callback: F)
    where
        F: Fn(SpeechEvent) + Send + Sync + 'static,
    {
        *self.write_handler() = Some(Box::new(callback));
    }

    fn clear(&self) {
        *self.write_handler() = None;
    }

    fn dispatch(&self, event: SpeechEvent) {
        if let Some(handler) = self.read_handler().as_ref() {
            handler(event);
        }
    }
}

struct WriteCallbackBox {
    buffer_handler: Mutex<BufferHandler>,
    marker_handler: Option<Mutex<MarkerHandler>>,
}

impl WriteCallbackBox {
    fn with_buffer<F>(callback: F) -> Self
    where
        F: FnMut(SpeechAudioBuffer) + Send + 'static,
    {
        Self {
            buffer_handler: Mutex::new(Box::new(callback)),
            marker_handler: None,
        }
    }

    fn with_callbacks<F, G>(buffer_callback: F, marker_callback: G) -> Self
    where
        F: FnMut(SpeechAudioBuffer) + Send + 'static,
        G: FnMut(Vec<SpeechSynthesisMarker>) + Send + 'static,
    {
        Self {
            buffer_handler: Mutex::new(Box::new(buffer_callback)),
            marker_handler: Some(Mutex::new(Box::new(marker_callback))),
        }
    }

    fn lock_buffer_handler(&self) -> MutexGuard<'_, BufferHandler> {
        match self.buffer_handler.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn lock_marker_handler(&self) -> Option<MutexGuard<'_, MarkerHandler>> {
        self.marker_handler
            .as_ref()
            .map(|handler| match handler.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            })
    }

    fn dispatch_buffer(&self, buffer: SpeechAudioBuffer) {
        let mut handler = self.lock_buffer_handler();
        handler(buffer);
    }

    fn dispatch_markers(&self, markers: Vec<SpeechSynthesisMarker>) {
        if let Some(mut handler) = self.lock_marker_handler() {
            handler(markers);
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RangePayload {
    location: usize,
    length: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventPayload {
    event: String,
    utterance: UtterancePayload,
    character_range: Option<RangePayload>,
    marker: Option<MarkerPayload>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriteResultPayload {
    output_path: String,
    markers: Vec<MarkerPayload>,
}

impl From<RangePayload> for TextRange {
    fn from(payload: RangePayload) -> Self {
        Self {
            location: payload.location,
            length: payload.length,
        }
    }
}

impl From<WriteResultPayload> for WrittenAudioFile {
    fn from(payload: WriteResultPayload) -> Self {
        Self {
            path: PathBuf::from(payload.output_path),
            markers: payload.markers.into_iter().map(Into::into).collect(),
        }
    }
}

/// Wraps an AVSpeechSynthesis synthesizer instance.
pub struct SpeechSynthesizer {
    token: *mut c_void,
    callback: Arc<EventHandlerBox>,
}

impl Drop for SpeechSynthesizer {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe {
                ffi::synthesizer::avs_synthesizer_set_event_handler(
                    self.token,
                    None,
                    ptr::null_mut(),
                );
                ffi::synthesizer::avs_synthesizer_release(self.token);
            }
            self.token = ptr::null_mut();
        }
    }
}

impl SpeechSynthesizer {
    /// Creates a new AVSpeechSynthesis synthesizer wrapper.
    pub fn new() -> Result<Self, AvSpeechError> {
        let token = unsafe { ffi::synthesizer::avs_synthesizer_new() };
        if token.is_null() {
            return Err(AvSpeechError::Unknown(
                "failed to allocate AVSpeechSynthesizer bridge".to_owned(),
            ));
        }
        Ok(Self {
            token,
            callback: Arc::new(EventHandlerBox::default()),
        })
    }

    /// Returns the AVSpeechSynthesis notification name for voice-catalog changes.
    pub fn available_voices_did_change_notification_name() -> Result<String, AvSpeechError> {
        unsafe {
            string_from_ptr(
                ffi::synthesizer::avs_available_voices_did_change_notification_name(),
                "available voices notification name",
            )
        }
    }

    /// Installs an AVSpeechSynthesis delegate callback handler.
    pub fn set_event_handler<F>(&mut self, callback: F)
    where
        F: Fn(SpeechEvent) + Send + Sync + 'static,
    {
        self.callback.replace(callback);
        let callback_raw = Arc::as_ptr(&self.callback).cast::<c_void>().cast_mut();
        unsafe {
            ffi::synthesizer::avs_synthesizer_set_event_handler(
                self.token,
                Some(event_trampoline),
                callback_raw,
            );
        }
    }

    /// Removes the current AVSpeechSynthesis delegate callback handler.
    pub fn clear_event_handler(&mut self) {
        self.callback.clear();
        unsafe {
            ffi::synthesizer::avs_synthesizer_set_event_handler(self.token, None, ptr::null_mut());
        }
    }

    /// Starts speaking an AVSpeechSynthesis utterance.
    pub fn speak(&self, utterance: &SpeechUtterance) -> Result<(), AvSpeechError> {
        let utterance_json = json_cstring(&UtterancePayload::from(utterance))?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::synthesizer::avs_synthesizer_speak_json(
                self.token,
                utterance_json.as_ptr(),
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Streams AVSpeechSynthesis audio buffers for an utterance.
    pub fn write_utterance_with_buffer_callback<F>(
        &self,
        utterance: &SpeechUtterance,
        buffer_callback: F,
    ) -> Result<(), AvSpeechError>
    where
        F: FnMut(SpeechAudioBuffer) + Send + 'static,
    {
        let callbacks = WriteCallbackBox::with_buffer(buffer_callback);
        self.write_utterance_with_callback_box(utterance, &callbacks)
    }

    /// Streams AVSpeechSynthesis audio buffers and markers for an utterance.
    pub fn write_utterance_with_callbacks<F, G>(
        &self,
        utterance: &SpeechUtterance,
        buffer_callback: F,
        marker_callback: G,
    ) -> Result<(), AvSpeechError>
    where
        F: FnMut(SpeechAudioBuffer) + Send + 'static,
        G: FnMut(Vec<SpeechSynthesisMarker>) + Send + 'static,
    {
        let callbacks = WriteCallbackBox::with_callbacks(buffer_callback, marker_callback);
        self.write_utterance_with_callback_box(utterance, &callbacks)
    }

    fn write_utterance_with_callback_box(
        &self,
        utterance: &SpeechUtterance,
        callbacks: &WriteCallbackBox,
    ) -> Result<(), AvSpeechError> {
        let utterance_json = json_cstring(&UtterancePayload::from(utterance))?;
        let mut result_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::buffer_callback::avs_synthesizer_collect_buffers_json(
                self.token,
                utterance_json.as_ptr(),
                &mut result_json,
                &mut err_msg,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, err_msg) });
        }

        let payload: CollectedBufferWritePayload =
            unsafe { parse_json_ptr(result_json, "collected speech audio buffers") }?;
        for event in payload.events {
            match event.kind {
                CollectedBufferWriteEventKind::Buffer => {
                    if let Some(buffer) = event
                        .buffer
                        .and_then(|value| SpeechAudioBuffer::try_from(value).ok())
                    {
                        callbacks.dispatch_buffer(buffer);
                    }
                }
                CollectedBufferWriteEventKind::MarkerBatch => {
                    if let Some(marker_batch) = event.marker_batch {
                        callbacks.dispatch_markers(marker_batch.into());
                    }
                }
            }
        }
        Ok(())
    }

    #[must_use]
    /// Requests AVSpeechSynthesis to pause at `boundary`.
    pub fn pause_speaking(&self, boundary: SpeechBoundary) -> bool {
        unsafe { ffi::synthesizer::avs_synthesizer_pause(self.token, boundary.as_raw()) }
    }

    #[must_use]
    /// Requests AVSpeechSynthesis to stop at `boundary`.
    pub fn stop_speaking(&self, boundary: SpeechBoundary) -> bool {
        unsafe { ffi::synthesizer::avs_synthesizer_stop(self.token, boundary.as_raw()) }
    }

    #[must_use]
    /// Requests AVSpeechSynthesis to continue after a pause.
    pub fn continue_speaking(&self) -> bool {
        unsafe { ffi::synthesizer::avs_synthesizer_continue(self.token) }
    }

    #[must_use]
    /// Returns whether AVSpeechSynthesis is currently speaking.
    pub fn is_speaking(&self) -> bool {
        unsafe { ffi::synthesizer::avs_synthesizer_is_speaking(self.token) }
    }

    #[must_use]
    /// Returns whether AVSpeechSynthesis is currently paused.
    pub fn is_paused(&self) -> bool {
        unsafe { ffi::synthesizer::avs_synthesizer_is_paused(self.token) }
    }

    #[cfg(feature = "async")]
    #[allow(dead_code, reason = "used by the optional async API surface")]
    #[must_use]
    pub(crate) fn as_raw(&self) -> *mut c_void {
        self.token
    }

    /// Writes an AVSpeechSynthesis utterance to an audio file.
    pub fn write_utterance_to_file<P>(
        &self,
        utterance: &SpeechUtterance,
        output_path: P,
    ) -> Result<WrittenAudioFile, AvSpeechError>
    where
        P: AsRef<Path>,
    {
        let utterance_json = json_cstring(&UtterancePayload::from(utterance))?;
        let output_path = output_path.as_ref().to_str().ok_or_else(|| {
            AvSpeechError::InvalidArgument("output path must be valid UTF-8".to_owned())
        })?;
        let output_path = to_cstring(output_path)?;
        let mut result_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::synthesizer::avs_synthesizer_write_utterance_to_file_json(
                self.token,
                utterance_json.as_ptr(),
                output_path.as_ptr(),
                &mut result_json,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            let payload: WriteResultPayload =
                unsafe { parse_json_ptr(result_json, "written audio file") }?;
            Ok(payload.into())
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Pumps the AVSpeechSynthesis run loop for `duration`.
    pub fn pump_run_loop(&self, duration: Duration) {
        unsafe {
            ffi::synthesizer::avs_run_loop_pump(duration.as_secs_f64());
        }
    }
}

unsafe extern "C" fn event_trampoline(user_info: *mut c_void, payload_json: *const c_char) {
    if user_info.is_null() || payload_json.is_null() {
        return;
    }

    let callback_box = &*user_info.cast::<EventHandlerBox>();
    let Ok(payload) = CStr::from_ptr(payload_json).to_str() else {
        return;
    };
    let Ok(raw) = serde_json::from_str::<EventPayload>(payload) else {
        return;
    };
    let Ok(utterance) = SpeechUtterance::try_from(raw.utterance) else {
        return;
    };

    let event = match raw.event.as_str() {
        "didStart" => SpeechEvent::DidStart(utterance),
        "didFinish" => SpeechEvent::DidFinish(utterance),
        "didPause" => SpeechEvent::DidPause(utterance),
        "didContinue" => SpeechEvent::DidContinue(utterance),
        "didCancel" => SpeechEvent::DidCancel(utterance),
        "willSpeakRangeOfSpeechString" => match raw.character_range {
            Some(character_range) => SpeechEvent::WillSpeakRangeOfSpeechString {
                character_range: character_range.into(),
                utterance,
            },
            None => return,
        },
        "willSpeakMarker" => match raw.marker {
            Some(marker) => SpeechEvent::WillSpeakMarker {
                marker: marker.into(),
                utterance,
            },
            None => return,
        },
        _ => return,
    };

    callback_box.dispatch(event);
}
