use core::ffi::c_char;

extern "C" {
    pub fn avs_personal_voice_authorization_status(
        out_status: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn avs_request_personal_voice_authorization(
        timeout_seconds: i32,
        out_status: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
