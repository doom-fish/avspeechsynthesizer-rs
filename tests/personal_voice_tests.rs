use avspeechsynthesizer::prelude::*;

#[test]
fn personal_voice_status_is_queryable() -> Result<(), Box<dyn std::error::Error>> {
    let status = personal_voice_authorization_status()?;
    let voices = available_personal_voices()?;
    if status.is_authorized() {
        for voice in &voices {
            assert!(voice
                .voice_traits()
                .contains(SpeechSynthesisVoiceTraits::IS_PERSONAL_VOICE));
        }
    }
    Ok(())
}
