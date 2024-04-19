#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("generic error: {0}")]
    Generic(String),

    #[error("builder failed: {0}")]
    BuilderError(#[from] derive_builder::UninitializedFieldError),

    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("failed to parse url: {0}")]
    UrlParseError(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
