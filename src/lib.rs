#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

/// AVSpeechSynthesis audio-buffer models and callback payload helpers.
pub mod buffer_callback;
/// AVSpeechSynthesis error values returned by this crate.
pub mod error;
pub mod ffi;
/// AVSpeechSynthesis marker wrappers for spoken ranges and bookmarks.
pub mod marker;
/// AVSpeechSynthesis personal-voice authorization and discovery helpers.
pub mod personal_voice;
mod private;
/// AVSpeechSynthesis provider-side request and voice wrappers.
pub mod provider;
/// AVSpeechSynthesis synthesizer controls, events, and file output helpers.
pub mod synthesizer;
/// AVSpeechSynthesis utterance builders and attributed-string helpers.
pub mod utterance;
/// AVSpeechSynthesis voice lookup and metadata wrappers.
pub mod voice;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
/// AVSpeechSynthesis async event-stream helpers.
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

/// AVSpeechSynthesis convenience re-exports for common crate types.
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
