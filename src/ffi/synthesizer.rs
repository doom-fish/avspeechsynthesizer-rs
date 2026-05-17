use core::ffi::{c_char, c_void};

use super::AVSEventCallback;

pub type AVSAsyncStreamCallback =
    unsafe extern "C" fn(kind: i32, payload: *mut c_void, ctx: *mut c_void);

extern "C" {
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
    pub fn avs_available_voices_did_change_notification_name() -> *mut c_char;
    pub fn avs_run_loop_pump(seconds: f64);
    pub fn avs_synthesis_event_subscribe(
        token: *mut c_void,
        on_event: AVSAsyncStreamCallback,
        ctx: *mut c_void,
    ) -> *mut c_void;
    pub fn avs_synthesis_event_unsubscribe(handle: *mut c_void);
}
