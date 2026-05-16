use core::ffi::c_char;

extern "C" {
    pub fn avs_current_language_code() -> *mut c_char;
    pub fn avs_speech_voices_json() -> *mut c_char;
    pub fn avs_voices_with_language_json(
        language: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn avs_voice_with_language_json(
        language: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn avs_voice_with_identifier_json(
        identifier: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn avs_alex_voice_identifier() -> *mut c_char;
}
