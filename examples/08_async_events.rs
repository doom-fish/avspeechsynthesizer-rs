// Example demonstrating the async stream API for speech synthesis events

#[cfg(feature = "async")]
use avspeechsynthesizer::async_api::SpeechSynthesisEventStream;
#[cfg(feature = "async")]
use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "async")]
    {
        run_async_example()
    }

    #[cfg(not(feature = "async"))]
    {
        eprintln!("This example requires the 'async' feature");
        eprintln!("Run with: cargo run --example 08_async_events --all-features");
        std::process::exit(1);
    }
}

#[cfg(feature = "async")]
fn run_async_example() -> Result<(), Box<dyn std::error::Error>> {
    use avspeechsynthesizer::async_api::SpeechSynthesisEvent;

    // Create a synthesizer
    let synthesizer = SpeechSynthesizer::new()?;

    // Subscribe to events with a buffer capacity of 16
    let mut event_stream = SpeechSynthesisEventStream::subscribe(&synthesizer, 16)?;

    // Create and speak an utterance
    let utterance = SpeechUtterance::new("Hello, this is a test of the async event stream.");
    println!("Synthesizer speaking: {}", utterance.speech_string());
    synthesizer.speak(&utterance)?;

    // Process events asynchronously
    println!("Listening for events:");
    pollster::block_on(async {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(5);

        loop {
            // Check timeout
            if start.elapsed() > timeout {
                eprintln!("Timeout waiting for speech to finish");
                break;
            }

            if let Some(event) = event_stream.next().await {
                match event {
                    SpeechSynthesisEvent::DidStart(utterance) => {
                        println!("  → Started speaking: '{}'", utterance.speech_string());
                    }
                    SpeechSynthesisEvent::WillSpeakRange {
                        character_range,
                        utterance: _,
                    } => {
                        println!(
                            "  → About to speak range: location={}, length={}",
                            character_range.location, character_range.length
                        );
                    }
                    SpeechSynthesisEvent::DidFinish(utterance) => {
                        println!("  → Finished speaking: '{}'", utterance.speech_string());
                        break; // Exit loop on finish
                    }
                    SpeechSynthesisEvent::DidPause(_) => {
                        println!("  → Paused");
                    }
                    SpeechSynthesisEvent::DidContinue(_) => {
                        println!("  → Continued");
                    }
                    SpeechSynthesisEvent::DidCancel(_) => {
                        println!("  → Cancelled");
                        break;
                    }
                    SpeechSynthesisEvent::WillSpeakMarker {
                        marker: _,
                        utterance: _,
                    } => {
                        println!("  → About to speak marker");
                    }
                }
            }
        }
    });

    println!("\nEvent stream closed. Buffered events: {}", event_stream.buffered_count());
    Ok(())
}

