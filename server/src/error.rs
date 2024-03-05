#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error("No model available")]
    NoModelAvailableError,
}

pub type Result<T> = std::result::Result<T, Error>;
