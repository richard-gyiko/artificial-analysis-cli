//! API client for Artificial Analysis.

pub mod endpoints;

use crate::cache::{Cache, QuotaInfo};
use crate::error::{AppError, Result};
use crate::models::{ApiResponse, LlmModel, MediaModel};
use crate::parquet;
use chrono::Utc;
use endpoints::{
    API_BASE, IMAGE_EDITING, IMAGE_TO_VIDEO, LLM_MODELS, TEXT_TO_IMAGE, TEXT_TO_SPEECH,
    TEXT_TO_VIDEO,
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;

/// API client.
pub struct Client {
    http: reqwest::Client,
    #[allow(dead_code)]
    api_key: String,
    cache: Cache,
    profile_name: String,
}

impl Client {
    /// Create a new API client.
    pub fn new(api_key: String, profile_name: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_str(&api_key).map_err(|e| AppError::Config(e.to_string()))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(format!("aa-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        let cache = Cache::new()?;

        Ok(Self {
            http,
            api_key,
            cache,
            profile_name,
        })
    }

    /// Make an API request, using cache if available.
    async fn request<T: DeserializeOwned + serde::Serialize + Clone>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
        refresh: bool,
    ) -> Result<(T, Option<QuotaInfo>)> {
        let cache_key = Cache::cache_key(endpoint, params);

        // Check cache unless refresh requested
        if !refresh {
            if let Some(cached) = self.cache.get::<T>(&cache_key) {
                return Ok((cached, None));
            }
        }

        // Make the request
        let url = format!("{}{}", API_BASE, endpoint);
        let response = self.http.get(&url).query(params).send().await?;

        // Extract quota info from headers
        let quota = self.extract_quota(&response);

        // Handle response status
        let status = response.status();
        if status == 401 {
            return Err(AppError::InvalidApiKey);
        }
        if status == 429 {
            let reset = quota
                .as_ref()
                .map(|q| q.reset.clone())
                .unwrap_or_else(|| "unknown".into());
            return Err(AppError::RateLimited(reset));
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

        // Update quota cache
        if let Some(ref q) = quota {
            let _ = self.cache.set_quota(&self.profile_name, q);
        }

        Ok((data, quota))
    }

    /// Extract quota information from response headers.
    fn extract_quota(&self, response: &reqwest::Response) -> Option<QuotaInfo> {
        let headers = response.headers();

        let limit = headers
            .get("X-RateLimit-Limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())?;

        let remaining = headers
            .get("X-RateLimit-Remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())?;

        let reset = headers
            .get("X-RateLimit-Reset")
            .and_then(|v| v.to_str().ok())
            .map(String::from)
            .unwrap_or_else(|| "unknown".into());

        Some(QuotaInfo {
            limit,
            remaining,
            reset,
            updated_at: Utc::now(),
        })
    }

    /// Fetch LLM models.
    pub async fn get_llm_models(
        &self,
        refresh: bool,
    ) -> Result<(Vec<LlmModel>, Option<QuotaInfo>)> {
        let (response, quota): (ApiResponse<Vec<LlmModel>>, _) =
            self.request(LLM_MODELS, &[], refresh).await?;

        // Write Parquet file for SQL queries
        let parquet_path = self.cache.parquet_path("llms");
        if let Err(e) = parquet::write_llms_parquet(&response.data, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }

        Ok((response.data, quota))
    }

    /// Fetch media models for a given endpoint.
    async fn get_media_models(
        &self,
        endpoint: &str,
        parquet_name: &str,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        let (response, quota): (ApiResponse<Vec<MediaModel>>, _) =
            self.request(endpoint, &[], refresh).await?;

        // Write Parquet file for SQL queries
        let parquet_path = self.cache.parquet_path(parquet_name);
        if let Err(e) = parquet::write_media_parquet(&response.data, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }

        Ok((response.data, quota))
    }

    /// Fetch text-to-image models.
    pub async fn get_text_to_image(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        self.get_media_models(TEXT_TO_IMAGE, "text_to_image", refresh)
            .await
    }

    /// Fetch image-editing models.
    pub async fn get_image_editing(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        self.get_media_models(IMAGE_EDITING, "image_editing", refresh)
            .await
    }

    /// Fetch text-to-speech models.
    pub async fn get_text_to_speech(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        self.get_media_models(TEXT_TO_SPEECH, "text_to_speech", refresh)
            .await
    }

    /// Fetch text-to-video models.
    pub async fn get_text_to_video(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        self.get_media_models(TEXT_TO_VIDEO, "text_to_video", refresh)
            .await
    }

    /// Fetch image-to-video models.
    pub async fn get_image_to_video(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        self.get_media_models(IMAGE_TO_VIDEO, "image_to_video", refresh)
            .await
    }

    /// Get cached quota info.
    pub fn get_cached_quota(&self) -> Option<QuotaInfo> {
        self.cache.get_quota(&self.profile_name)
    }

    /// Get the cache instance.
    pub fn cache(&self) -> &Cache {
        &self.cache
    }
}
