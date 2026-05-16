use avspeechsynthesizer::prelude::*;

#[test]
fn marker_constructors_round_trip_through_swift() -> Result<(), Box<dyn std::error::Error>> {
    let word = SpeechSynthesisMarker::word(TextRange::new(0, 4), 128)?;
    let sentence = SpeechSynthesisMarker::sentence(TextRange::new(0, 9), 256)?;
    let paragraph = SpeechSynthesisMarker::paragraph(TextRange::new(0, 18), 512)?;
    let phoneme = SpeechSynthesisMarker::phoneme("fɪʃ", 768)?;
    let bookmark = SpeechSynthesisMarker::bookmark("chapter-1", 1024)?;

    assert_eq!(word.mark(), SpeechSynthesisMarkerMark::Word);
    assert_eq!(sentence.mark(), SpeechSynthesisMarkerMark::Sentence);
    assert_eq!(paragraph.mark(), SpeechSynthesisMarkerMark::Paragraph);
    assert_eq!(phoneme.mark(), SpeechSynthesisMarkerMark::Phoneme);
    assert_eq!(bookmark.mark(), SpeechSynthesisMarkerMark::Bookmark);
    Ok(())
}
