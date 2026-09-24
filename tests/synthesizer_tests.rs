use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use avspeechsynthesizer::prelude::*;

const TEXT: &str = "Lifecycle event check.";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notification = SpeechSynthesizer::available_voices_did_change_notification_name()?;
    assert!(!notification.is_empty());

    synthesizer_emits_lifecycle_events_while_writing()?;
    framework_smoke_example_speaks_aloud()
}

fn synthesizer_emits_lifecycle_events_while_writing() -> Result<(), Box<dyn std::error::Error>> {
    let mut synthesizer = SpeechSynthesizer::new()?;
    let (tx, rx) = mpsc::channel();
    synthesizer.set_event_handler(move |event| {
        let _ = tx.send(event);
    });

    let buffers = Arc::new(AtomicUsize::new(0));
    let saw_end = Arc::new(AtomicBool::new(false));
    let (buffer_count, end) = (Arc::clone(&buffers), Arc::clone(&saw_end));
    synthesizer.write_utterance_with_buffer_callback(
        &SpeechUtterance::new(TEXT).with_volume(0.0),
        move |buffer| {
            buffer_count.fetch_add(1, Ordering::SeqCst);
            if buffer.is_end_of_stream() {
                end.store(true, Ordering::SeqCst);
            }
        },
    )?;
    assert!(buffers.load(Ordering::SeqCst) > 1);
    assert!(saw_end.load(Ordering::SeqCst));

    let mut events: Vec<SpeechEvent> = rx.try_iter().collect();
    let deadline = Instant::now() + Duration::from_secs(20);
    while !events
        .iter()
        .any(|event| matches!(event, SpeechEvent::DidFinish(_)))
    {
        assert!(
            Instant::now() < deadline,
            "expected a DidFinish event, got {events:?}"
        );
        synthesizer.pump_run_loop(Duration::from_millis(50));
        events.extend(rx.try_iter());
    }

    assert!(
        matches!(events.first(), Some(SpeechEvent::DidStart(utterance)) if utterance.speech_string() == TEXT),
        "expected DidStart first, got {events:?}"
    );
    assert!(
        matches!(events.last(), Some(SpeechEvent::DidFinish(utterance)) if utterance.speech_string() == TEXT),
        "expected DidFinish last, got {events:?}"
    );
    let text_length = TEXT.encode_utf16().count();
    for event in &events {
        if let SpeechEvent::WillSpeakRangeOfSpeechString {
            character_range, ..
        } = event
        {
            assert!(
                character_range.end().is_some_and(|end| end <= text_length),
                "range {character_range:?} lies outside {TEXT:?}"
            );
        }
    }

    println!("buffer write delivered {} delegate events", events.len());
    Ok(())
}

fn framework_smoke_example_speaks_aloud() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("AVSPEECHSYNTHESIZER_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!(
            "skip: set AVSPEECHSYNTHESIZER_LIVE_TESTS=1 to run the 02_framework_smoke example, which speaks aloud"
        );
        return Ok(());
    }

    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("examples")
        .join("02_framework_smoke");
    let status = Command::new(example).status()?;
    assert!(status.success());
    Ok(())
}
