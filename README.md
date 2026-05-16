# avspeechsynthesizer-rs

Safe Rust bindings for Apple's `AVSpeechSynthesizer` text-to-speech APIs on macOS.

> **Status:** v0.1.0 covers the high-value `AVSpeechSynthesizer` surface: utterance configuration, voice lookup and enumeration, pause/stop/continue controls, delegate-driven speech events, macOS 14+ marker callbacks, and offline audio synthesis to a file.

## Quick start

```rust,no_run
use avspeechsynthesizer::prelude::*;
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let en_us_voices = SpeechSynthesisVoice::voices_with_language("en-US")?;
    println!("en-US voices: {}", en_us_voices.len());

    let mut synthesizer = SpeechSynthesizer::new()?;
    let (tx, rx) = mpsc::channel();
    synthesizer.set_event_handler(move |event| {
        println!("event: {event:?}");
        let _ = tx.send(event);
    });

    let utterance = SpeechUtterance::new("Hello from doom-fish")
        .with_rate(SpeechUtterance::DEFAULT_SPEECH_RATE)
        .with_pitch_multiplier(1.0)
        .with_volume(1.0);

    synthesizer.speak(&utterance)?;

    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        synthesizer.pump_run_loop(Duration::from_millis(100));
        if let Ok(SpeechEvent::DidFinish(_)) = rx.try_recv() {
            break;
        }
    }

    Ok(())
}
```

## Highlights

- `SpeechSynthesizer::speak`, `pause_speaking`, `stop_speaking`, `continue_speaking`
- `SpeechUtterance` builder for text, voice, rate, pitch, volume, and pre/post delays
- `SpeechSynthesisVoice::speech_voices`, `voices_with_language`, `voice_with_language`, `voice_with_identifier`, and `current_language_code`
- `SpeechEvent` delegate callbacks:
  - `DidStart`
  - `DidFinish`
  - `DidPause`
  - `DidContinue`
  - `DidCancel`
  - `WillSpeakRangeOfSpeechString`
  - `WillSpeakMarker` (macOS 14+)
- `SpeechSynthesizer::write_utterance_to_file` for offline synthesis with collected markers

## Availability

- Base speech synthesis APIs are available on macOS 10.14+.
- Premium voices and the marker-writing callback require newer SDK/runtime support.
- `SpeechEvent::WillSpeakMarker` requires macOS 14+.

## Smoke example

Run the end-to-end framework smoke test with:

```bash
cargo run --all-features --example 02_framework_smoke
```

It enumerates `en-US` voices, speaks `"Hello from doom-fish"` with the default voice, pumps the main run loop for delegate delivery, and verifies that start/finish callbacks arrive without crashing.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
