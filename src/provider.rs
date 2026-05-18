use core::{
    ffi::{c_char, c_void},
    ptr,
};

use serde::{Deserialize, Serialize};

use crate::error::AvSpeechError;
use crate::ffi;
use crate::private::{
    error_from_status, json_cstring, parse_json_ptr, result_from_status, string_from_ptr,
    to_cstring,
};
use crate::voice::SpeechSynthesisVoiceGender;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderVoiceConfig {
    name: String,
    identifier: String,
    primary_languages: Vec<String>,
    supported_languages: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderVoiceSnapshot {
    name: String,
    identifier: String,
    primary_languages: Vec<String>,
    supported_languages: Vec<String>,
    voice_size: i64,
    version: String,
    gender: i64,
    age: i64,
}

/// Wraps an AVSpeechSynthesis provider voice.
pub struct SpeechSynthesisProviderVoice {
    token: *mut c_void,
}

impl Drop for SpeechSynthesisProviderVoice {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe {
                ffi::provider::avs_provider_voice_release(self.token);
            }
            self.token = ptr::null_mut();
        }
    }
}

impl SpeechSynthesisProviderVoice {
    /// Creates an AVSpeechSynthesis provider voice wrapper.
    pub fn new(
        name: impl Into<String>,
        identifier: impl Into<String>,
        primary_languages: impl IntoIterator<Item = impl Into<String>>,
        supported_languages: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, AvSpeechError> {
        let config = ProviderVoiceConfig {
            name: name.into(),
            identifier: identifier.into(),
            primary_languages: primary_languages.into_iter().map(Into::into).collect(),
            supported_languages: supported_languages.into_iter().map(Into::into).collect(),
        };
        let config_json = json_cstring(&config)?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let token = unsafe {
            ffi::provider::avs_provider_voice_new_json(config_json.as_ptr(), &mut err_msg)
        };
        if token.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        Ok(Self { token })
    }

    /// Triggers the AVSpeechSynthesis provider voice catalog to refresh.
    pub fn update_speech_voices() -> Result<(), AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::provider::avs_provider_voice_update_speech_voices(&mut err_msg) };
        unsafe { result_from_status(status, err_msg) }
    }

    fn snapshot(&self) -> Result<ProviderVoiceSnapshot, AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let json =
            unsafe { ffi::provider::avs_provider_voice_snapshot_json(self.token, &mut err_msg) };
        if !err_msg.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        unsafe { parse_json_ptr(json, "provider voice snapshot") }
    }

    /// Returns the AVSpeechSynthesis provider voice display name.
    pub fn name(&self) -> Result<String, AvSpeechError> {
        Ok(self.snapshot()?.name)
    }

    /// Returns the AVSpeechSynthesis provider voice identifier.
    pub fn identifier(&self) -> Result<String, AvSpeechError> {
        Ok(self.snapshot()?.identifier)
    }

    /// Returns the primary AVSpeechSynthesis languages for this provider voice.
    pub fn primary_languages(&self) -> Result<Vec<String>, AvSpeechError> {
        Ok(self.snapshot()?.primary_languages)
    }

    /// Returns every AVSpeechSynthesis language supported by this provider voice.
    pub fn supported_languages(&self) -> Result<Vec<String>, AvSpeechError> {
        Ok(self.snapshot()?.supported_languages)
    }

    /// Returns the AVSpeechSynthesis provider voice size in bytes.
    pub fn voice_size(&self) -> Result<i64, AvSpeechError> {
        Ok(self.snapshot()?.voice_size)
    }

    /// Returns the AVSpeechSynthesis provider voice version string.
    pub fn version(&self) -> Result<String, AvSpeechError> {
        Ok(self.snapshot()?.version)
    }

    /// Returns the AVSpeechSynthesis provider voice gender.
    pub fn gender(&self) -> Result<SpeechSynthesisVoiceGender, AvSpeechError> {
        Ok(SpeechSynthesisVoiceGender::from_raw(
            self.snapshot()?.gender,
        ))
    }

    /// Returns the AVSpeechSynthesis provider voice age hint.
    pub fn age(&self) -> Result<i64, AvSpeechError> {
        Ok(self.snapshot()?.age)
    }

    /// Sets the AVSpeechSynthesis provider voice size in bytes.
    pub fn set_voice_size(&self, voice_size: i64) -> Result<(), AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::provider::avs_provider_voice_set_voice_size(self.token, voice_size, &mut err_msg)
        };
        unsafe { result_from_status(status, err_msg) }
    }

    /// Sets the AVSpeechSynthesis provider voice version string.
    pub fn set_version(&self, version: &str) -> Result<(), AvSpeechError> {
        let version = to_cstring(version)?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::provider::avs_provider_voice_set_version(
                self.token,
                version.as_ptr(),
                &mut err_msg,
            )
        };
        unsafe { result_from_status(status, err_msg) }
    }

    /// Sets the AVSpeechSynthesis provider voice gender.
    pub fn set_gender(&self, gender: SpeechSynthesisVoiceGender) -> Result<(), AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::provider::avs_provider_voice_set_gender(self.token, gender.as_raw(), &mut err_msg)
        };
        unsafe { result_from_status(status, err_msg) }
    }

    /// Sets the AVSpeechSynthesis provider voice age hint.
    pub fn set_age(&self, age: i64) -> Result<(), AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::provider::avs_provider_voice_set_age(self.token, age, &mut err_msg) };
        unsafe { result_from_status(status, err_msg) }
    }
}

/// Wraps an AVSpeechSynthesis provider request.
pub struct SpeechSynthesisProviderRequest {
    token: *mut c_void,
}

impl Drop for SpeechSynthesisProviderRequest {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe {
                ffi::provider::avs_provider_request_release(self.token);
            }
            self.token = ptr::null_mut();
        }
    }
}

impl SpeechSynthesisProviderRequest {
    /// Creates an AVSpeechSynthesis provider request from SSML and a voice.
    pub fn new(
        ssml_representation: &str,
        voice: &SpeechSynthesisProviderVoice,
    ) -> Result<Self, AvSpeechError> {
        let ssml_representation = to_cstring(ssml_representation)?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let token = unsafe {
            ffi::provider::avs_provider_request_new(
                voice.token,
                ssml_representation.as_ptr(),
                &mut err_msg,
            )
        };
        if token.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        Ok(Self { token })
    }

    /// Returns the AVSpeechSynthesis provider request SSML representation.
    pub fn ssml_representation(&self) -> Result<String, AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let string = unsafe {
            ffi::provider::avs_provider_request_ssml_representation(self.token, &mut err_msg)
        };
        if !err_msg.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        unsafe { string_from_ptr(string, "provider request ssml representation") }
    }

    /// Returns the AVSpeechSynthesis provider voice attached to this request.
    pub fn voice(&self) -> Result<SpeechSynthesisProviderVoice, AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let token =
            unsafe { ffi::provider::avs_provider_request_copy_voice(self.token, &mut err_msg) };
        if token.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        Ok(SpeechSynthesisProviderVoice { token })
    }
}
