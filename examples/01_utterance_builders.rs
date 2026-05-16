use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let attributed = AttributedSpeechString::new("doom fish")
        .with_ipa_notation(TextRange::new(0, 4), "ˈduːm")?
        .with_ipa_notation(TextRange::new(5, 4), "fɪʃ")?;

    let utterance = SpeechUtterance::from_attributed(attributed)
        .with_rate(SpeechUtterance::default_speech_rate())
        .with_prefers_assistive_technology_settings(true)
        .with_post_utterance_delay(0.05);
    let resolved = utterance.resolved()?;

    let ssml = SpeechUtterance::from_ssml(
        "<speak>Hello <break time=\"100ms\"/>from <emphasis level=\"moderate\">Rust</emphasis>.</speak>",
    )?;

    println!(
        "kind={:?} runs={} min_rate={} default_rate={} max_rate={}",
        resolved.kind(),
        resolved
            .attributed_speech_string()
            .map_or(0, |value| value.runs().len()),
        SpeechUtterance::minimum_speech_rate(),
        SpeechUtterance::default_speech_rate(),
        SpeechUtterance::maximum_speech_rate(),
    );
    println!("ssml={}", ssml.ssml_representation().unwrap_or_default());

    Ok(())
}
