#![allow(missing_docs, non_camel_case_types)]

use std::ffi::{c_char, c_void};

pub mod buffer_callback;
pub mod core;
pub mod marker;
pub mod personal_voice;
pub mod provider;
pub mod synthesizer;
pub mod utterance;
pub mod voice;

pub use core::avs_string_free;

pub type AVSEventCallback =
    unsafe extern "C" fn(user_info: *mut c_void, payload_json: *const c_char);
pub type AVSBufferCallback =
    unsafe extern "C" fn(user_info: *mut c_void, payload_json: *const c_char);
pub type AVSMarkerCallback =
    unsafe extern "C" fn(user_info: *mut c_void, payload_json: *const c_char);

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const UNAVAILABLE_ON_THIS_MACOS: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
    pub const IO_ERROR: i32 = -4;
    pub const FRAMEWORK_ERROR: i32 = -5;
    pub const UNKNOWN: i32 = -99;
}
