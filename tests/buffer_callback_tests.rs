use std::path::PathBuf;
use std::process::Command;

#[test]
fn buffer_callbacks_receive_audio_and_end_of_stream() -> Result<(), Box<dyn std::error::Error>> {
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("examples")
        .join("04_buffer_callback");
    let status = Command::new(example).status()?;
    assert!(status.success());
    Ok(())
}
