//! Error types for the CLI.

use thiserror::Error;

/// Application-level errors.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid API key. Run 'which-llm profile create' to configure.")]
    InvalidApiKey,

    #[error("Rate limit exceeded. Free tier allows 1,000 requests/day. Reset: {0}")]
    RateLimited(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Artificial Analysis API error. Please try again later.")]
    ServerError,

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Profile '{0}' not found")]
    ProfileNotFound(String),

    #[error(
        "No API key configured. Set ARTIFICIAL_ANALYSIS_API_KEY environment variable or run 'which-llm profile create'."
    )]
    NoApiKey,

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Query(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
