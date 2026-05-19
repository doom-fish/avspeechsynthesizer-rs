use core::{ffi::c_char, ptr};
use std::{ffi::CString, path::Path};

use crate::error::AvSpeechError;
use crate::ffi;
use crate::private::error_from_status;

fn path_to_cstring(path: impl AsRef<Path>, field: &str) -> Result<CString, AvSpeechError> {
    let path = path.as_ref().to_str().ok_or_else(|| {
        AvSpeechError::InvalidArgument(format!("{field} path is not valid UTF-8"))
    })?;
    CString::new(path).map_err(|_| {
        AvSpeechError::InvalidArgument(format!("{field} path contained an interior NUL byte"))
    })
}

/// Wraps an `AVMIDIPlayer` instance.
pub struct MidiPlayer {
    token: *mut core::ffi::c_void,
}

impl Drop for MidiPlayer {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe {
                ffi::midi::avs_midi_player_release(self.token);
            }
            self.token = ptr::null_mut();
        }
    }
}

impl MidiPlayer {
    /// Creates a MIDI player from a file path and optional SoundFont/DLS bank path.
    pub fn new_from_path(
        midi_path: impl AsRef<Path>,
        sound_bank_path: Option<impl AsRef<Path>>,
    ) -> Result<Self, AvSpeechError> {
        let midi_path = path_to_cstring(midi_path, "MIDI file")?;
        let sound_bank_path = sound_bank_path
            .map(|path| path_to_cstring(path, "sound bank"))
            .transpose()?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let token = unsafe {
            ffi::midi::avs_midi_player_new_with_contents_of_path(
                midi_path.as_ptr(),
                sound_bank_path
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut err_msg,
            )
        };
        if token.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        Ok(Self { token })
    }

    /// Creates a MIDI player from in-memory file bytes and an optional SoundFont/DLS bank path.
    pub fn new_from_bytes(
        bytes: &[u8],
        sound_bank_path: Option<impl AsRef<Path>>,
    ) -> Result<Self, AvSpeechError> {
        let sound_bank_path = sound_bank_path
            .map(|path| path_to_cstring(path, "sound bank"))
            .transpose()?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let token = unsafe {
            ffi::midi::avs_midi_player_new_with_data(
                bytes.as_ptr(),
                bytes.len(),
                sound_bank_path
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut err_msg,
            )
        };
        if token.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        Ok(Self { token })
    }

    /// Prerolls the MIDI sequence so playback can start without extra setup latency.
    pub fn prepare_to_play(&self) {
        unsafe { ffi::midi::avs_midi_player_prepare_to_play(self.token) };
    }

    /// Starts playback.
    pub fn play(&self) {
        unsafe { ffi::midi::avs_midi_player_play(self.token) };
    }

    /// Stops playback.
    pub fn stop(&self) {
        unsafe { ffi::midi::avs_midi_player_stop(self.token) };
    }

    /// Returns the loaded sequence duration in seconds.
    #[must_use]
    pub fn duration(&self) -> f64 {
        unsafe { ffi::midi::avs_midi_player_duration(self.token) }
    }

    /// Returns whether the player is currently playing.
    #[must_use]
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::midi::avs_midi_player_is_playing(self.token) }
    }

    /// Returns the playback rate.
    #[must_use]
    pub fn rate(&self) -> f32 {
        unsafe { ffi::midi::avs_midi_player_rate(self.token) }
    }

    /// Sets the playback rate.
    pub fn set_rate(&self, rate: f32) {
        unsafe { ffi::midi::avs_midi_player_set_rate(self.token, rate) };
    }

    /// Returns the current playback position in seconds.
    #[must_use]
    pub fn current_position(&self) -> f64 {
        unsafe { ffi::midi::avs_midi_player_current_position(self.token) }
    }

    /// Seeks to the given playback position in seconds.
    pub fn set_current_position(&self, position: f64) {
        unsafe { ffi::midi::avs_midi_player_set_current_position(self.token, position) };
    }
}

/// Apple-style alias for [`MidiPlayer`].
pub type AVMIDIPlayer = MidiPlayer;

/// Wraps an `AVMIDIChannelEvent` instance.
pub struct MidiChannelEvent {
    token: *mut core::ffi::c_void,
}

impl Drop for MidiChannelEvent {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe {
                ffi::midi::avs_midi_channel_event_release(self.token);
            }
            self.token = ptr::null_mut();
        }
    }
}

impl MidiChannelEvent {
    /// Creates a base MIDI channel event with the given channel number.
    pub fn new(channel: u32) -> Result<Self, AvSpeechError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let token = unsafe { ffi::midi::avs_midi_channel_event_new(channel, &mut err_msg) };
        if token.is_null() {
            return Err(unsafe { error_from_status(ffi::status::UNKNOWN, err_msg) });
        }
        Ok(Self { token })
    }

    /// Returns the MIDI channel.
    #[must_use]
    pub fn channel(&self) -> u32 {
        unsafe { ffi::midi::avs_midi_channel_event_channel(self.token) }
    }

    /// Updates the MIDI channel.
    pub fn set_channel(&self, channel: u32) {
        unsafe { ffi::midi::avs_midi_channel_event_set_channel(self.token, channel) };
    }
}

/// Apple-style alias for [`MidiChannelEvent`].
pub type AVMIDIChannelEvent = MidiChannelEvent;
