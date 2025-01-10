use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Whisper error: {0}")]
    Whisper(String),

    #[error("Codec error: {0}")]
    Codec(String),

    #[error("Audio processing error: {0}")]
    AudioProcesing(#[from] hound::Error),
}

impl From<whisper_rs::WhisperError> for Error {
    fn from(err: whisper_rs::WhisperError) -> Self {
        Error::Whisper(err.to_string())
    }
}
