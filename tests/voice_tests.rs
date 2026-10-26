use avspeechsynthesizer::prelude::*;

#[test]
fn voice_queries_cover_catalog_and_audio_settings() -> Result<(), Box<dyn std::error::Error>> {
    let alex_identifier = SpeechSynthesisVoice::alex_identifier()?;
    assert_ne!(alex_identifier, "");

    let voices = SpeechSynthesisVoice::speech_voices()?;
    assert_ne!(voices, [] as [avspeechsynthesizer::SpeechSynthesisVoice; 0]);

    let default_voice = SpeechSynthesisVoice::default_voice()?;
    if let Some(voice) = default_voice.or_else(|| voices.first().cloned()) {
        let _ = voice.audio_file_settings()?;
        let _ = voice.voice_traits();
        assert_ne!(voice.name(), "");
    }

    Ok(())
}
