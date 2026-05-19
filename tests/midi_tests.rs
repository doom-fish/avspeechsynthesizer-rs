use avspeechsynthesizer::{MidiChannelEvent, MidiPlayer};

const MINI_MIDI_FILE: &[u8] = &[
    0x4d, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x00, 0x60,
    0x4d, 0x54, 0x72, 0x6b, 0x00, 0x00, 0x00, 0x0f, 0x00, 0xc0, 0x00, 0x00, 0x90, 0x3c,
    0x40, 0x60, 0x80, 0x3c, 0x40, 0x00, 0xff, 0x2f, 0x00,
];

#[test]
fn midi_player_accepts_in_memory_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let player = MidiPlayer::new_from_bytes(MINI_MIDI_FILE, None::<&str>)?;
    player.prepare_to_play();
    assert!(!player.is_playing());
    player.set_rate(1.25);
    assert!((player.rate() - 1.25).abs() < f32::EPSILON);
    player.set_current_position(0.0);
    assert!(player.current_position() >= 0.0);
    assert!(player.duration() >= 0.0);
    Ok(())
}

#[test]
fn midi_channel_event_round_trips_channel() -> Result<(), Box<dyn std::error::Error>> {
    let event = MidiChannelEvent::new(2)?;
    assert_eq!(event.channel(), 2);
    event.set_channel(9);
    assert_eq!(event.channel(), 9);
    Ok(())
}
