#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type AVSEventCallback =
    unsafe extern "C" fn(user_info: *mut c_void, payload_json: *const c_char);

extern "C" {
    pub fn avs_string_free(s: *mut c_char);

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

    pub fn avs_synthesizer_new() -> *mut c_void;
    pub fn avs_synthesizer_release(token: *mut c_void);
    pub fn avs_synthesizer_set_event_handler(
        token: *mut c_void,
        callback: Option<AVSEventCallback>,
        user_info: *mut c_void,
    );
    pub fn avs_synthesizer_is_speaking(token: *mut c_void) -> bool;
    pub fn avs_synthesizer_is_paused(token: *mut c_void) -> bool;
    pub fn avs_synthesizer_speak_json(
        token: *mut c_void,
        utterance_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn avs_synthesizer_pause(token: *mut c_void, boundary: i32) -> bool;
    pub fn avs_synthesizer_stop(token: *mut c_void, boundary: i32) -> bool;
    pub fn avs_synthesizer_continue(token: *mut c_void) -> bool;
    pub fn avs_synthesizer_write_utterance_to_file_json(
        token: *mut c_void,
        utterance_json: *const c_char,
        output_path: *const c_char,
        out_result_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn avs_run_loop_pump(seconds: f64);
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const UNAVAILABLE_ON_THIS_MACOS: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
    pub const IO_ERROR: i32 = -4;
    pub const FRAMEWORK_ERROR: i32 = -5;
    pub const UNKNOWN: i32 = -99;
}
