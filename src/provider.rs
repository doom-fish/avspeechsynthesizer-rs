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

    pub fn name(&self) -> Result<String, AvSpeechError> {
        Ok(self.snapshot()?.name)
    }

    pub fn identifier(&self) -> Result<String, AvSpeechError> {
        Ok(self.snapshot()?.identifier)
    }

    pub fn primary_languages(&self) -> Result<Vec<String>, AvSpeechError> {
        Ok(self.snapshot()?.primary_languages)
    }

    pub fn supported_languages(&self) -> Result<Vec<String>, AvSpeechError> {
        Ok(self.snapshot()?.supported_languages)
    }

    pub fn voice_size(&self) -> Result<i64, AvSpeechError> {
        Ok(self.snapshot()?.voice_size)
    }

    pub fn version(&self) -> Result<String, AvSpeechError> {
        Ok(self.snapshot()?.version)
    }

    pub fn gender(&self) -> Result<SpeechSynthesisVoiceGender, AvSpeechError> {
        Ok(SpeechSynthesisVoiceGender::from_raw(
            self.snapshot()?.gender,
        ))
    }

    pub fn age(&self) -> Result<i64, AvSpeechError> {
        Ok(self.snapshot()?.age)
    }

    pub fn set_voice_size(&self, voice_size: i64) -> Result<(), AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::provider::avs_provider_voice_set_voice_size(self.token, voice_size, &mut err_msg)
        };
        unsafe { result_from_status(status, err_msg) }
    }

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

    pub fn set_gender(&self, gender: SpeechSynthesisVoiceGender) -> Result<(), AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::provider::avs_provider_voice_set_gender(self.token, gender.as_raw(), &mut err_msg)
        };
        unsafe { result_from_status(status, err_msg) }
    }

    pub fn set_age(&self, age: i64) -> Result<(), AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::provider::avs_provider_voice_set_age(self.token, age, &mut err_msg) };
        unsafe { result_from_status(status, err_msg) }
    }
}

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
