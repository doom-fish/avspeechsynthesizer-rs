# AVSpeech SDK coverage audit

Legend:

- ✅ implemented
- ⏭️ skipped — unavailable on macOS or extension-only

## AVSpeechSynthesis.h

| Apple API | Status | Rust surface / note |
| --- | --- | --- |
| `AVSpeechBoundary.{immediate,word}` | ✅ | `SpeechBoundary::{Immediate, Word}` |
| `AVSpeechSynthesisVoiceQuality.{default,enhanced,premium}` | ✅ | `SpeechSynthesisVoiceQuality` |
| `AVSpeechSynthesisVoiceGender.{unspecified,male,female}` | ✅ | `SpeechSynthesisVoiceGender` |
| `AVSpeechSynthesisMarker.Mark.{phoneme,word,sentence,paragraph,bookmark}` | ✅ | `SpeechSynthesisMarkerMark` |
| `AVSpeechUtteranceMinimumSpeechRate` | ✅ | `SpeechUtterance::MINIMUM_SPEECH_RATE`, `SpeechUtterance::minimum_speech_rate()` |
| `AVSpeechUtteranceDefaultSpeechRate` | ✅ | `SpeechUtterance::DEFAULT_SPEECH_RATE`, `SpeechUtterance::default_speech_rate()` |
| `AVSpeechUtteranceMaximumSpeechRate` | ✅ | `SpeechUtterance::MAXIMUM_SPEECH_RATE`, `SpeechUtterance::maximum_speech_rate()` |
| `AVSpeechSynthesisVoiceIdentifierAlex` | ✅ | `SpeechSynthesisVoice::alex_identifier()` |
| `AVSpeechSynthesisIPANotationAttribute` | ✅ | `SpeechUtterance::ipa_notation_attribute_name()`, `AttributedSpeechString::with_ipa_notation(...)` |
| `AVSpeechSynthesizer.BufferCallback` | ✅ | `SpeechSynthesizer::write_utterance_with_buffer_callback(...)` |
| `AVSpeechSynthesizer.MarkerCallback` | ✅ | `SpeechSynthesizer::write_utterance_with_callbacks(...)` |
| `AVSpeechSynthesizer.PersonalVoiceAuthorizationStatus` | ✅ | `PersonalVoiceAuthorizationStatus` |
| `AVSpeechSynthesisVoice.Traits.{none,isNoveltyVoice,isPersonalVoice}` | ✅ | `SpeechSynthesisVoiceTraits` |
| `AVSpeechSynthesizer.availableVoicesDidChangeNotification` | ✅ | `SpeechSynthesizer::available_voices_did_change_notification_name()` |
| `AVSpeechSynthesisVoice.speechVoices()` | ✅ | `SpeechSynthesisVoice::speech_voices()` |
| `AVSpeechSynthesisVoice.currentLanguageCode()` | ✅ | `SpeechSynthesisVoice::current_language_code()` |
| `AVSpeechSynthesisVoice.voiceWithLanguage(_:)` | ✅ | `SpeechSynthesisVoice::voice_with_language(...)`, `SpeechSynthesisVoice::default_voice()` |
| `AVSpeechSynthesisVoice.voiceWithIdentifier(_:)` | ✅ | `SpeechSynthesisVoice::voice_with_identifier(...)` |
| `AVSpeechSynthesisVoice.language` | ✅ | `SpeechSynthesisVoice::language()` |
| `AVSpeechSynthesisVoice.identifier` | ✅ | `SpeechSynthesisVoice::identifier()` |
| `AVSpeechSynthesisVoice.name` | ✅ | `SpeechSynthesisVoice::name()` |
| `AVSpeechSynthesisVoice.quality` | ✅ | `SpeechSynthesisVoice::quality()` |
| `AVSpeechSynthesisVoice.gender` | ✅ | `SpeechSynthesisVoice::gender()` |
| `AVSpeechSynthesisVoice.audioFileSettings` | ✅ | `SpeechSynthesisVoice::audio_file_settings_json()`, `audio_file_settings()` |
| `AVSpeechSynthesisVoice.voiceTraits` | ✅ | `SpeechSynthesisVoice::voice_traits()`, `traits()` |
| `AVSpeechUtterance.speechUtterance(with:)` / `init(string:)` | ✅ | `SpeechUtterance::new(...)` |
| `AVSpeechUtterance.speechUtterance(attributedString:)` / `init(attributedString:)` | ✅ | `SpeechUtterance::from_attributed(...)` |
| `AVSpeechUtterance.speechUtterance(ssmlRepresentation:)` / `init(ssmlRepresentation:)` | ✅ | `SpeechUtterance::from_ssml(...)` |
| `AVSpeechUtterance.voice` | ✅ | `SpeechUtterance::voice()`, `with_voice(...)` |
| `AVSpeechUtterance.speechString` | ✅ | `SpeechUtterance::speech_string()` |
| `AVSpeechUtterance.attributedSpeechString` | ✅ | `SpeechUtterance::attributed_speech_string()` |
| `AVSpeechUtterance.rate` | ✅ | `SpeechUtterance::rate()`, `with_rate(...)` |
| `AVSpeechUtterance.pitchMultiplier` | ✅ | `SpeechUtterance::pitch_multiplier()`, `with_pitch_multiplier(...)` |
| `AVSpeechUtterance.volume` | ✅ | `SpeechUtterance::volume()`, `with_volume(...)` |
| `AVSpeechUtterance.prefersAssistiveTechnologySettings` | ✅ | `SpeechUtterance::prefers_assistive_technology_settings()`, `with_prefers_assistive_technology_settings(...)` |
| `AVSpeechUtterance.preUtteranceDelay` | ✅ | `SpeechUtterance::pre_utterance_delay()`, `with_pre_utterance_delay(...)` |
| `AVSpeechUtterance.postUtteranceDelay` | ✅ | `SpeechUtterance::post_utterance_delay()`, `with_post_utterance_delay(...)` |
| `AVSpeechSynthesizer.delegate` | ✅ | `SpeechSynthesizer::set_event_handler(...)`, `clear_event_handler()` |
| `AVSpeechSynthesizer.isSpeaking` | ✅ | `SpeechSynthesizer::is_speaking()` |
| `AVSpeechSynthesizer.isPaused` | ✅ | `SpeechSynthesizer::is_paused()` |
| `AVSpeechSynthesizer.speak(_:)` | ✅ | `SpeechSynthesizer::speak(...)` |
| `AVSpeechSynthesizer.write(_:toBufferCallback:)` | ✅ | `SpeechSynthesizer::write_utterance_with_buffer_callback(...)` |
| `AVSpeechSynthesizer.write(_:toBufferCallback:toMarkerCallback:)` | ✅ | `SpeechSynthesizer::write_utterance_with_callbacks(...)` |
| `AVSpeechSynthesizer.stopSpeaking(at:)` | ✅ | `SpeechSynthesizer::stop_speaking(...)` |
| `AVSpeechSynthesizer.pauseSpeaking(at:)` | ✅ | `SpeechSynthesizer::pause_speaking(...)` |
| `AVSpeechSynthesizer.continueSpeaking()` | ✅ | `SpeechSynthesizer::continue_speaking()` |
| `AVSpeechSynthesizer.outputChannels` | ⏭️ skipped | API is `API_UNAVAILABLE(macos)` |
| `AVSpeechSynthesizer.usesApplicationAudioSession` | ⏭️ skipped | API is `API_UNAVAILABLE(macos)` |
| `AVSpeechSynthesizer.mixToTelephonyUplink` | ⏭️ skipped | API is `API_UNAVAILABLE(macos)` |
| `AVSpeechSynthesizer.requestPersonalVoiceAuthorization(...)` | ✅ | `request_personal_voice_authorization(...)` |
| `AVSpeechSynthesizer.personalVoiceAuthorizationStatus` | ✅ | `personal_voice_authorization_status()` |
| `AVSpeechSynthesizerDelegate.didStartSpeechUtterance` | ✅ | `SpeechEvent::DidStart` |
| `AVSpeechSynthesizerDelegate.didFinishSpeechUtterance` | ✅ | `SpeechEvent::DidFinish` |
| `AVSpeechSynthesizerDelegate.didPauseSpeechUtterance` | ✅ | `SpeechEvent::DidPause` |
| `AVSpeechSynthesizerDelegate.didContinueSpeechUtterance` | ✅ | `SpeechEvent::DidContinue` |
| `AVSpeechSynthesizerDelegate.didCancelSpeechUtterance` | ✅ | `SpeechEvent::DidCancel` |
| `AVSpeechSynthesizerDelegate.willSpeakRangeOfSpeechString` | ✅ | `SpeechEvent::WillSpeakRangeOfSpeechString` |
| `AVSpeechSynthesizerDelegate.willSpeakMarker` | ✅ | `SpeechEvent::WillSpeakMarker` |
| `AVSpeechSynthesisMarker.mark` | ✅ | `SpeechSynthesisMarker::mark()`, public `mark` field |
| `AVSpeechSynthesisMarker.byteSampleOffset` | ✅ | `SpeechSynthesisMarker::byte_sample_offset()`, public `byte_sample_offset` field |
| `AVSpeechSynthesisMarker.textRange` | ✅ | `SpeechSynthesisMarker::text_range()`, public `text_range` field |
| `AVSpeechSynthesisMarker.bookmarkName` | ✅ | `SpeechSynthesisMarker::bookmark_name()`, public `bookmark_name` field |
| `AVSpeechSynthesisMarker.phoneme` | ✅ | `SpeechSynthesisMarker::phoneme_text()`, public `phoneme` field |
| `AVSpeechSynthesisMarker.init(markerType:forTextRange:atByteSampleOffset:)` | ✅ | `SpeechSynthesisMarker::new(...)` |
| `AVSpeechSynthesisMarker.init(wordRange:atByteSampleOffset:)` | ✅ | `SpeechSynthesisMarker::word(...)` |
| `AVSpeechSynthesisMarker.init(sentenceRange:atByteSampleOffset:)` | ✅ | `SpeechSynthesisMarker::sentence(...)` |
| `AVSpeechSynthesisMarker.init(paragraphRange:atByteSampleOffset:)` | ✅ | `SpeechSynthesisMarker::paragraph(...)` |
| `AVSpeechSynthesisMarker.init(phonemeString:atByteSampleOffset:)` | ✅ | `SpeechSynthesisMarker::phoneme(...)` |
| `AVSpeechSynthesisMarker.init(bookmarkName:atByteSampleOffset:)` | ✅ | `SpeechSynthesisMarker::bookmark(...)` |

