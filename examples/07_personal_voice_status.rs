use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = personal_voice_authorization_status()?;
    let voices = available_personal_voices()?;
    println!("personal voice status={status:?} voices={}", voices.len());
    Ok(())
}
