use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AvSpeechError {
    InvalidArgument(String),
    UnavailableOnThisMacOS(String),
    TimedOut(String),
    Io(String),
    Framework(String),
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
