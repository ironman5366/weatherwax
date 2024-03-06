#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error("No model available")]
    NoModelAvailableError,
    #[error("Model {0} not found")]
    ModelNotFoundError(String),

    #[cfg(feature = "openai")]
    #[error("OpenAI API conversion error: {}")]
    OpenAIConversionError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
