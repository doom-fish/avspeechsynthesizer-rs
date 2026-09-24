#![cfg(feature = "async")]

use avspeechsynthesizer::async_api::SpeechSynthesisEventStream;
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
