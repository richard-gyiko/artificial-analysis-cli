//! API response wrappers.

use serde::{Deserialize, Serialize};

/// Generic API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub data: T,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Error response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default)]
    pub status: Option<u16>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

impl ErrorResponse {
    /// Get the error message.
    pub fn message(&self) -> &str {
        self.message
            .as_deref()
            .or(self.error.as_deref())
            .unwrap_or("Unknown error")
    }
}
