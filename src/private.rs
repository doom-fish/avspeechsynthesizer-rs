use core::ffi::{c_char, CStr};
use std::ffi::CString;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::AvSpeechError;
use crate::ffi;

pub fn to_cstring(value: &str) -> Result<CString, AvSpeechError> {
    CString::new(value).map_err(|_| {
        AvSpeechError::InvalidArgument("string contained an interior NUL byte".to_owned())
    })
}

pub fn json_cstring<T: Serialize>(value: &T) -> Result<CString, AvSpeechError> {
    let json = serde_json::to_string(value).map_err(|error| {
        AvSpeechError::Unknown(format!("failed to encode JSON payload: {error}"))
    })?;
    to_cstring(&json)
}

pub unsafe fn take_optional_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::avs_string_free(ptr);
    Some(string)
}

pub unsafe fn string_from_ptr(ptr: *mut c_char, context: &str) -> Result<String, AvSpeechError> {
    take_optional_string(ptr).ok_or_else(|| {
        AvSpeechError::Unknown(format!("missing {context} response from Swift bridge"))
    })
}

pub unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, AvSpeechError> {
    let json = string_from_ptr(ptr, context)?;
    serde_json::from_str(&json).map_err(|error| {
        AvSpeechError::Unknown(format!("failed to decode {context} JSON payload: {error}"))
    })
}

pub unsafe fn optional_json_from_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    err_msg: *mut c_char,
    context: &str,
) -> Result<Option<T>, AvSpeechError> {
    if !err_msg.is_null() {
        return Err(error_from_status(ffi::status::UNKNOWN, err_msg));
    }
    if ptr.is_null() {
        return Ok(None);
    }
    parse_json_ptr(ptr, context).map(Some)
}

pub unsafe fn error_from_status(status: i32, err_msg: *mut c_char) -> AvSpeechError {
    let message = take_optional_string(err_msg)
        .unwrap_or_else(|| format!("Swift bridge call failed with status code {status}"));
    match status {
        ffi::status::INVALID_ARGUMENT => AvSpeechError::InvalidArgument(message),
        ffi::status::UNAVAILABLE_ON_THIS_MACOS => AvSpeechError::UnavailableOnThisMacOS(message),
        ffi::status::TIMED_OUT => AvSpeechError::TimedOut(message),
        ffi::status::IO_ERROR => AvSpeechError::Io(message),
        ffi::status::FRAMEWORK_ERROR => AvSpeechError::Framework(message),
        _ => AvSpeechError::Unknown(message),
    }
}
