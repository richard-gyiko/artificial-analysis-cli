//! API client for Artificial Analysis.

use super::models::AaLlmModel;
use crate::cache::Cache;
use crate::error::{AppError, Result};
use crate::models::{ApiResponse, MediaModel};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Base URL for the Artificial Analysis API.
pub const API_BASE: &str = "https://artificialanalysis.ai/api/v2";

/// Endpoint paths.
pub const LLM_MODELS: &str = "/data/llms/models";
pub const TEXT_TO_IMAGE: &str = "/data/media/text-to-image";
pub const IMAGE_EDITING: &str = "/data/media/image-editing";
pub const TEXT_TO_SPEECH: &str = "/data/media/text-to-speech";
pub const TEXT_TO_VIDEO: &str = "/data/media/text-to-video";
pub const IMAGE_TO_VIDEO: &str = "/data/media/image-to-video";

/// Default timeout for requests (30 seconds).
const REQUEST_TIMEOUT_SECS: u64 = 30;

/// Default connect timeout (10 seconds).
const CONNECT_TIMEOUT_SECS: u64 = 10;

/// API client for Artificial Analysis.
pub struct AaClient {
    http: reqwest::Client,
    #[allow(dead_code)]
    api_key: String,
    cache: Cache,
}

impl AaClient {
    /// Create a new API client.
    pub fn new(api_key: String, _profile_name: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_str(&api_key).map_err(|e| AppError::Config(e.to_string()))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(format!("aa-cli/{}", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
            .build()?;

        let cache = Cache::new()?;

        Ok(Self {
            http,
            api_key,
            cache,
        })
    }

    /// Make an API request, using cache if available.
    async fn request<T: DeserializeOwned + serde::Serialize + Clone>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
        refresh: bool,
    ) -> Result<T> {
        let cache_key = Cache::cache_key(endpoint, params);

        // Check cache unless refresh requested
        if !refresh {
            if let Some(cached) = self.cache.get::<T>(&cache_key) {
                return Ok(cached);
            }
        }

        // Make the request
        let url = format!("{}{}", API_BASE, endpoint);
        let response = self.http.get(&url).query(params).send().await?;

        // Handle response status
        let status = response.status();
        if status == 401 {
            return Err(AppError::InvalidApiKey);
        }
        if status == 429 {
            return Err(AppError::RateLimited("unknown".into()));
        }
        if status.is_server_error() {
            return Err(AppError::ServerError);
        }
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Api {
                status: status.as_u16(),
                message: body,
            });
        }

        let data: T = response.json().await?;

        // Cache the response
        let _ = self.cache.set(&cache_key, &data);

        Ok(data)
    }

    /// Fetch raw LLM models from AA API.
    pub async fn fetch_llm_models(&self, refresh: bool) -> Result<Vec<AaLlmModel>> {
        let response: ApiResponse<Vec<AaLlmModel>> = self.request(LLM_MODELS, &[], refresh).await?;

        Ok(response.data)
    }

    /// Fetch media models for a given endpoint.
    pub async fn fetch_media_models(
        &self,
        endpoint: &str,
        refresh: bool,
    ) -> Result<Vec<MediaModel>> {
        let response: ApiResponse<Vec<MediaModel>> = self.request(endpoint, &[], refresh).await?;

        Ok(response.data)
    }

    /// Fetch text-to-image models.
    pub async fn fetch_text_to_image(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        self.fetch_media_models(TEXT_TO_IMAGE, refresh).await
    }

    /// Fetch image-editing models.
    pub async fn fetch_image_editing(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        self.fetch_media_models(IMAGE_EDITING, refresh).await
    }

    /// Fetch text-to-speech models.
    pub async fn fetch_text_to_speech(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        self.fetch_media_models(TEXT_TO_SPEECH, refresh).await
    }

    /// Fetch text-to-video models.
    pub async fn fetch_text_to_video(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        self.fetch_media_models(TEXT_TO_VIDEO, refresh).await
    }

    /// Fetch image-to-video models.
    pub async fn fetch_image_to_video(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        self.fetch_media_models(IMAGE_TO_VIDEO, refresh).await
    }

    /// Get the cache instance.
    pub fn cache(&self) -> &Cache {
        &self.cache
    }
}
