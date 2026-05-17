#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod buffer_callback;
pub mod error;
pub mod ffi;
pub mod marker;
pub mod personal_voice;
mod private;
pub mod provider;
pub mod synthesizer;
pub mod utterance;
pub mod voice;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;

pub use buffer_callback::{SpeechAudioBuffer, SpeechAudioCommonFormat};
pub use error::AvSpeechError;
pub use marker::{SpeechSynthesisMarker, SpeechSynthesisMarkerMark, TextRange};
pub use personal_voice::{
    available_personal_voices, personal_voice_authorization_status,
    request_personal_voice_authorization, PersonalVoiceAuthorizationStatus,
};
pub use provider::{SpeechSynthesisProviderRequest, SpeechSynthesisProviderVoice};
pub use synthesizer::{SpeechBoundary, SpeechEvent, SpeechSynthesizer, WrittenAudioFile};
pub use utterance::{
    AttributedSpeechString, SpeechAttributeRun, SpeechUtterance, SpeechUtteranceKind,
};
pub use voice::{
    SpeechSynthesisVoice, SpeechSynthesisVoiceGender, SpeechSynthesisVoiceQuality,
    SpeechSynthesisVoiceTraits,
};

pub mod prelude {
    pub use crate::buffer_callback::{SpeechAudioBuffer, SpeechAudioCommonFormat};
    pub use crate::error::AvSpeechError;
    pub use crate::marker::{SpeechSynthesisMarker, SpeechSynthesisMarkerMark, TextRange};
    pub use crate::personal_voice::{
        available_personal_voices, personal_voice_authorization_status,
        request_personal_voice_authorization, PersonalVoiceAuthorizationStatus,
    };
    pub use crate::provider::{SpeechSynthesisProviderRequest, SpeechSynthesisProviderVoice};
    pub use crate::synthesizer::{
        SpeechBoundary, SpeechEvent, SpeechSynthesizer, WrittenAudioFile,
    };
    pub use crate::utterance::{
        AttributedSpeechString, SpeechAttributeRun, SpeechUtterance, SpeechUtteranceKind,
    };
    pub use crate::voice::{
        SpeechSynthesisVoice, SpeechSynthesisVoiceGender, SpeechSynthesisVoiceQuality,
        SpeechSynthesisVoiceTraits,
    };
}
