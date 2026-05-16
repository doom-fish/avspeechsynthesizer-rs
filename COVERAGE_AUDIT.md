# avspeechsynthesizer-rs coverage audit (vs MacOSX26.2.sdk)

Audited headers: `AVSpeechSynthesis.h` and `AVSpeechSynthesisProvider.h`, which are re-exported by `AVFoundation.framework/Headers/AVFAudio.h` and physically live in `AVFAudio.framework/Headers/` in `MacOSX26.2.sdk`.

Methodology notes:
- Objective-C factory/initializer pairs that map to one Rust constructor are grouped into one audit row.
- Per the audit instructions, macOS-unavailable members were filtered out and are not counted: `AVSpeechSynthesizer.outputChannels`, `AVSpeechSynthesizer.usesApplicationAudioSession`, and `AVSpeechSynthesizer.mixToTelephonyUplink`.
- `AVSpeechSynthesisProviderAudioUnit` is treated as EXEMPT because this crate targets regular macOS processes, not speech-synthesis Audio Unit extensions; the crate README documents that extension-only provider APIs are intentionally not wrapped.

SDK_PUBLIC_SYMBOLS: 85
VERIFIED: 79
GAPS: 0
EXEMPT: 6
COVERAGE_PCT: 100.0%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `AVSpeechBoundary.{immediate,word}` | enum | `AVSpeechSynthesis.h` | `SpeechBoundary::{Immediate, Word}` |
| `AVSpeechSynthesisVoiceQuality.{default,enhanced,premium}` | enum | `AVSpeechSynthesis.h` | `SpeechSynthesisVoiceQuality` |
| `AVSpeechSynthesisVoiceGender.{unspecified,male,female}` | enum | `AVSpeechSynthesis.h` | `SpeechSynthesisVoiceGender` |
| `AVSpeechSynthesisMarker.Mark.{phoneme,word,sentence,paragraph,bookmark}` | enum | `AVSpeechSynthesis.h` | `SpeechSynthesisMarkerMark` |
| `AVSpeechUtteranceMinimumSpeechRate` | constant | `AVSpeechSynthesis.h` | `SpeechUtterance::MINIMUM_SPEECH_RATE`; `SpeechUtterance::minimum_speech_rate()` |
| `AVSpeechUtteranceDefaultSpeechRate` | constant | `AVSpeechSynthesis.h` | `SpeechUtterance::DEFAULT_SPEECH_RATE`; `SpeechUtterance::default_speech_rate()` |
| `AVSpeechUtteranceMaximumSpeechRate` | constant | `AVSpeechSynthesis.h` | `SpeechUtterance::MAXIMUM_SPEECH_RATE`; `SpeechUtterance::maximum_speech_rate()` |
| `AVSpeechSynthesisVoiceIdentifierAlex` | constant | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::alex_identifier()` |
| `AVSpeechSynthesisIPANotationAttribute` | constant | `AVSpeechSynthesis.h` | `SpeechUtterance::ipa_notation_attribute_name()`; `AttributedSpeechString::with_ipa_notation(...)` |
| `AVSpeechSynthesizerBufferCallback` | typealias | `AVSpeechSynthesis.h` | `SpeechSynthesizer::write_utterance_with_buffer_callback(...)`; `SpeechSynthesizer::write_utterance_to_file(...)` |
| `AVSpeechSynthesizerMarkerCallback` | typealias | `AVSpeechSynthesis.h` | `SpeechSynthesizer::write_utterance_with_callbacks(...)`; `SpeechSynthesizer::write_utterance_to_file(...)` |
| `AVSpeechSynthesisPersonalVoiceAuthorizationStatus` | enum | `AVSpeechSynthesis.h` | `PersonalVoiceAuthorizationStatus` |
| `AVSpeechSynthesisVoiceTraits.{none,isNoveltyVoice,isPersonalVoice}` | option set | `AVSpeechSynthesis.h` | `SpeechSynthesisVoiceTraits` |
| `AVSpeechSynthesisAvailableVoicesDidChangeNotification` | notification | `AVSpeechSynthesis.h` | `SpeechSynthesizer::available_voices_did_change_notification_name()` |
| `AVSpeechSynthesisVoice.speechVoices()` | class method | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::speech_voices()` |
| `AVSpeechSynthesisVoice.currentLanguageCode()` | class method | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::current_language_code()` |
| `AVSpeechSynthesisVoice.voiceWithLanguage(_:)` | class method | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::voice_with_language(...)`; `SpeechSynthesisVoice::default_voice()` |
| `AVSpeechSynthesisVoice.voiceWithIdentifier(_:)` | class method | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::voice_with_identifier(...)` |
| `AVSpeechSynthesisVoice.language` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::language()` |
| `AVSpeechSynthesisVoice.identifier` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::identifier()` |
| `AVSpeechSynthesisVoice.name` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::name()` |
| `AVSpeechSynthesisVoice.quality` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::quality()` |
| `AVSpeechSynthesisVoice.gender` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::gender()` |
| `AVSpeechSynthesisVoice.audioFileSettings` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::audio_file_settings_json()`; `SpeechSynthesisVoice::audio_file_settings()` |
| `AVSpeechSynthesisVoice.voiceTraits` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisVoice::traits()`; `SpeechSynthesisVoice::voice_traits()` |
| `AVSpeechUtterance.speechUtteranceWithString(_:)` / `init(string:)` | initializer | `AVSpeechSynthesis.h` | `SpeechUtterance::new(...)` |
| `AVSpeechUtterance.speechUtteranceWithAttributedString(_:)` / `init(attributedString:)` | initializer | `AVSpeechSynthesis.h` | `SpeechUtterance::from_attributed(...)` |
| `AVSpeechUtterance.speechUtteranceWithSSMLRepresentation(_:)` / `init(ssmlRepresentation:)` | initializer | `AVSpeechSynthesis.h` | `SpeechUtterance::from_ssml(...)` |
| `AVSpeechUtterance.voice` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::voice()`; `SpeechUtterance::with_voice(...)` |
| `AVSpeechUtterance.speechString` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::speech_string()` |
| `AVSpeechUtterance.attributedSpeechString` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::attributed_speech_string()` |
| `AVSpeechUtterance.rate` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::rate()`; `SpeechUtterance::with_rate(...)` |
| `AVSpeechUtterance.pitchMultiplier` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::pitch_multiplier()`; `SpeechUtterance::with_pitch_multiplier(...)` |
| `AVSpeechUtterance.volume` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::volume()`; `SpeechUtterance::with_volume(...)` |
| `AVSpeechUtterance.prefersAssistiveTechnologySettings` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::prefers_assistive_technology_settings()`; `SpeechUtterance::with_prefers_assistive_technology_settings(...)` |
| `AVSpeechUtterance.preUtteranceDelay` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::pre_utterance_delay()`; `SpeechUtterance::with_pre_utterance_delay(...)` |
| `AVSpeechUtterance.postUtteranceDelay` | property | `AVSpeechSynthesis.h` | `SpeechUtterance::post_utterance_delay()`; `SpeechUtterance::with_post_utterance_delay(...)` |
| `AVSpeechSynthesizer.delegate` | property | `AVSpeechSynthesis.h` | `SpeechSynthesizer::set_event_handler(...)`; `SpeechSynthesizer::clear_event_handler()` |
| `AVSpeechSynthesizer.isSpeaking` | property | `AVSpeechSynthesis.h` | `SpeechSynthesizer::is_speaking()` |
| `AVSpeechSynthesizer.isPaused` | property | `AVSpeechSynthesis.h` | `SpeechSynthesizer::is_paused()` |
| `AVSpeechSynthesizer.speakUtterance(_:)` | instance method | `AVSpeechSynthesis.h` | `SpeechSynthesizer::speak(...)` |
| `AVSpeechSynthesizer.writeUtterance(_:toBufferCallback:)` | instance method | `AVSpeechSynthesis.h` | `SpeechSynthesizer::write_utterance_with_buffer_callback(...)`; `SpeechSynthesizer::write_utterance_to_file(...)` |
| `AVSpeechSynthesizer.writeUtterance(_:toBufferCallback:toMarkerCallback:)` | instance method | `AVSpeechSynthesis.h` | `SpeechSynthesizer::write_utterance_with_callbacks(...)`; `SpeechSynthesizer::write_utterance_to_file(...)` |
| `AVSpeechSynthesizer.stopSpeakingAtBoundary(_:)` | instance method | `AVSpeechSynthesis.h` | `SpeechSynthesizer::stop_speaking(...)` |
| `AVSpeechSynthesizer.pauseSpeakingAtBoundary(_:)` | instance method | `AVSpeechSynthesis.h` | `SpeechSynthesizer::pause_speaking(...)` |
| `AVSpeechSynthesizer.continueSpeaking()` | instance method | `AVSpeechSynthesis.h` | `SpeechSynthesizer::continue_speaking()` |
| `AVSpeechSynthesizer.requestPersonalVoiceAuthorization(...)` | class method | `AVSpeechSynthesis.h` | `request_personal_voice_authorization(...)` |
| `AVSpeechSynthesizer.personalVoiceAuthorizationStatus` | class property | `AVSpeechSynthesis.h` | `personal_voice_authorization_status()` |
| `AVSpeechSynthesizerDelegate.didStartSpeechUtterance` | delegate callback | `AVSpeechSynthesis.h` | `SpeechEvent::DidStart` |
| `AVSpeechSynthesizerDelegate.didFinishSpeechUtterance` | delegate callback | `AVSpeechSynthesis.h` | `SpeechEvent::DidFinish` |
| `AVSpeechSynthesizerDelegate.didPauseSpeechUtterance` | delegate callback | `AVSpeechSynthesis.h` | `SpeechEvent::DidPause` |
| `AVSpeechSynthesizerDelegate.didContinueSpeechUtterance` | delegate callback | `AVSpeechSynthesis.h` | `SpeechEvent::DidContinue` |
| `AVSpeechSynthesizerDelegate.didCancelSpeechUtterance` | delegate callback | `AVSpeechSynthesis.h` | `SpeechEvent::DidCancel` |
| `AVSpeechSynthesizerDelegate.willSpeakRangeOfSpeechString` | delegate callback | `AVSpeechSynthesis.h` | `SpeechEvent::WillSpeakRangeOfSpeechString` |
| `AVSpeechSynthesizerDelegate.willSpeakMarker` | delegate callback | `AVSpeechSynthesis.h` | `SpeechEvent::WillSpeakMarker` |
| `AVSpeechSynthesisMarker.mark` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::mark()`; public `mark` field |
| `AVSpeechSynthesisMarker.byteSampleOffset` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::byte_sample_offset()`; public `byte_sample_offset` field |
| `AVSpeechSynthesisMarker.textRange` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::text_range()`; public `text_range` field |
| `AVSpeechSynthesisMarker.bookmarkName` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::bookmark_name()`; public `bookmark_name` field |
| `AVSpeechSynthesisMarker.phoneme` | property | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::phoneme_text()`; public `phoneme` field |
| `AVSpeechSynthesisMarker.init(markerType:forTextRange:atByteSampleOffset:)` | initializer | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::new(...)` |
| `AVSpeechSynthesisMarker.init(wordRange:atByteSampleOffset:)` | initializer | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::word(...)` |
| `AVSpeechSynthesisMarker.init(sentenceRange:atByteSampleOffset:)` | initializer | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::sentence(...)` |
| `AVSpeechSynthesisMarker.init(paragraphRange:atByteSampleOffset:)` | initializer | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::paragraph(...)` |
| `AVSpeechSynthesisMarker.init(phonemeString:atByteSampleOffset:)` | initializer | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::phoneme(...)` |
| `AVSpeechSynthesisMarker.init(bookmarkName:atByteSampleOffset:)` | initializer | `AVSpeechSynthesis.h` | `SpeechSynthesisMarker::bookmark(...)` |
| `AVSpeechSynthesisProviderVoice.name` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::name()` |
| `AVSpeechSynthesisProviderVoice.identifier` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::identifier()` |
| `AVSpeechSynthesisProviderVoice.primaryLanguages` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::primary_languages()` |
| `AVSpeechSynthesisProviderVoice.supportedLanguages` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::supported_languages()` |
| `AVSpeechSynthesisProviderVoice.voiceSize` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::voice_size()`; `SpeechSynthesisProviderVoice::set_voice_size(...)` |
| `AVSpeechSynthesisProviderVoice.version` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::version()`; `SpeechSynthesisProviderVoice::set_version(...)` |
| `AVSpeechSynthesisProviderVoice.gender` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::gender()`; `SpeechSynthesisProviderVoice::set_gender(...)` |
| `AVSpeechSynthesisProviderVoice.age` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::age()`; `SpeechSynthesisProviderVoice::set_age(...)` |
| `AVSpeechSynthesisProviderVoice.init(name:identifier:primaryLanguages:supportedLanguages:)` | initializer | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::new(...)` |
| `AVSpeechSynthesisProviderVoice.updateSpeechVoices()` | class method | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderVoice::update_speech_voices()` |
| `AVSpeechSynthesisProviderRequest.ssmlRepresentation` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderRequest::ssml_representation()` |
| `AVSpeechSynthesisProviderRequest.voice` | property | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderRequest::voice()` |
| `AVSpeechSynthesisProviderRequest.init(ssmlRepresentation:voice:)` | initializer | `AVSpeechSynthesisProvider.h` | `SpeechSynthesisProviderRequest::new(...)` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

