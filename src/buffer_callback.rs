use core::ffi::{c_char, CStr};
use serde_json::Value;
use std::time::Duration;

use crate::error::AvSpeechError;
use crate::ffi::buffer_callback::AudioPlaneList;
use crate::private::parse_json_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the AVSpeechSynthesis common audio-buffer format.
pub enum SpeechAudioCommonFormat {
    /// Represents a non-PCM AVSpeechSynthesis audio format.
    Other,
    /// Represents AVSpeechSynthesis audio encoded as 32-bit floating-point PCM.
    PcmFloat32,
    /// Represents AVSpeechSynthesis audio encoded as 64-bit floating-point PCM.
    PcmFloat64,
    /// Represents AVSpeechSynthesis audio encoded as 16-bit integer PCM.
    PcmInt16,
    /// Represents AVSpeechSynthesis audio encoded as 32-bit integer PCM.
    PcmInt32,
    /// Represents an unknown AVSpeechSynthesis audio format.
    Unknown,
}

impl SpeechAudioCommonFormat {
    #[must_use]
    /// Converts an AVSpeechSynthesis format name into a wrapper enum.
    pub fn from_name(value: &str) -> Self {
        match value {
            "pcmFormatFloat32" => Self::PcmFloat32,
            "pcmFormatFloat64" => Self::PcmFloat64,
            "pcmFormatInt16" => Self::PcmInt16,
            "pcmFormatInt32" => Self::PcmInt32,
            "otherFormat" => Self::Other,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents an AVSpeechSynthesis audio buffer yielded by write callbacks.
pub struct SpeechAudioBuffer {
    sample_rate: f64,
    channel_count: usize,
    frame_length: usize,
    common_format: SpeechAudioCommonFormat,
    is_interleaved: bool,
    planes: Vec<Vec<u8>>,
    audio_file_settings_json: Option<String>,
    is_end_of_stream: bool,
}

impl SpeechAudioBuffer {
    #[must_use]
    /// Returns the AVSpeechSynthesis buffer sample rate.
    pub const fn sample_rate(&self) -> f64 {
        self.sample_rate
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis buffer channel count.
    pub const fn channel_count(&self) -> usize {
        self.channel_count
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis buffer frame length.
    pub const fn frame_length(&self) -> usize {
        self.frame_length
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis buffer common format.
    pub const fn common_format(&self) -> SpeechAudioCommonFormat {
        self.common_format
    }

    #[must_use]
    /// Returns whether the AVSpeechSynthesis buffer stores interleaved samples.
    pub const fn is_interleaved(&self) -> bool {
        self.is_interleaved
    }

    #[must_use]
    /// Returns the raw AVSpeechSynthesis audio planes.
    pub fn planes(&self) -> &[Vec<u8>] {
        &self.planes
    }

    #[must_use]
    /// Returns the serialized AVSpeechSynthesis audio file settings, if present.
    pub fn audio_file_settings_json(&self) -> Option<&str> {
        self.audio_file_settings_json.as_deref()
    }

    /// Parses the AVSpeechSynthesis audio file settings into JSON values.
    pub fn audio_file_settings(&self) -> Result<Option<Value>, AvSpeechError> {
        self.audio_file_settings_json
            .as_deref()
            .map(|json| parse_json_str(json, "audio file settings"))
            .transpose()
    }

    #[must_use]
    /// Returns whether this AVSpeechSynthesis buffer ends the stream.
    pub const fn is_end_of_stream(&self) -> bool {
        self.is_end_of_stream
    }

    #[must_use]
    /// Returns the total byte size across all AVSpeechSynthesis audio planes.
    pub fn total_bytes(&self) -> usize {
        self.planes.iter().map(Vec::len).sum()
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    /// Returns the approximate AVSpeechSynthesis buffer duration.
    pub fn duration(&self) -> Duration {
        if self.sample_rate <= 0.0 {
            return Duration::ZERO;
        }
        Duration::from_secs_f64(self.frame_length as f64 / self.sample_rate)
    }
}

impl SpeechAudioBuffer {
    pub(crate) unsafe fn from_native(
        sample_rate: f64,
        channel_count: u32,
        frame_length: u32,
        format: usize,
        is_interleaved: bool,
        list: *const AudioPlaneList,
        settings: *const c_char,
    ) -> Result<Self, AvSpeechError> {
        let mut planes = Vec::new();
        if frame_length != 0 {
            if list.is_null() {
                return Err(AvSpeechError::Io("missing PCM buffer list".into()));
            }
            let count = unsafe { (*list).count } as usize;
            let first = unsafe { core::ptr::addr_of!((*list).planes) }.cast();
            let buffers = unsafe {
                std::slice::from_raw_parts::<crate::ffi::buffer_callback::AudioPlane>(first, count)
            };
            for buffer in buffers {
                if buffer.size == 0 {
                    planes.push(Vec::new());
                } else if buffer.data.is_null() {
                    return Err(AvSpeechError::Io("missing PCM plane data".into()));
                } else {
                    planes.push(
                        unsafe { std::slice::from_raw_parts(buffer.data, buffer.size as usize) }
                            .to_vec(),
                    );
                }
            }
        }
        let audio_file_settings_json = if settings.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(settings) }
                    .to_str()
                    .map_err(|e| AvSpeechError::Io(e.to_string()))?
                    .to_owned(),
            )
        };
        Ok(Self {
            sample_rate,
            channel_count: channel_count as usize,
            frame_length: frame_length as usize,
            common_format: match format {
                0 => SpeechAudioCommonFormat::Other,
                1 => SpeechAudioCommonFormat::PcmFloat32,
                2 => SpeechAudioCommonFormat::PcmFloat64,
                3 => SpeechAudioCommonFormat::PcmInt16,
                4 => SpeechAudioCommonFormat::PcmInt32,
                _ => SpeechAudioCommonFormat::Unknown,
            },
            is_interleaved,
            planes,
            audio_file_settings_json,
            is_end_of_stream: frame_length == 0,
        })
    }
}
