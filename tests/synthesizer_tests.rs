use std::path::PathBuf;
use std::process::Command;

use avspeechsynthesizer::prelude::*;

#[test]
fn synthesizer_emits_lifecycle_events() -> Result<(), Box<dyn std::error::Error>> {
    let notification = SpeechSynthesizer::available_voices_did_change_notification_name()?;
    assert!(!notification.is_empty());

    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("examples")
        .join("02_framework_smoke");
    let status = Command::new(example).status()?;
    assert!(status.success());
    Ok(())
}