No gaps identified.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `AVSpeechSynthesisProviderOutputBlock` | typealias | `AVSpeechSynthesisProvider.h` | Audio Unit extension host callback; the crate intentionally stops at provider voice/request models for regular processes. | `API_AVAILABLE(ios(16.0), macos(13.0), watchos(9.0), tvos(16.0))` |
| `AVSpeechSynthesisProviderAudioUnit` | class | `AVSpeechSynthesisProvider.h` | Speech-synthesis Audio Unit extension base class; intentionally out of scope for this crate. | `API_AVAILABLE(ios(16.0), macos(13.0), tvos(16.0)) API_UNAVAILABLE(watchos)` |
| `AVSpeechSynthesisProviderAudioUnit.speechVoices` | property | `AVSpeechSynthesisProvider.h` | Extension-only Audio Unit surface, not a regular-process AVSpeechSynthesizer binding. | `API_AVAILABLE(ios(16.0), macos(13.0), tvos(16.0)) API_UNAVAILABLE(watchos)` |
| `AVSpeechSynthesisProviderAudioUnit.speechSynthesisOutputMetadataBlock` | property | `AVSpeechSynthesisProvider.h` | Extension-only Audio Unit surface, not a regular-process AVSpeechSynthesizer binding. | `API_AVAILABLE(ios(16.0), macos(13.0), tvos(16.0)) API_UNAVAILABLE(watchos)` |
| `AVSpeechSynthesisProviderAudioUnit.synthesizeSpeechRequest(_:)` | instance method | `AVSpeechSynthesisProvider.h` | Extension-only Audio Unit surface, not a regular-process AVSpeechSynthesizer binding. | `API_AVAILABLE(ios(16.0), macos(13.0), tvos(16.0)) API_UNAVAILABLE(watchos)` |
| `AVSpeechSynthesisProviderAudioUnit.cancelSpeechRequest()` | instance method | `AVSpeechSynthesisProvider.h` | Extension-only Audio Unit surface, not a regular-process AVSpeechSynthesizer binding. | `API_AVAILABLE(ios(16.0), macos(13.0), tvos(16.0)) API_UNAVAILABLE(watchos)` |
