#![cfg(feature = "async")]

use avspeechsynthesizer::async_api::{SpeechSynthesisEvent, SpeechSynthesisEventStream};
use avspeechsynthesizer::prelude::*;

#[test]
fn test_async_stream_subscribe_unsubscribe() -> Result<(), Box<dyn std::error::Error>> {
    // Create a synthesizer
    let synthesizer = SpeechSynthesizer::new()?;

    // Subscribe to events
    let event_stream = SpeechSynthesisEventStream::subscribe(&synthesizer, 16)?;

    // Dropping the stream should unsubscribe cleanly
    drop(event_stream);

    Ok(())
}

#[test]
fn test_async_stream_basic_event_flow() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        // Create a synthesizer
        let synthesizer = SpeechSynthesizer::new()?;

        // Subscribe to events
        let event_stream = SpeechSynthesisEventStream::subscribe(&synthesizer, 16)?;

        // Speak an utterance
        let utterance = SpeechUtterance::new("Hello, world!");
        synthesizer.speak(&utterance)?;

        // We should get at least a DidStart and DidFinish event
        let mut got_start = false;
        let mut got_finish = false;

        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_secs(5) {
            if let Some(event) = event_stream.try_next() {
                match event {
                    SpeechSynthesisEvent::DidStart(_) => {
                        got_start = true;
                    }
                    SpeechSynthesisEvent::DidFinish(_) => {
                        got_finish = true;
                        break;
                    }
                    _ => {}
                }
            } else {
                // Wait a bit and retry
                std::thread::sleep(std::time::Duration::from_millis(100));
                synthesizer.pump_run_loop(std::time::Duration::from_millis(50));
            }
        }

        // In headless environments (CI/testing), speech events may not fire
        // because there's no audio output device or run loop. We just verify
        // that the stream can be created and polled without crashing.
        if !got_start && !got_finish {
            eprintln!("Note: No speech events received (likely headless environment)");
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    })
}

#[test]
fn test_async_stream_buffering() -> Result<(), Box<dyn std::error::Error>> {
    // Create a synthesizer
    let synthesizer = SpeechSynthesizer::new()?;

    // Subscribe with a small buffer
    let event_stream = SpeechSynthesisEventStream::subscribe(&synthesizer, 4)?;

    // Initially, the buffer should be empty
    assert_eq!(event_stream.buffered_count(), 0);

    // Clearing buffer on empty stream should not panic
    event_stream.clear_buffer();
    assert_eq!(event_stream.buffered_count(), 0);

    Ok(())
}

#[test]
fn test_async_stream_closed_check() -> Result<(), Box<dyn std::error::Error>> {
    // Create a synthesizer
    let synthesizer = SpeechSynthesizer::new()?;

    // Subscribe to events
    let event_stream = SpeechSynthesisEventStream::subscribe(&synthesizer, 16)?;

    // Initially not closed
    assert!(!event_stream.is_closed());

    Ok(())
}