## AVSpeechSynthesisProvider.h

| Apple API | Status | Rust surface / note |
| --- | --- | --- |
| `AVSpeechSynthesisProviderVoice.name` | ✅ | `SpeechSynthesisProviderVoice::name()` |
| `AVSpeechSynthesisProviderVoice.identifier` | ✅ | `SpeechSynthesisProviderVoice::identifier()` |
| `AVSpeechSynthesisProviderVoice.primaryLanguages` | ✅ | `SpeechSynthesisProviderVoice::primary_languages()` |
| `AVSpeechSynthesisProviderVoice.supportedLanguages` | ✅ | `SpeechSynthesisProviderVoice::supported_languages()` |
| `AVSpeechSynthesisProviderVoice.voiceSize` | ✅ | `SpeechSynthesisProviderVoice::voice_size()`, `set_voice_size(...)` |
| `AVSpeechSynthesisProviderVoice.version` | ✅ | `SpeechSynthesisProviderVoice::version()`, `set_version(...)` |
| `AVSpeechSynthesisProviderVoice.gender` | ✅ | `SpeechSynthesisProviderVoice::gender()`, `set_gender(...)` |
| `AVSpeechSynthesisProviderVoice.age` | ✅ | `SpeechSynthesisProviderVoice::age()`, `set_age(...)` |
| `AVSpeechSynthesisProviderVoice.init(name:identifier:primaryLanguages:supportedLanguages:)` | ✅ | `SpeechSynthesisProviderVoice::new(...)` |
| `AVSpeechSynthesisProviderVoice.updateSpeechVoices()` | ✅ | `SpeechSynthesisProviderVoice::update_speech_voices()` |
| `AVSpeechSynthesisProviderRequest.ssmlRepresentation` | ✅ | `SpeechSynthesisProviderRequest::ssml_representation()` |
| `AVSpeechSynthesisProviderRequest.voice` | ✅ | `SpeechSynthesisProviderRequest::voice()` |
| `AVSpeechSynthesisProviderRequest.init(ssmlRepresentation:voice:)` | ✅ | `SpeechSynthesisProviderRequest::new(...)` |
| `AVSpeechSynthesisProviderOutputBlock` | ⏭️ skipped | Extension-only audio-unit callback surface |
| `AVSpeechSynthesisProviderAudioUnit` | ⏭️ skipped | Extension-only AUAudioUnit subclass |
| `AVSpeechSynthesisProviderAudioUnit.speechVoices` | ⏭️ skipped | Extension-only AUAudioUnit subclass |
| `AVSpeechSynthesisProviderAudioUnit.speechSynthesisOutputMetadataBlock` | ⏭️ skipped | Extension-only AUAudioUnit subclass |
| `AVSpeechSynthesisProviderAudioUnit.synthesizeSpeechRequest(_:)` | ⏭️ skipped | Extension-only AUAudioUnit subclass |
| `AVSpeechSynthesisProviderAudioUnit.cancelSpeechRequest()` | ⏭️ skipped | Extension-only AUAudioUnit subclass |
