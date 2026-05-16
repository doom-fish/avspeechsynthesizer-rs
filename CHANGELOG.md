# Changelog

## [0.1.0] - 2026-05-16

### Added

- `SpeechSynthesizer` wrapper for `AVSpeechSynthesizer` with speaking state inspection plus pause/stop/continue controls.
- `SpeechUtterance` builder covering text, voice, rate, pitch multiplier, volume, and pre/post utterance delays.
- `SpeechSynthesisVoice` enumeration and lookup helpers for `speechVoices()`, `currentLanguageCode()`, `voiceWithLanguage:`, and `voiceWithIdentifier:`.
- `SpeechEvent` delegate coverage for utterance lifecycle callbacks, `willSpeakRangeOfSpeechString`, and macOS 14+ `willSpeakMarker` events.
- `SpeechSynthesisMarker` and `SpeechSynthesisMarkerMark` for word/sentence/phoneme metadata.
- `write_utterance_to_file` for offline synthesis to an audio file with collected synthesis markers.
- End-to-end smoke example `examples/02_framework_smoke.rs`.
