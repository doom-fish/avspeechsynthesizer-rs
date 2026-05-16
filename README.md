# avspeechsynthesizer-rs

Safe Rust bindings for Apple's `AVSpeechSynthesizer` text-to-speech APIs on macOS.

> **Status:** v0.2.0 covers the complete public AVSpeech surface that is available to regular macOS apps, including utterances, voices, synthesizer lifecycle control, offline/audio-buffer writes, synthesis markers, provider voice/request models, and personal voice authorization.

## Quick start

```rust,no_run
use avspeechsynthesizer::prelude::*;
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut synthesizer = SpeechSynthesizer::new()?;
    let (tx, rx) = mpsc::channel();
    synthesizer.set_event_handler(move |event| {
        let _ = tx.send(event);
    });

    let voice = SpeechSynthesisVoice::default_voice()?;
    let utterance = voice.map_or_else(
        || SpeechUtterance::new("Hello from avspeechsynthesizer-rs"),
        |voice| SpeechUtterance::new("Hello from avspeechsynthesizer-rs").with_voice(voice),
    );

    synthesizer.speak(&utterance)?;

    let deadline = Instant::now() + Duration::from_secs(20);
    while Instant::now() < deadline {
        synthesizer.pump_run_loop(Duration::from_millis(100));
        if let Ok(SpeechEvent::DidFinish(_)) = rx.try_recv() {
            break;
        }
    }

    Ok(())
}
```

## Covered areas

- `AVSpeechSynthesizer`
  - speak / pause / stop / continue
  - speaking + paused state inspection
  - delegate-style lifecycle events and range / marker callbacks
  - buffer collection and offline synthesis to a file
- `AVSpeechUtterance`
  - plain text, attributed text, and SSML constructors
  - voice, rate, pitch, volume, assistive-technology preference, and delay controls
  - IPA-notation attributed-string helper
- `AVSpeechSynthesisVoice`
  - voice enumeration and lookup
  - audio file settings, voice quality / gender, and voice traits
  - Alex voice identifier + available-voices notification name
- `AVSpeechSynthesisMarker`
  - generic, word, sentence, paragraph, phoneme, and bookmark constructors
- `AVSpeechSynthesisProvider`
  - provider voice and provider request wrappers
  - extension-only audio-unit APIs are documented in `COVERAGE.md`
- Personal Voice
  - authorization status and authorization request bridge
  - filtering installed personal voices via `available_personal_voices()`

## Examples

```bash
cargo run --example 01_utterance_builders
cargo run --example 02_framework_smoke
cargo run --example 03_voice_catalog
cargo run --example 04_buffer_callback
cargo run --example 05_marker_roundtrip
cargo run --example 06_provider_roundtrip
cargo run --example 07_personal_voice_status
```

## Availability notes

- Base speech synthesis APIs are available on macOS 10.14+.
- SSML utterances, synthesis providers, and marker callbacks require newer AVFAudio SDKs/runtime support.
- Personal Voice authorization and personal-voice traits require macOS 14+.
- Extension-only provider audio-unit APIs are intentionally not wrapped for regular processes.

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the row-by-row SDK audit and the skipped macOS-unavailable / extension-only items.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
