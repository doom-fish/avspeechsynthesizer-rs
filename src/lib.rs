#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod error;
pub mod ffi;
mod private;
pub mod synthesizer;
pub mod utterance;
pub mod voice;

pub use error::AvSpeechError;
pub use synthesizer::{
    SpeechBoundary, SpeechEvent, SpeechSynthesisMarker, SpeechSynthesisMarkerMark,
    SpeechSynthesizer, TextRange, WrittenAudioFile,
};
pub use utterance::SpeechUtterance;
pub use voice::{SpeechSynthesisVoice, SpeechSynthesisVoiceGender, SpeechSynthesisVoiceQuality};

pub mod prelude {
    pub use crate::error::AvSpeechError;
    pub use crate::synthesizer::{
        SpeechBoundary, SpeechEvent, SpeechSynthesisMarker, SpeechSynthesisMarkerMark,
        SpeechSynthesizer, TextRange, WrittenAudioFile,
    };
    pub use crate::utterance::SpeechUtterance;
    pub use crate::voice::{
        SpeechSynthesisVoice, SpeechSynthesisVoiceGender, SpeechSynthesisVoiceQuality,
    };
}
