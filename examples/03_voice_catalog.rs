use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let alex_identifier = SpeechSynthesisVoice::alex_identifier()?;
    let voices = SpeechSynthesisVoice::speech_voices()?;
    let default_voice = SpeechSynthesisVoice::default_voice()?;

    println!("alex voice identifier: {alex_identifier}");
    println!("installed voices: {}", voices.len());
    println!(
        "personal voices visible: {}",
        voices
            .iter()
            .filter(|voice| {
                voice
                    .voice_traits()
                    .contains(SpeechSynthesisVoiceTraits::IS_PERSONAL_VOICE)
            })
            .count()
    );

    if let Some(voice) = default_voice.or_else(|| voices.first().cloned()) {
        println!(
            "default/first voice: {} [{}] traits={:?}",
            voice.name(),
            voice.language(),
            voice.voice_traits(),
        );
        if let Some(settings) = voice.audio_file_settings()? {
            println!("audio file settings: {settings}");
        }
    }

    Ok(())
}
