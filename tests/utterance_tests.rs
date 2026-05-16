use avspeechsynthesizer::prelude::*;

#[test]
fn utterance_builders_cover_plain_attributed_and_ssml() -> Result<(), Box<dyn std::error::Error>> {
    let attributed = AttributedSpeechString::new("doom fish")
        .with_ipa_notation(TextRange::new(0, 4), "ˈduːm")?
        .with_ipa_notation(TextRange::new(5, 4), "fɪʃ")?;

    let utterance = SpeechUtterance::from_attributed(attributed)
        .with_rate(SpeechUtterance::default_speech_rate())
        .with_prefers_assistive_technology_settings(true)
        .with_pre_utterance_delay(0.05)
        .with_post_utterance_delay(0.05);
    let resolved = utterance.resolved()?;

    assert_eq!(resolved.kind(), SpeechUtteranceKind::AttributedText);
    assert!(resolved.prefers_assistive_technology_settings());
    assert_eq!(
        resolved
            .attributed_speech_string()
            .map_or(0, |value| value.runs().len()),
        2,
    );

    let ssml =
        SpeechUtterance::from_ssml("<speak>Hello <break time=\"100ms\"/>from Rust.</speak>")?;
    assert_eq!(ssml.kind(), SpeechUtteranceKind::Ssml);
    assert!(ssml.ssml_representation().is_some());
    Ok(())
}
