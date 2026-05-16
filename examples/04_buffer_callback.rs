use std::sync::{Arc, Mutex};

use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let synthesizer = SpeechSynthesizer::new()?;
    let utterance = SpeechUtterance::new("Buffer callback smoke");
    let total_frames = Arc::new(Mutex::new(0usize));
    let saw_end = Arc::new(Mutex::new(false));

    let total_frames_clone = Arc::clone(&total_frames);
    let saw_end_clone = Arc::clone(&saw_end);
    synthesizer.write_utterance_with_buffer_callback(&utterance, move |buffer| {
        if let Ok(mut total) = total_frames_clone.lock() {
            *total += buffer.frame_length();
        }
        if buffer.is_end_of_stream() {
            if let Ok(mut saw_end) = saw_end_clone.lock() {
                *saw_end = true;
            }
        }
    })?;

    let total_frames = *total_frames.lock().map_err(|_| "frames mutex poisoned")?;
    let saw_end = *saw_end.lock().map_err(|_| "end mutex poisoned")?;
    println!("buffer callback frames={total_frames} saw_end={saw_end}");
    assert!(saw_end, "expected an end-of-stream buffer");
    Ok(())
}
