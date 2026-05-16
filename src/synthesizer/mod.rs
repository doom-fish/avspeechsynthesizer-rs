use core::ffi::{c_char, c_void, CStr};
use core::ptr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;

use serde::Deserialize;

use crate::error::AvSpeechError;
use crate::ffi;
use crate::private::{error_from_status, json_cstring, parse_json_ptr, to_cstring};
use crate::utterance::{SpeechUtterance, UtterancePayload};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeechBoundary {
    Immediate,
    Word,
}

impl SpeechBoundary {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        match self {
            Self::Immediate => 0,
            Self::Word => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextRange {
    pub location: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeechSynthesisMarkerMark {
    Phoneme,
    Word,
    Sentence,
    Paragraph,
    Bookmark,
    Unknown(i64),
}

impl SpeechSynthesisMarkerMark {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Phoneme,
            1 => Self::Word,
            2 => Self::Sentence,
            3 => Self::Paragraph,
            4 => Self::Bookmark,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpeechSynthesisMarker {
    pub mark: SpeechSynthesisMarkerMark,
    pub byte_sample_offset: u64,
    pub text_range: TextRange,
    pub bookmark_name: Option<String>,
    pub phoneme: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpeechEvent {
    DidStart(SpeechUtterance),
    DidFinish(SpeechUtterance),
    DidPause(SpeechUtterance),
    DidContinue(SpeechUtterance),
    DidCancel(SpeechUtterance),
    WillSpeakRangeOfSpeechString {
        character_range: TextRange,
        utterance: SpeechUtterance,
    },
    WillSpeakMarker {
        marker: SpeechSynthesisMarker,
        utterance: SpeechUtterance,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrittenAudioFile {
    path: PathBuf,
    markers: Vec<SpeechSynthesisMarker>,
}

impl WrittenAudioFile {
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn markers(&self) -> &[SpeechSynthesisMarker] {
        &self.markers
    }
}

type EventHandler = Box<dyn Fn(SpeechEvent) + Send + Sync + 'static>;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RangePayload {
    location: usize,
    length: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MarkerPayload {
    mark: i64,
    byte_sample_offset: u64,
    text_range: RangePayload,
    bookmark_name: Option<String>,
    phoneme: Option<String>,
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

impl From<MarkerPayload> for SpeechSynthesisMarker {
    fn from(payload: MarkerPayload) -> Self {
        Self {
            mark: SpeechSynthesisMarkerMark::from_raw(payload.mark),
            byte_sample_offset: payload.byte_sample_offset,
            text_range: payload.text_range.into(),
            bookmark_name: payload.bookmark_name,
            phoneme: payload.phoneme,
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

pub struct SpeechSynthesizer {
    token: *mut c_void,
    callback: Arc<EventHandlerBox>,
}

impl Drop for SpeechSynthesizer {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe {
                ffi::avs_synthesizer_set_event_handler(self.token, None, ptr::null_mut());
                ffi::avs_synthesizer_release(self.token);
            }
            self.token = ptr::null_mut();
        }
    }
}

impl SpeechSynthesizer {
    pub fn new() -> Result<Self, AvSpeechError> {
        let token = unsafe { ffi::avs_synthesizer_new() };
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

    pub fn set_event_handler<F>(&mut self, callback: F)
    where
        F: Fn(SpeechEvent) + Send + Sync + 'static,
    {
        self.callback.replace(callback);
        let callback_raw = Arc::as_ptr(&self.callback).cast::<c_void>().cast_mut();
        unsafe {
            ffi::avs_synthesizer_set_event_handler(
                self.token,
                Some(event_trampoline),
                callback_raw,
            );
        }
    }

    pub fn clear_event_handler(&mut self) {
        self.callback.clear();
        unsafe {
            ffi::avs_synthesizer_set_event_handler(self.token, None, ptr::null_mut());
        }
    }

    pub fn speak(&self, utterance: &SpeechUtterance) -> Result<(), AvSpeechError> {
        let utterance_json = json_cstring(&UtterancePayload::from(utterance))?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::avs_synthesizer_speak_json(self.token, utterance_json.as_ptr(), &mut err_msg)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    #[must_use]
    pub fn pause_speaking(&self, boundary: SpeechBoundary) -> bool {
        unsafe { ffi::avs_synthesizer_pause(self.token, boundary.as_raw()) }
    }

    #[must_use]
    pub fn stop_speaking(&self, boundary: SpeechBoundary) -> bool {
        unsafe { ffi::avs_synthesizer_stop(self.token, boundary.as_raw()) }
    }

    #[must_use]
    pub fn continue_speaking(&self) -> bool {
        unsafe { ffi::avs_synthesizer_continue(self.token) }
    }

    #[must_use]
    pub fn is_speaking(&self) -> bool {
        unsafe { ffi::avs_synthesizer_is_speaking(self.token) }
    }

    #[must_use]
    pub fn is_paused(&self) -> bool {
        unsafe { ffi::avs_synthesizer_is_paused(self.token) }
    }

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
            ffi::avs_synthesizer_write_utterance_to_file_json(
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

    pub fn pump_run_loop(&self, duration: Duration) {
        unsafe {
            ffi::avs_run_loop_pump(duration.as_secs_f64());
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

    let utterance = SpeechUtterance::from(raw.utterance);
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
