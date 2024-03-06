use axum::response::IntoResponse;

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
    #[error("OpenAI API conversion error: {0}")]
    OpenAIConversionError(String),

    #[cfg(feature = "openai")]
    #[error(transparent)]
    OpenAIError(#[from] async_openai::error::OpenAIError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let (status_code, error_message) = match self {
            Error::IOError(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            Error::AxumError(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            Error::NoModelAvailableError => (axum::http::StatusCode::NOT_FOUND, "No model available"),
            Error::ModelNotFoundError(_) => (axum::http::StatusCode::NOT_FOUND, "Model not found"),
            #[cfg(feature = "openai")]
            Error::OpenAIConversionError(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            #[cfg(feature = "openai")]
            Error::OpenAIError(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };

        axum::http::Response::builder()
            .status(status_code)
            .body(axum::body::Body::from(error_message))
            .unwrap()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
