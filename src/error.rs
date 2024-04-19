#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("generic error: {0}")]
    Generic(String),

    #[error("builder failed: {0}")]
    BuilderError(#[from] derive_builder::UninitializedFieldError),

    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("impersonate network error: {0}")]
    ImpersonateNetworkError(#[from] reqwest_impersonate::Error),

    #[error("failed to parse url: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
