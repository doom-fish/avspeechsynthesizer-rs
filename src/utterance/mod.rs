use serde::{Deserialize, Serialize};

use crate::voice::{SpeechSynthesisVoice, VoicePayload};

#[derive(Debug, Clone, PartialEq)]
pub struct SpeechUtterance {
    speech_string: String,
    voice: Option<SpeechSynthesisVoice>,
    rate: f32,
    pitch_multiplier: f32,
    volume: f32,
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
            speech_string: text.into(),
            voice: None,
            rate: Self::DEFAULT_SPEECH_RATE,
            pitch_multiplier: 1.0,
            volume: 1.0,
            pre_utterance_delay: 0.0,
            post_utterance_delay: 0.0,
        }
    }

    #[must_use]
    pub fn speech_string(&self) -> &str {
        &self.speech_string
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
    pub fn with_pre_utterance_delay(mut self, pre_utterance_delay: f64) -> Self {
        self.pre_utterance_delay = pre_utterance_delay;
        self
    }

    #[must_use]
    pub fn with_post_utterance_delay(mut self, post_utterance_delay: f64) -> Self {
        self.post_utterance_delay = post_utterance_delay;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UtterancePayload {
    speech_string: String,
    voice: Option<VoicePayload>,
    rate: f32,
    pitch_multiplier: f32,
    volume: f32,
    pre_utterance_delay: f64,
    post_utterance_delay: f64,
}

impl From<&SpeechUtterance> for UtterancePayload {
    fn from(utterance: &SpeechUtterance) -> Self {
        Self {
            speech_string: utterance.speech_string.clone(),
            voice: utterance.voice.as_ref().map(VoicePayload::from),
            rate: utterance.rate,
            pitch_multiplier: utterance.pitch_multiplier,
            volume: utterance.volume,
            pre_utterance_delay: utterance.pre_utterance_delay,
            post_utterance_delay: utterance.post_utterance_delay,
        }
    }
}

impl From<UtterancePayload> for SpeechUtterance {
    fn from(payload: UtterancePayload) -> Self {
        Self {
            speech_string: payload.speech_string,
            voice: payload.voice.map(Into::into),
            rate: payload.rate,
            pitch_multiplier: payload.pitch_multiplier,
            volume: payload.volume,
            pre_utterance_delay: payload.pre_utterance_delay,
            post_utterance_delay: payload.post_utterance_delay,
        }
    }
}
