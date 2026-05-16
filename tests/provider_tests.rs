use avspeechsynthesizer::prelude::*;

#[test]
fn provider_voice_and_request_round_trip() -> Result<(), Box<dyn std::error::Error>> {
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

    assert_eq!(provider_voice.name()?, "Rust Demo Voice");
    assert_eq!(provider_voice.identifier()?, "com.example.rust-demo.voice");
    assert_eq!(provider_voice.voice_size()?, 65_536);
    assert_eq!(provider_voice.version()?, "1.0.0");
    assert_eq!(provider_voice.gender()?, SpeechSynthesisVoiceGender::Female);
    assert_eq!(provider_voice.age()?, 4);

    let request = SpeechSynthesisProviderRequest::new(
        "<speak>Hello from the provider bridge.</speak>",
        &provider_voice,
    )?;
    let request_voice = request.voice()?;
    assert_eq!(
        request.ssml_representation()?,
        "<speak>Hello from the provider bridge.</speak>"
    );
    assert_eq!(request_voice.name()?, "Rust Demo Voice");

    Ok(())
}
