use core::ffi::c_char;

extern "C" {
    pub fn avs_utterance_minimum_speech_rate() -> f32;
    pub fn avs_utterance_default_speech_rate() -> f32;
    pub fn avs_utterance_maximum_speech_rate() -> f32;
    pub fn avs_utterance_ipa_notation_attribute_name() -> *mut c_char;
    pub fn avs_utterance_roundtrip_json(
        utterance_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
}
