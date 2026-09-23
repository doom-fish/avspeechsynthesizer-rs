use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let synthesizer = SpeechSynthesizer::new()?;
    let utterance = SpeechUtterance::new("Offline write check.").with_volume(0.0);

    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("offline-write.caf");
    let written = synthesizer.write_utterance_to_file(&utterance, &path)?;
    assert_eq!(written.path(), path.as_path());
    let length = std::fs::metadata(&path)?.len();
    assert!(length > 1024, "offline write produced only {length} bytes");
    std::fs::remove_file(&path)?;

    let buffers = Arc::new(AtomicUsize::new(0));
    let audio_bytes = Arc::new(AtomicUsize::new(0));
    let saw_end = Arc::new(AtomicBool::new(false));
    let (buffer_count, byte_count, end) = (
        Arc::clone(&buffers),
        Arc::clone(&audio_bytes),
        Arc::clone(&saw_end),
    );
    synthesizer.write_utterance_with_callbacks(
        &utterance,
        move |buffer| {
            buffer_count.fetch_add(1, Ordering::SeqCst);
            byte_count.fetch_add(buffer.total_bytes(), Ordering::SeqCst);
            if buffer.is_end_of_stream() {
                end.store(true, Ordering::SeqCst);
            }
        },
        |_markers| {},
    )?;
    assert!(buffers.load(Ordering::SeqCst) > 1);
    assert!(audio_bytes.load(Ordering::SeqCst) > 0);
    assert!(saw_end.load(Ordering::SeqCst));

    println!("offline write and buffer collection completed");
    Ok(())
}
