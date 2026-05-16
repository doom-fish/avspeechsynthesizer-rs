use core::ffi::c_char;

extern "C" {
    pub fn avs_marker_make_json(
        marker_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
}
