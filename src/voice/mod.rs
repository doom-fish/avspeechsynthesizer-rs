use core::ptr;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::AvSpeechError;
use crate::ffi;
use crate::private::{
    optional_json_from_ptr, parse_json_ptr, parse_json_str, string_from_ptr, to_cstring,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the AVSpeechSynthesis voice quality.
pub enum SpeechSynthesisVoiceQuality {
    /// Represents the default AVSpeechSynthesis voice quality.
    Default,
    /// Represents the enhanced AVSpeechSynthesis voice quality.
    Enhanced,
    /// Represents the premium AVSpeechSynthesis voice quality.
    Premium,
    /// Represents an unknown AVSpeechSynthesis voice-quality raw value.
    Unknown(i64),
}

impl SpeechSynthesisVoiceQuality {
    #[must_use]
    /// Converts an AVSpeechSynthesis voice-quality raw value into a wrapper enum.
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            1 => Self::Default,
            2 => Self::Enhanced,
            3 => Self::Premium,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    /// Returns the raw AVSpeechSynthesis voice-quality value.
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
/// Represents the AVSpeechSynthesis voice gender.
pub enum SpeechSynthesisVoiceGender {
    /// Represents an unspecified AVSpeechSynthesis voice gender.
    Unspecified,
    /// Represents a male AVSpeechSynthesis voice gender.
    Male,
    /// Represents a female AVSpeechSynthesis voice gender.
    Female,
    /// Represents an unknown AVSpeechSynthesis voice-gender raw value.
    Unknown(i64),
}

impl SpeechSynthesisVoiceGender {
    #[must_use]
    /// Converts an AVSpeechSynthesis voice-gender raw value into a wrapper enum.
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Unspecified,
            1 => Self::Male,
            2 => Self::Female,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    /// Returns the raw AVSpeechSynthesis voice-gender value.
    pub const fn as_raw(self) -> i64 {
        match self {
            Self::Unspecified => 0,
            Self::Male => 1,
            Self::Female => 2,
            Self::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Represents the AVSpeechSynthesis voice trait bitflags.
pub struct SpeechSynthesisVoiceTraits(u64);

impl SpeechSynthesisVoiceTraits {
    /// Represents the empty AVSpeechSynthesis voice-trait set.
    pub const NONE: Self = Self(0);
    /// Represents the AVSpeechSynthesis novelty-voice trait.
    pub const IS_NOVELTY_VOICE: Self = Self(1 << 0);
    /// Represents the AVSpeechSynthesis personal-voice trait.
    pub const IS_PERSONAL_VOICE: Self = Self(1 << 1);

    #[must_use]
    /// Creates AVSpeechSynthesis voice traits from raw bits.
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    #[must_use]
    /// Returns the raw AVSpeechSynthesis voice-trait bits.
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[must_use]
    /// Returns whether these AVSpeechSynthesis voice traits include `other`.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[must_use]
    /// Returns whether no AVSpeechSynthesis voice traits are set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl BitOr for SpeechSynthesisVoiceTraits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for SpeechSynthesisVoiceTraits {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for SpeechSynthesisVoiceTraits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for SpeechSynthesisVoiceTraits {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents an AVSpeechSynthesis voice.
pub struct SpeechSynthesisVoice {
    language: String,
    identifier: String,
    name: String,
    quality: SpeechSynthesisVoiceQuality,
    gender: Option<SpeechSynthesisVoiceGender>,
    audio_file_settings_json: Option<String>,
    voice_traits: SpeechSynthesisVoiceTraits,
}

impl SpeechSynthesisVoice {
    #[must_use]
    /// Returns the AVSpeechSynthesis voice language code.
    pub fn language(&self) -> &str {
        &self.language
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis voice identifier.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis voice display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis voice quality.
    pub const fn quality(&self) -> SpeechSynthesisVoiceQuality {
        self.quality
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis voice gender, if available.
    pub const fn gender(&self) -> Option<SpeechSynthesisVoiceGender> {
        self.gender
    }

    #[must_use]
    /// Returns the serialized AVSpeechSynthesis audio file settings, if available.
    pub fn audio_file_settings_json(&self) -> Option<&str> {
        self.audio_file_settings_json.as_deref()
    }

    /// Parses the AVSpeechSynthesis audio file settings into JSON values.
    pub fn audio_file_settings(&self) -> Result<Option<Value>, AvSpeechError> {
        self.audio_file_settings_json
            .as_deref()
            .map(|json| parse_json_str(json, "speech synthesis voice audio file settings"))
            .transpose()
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis voice traits.
    pub const fn traits(&self) -> SpeechSynthesisVoiceTraits {
        self.voice_traits
    }

    #[must_use]
    /// Returns the AVSpeechSynthesis voice traits alias used by earlier releases.
    pub const fn voice_traits(&self) -> SpeechSynthesisVoiceTraits {
        self.voice_traits
    }

    /// Returns every available AVSpeechSynthesis voice.
    pub fn speech_voices() -> Result<Vec<Self>, AvSpeechError> {
        let payloads: Vec<VoicePayload> =
            unsafe { parse_json_ptr(ffi::voice::avs_speech_voices_json(), "speech voices") }?;
        Ok(payloads.into_iter().map(Into::into).collect())
    }

    /// Returns the AVSpeechSynthesis voices that support `language`.
    pub fn voices_with_language(language: &str) -> Result<Vec<Self>, AvSpeechError> {
        let language = to_cstring(language)?;
        let mut err_msg = ptr::null_mut();
        let payloads = unsafe {
            optional_json_from_ptr::<Vec<VoicePayload>>(
                ffi::voice::avs_voices_with_language_json(language.as_ptr(), &mut err_msg),
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

    /// Returns the default AVSpeechSynthesis voice, if one is available.
    pub fn default_voice() -> Result<Option<Self>, AvSpeechError> {
        Self::voice_with_optional_language(None)
    }

    /// Returns the preferred AVSpeechSynthesis voice for `language`, if available.
    pub fn voice_with_language(language: &str) -> Result<Option<Self>, AvSpeechError> {
        Self::voice_with_optional_language(Some(language))
    }

    /// Returns the preferred AVSpeechSynthesis voice for an optional language code.
    pub fn voice_with_optional_language(
        language: Option<&str>,
    ) -> Result<Option<Self>, AvSpeechError> {
        let mut err_msg = ptr::null_mut();
        let language = language.map(to_cstring).transpose()?;
        let payload = unsafe {
            optional_json_from_ptr::<VoicePayload>(
                ffi::voice::avs_voice_with_language_json(
                    language
                        .as_ref()
                        .map_or(ptr::null(), |value| value.as_ptr()),
                    &mut err_msg,
                ),
                err_msg,
                "voice with language",
            )?
        };
        Ok(payload.map(Into::into))
    }

    /// Returns the AVSpeechSynthesis voice matching `identifier`, if available.
    pub fn voice_with_identifier(identifier: &str) -> Result<Option<Self>, AvSpeechError> {
        let identifier = to_cstring(identifier)?;
        let mut err_msg = ptr::null_mut();
        let payload = unsafe {
            optional_json_from_ptr::<VoicePayload>(
                ffi::voice::avs_voice_with_identifier_json(identifier.as_ptr(), &mut err_msg),
                err_msg,
                "voice with identifier",
            )?
        };
        Ok(payload.map(Into::into))
    }

    /// Returns the current AVSpeechSynthesis language code.
    pub fn current_language_code() -> Result<String, AvSpeechError> {
        unsafe {
            string_from_ptr(
                ffi::voice::avs_current_language_code(),
                "current language code",
            )
        }
    }

    /// Returns the special AVSpeechSynthesis identifier for the Alex voice.
    pub fn alex_identifier() -> Result<String, AvSpeechError> {
        unsafe {
            string_from_ptr(
                ffi::voice::avs_alex_voice_identifier(),
                "Alex voice identifier",
            )
        }
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
    #[serde(default)]
    audio_file_settings_json: Option<String>,
    #[serde(default)]
    voice_traits: Option<u64>,
}

impl From<VoicePayload> for SpeechSynthesisVoice {
    fn from(payload: VoicePayload) -> Self {
        Self {
            language: payload.language,
            identifier: payload.identifier,
            name: payload.name,
            quality: SpeechSynthesisVoiceQuality::from_raw(payload.quality),
            gender: payload.gender.map(SpeechSynthesisVoiceGender::from_raw),
            audio_file_settings_json: payload.audio_file_settings_json,
            voice_traits: SpeechSynthesisVoiceTraits::from_bits(payload.voice_traits.unwrap_or(0)),
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
            audio_file_settings_json: voice.audio_file_settings_json.clone(),
            voice_traits: Some(voice.voice_traits.bits()),
        }
    }
}
