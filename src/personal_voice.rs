use core::{ffi::c_char, ptr};
use std::time::Duration;

use crate::error::AvSpeechError;
use crate::ffi;
use crate::private::result_from_status;
use crate::voice::{SpeechSynthesisVoice, SpeechSynthesisVoiceTraits};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the AVSpeechSynthesis personal-voice authorization state.
pub enum PersonalVoiceAuthorizationStatus {
    /// Represents an AVSpeechSynthesis authorization state that has not been requested yet.
    NotDetermined,
    /// Represents an AVSpeechSynthesis authorization state that was denied.
    Denied,
    /// Represents an AVSpeechSynthesis authorization state that is unsupported.
    Unsupported,
    /// Represents an AVSpeechSynthesis authorization state that is granted.
    Authorized,
    /// Represents an unknown AVSpeechSynthesis authorization raw value.
    Unknown(i32),
}

impl PersonalVoiceAuthorizationStatus {
    #[must_use]
    /// Converts an AVSpeechSynthesis authorization raw value into a wrapper enum.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::NotDetermined,
            1 => Self::Denied,
            2 => Self::Unsupported,
            3 => Self::Authorized,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    /// Returns whether the AVSpeechSynthesis authorization state is granted.
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::Authorized)
    }
}

/// Returns the current AVSpeechSynthesis personal-voice authorization state.
pub fn personal_voice_authorization_status(
) -> Result<PersonalVoiceAuthorizationStatus, AvSpeechError> {
    let mut raw_status = 0;
    let mut err_msg: *mut c_char = ptr::null_mut();
    let status = unsafe {
        ffi::personal_voice::avs_personal_voice_authorization_status(&mut raw_status, &mut err_msg)
    };
    unsafe { result_from_status(status, err_msg) }?;
    Ok(PersonalVoiceAuthorizationStatus::from_raw(raw_status))
}

/// Requests AVSpeechSynthesis personal-voice authorization and waits up to `timeout`.
pub fn request_personal_voice_authorization(
    timeout: Duration,
) -> Result<PersonalVoiceAuthorizationStatus, AvSpeechError> {
    let timeout_seconds = timeout
        .as_secs()
        .saturating_add(u64::from(timeout.subsec_nanos() > 0));
    let timeout_seconds = i32::try_from(timeout_seconds.min(i32::MAX as u64)).unwrap_or(i32::MAX);

    let mut raw_status = 0;
    let mut err_msg: *mut c_char = ptr::null_mut();
    let status = unsafe {
        ffi::personal_voice::avs_request_personal_voice_authorization(
            timeout_seconds,
            &mut raw_status,
            &mut err_msg,
        )
    };
    unsafe { result_from_status(status, err_msg) }?;
    Ok(PersonalVoiceAuthorizationStatus::from_raw(raw_status))
}

/// Returns the available AVSpeechSynthesis voices flagged as personal voices.
pub fn available_personal_voices() -> Result<Vec<SpeechSynthesisVoice>, AvSpeechError> {
    let traits = SpeechSynthesisVoiceTraits::IS_PERSONAL_VOICE;
    Ok(SpeechSynthesisVoice::speech_voices()?
        .into_iter()
        .filter(|voice| voice.voice_traits().contains(traits))
        .collect())
}
