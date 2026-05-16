use avspeechsynthesizer::prelude::*;

#[test]
fn avspeech_surface_is_available() -> Result<(), Box<dyn std::error::Error>> {
    let current_language = SpeechSynthesisVoice::current_language_code()?;
    let all_voices = SpeechSynthesisVoice::speech_voices()?;
    assert!(!all_voices.is_empty());

    let filtered = SpeechSynthesisVoice::voices_with_language(&current_language)?;
    let default_voice = SpeechSynthesisVoice::voice_with_language(&current_language)?;
    let _missing_voice = SpeechSynthesisVoice::voice_with_identifier("not.a.real.voice")?;

    let utterance = default_voice.map_or_else(
        || SpeechUtterance::new("doom fish"),
        |voice| {
            SpeechUtterance::new("doom fish")
                .with_voice(voice)
                .with_rate(SpeechUtterance::DEFAULT_SPEECH_RATE)
                .with_pitch_multiplier(1.0)
                .with_volume(1.0)
                .with_pre_utterance_delay(0.0)
                .with_post_utterance_delay(0.0)
        },
    );

    let mut synthesizer = SpeechSynthesizer::new()?;
    synthesizer.set_event_handler(|event| {
        let _ = format!("{event:?}");
    });
    let _ = synthesizer.is_speaking();
    let _ = synthesizer.is_paused();
    let _ = synthesizer.pause_speaking(SpeechBoundary::Immediate);
    let _ = synthesizer.continue_speaking();
    let _ = synthesizer.stop_speaking(SpeechBoundary::Immediate);
    synthesizer.clear_event_handler();

    assert!(!filtered.is_empty() || !current_language.is_empty());
    assert_eq!(utterance.speech_string(), "doom fish");
    Ok(())
}
