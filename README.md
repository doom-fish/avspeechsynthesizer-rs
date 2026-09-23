# avspeechsynthesizer-rs

Safe Rust bindings for Apple's `AVSpeechSynthesizer` text-to-speech APIs on macOS.

> **Status:** v0.4 covers the complete public AVSpeech surface that is available to regular macOS apps, including utterances, voices, synthesizer lifecycle control, offline/audio-buffer writes, synthesis markers, provider voice/request models, and personal voice authorization.

## Installation

```toml
[dependencies]
avspeechsynthesizer-rs = { version = "0.4", features = ["async"] }
```

The library is imported as `avspeechsynthesizer`. Leave out `features` if you don't need the async event stream.

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

## Events and threading

`AVSpeechSynthesizer` delivers its delegate events and the audio-buffer callbacks behind `write_utterance_to_file` and `write_utterance_with_*` on the main thread, whichever thread owns the synthesizer. Call these from the main thread and pump its run loop (`SpeechSynthesizer::pump_run_loop`), or run an app event loop. Otherwise events never arrive and the write calls fail with `AvSpeechError::TimedOut` after 120 seconds.

## Async API

Enable the `async` feature for `async_api::SpeechSynthesisEventStream`, an executor-agnostic stream of delegate events. The handler from `set_event_handler` and any number of streams receive every event side by side; dropping a stream removes only that stream. The buffer drops its oldest event when it is full, and a capacity of zero is rejected with `AvSpeechError::InvalidArgument`.

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
cargo run --example 08_async_events --features async
```

## Availability notes

- The crate requires macOS 13 or later, the deployment target of its Swift bridge.
- Word, sentence, paragraph, phoneme and bookmark markers, marker callbacks, Personal Voice authorization and voice traits require macOS 14 or later; on macOS 13 those marker constructors and the Personal Voice authorization calls return `AvSpeechError::UnavailableOnThisMacOS`, and `available_personal_voices` returns an empty list.
- `request_personal_voice_authorization` shows a system permission prompt.
- Extension-only provider audio-unit APIs are intentionally not wrapped for regular processes.

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the row-by-row SDK audit and the skipped macOS-unavailable / extension-only items.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
