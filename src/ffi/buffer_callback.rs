use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct AudioPlane {
    pub channels: u32,
    pub size: u32,
    pub data: *const u8,
}

#[repr(C)]
pub struct AudioPlaneList {
    pub count: u32,
    pub planes: [AudioPlane; 0],
}

pub type PcmCallback = unsafe extern "C" fn(
    *mut c_void,
    f64,
    u32,
    u32,
    usize,
    bool,
    *const AudioPlaneList,
    *const c_char,
) -> bool;
pub type MarkersCallback = unsafe extern "C" fn(*mut c_void, *const c_char) -> bool;

extern "C" {
    pub fn avs_synthesizer_stream_buffers(
        token: *mut c_void,
        utterance_json: *const c_char,
        context: *mut c_void,
        retain: super::AVSContextCallback,
        release: super::AVSContextCallback,
        buffer: PcmCallback,
        markers: Option<MarkersCallback>,
        dispatch: unsafe extern "C" fn(*mut c_void) -> bool,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
