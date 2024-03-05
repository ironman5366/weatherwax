#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
