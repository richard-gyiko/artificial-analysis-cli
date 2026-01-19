//! API client for models.dev.

use super::models::{flatten_response, ModelsDevResponse, ModelsDevRow};
use crate::error::{AppError, Result};

/// Endpoint for models.dev API.
pub const MODELS_DEV_API: &str = "https://models.dev/api.json";

/// Client for fetching data from models.dev.
pub struct ModelsDevClient {
    http: reqwest::Client,
}

impl ModelsDevClient {
    /// Create a new models.dev client.
    pub fn new() -> Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent(format!("aa-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self { http })
    }

    /// Fetch all model data from models.dev.
    pub async fn fetch(&self) -> Result<ModelsDevResponse> {
        let response = self.http.get(MODELS_DEV_API).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Api {
                status: status.as_u16(),
                message: format!("models.dev API error: {}", body),
            });
        }

        let data: ModelsDevResponse = response.json().await?;
        Ok(data)
    }

    /// Fetch and flatten model data into rows for storage.
    pub async fn fetch_rows(&self) -> Result<Vec<ModelsDevRow>> {
        let response = self.fetch().await?;
        Ok(flatten_response(&response))
    }
}

impl Default for ModelsDevClient {
    fn default() -> Self {
        Self::new().expect("Failed to create ModelsDevClient")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ModelsDevClient::new();
        assert!(client.is_ok());
    }
}
