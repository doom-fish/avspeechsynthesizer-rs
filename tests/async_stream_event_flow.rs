use std::time::{Duration, Instant};

use avspeechsynthesizer::async_api::{SpeechSynthesisEvent, SpeechSynthesisEventStream};
use avspeechsynthesizer::prelude::*;

const TEXT: &str = "Event stream check.";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let synthesizer = SpeechSynthesizer::new()?;
    let event_stream = SpeechSynthesisEventStream::subscribe(&synthesizer, 64)?;
    synthesizer.write_utterance_with_buffer_callback(
        &SpeechUtterance::new(TEXT).with_volume(0.0),
        |_| {},
    )?;

    let mut events = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(20);
    while !events
        .iter()
        .any(|event| matches!(event, SpeechSynthesisEvent::DidFinish(_)))
    {
        assert!(
            Instant::now() < deadline,
            "expected a DidFinish event, got {events:?}"
        );
        match event_stream.try_next() {
            Some(event) => events.push(event),
            None => synthesizer.pump_run_loop(Duration::from_millis(50)),
        }
    }

    assert!(
        matches!(events.first(), Some(SpeechSynthesisEvent::DidStart(utterance)) if utterance.speech_string() == TEXT),
        "expected DidStart first, got {events:?}"
    );
    assert!(
        matches!(events.last(), Some(SpeechSynthesisEvent::DidFinish(utterance)) if utterance.speech_string() == TEXT),
        "expected DidFinish last, got {events:?}"
    );
    let text_length = TEXT.encode_utf16().count();
    for event in &events {
        if let SpeechSynthesisEvent::WillSpeakRange {
            character_range, ..
        } = event
        {
            assert!(
                character_range.end().is_some_and(|end| end <= text_length),
                "range {character_range:?} lies outside {TEXT:?}"
            );
        }
    }
    assert!(!event_stream.is_closed());

    println!("event stream delivered {} events", events.len());
    Ok(())
}
