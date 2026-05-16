use core::ptr;

use serde::{Deserialize, Serialize};

use crate::error::AvSpeechError;
use crate::ffi;
use crate::private::{optional_json_from_ptr, parse_json_ptr, string_from_ptr, to_cstring};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeechSynthesisVoiceQuality {
    Default,
    Enhanced,
    Premium,
    Unknown(i64),
}

impl SpeechSynthesisVoiceQuality {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            1 => Self::Default,
            2 => Self::Enhanced,
            3 => Self::Premium,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    pub const fn as_raw(self) -> i64 {
        match self {
            Self::Default => 1,
            Self::Enhanced => 2,
            Self::Premium => 3,
            Self::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeechSynthesisVoiceGender {
    Unspecified,
    Male,
    Female,
    Unknown(i64),
}

impl SpeechSynthesisVoiceGender {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Unspecified,
            1 => Self::Male,
            2 => Self::Female,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    pub const fn as_raw(self) -> i64 {
        match self {
            Self::Unspecified => 0,
            Self::Male => 1,
            Self::Female => 2,
            Self::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpeechSynthesisVoice {
    language: String,
    identifier: String,
    name: String,
    quality: SpeechSynthesisVoiceQuality,
    gender: Option<SpeechSynthesisVoiceGender>,
}

impl SpeechSynthesisVoice {
    #[must_use]
    pub fn language(&self) -> &str {
        &self.language
    }

    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn quality(&self) -> SpeechSynthesisVoiceQuality {
        self.quality
    }

    #[must_use]
    pub const fn gender(&self) -> Option<SpeechSynthesisVoiceGender> {
        self.gender
    }

    pub fn speech_voices() -> Result<Vec<Self>, AvSpeechError> {
        let payloads: Vec<VoicePayload> =
            unsafe { parse_json_ptr(ffi::avs_speech_voices_json(), "speech voices") }?;
        Ok(payloads.into_iter().map(Into::into).collect())
    }

    pub fn voices_with_language(language: &str) -> Result<Vec<Self>, AvSpeechError> {
        let language = to_cstring(language)?;
        let mut err_msg = ptr::null_mut();
        let payloads = unsafe {
            optional_json_from_ptr::<Vec<VoicePayload>>(
                ffi::avs_voices_with_language_json(language.as_ptr(), &mut err_msg),
                err_msg,
                "voices with language",
            )?
        };
        Ok(payloads
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub fn voice_with_language(language: &str) -> Result<Option<Self>, AvSpeechError> {
        let language = to_cstring(language)?;
        let mut err_msg = ptr::null_mut();
        let payload = unsafe {
            optional_json_from_ptr::<VoicePayload>(
                ffi::avs_voice_with_language_json(language.as_ptr(), &mut err_msg),
                err_msg,
                "voice with language",
            )?
        };
        Ok(payload.map(Into::into))
    }

    pub fn voice_with_identifier(identifier: &str) -> Result<Option<Self>, AvSpeechError> {
        let identifier = to_cstring(identifier)?;
        let mut err_msg = ptr::null_mut();
        let payload = unsafe {
            optional_json_from_ptr::<VoicePayload>(
                ffi::avs_voice_with_identifier_json(identifier.as_ptr(), &mut err_msg),
                err_msg,
                "voice with identifier",
            )?
        };
        Ok(payload.map(Into::into))
    }

    pub fn current_language_code() -> Result<String, AvSpeechError> {
        unsafe { string_from_ptr(ffi::avs_current_language_code(), "current language code") }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VoicePayload {
    language: String,
    identifier: String,
    name: String,
    quality: i64,
    gender: Option<i64>,
}

impl From<VoicePayload> for SpeechSynthesisVoice {
    fn from(payload: VoicePayload) -> Self {
        Self {
            language: payload.language,
            identifier: payload.identifier,
            name: payload.name,
            quality: SpeechSynthesisVoiceQuality::from_raw(payload.quality),
            gender: payload.gender.map(SpeechSynthesisVoiceGender::from_raw),
        }
    }
}

impl From<&SpeechSynthesisVoice> for VoicePayload {
    fn from(voice: &SpeechSynthesisVoice) -> Self {
        Self {
            language: voice.language.clone(),
            identifier: voice.identifier.clone(),
            name: voice.name.clone(),
            quality: voice.quality.as_raw(),
            gender: voice.gender.map(SpeechSynthesisVoiceGender::as_raw),
        }
    }
}
