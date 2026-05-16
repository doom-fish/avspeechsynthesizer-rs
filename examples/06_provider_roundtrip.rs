use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider_voice = SpeechSynthesisProviderVoice::new(
        "Rust Demo Voice",
        "com.example.rust-demo.voice",
        ["en-US"],
        ["en-US", "en-GB"],
    )?;
    provider_voice.set_voice_size(65_536)?;
    provider_voice.set_version("1.0.0")?;
    provider_voice.set_gender(SpeechSynthesisVoiceGender::Female)?;
    provider_voice.set_age(4)?;
    SpeechSynthesisProviderVoice::update_speech_voices()?;

    let request = SpeechSynthesisProviderRequest::new(
        "<speak>Hello from the provider bridge.</speak>",
        &provider_voice,
    )?;
    let request_voice = request.voice()?;

    println!(
        "provider voice={} version={} request_ssml={}",
        request_voice.name()?,
        request_voice.version()?,
        request.ssml_representation()?,
    );
    Ok(())
}
