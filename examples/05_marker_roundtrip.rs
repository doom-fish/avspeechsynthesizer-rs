use avspeechsynthesizer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let word = SpeechSynthesisMarker::word(TextRange::new(0, 4), 128)?;
    let sentence = SpeechSynthesisMarker::sentence(TextRange::new(0, 9), 256)?;
    let paragraph = SpeechSynthesisMarker::paragraph(TextRange::new(0, 18), 512)?;
    let phoneme = SpeechSynthesisMarker::phoneme("fɪʃ", 768)?;
    let bookmark = SpeechSynthesisMarker::bookmark("chapter-1", 1024)?;

    println!("word={word:?}");
    println!("sentence={sentence:?}");
    println!("paragraph={paragraph:?}");
    println!("phoneme={phoneme:?}");
    println!("bookmark={bookmark:?}");
    Ok(())
}
