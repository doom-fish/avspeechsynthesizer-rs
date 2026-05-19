use core::ffi::{c_char, c_void};

extern "C" {
    pub fn avs_midi_player_new_with_contents_of_path(
        midi_path: *const c_char,
        sound_bank_path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn avs_midi_player_new_with_data(
        bytes: *const u8,
        len: usize,
        sound_bank_path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn avs_midi_player_release(token: *mut c_void);
    pub fn avs_midi_player_prepare_to_play(token: *mut c_void);
    pub fn avs_midi_player_play(token: *mut c_void);
    pub fn avs_midi_player_stop(token: *mut c_void);
    pub fn avs_midi_player_duration(token: *mut c_void) -> f64;
    pub fn avs_midi_player_is_playing(token: *mut c_void) -> bool;
    pub fn avs_midi_player_rate(token: *mut c_void) -> f32;
    pub fn avs_midi_player_set_rate(token: *mut c_void, rate: f32);
    pub fn avs_midi_player_current_position(token: *mut c_void) -> f64;
    pub fn avs_midi_player_set_current_position(token: *mut c_void, position: f64);

    pub fn avs_midi_channel_event_new(
        channel: u32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn avs_midi_channel_event_release(token: *mut c_void);
    pub fn avs_midi_channel_event_channel(token: *mut c_void) -> u32;
    pub fn avs_midi_channel_event_set_channel(token: *mut c_void, channel: u32);
}
