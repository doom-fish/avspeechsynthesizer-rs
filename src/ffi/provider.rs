use core::ffi::{c_char, c_void};

extern "C" {
    pub fn avs_provider_voice_new_json(
        config_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn avs_provider_voice_release(token: *mut c_void);
    pub fn avs_provider_voice_snapshot_json(
        token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn avs_provider_voice_set_voice_size(
        token: *mut c_void,
        voice_size: i64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn avs_provider_voice_set_version(
        token: *mut c_void,
        version: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn avs_provider_voice_set_gender(
        token: *mut c_void,
        gender: i64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn avs_provider_voice_set_age(
        token: *mut c_void,
        age: i64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn avs_provider_voice_update_speech_voices(out_error_message: *mut *mut c_char) -> i32;
    pub fn avs_provider_request_new(
        voice_token: *mut c_void,
        ssml_representation: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn avs_provider_request_release(token: *mut c_void);
    pub fn avs_provider_request_ssml_representation(
        token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn avs_provider_request_copy_voice(
        token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
}
