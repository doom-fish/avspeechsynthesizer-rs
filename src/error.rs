use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents an AVSpeechSynthesis error surfaced by the Rust wrapper.
pub enum AvSpeechError {
    /// Represents an invalid AVSpeechSynthesis argument.
    InvalidArgument(String),
    /// Represents an AVSpeechSynthesis API that is unavailable on this macOS version.
    UnavailableOnThisMacOS(String),
    /// Represents an AVSpeechSynthesis operation that timed out.
    TimedOut(String),
    /// Represents an I/O failure while using AVSpeechSynthesis APIs.
    Io(String),
    /// Represents an AVSpeechSynthesis framework-reported failure.
    Framework(String),
    /// Represents an unknown AVSpeechSynthesis failure.
    Unknown(String),
}

impl fmt::Display for AvSpeechError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message)
            | Self::UnavailableOnThisMacOS(message)
            | Self::TimedOut(message)
            | Self::Io(message)
            | Self::Framework(message)
            | Self::Unknown(message) => f.write_str(message),
        }
    }
}

impl Error for AvSpeechError {}
