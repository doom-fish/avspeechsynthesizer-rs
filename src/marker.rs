use core::{ffi::c_char, ptr};

use serde::{Deserialize, Serialize};

use crate::error::AvSpeechError;
use crate::ffi;
use crate::private::{error_from_status, json_cstring, parse_json_ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
/// Represents an AVSpeechSynthesis text range.
pub struct TextRange {
    /// Stores the AVSpeechSynthesis range start location.
    pub location: usize,
    /// Stores the AVSpeechSynthesis range length.
    pub length: usize,
}

impl TextRange {
    #[must_use]
    /// Creates an AVSpeechSynthesis text range wrapper.
    pub const fn new(location: usize, length: usize) -> Self {
        Self { location, length }
    }

    #[must_use]
    /// Returns the exclusive end offset of the AVSpeechSynthesis text range.
    pub const fn end(self) -> usize {
        self.location + self.length
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Represents the AVSpeechSynthesis marker kind.
pub enum SpeechSynthesisMarkerMark {
    /// Represents an AVSpeechSynthesis phoneme marker.
    Phoneme,
    /// Represents an AVSpeechSynthesis word marker.
    Word,
    /// Represents an AVSpeechSynthesis sentence marker.
    Sentence,
    /// Represents an AVSpeechSynthesis paragraph marker.
    Paragraph,
    /// Represents an AVSpeechSynthesis bookmark marker.
    Bookmark,
    /// Represents an unknown AVSpeechSynthesis marker raw value.
    Unknown(i64),
}

impl SpeechSynthesisMarkerMark {
    #[must_use]
    /// Converts an AVSpeechSynthesis marker raw value into a wrapper enum.
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

    #[must_use]
    /// Returns the raw AVSpeechSynthesis marker value.
    pub const fn as_raw(self) -> i64 {
        match self {
            Self::Phoneme => 0,
            Self::Word => 1,
            Self::Sentence => 2,
            Self::Paragraph => 3,
            Self::Bookmark => 4,
            Self::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents an AVSpeechSynthesis speech marker.
pub struct SpeechSynthesisMarker {
    /// Stores the AVSpeechSynthesis marker kind.
    pub mark: SpeechSynthesisMarkerMark,
    /// Stores the AVSpeechSynthesis byte-sample offset for the marker.
    pub byte_sample_offset: u64,
    /// Stores the AVSpeechSynthesis text range for the marker.
    pub text_range: TextRange,
    /// Stores the AVSpeechSynthesis bookmark name, if present.
    pub bookmark_name: Option<String>,
    /// Stores the AVSpeechSynthesis phoneme text, if present.
    pub phoneme: Option<String>,
}

impl SpeechSynthesisMarker {
    /// Creates a generic AVSpeechSynthesis marker wrapper.
    pub fn new(
        mark: SpeechSynthesisMarkerMark,
        text_range: TextRange,
        byte_sample_offset: u64,
    ) -> Result<Self, AvSpeechError> {
        construct_marker(&MarkerConstructorPayload {
            constructor: MarkerConstructor::Generic,
            mark: mark.as_raw(),
            byte_sample_offset,
            text_range,
            bookmark_name: None,
            phoneme: None,
        })
    }

    /// Creates an AVSpeechSynthesis word marker wrapper.
    pub fn word(text_range: TextRange, byte_sample_offset: u64) -> Result<Self, AvSpeechError> {
        construct_marker(&MarkerConstructorPayload {
            constructor: MarkerConstructor::Word,
            mark: SpeechSynthesisMarkerMark::Word.as_raw(),
            byte_sample_offset,
            text_range,
            bookmark_name: None,
            phoneme: None,
        })
    }

    /// Creates an AVSpeechSynthesis sentence marker wrapper.
    pub fn sentence(text_range: TextRange, byte_sample_offset: u64) -> Result<Self, AvSpeechError> {
        construct_marker(&MarkerConstructorPayload {
            constructor: MarkerConstructor::Sentence,
            mark: SpeechSynthesisMarkerMark::Sentence.as_raw(),
            byte_sample_offset,
            text_range,
            bookmark_name: None,
            phoneme: None,
        })
    }

    /// Creates an AVSpeechSynthesis paragraph marker wrapper.
    pub fn paragraph(
        text_range: TextRange,
        byte_sample_offset: u64,
    ) -> Result<Self, AvSpeechError> {
        construct_marker(&MarkerConstructorPayload {
            constructor: MarkerConstructor::Paragraph,
            mark: SpeechSynthesisMarkerMark::Paragraph.as_raw(),
            byte_sample_offset,
            text_range,
            bookmark_name: None,
            phoneme: None,
        })
    }

    /// Creates an AVSpeechSynthesis phoneme marker wrapper.
    pub fn phoneme(
        phoneme: impl Into<String>,
        byte_sample_offset: u64,
    ) -> Result<Self, AvSpeechError> {
        construct_marker(&MarkerConstructorPayload {
            constructor: MarkerConstructor::Phoneme,
            mark: SpeechSynthesisMarkerMark::Phoneme.as_raw(),
            byte_sample_offset,
            text_range: TextRange::default(),
            bookmark_name: None,
            phoneme: Some(phoneme.into()),
        })
    }

    /// Creates an AVSpeechSynthesis bookmark marker wrapper.
    pub fn bookmark(
        bookmark_name: impl Into<String>,
        byte_sample_offset: u64,
    ) -> Result<Self, AvSpeechError> {
        construct_marker(&MarkerConstructorPayload {
            constructor: MarkerConstructor::Bookmark,
            mark: SpeechSynthesisMarkerMark::Bookmark.as_raw(),
            byte_sample_offset,
            text_range: TextRange::default(),
            bookmark_name: Some(bookmark_name.into()),
            phoneme: None,
        })
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis marker kind.
    pub const fn mark(&self) -> SpeechSynthesisMarkerMark {
        self.mark
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis byte-sample offset.
    pub const fn byte_sample_offset(&self) -> u64 {
        self.byte_sample_offset
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis text range.
    pub const fn text_range(&self) -> TextRange {
        self.text_range
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis bookmark name, if present.
    pub fn bookmark_name(&self) -> Option<&str> {
        self.bookmark_name.as_deref()
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis phoneme text, if present.
    pub fn phoneme_text(&self) -> Option<&str> {
        self.phoneme.as_deref()
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
enum MarkerConstructor {
    Generic,
    Word,
    Sentence,
    Paragraph,
    Phoneme,
    Bookmark,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkerConstructorPayload {
    constructor: MarkerConstructor,
    mark: i64,
    byte_sample_offset: u64,
    text_range: TextRange,
    bookmark_name: Option<String>,
    phoneme: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MarkerPayload {
    pub(crate) mark: i64,
    pub(crate) byte_sample_offset: u64,
    pub(crate) text_range: TextRange,
    pub(crate) bookmark_name: Option<String>,
    pub(crate) phoneme: Option<String>,
}

impl From<MarkerPayload> for SpeechSynthesisMarker {
    fn from(payload: MarkerPayload) -> Self {
        Self {
            mark: SpeechSynthesisMarkerMark::from_raw(payload.mark),
            byte_sample_offset: payload.byte_sample_offset,
            text_range: payload.text_range,
            bookmark_name: payload.bookmark_name,
            phoneme: payload.phoneme,
        }
    }
}

fn construct_marker(
    payload: &MarkerConstructorPayload,
) -> Result<SpeechSynthesisMarker, AvSpeechError> {
    let payload = json_cstring(&payload)?;
    let mut err_msg: *mut c_char = ptr::null_mut();
    let marker_json = unsafe { ffi::marker::avs_marker_make_json(payload.as_ptr(), &mut err_msg) };
    if !err_msg.is_null() {
        return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
    }
    let payload: MarkerPayload = unsafe { parse_json_ptr(marker_json, "speech synthesis marker") }?;
    Ok(payload.into())
}
