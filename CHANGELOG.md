# Changelog

## [0.3.3] - 2026-05-18

- Added concise AVSpeechSynthesis doc comments across the public non-FFI API surface and raised public-item coverage above the release target.

## [0.3.2] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

All notable changes to this project will be documented in this file.

## [0.3.1] — 2024-12-19

### Fixed

- **Async API safety audit** — Added panic-safe wrapper to `event_callback` FFI callback to prevent unwinding into Swift code
- **Unsafe correctness** — Added SAFETY comments to all unsafe blocks in `async_api.rs` explaining pointer validity and lifetime guarantees
- **Doc link** — Fixed redundant explicit link target in `SpeechSynthesisEventStream` documentation

## [0.3.0] — 2024-05-17

### Added

- **Async API (Tier 2 Stream)** — New `async_api` module (gated behind `async` feature) providing executor-agnostic async event streams
  - `SpeechSynthesisEventStream` wraps delegate callbacks as a `BoundedAsyncStream<SpeechSynthesisEvent>`
  - `SpeechSynthesisEvent` enum covers all delegate events: `DidStart`, `DidFinish`, `DidPause`, `DidContinue`, `DidCancel`, `WillSpeakRange`, `WillSpeakMarker`
  - Example: `examples/08_async_events.rs` demonstrates event listening with `pollster::block_on`
  - Tests: `tests/async_stream_tests.rs` covers subscribe/unsubscribe, event flow, buffering, and state checks
- New `async` Cargo feature flag; new optional dependency on `doom-fish-utils`
- New dev-dependency: `pollster = "0.3"` for examples

### Changed

- Bumped minor version (0.2 → 0.3) for new public module and feature

## [0.2.0] - 2026-05-16

### Added

- Completed the AVSpeech safe wrapper surface across seven logical areas: synthesizer, utterance, voice, buffer callbacks, markers, provider models, and personal voice authorization.
- Added attributed-string and SSML `SpeechUtterance` construction plus assistive-technology preference coverage.
- Added `SpeechSynthesisVoice` audio-file settings, Alex identifier lookup, and voice trait inspection.
- Added `SpeechSynthesizer::write_utterance_with_buffer_callback` and `write_utterance_with_callbacks` for offline buffer collection.
- Added `SpeechSynthesisMarker` constructors for generic, word, sentence, paragraph, phoneme, and bookmark markers.
- Added `SpeechSynthesisProviderVoice` and `SpeechSynthesisProviderRequest` wrappers for provider-side AVSpeech models.
- Added personal-voice authorization helpers and `available_personal_voices()`.
- Added seven numbered examples and one test file per logical area.
- Added `COVERAGE.md` with the SDK-by-SDK audit and deferred extension-only / macOS-unavailable rows.

## [0.1.0] - 2026-05-16

### Added

- `SpeechSynthesizer` wrapper for `AVSpeechSynthesizer` with speaking state inspection plus pause/stop/continue controls.
- `SpeechUtterance` builder covering text, voice, rate, pitch multiplier, volume, and pre/post utterance delays.
- `SpeechSynthesisVoice` enumeration and lookup helpers for `speechVoices()`, `currentLanguageCode()`, `voiceWithLanguage:`, and `voiceWithIdentifier:`.
- `SpeechEvent` delegate coverage for utterance lifecycle callbacks, `willSpeakRangeOfSpeechString`, and macOS 14+ `willSpeakMarker` events.
- `SpeechSynthesisMarker` and `SpeechSynthesisMarkerMark` for word/sentence/phoneme metadata.
- `write_utterance_to_file` for offline synthesis to an audio file with collected synthesis markers.
- End-to-end smoke example `examples/02_framework_smoke.rs`.
