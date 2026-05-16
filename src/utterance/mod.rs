use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::AvSpeechError;
use crate::ffi;
use crate::marker::TextRange;
use crate::private::{
    error_from_status, json_cstring, parse_json_ptr, parse_json_str, string_from_ptr,
};
use crate::voice::{SpeechSynthesisVoice, VoicePayload};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpeechUtteranceKind {
    PlainText,
    AttributedText,
    Ssml,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeechAttributeRun {
    pub range: TextRange,
    pub attributes: BTreeMap<String, Value>,
}

impl SpeechAttributeRun {
    #[must_use]
    pub fn new(range: TextRange, attributes: BTreeMap<String, Value>) -> Self {
        Self { range, attributes }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributedSpeechString {
    text: String,
    runs: Vec<SpeechAttributeRun>,
}

impl AttributedSpeechString {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            runs: Vec::new(),
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn runs(&self) -> &[SpeechAttributeRun] {
        &self.runs
    }

    #[must_use]
    pub fn with_attribute_value(
        mut self,
        range: TextRange,
        key: impl Into<String>,
        value: Value,
    ) -> Self {
        let mut attributes = BTreeMap::new();
        attributes.insert(key.into(), value);
        self.runs.push(SpeechAttributeRun::new(range, attributes));
        self
    }

    #[must_use]
    pub fn with_attributes(
        mut self,
        range: TextRange,
        attributes: BTreeMap<String, Value>,
    ) -> Self {
        self.runs.push(SpeechAttributeRun::new(range, attributes));
        self
    }

    pub fn with_ipa_notation(
        self,
        range: TextRange,
        ipa: impl Into<String>,
    ) -> Result<Self, AvSpeechError> {
        let key = SpeechUtterance::ipa_notation_attribute_name()?;
        Ok(self.with_attribute_value(range, key, Value::String(ipa.into())))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpeechUtterance {
    kind: SpeechUtteranceKind,
    speech_string: String,
    attributed_speech_string: Option<AttributedSpeechString>,
    ssml_representation: Option<String>,
    voice: Option<SpeechSynthesisVoice>,
    rate: f32,
    pitch_multiplier: f32,
    volume: f32,
    prefers_assistive_technology_settings: bool,
    pre_utterance_delay: f64,
    post_utterance_delay: f64,
}

impl SpeechUtterance {
    pub const MINIMUM_SPEECH_RATE: f32 = 0.0;
    pub const DEFAULT_SPEECH_RATE: f32 = 0.5;
    pub const MAXIMUM_SPEECH_RATE: f32 = 1.0;

    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            kind: SpeechUtteranceKind::PlainText,
            speech_string: text.into(),
            attributed_speech_string: None,
            ssml_representation: None,
            voice: None,
            rate: Self::DEFAULT_SPEECH_RATE,
            pitch_multiplier: 1.0,
            volume: 1.0,
            prefers_assistive_technology_settings: false,
            pre_utterance_delay: 0.0,
            post_utterance_delay: 0.0,
        }
    }

    #[must_use]
    pub fn from_attributed(attributed_speech_string: AttributedSpeechString) -> Self {
        let speech_string = attributed_speech_string.text.clone();
        Self {
            kind: SpeechUtteranceKind::AttributedText,
            speech_string,
            attributed_speech_string: Some(attributed_speech_string),
            ssml_representation: None,
            voice: None,
            rate: Self::DEFAULT_SPEECH_RATE,
            pitch_multiplier: 1.0,
            volume: 1.0,
            prefers_assistive_technology_settings: false,
            pre_utterance_delay: 0.0,
            post_utterance_delay: 0.0,
        }
    }

    pub fn from_ssml(ssml_representation: impl Into<String>) -> Result<Self, AvSpeechError> {
        let ssml_representation = ssml_representation.into();
        let utterance = Self {
            kind: SpeechUtteranceKind::Ssml,
            speech_string: ssml_representation.clone(),
            attributed_speech_string: None,
            ssml_representation: Some(ssml_representation),
            voice: None,
            rate: Self::DEFAULT_SPEECH_RATE,
            pitch_multiplier: 1.0,
            volume: 1.0,
            prefers_assistive_technology_settings: false,
            pre_utterance_delay: 0.0,
            post_utterance_delay: 0.0,
        };
        utterance.resolved().map(|_| utterance)
    }

    #[must_use]
    pub fn speech_string(&self) -> &str {
        &self.speech_string
    }

    #[must_use]
    pub const fn kind(&self) -> SpeechUtteranceKind {
        self.kind
    }

    #[must_use]
    pub fn attributed_speech_string(&self) -> Option<&AttributedSpeechString> {
        self.attributed_speech_string.as_ref()
    }

    #[must_use]
    pub fn ssml_representation(&self) -> Option<&str> {
        self.ssml_representation.as_deref()
    }

    #[must_use]
    pub fn voice(&self) -> Option<&SpeechSynthesisVoice> {
        self.voice.as_ref()
    }

    #[must_use]
    pub const fn rate(&self) -> f32 {
        self.rate
    }

    #[must_use]
    pub const fn pitch_multiplier(&self) -> f32 {
        self.pitch_multiplier
    }

    #[must_use]
    pub const fn volume(&self) -> f32 {
        self.volume
    }

    #[must_use]
    pub const fn prefers_assistive_technology_settings(&self) -> bool {
        self.prefers_assistive_technology_settings
    }

    #[must_use]
    pub const fn pre_utterance_delay(&self) -> f64 {
        self.pre_utterance_delay
    }

    #[must_use]
    pub const fn post_utterance_delay(&self) -> f64 {
        self.post_utterance_delay
    }

    #[must_use]
    pub fn with_voice(mut self, voice: SpeechSynthesisVoice) -> Self {
        self.voice = Some(voice);
        self
    }

    #[must_use]
    pub fn with_rate(mut self, rate: f32) -> Self {
        self.rate = rate;
        self
    }

    #[must_use]
    pub fn with_pitch_multiplier(mut self, pitch_multiplier: f32) -> Self {
        self.pitch_multiplier = pitch_multiplier;
        self
    }

    #[must_use]
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    #[must_use]
    pub fn with_prefers_assistive_technology_settings(
        mut self,
        prefers_assistive_technology_settings: bool,
    ) -> Self {
        self.prefers_assistive_technology_settings = prefers_assistive_technology_settings;
        self
    }

    #[must_use]
    pub fn with_pre_utterance_delay(mut self, pre_utterance_delay: f64) -> Self {
        self.pre_utterance_delay = pre_utterance_delay;
        self
    }

    #[must_use]
    pub fn with_post_utterance_delay(mut self, post_utterance_delay: f64) -> Self {
        self.post_utterance_delay = post_utterance_delay;
        self
    }

    #[must_use]
    pub fn minimum_speech_rate() -> f32 {
        unsafe { ffi::utterance::avs_utterance_minimum_speech_rate() }
    }

    #[must_use]
    pub fn default_speech_rate() -> f32 {
        unsafe { ffi::utterance::avs_utterance_default_speech_rate() }
    }

    #[must_use]
    pub fn maximum_speech_rate() -> f32 {
        unsafe { ffi::utterance::avs_utterance_maximum_speech_rate() }
    }

    pub fn ipa_notation_attribute_name() -> Result<String, AvSpeechError> {
        unsafe {
            string_from_ptr(
                ffi::utterance::avs_utterance_ipa_notation_attribute_name(),
                "IPA notation attribute name",
            )
        }
    }

    pub fn resolved(&self) -> Result<Self, AvSpeechError> {
        let utterance_json = json_cstring(&UtterancePayload::from(self))?;
        let mut err_msg = std::ptr::null_mut();
        let payload_json = unsafe {
            ffi::utterance::avs_utterance_roundtrip_json(utterance_json.as_ptr(), &mut err_msg)
        };
        if !err_msg.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        let payload: UtterancePayload =
            unsafe { parse_json_ptr(payload_json, "utterance roundtrip") }?;
        Self::try_from(payload)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttributeRunPayload {
    range: TextRange,
    attributes_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttributedSpeechStringPayload {
    text: String,
    runs: Vec<AttributeRunPayload>,
}

impl From<&AttributedSpeechString> for AttributedSpeechStringPayload {
    fn from(value: &AttributedSpeechString) -> Self {
        Self {
            text: value.text.clone(),
            runs: value
                .runs
                .iter()
                .map(|run| AttributeRunPayload {
                    range: run.range,
                    attributes_json: serde_json::to_string(&run.attributes)
                        .expect("speech attribute map must be JSON serializable"),
                })
                .collect(),
        }
    }
}

impl TryFrom<AttributedSpeechStringPayload> for AttributedSpeechString {
    type Error = AvSpeechError;

    fn try_from(value: AttributedSpeechStringPayload) -> Result<Self, Self::Error> {
        let runs = value
            .runs
            .into_iter()
            .map(|run| {
                parse_json_str(&run.attributes_json, "speech attribute run").map(|attributes| {
                    SpeechAttributeRun {
                        range: run.range,
                        attributes,
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            text: value.text,
            runs,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UtterancePayload {
    kind: SpeechUtteranceKind,
    speech_string: String,
    attributed_speech_string: Option<AttributedSpeechStringPayload>,
    ssml_representation: Option<String>,
    voice: Option<VoicePayload>,
    rate: f32,
    pitch_multiplier: f32,
    volume: f32,
    prefers_assistive_technology_settings: bool,
    pre_utterance_delay: f64,
    post_utterance_delay: f64,
}

impl From<&SpeechUtterance> for UtterancePayload {
    fn from(utterance: &SpeechUtterance) -> Self {
        Self {
            kind: utterance.kind,
            speech_string: utterance.speech_string.clone(),
            attributed_speech_string: utterance
                .attributed_speech_string
                .as_ref()
                .map(AttributedSpeechStringPayload::from),
            ssml_representation: utterance.ssml_representation.clone(),
            voice: utterance.voice.as_ref().map(VoicePayload::from),
            rate: utterance.rate,
            pitch_multiplier: utterance.pitch_multiplier,
            volume: utterance.volume,
            prefers_assistive_technology_settings: utterance.prefers_assistive_technology_settings,
            pre_utterance_delay: utterance.pre_utterance_delay,
            post_utterance_delay: utterance.post_utterance_delay,
        }
    }
}

impl TryFrom<UtterancePayload> for SpeechUtterance {
    type Error = AvSpeechError;

    fn try_from(payload: UtterancePayload) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: payload.kind,
            speech_string: payload.speech_string,
            attributed_speech_string: payload
                .attributed_speech_string
                .map(AttributedSpeechString::try_from)
                .transpose()?,
            ssml_representation: payload.ssml_representation,
            voice: payload.voice.map(Into::into),
            rate: payload.rate,
            pitch_multiplier: payload.pitch_multiplier,
            volume: payload.volume,
            prefers_assistive_technology_settings: payload.prefers_assistive_technology_settings,
            pre_utterance_delay: payload.pre_utterance_delay,
            post_utterance_delay: payload.post_utterance_delay,
        })
    }
}
