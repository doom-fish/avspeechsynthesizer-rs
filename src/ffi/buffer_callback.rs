use core::ffi::{c_char, c_void};

extern "C" {
    pub fn avs_synthesizer_collect_buffers_json(
        token: *mut c_void,
        utterance_json: *const c_char,
        out_result_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
