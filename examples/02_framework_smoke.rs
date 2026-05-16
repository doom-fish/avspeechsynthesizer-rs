#![allow(clippy::too_many_lines)]

use std::sync::mpsc;
use std::time::{Duration, Instant};

use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== AVSpeechSynthesizer.framework smoke ==");
    println!(
        "voices notification name: {}",
        SpeechSynthesizer::available_voices_did_change_notification_name()?
    );

    let current_language = SpeechSynthesisVoice::current_language_code()?;
    println!("current language: {current_language}");

    let en_us_voices = SpeechSynthesisVoice::voices_with_language("en-US")?;
    println!("en-US voices: {}", en_us_voices.len());
    for voice in en_us_voices.iter().take(5) {
        println!(
            "voice: {} [{}] id={} quality={:?} traits={:?}",
            voice.name(),
            voice.language(),
            voice.identifier(),
            voice.quality(),
            voice.voice_traits(),
        );
    }

    let default_voice = SpeechSynthesisVoice::default_voice()?;
    println!(
        "default voice: {}",
        default_voice.as_ref().map_or_else(
            || "<system default>".to_owned(),
            |voice| format!("{} ({})", voice.name(), voice.identifier()),
        ),
    );

    let mut synthesizer = SpeechSynthesizer::new()?;
    let (tx, rx) = mpsc::channel();
    synthesizer.set_event_handler(move |event| {
        println!("event: {event:?}");
        let _ = tx.send(event);
    });

    let utterance = default_voice.map_or_else(
        || SpeechUtterance::new("Hello from doom-fish"),
        |voice| SpeechUtterance::new("Hello from doom-fish").with_voice(voice),
    );

    synthesizer.speak(&utterance)?;

    let deadline = Instant::now() + Duration::from_secs(20);
    let mut saw_start = false;
    let mut saw_finish = false;
    while Instant::now() < deadline {
        synthesizer.pump_run_loop(Duration::from_millis(100));
        while let Ok(event) = rx.try_recv() {
            match event {
                SpeechEvent::DidStart(_) => saw_start = true,
                SpeechEvent::DidFinish(_) => {
                    saw_finish = true;
                    break;
                }
                _ => {}
            }
        }
        if saw_finish {
            break;
        }
        if saw_start && !synthesizer.is_speaking() {
            break;
        }
    }

    assert!(saw_start, "expected DidStart callback");
    assert!(
        saw_finish || !synthesizer.is_speaking(),
        "expected synthesized speech to finish cleanly"
    );

    println!("OK framework smoke finished successfully");
    Ok(())
}
