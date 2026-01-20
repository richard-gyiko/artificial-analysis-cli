//! Unified API client for fetching and merging data from multiple sources.

use crate::cache::{Cache, QuotaInfo};
use crate::error::Result;
use crate::merge::merge_models;
use crate::models::{LlmModel, MediaModel};
use crate::parquet;
use crate::sources::artificial_analysis::models::{AaLlmModel, AaLlmRow};
use crate::sources::artificial_analysis::AaClient;
use crate::sources::models_dev::models::{flatten_response, ModelsDevResponse, ModelsDevRow};
use crate::sources::models_dev::ModelsDevClient;
use chrono::{Duration, Utc};
use duckdb::Connection;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Unified client that fetches from both AA and models.dev.
pub struct Client {
    aa_client: AaClient,
    md_client: ModelsDevClient,
    cache: Cache,
}

/// TTL for models.dev cache (24 hours).
const MODELS_DEV_TTL_HOURS: i64 = 24;

impl Client {
    /// Create a new unified client.
    pub fn new(api_key: String, profile_name: String) -> Result<Self> {
        let aa_client = AaClient::new(api_key, profile_name)?;
        let md_client = ModelsDevClient::new()?;
        let cache = Cache::new()?;

        Ok(Self {
            aa_client,
            md_client,
            cache,
        })
    }

    /// Get the cache instance.
    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// Get cached quota info.
    pub fn get_cached_quota(&self) -> Option<QuotaInfo> {
        self.aa_client.get_cached_quota()
    }

    // ========== LLM Data (Three-Layer Cache) ==========

    /// Fetch merged LLM models.
    ///
    /// This implements the three-layer cache architecture:
    /// 1. Check if merged cache is valid (both sources cached and not expired)
    /// 2. Fetch/use cached AA data
    /// 3. Fetch/use cached models.dev data (with 24h TTL)
    /// 4. Merge and cache the result
    pub async fn get_llm_models(
        &self,
        refresh: bool,
    ) -> Result<(Vec<LlmModel>, Option<QuotaInfo>)> {
        let merged_path = self.cache.parquet_path("llms");
        let aa_path = self.cache.parquet_path("aa_llms");
        let md_path = self.cache.parquet_path("models_dev");
        let meta_path = self.models_dev_meta_path();

        // Quick path: if not refreshing and all caches exist and models.dev not expired,
        // we can skip fetching entirely and just load merged cache
        if !refresh
            && merged_path.exists()
            && aa_path.exists()
            && md_path.exists()
            && !self.is_models_dev_expired(&meta_path)
        {
            // Load from merged parquet cache
            if let Ok(models) = self.load_merged_cache(&merged_path) {
                return Ok((models, self.get_cached_quota()));
            }
        }

        // Fetch AA data
        let (aa_models, quota) = self.fetch_aa_llms(refresh).await?;
        let aa_changed = refresh || !aa_path.exists();

        // Fetch models.dev data (with TTL check)
        let md_providers = self.fetch_models_dev_if_needed().await;
        let md_changed = self.is_models_dev_changed(&md_path);

        // Merge data
        let merged = match &md_providers {
            Ok(providers) => merge_models(&aa_models, providers),
            Err(e) => {
                // Log warning but continue with AA-only data
                eprintln!(
                    "Warning: Could not fetch models.dev data: {}. Using AA data only.",
                    e
                );
                merge_models(&aa_models, &HashMap::new())
            }
        };

        // Write merged parquet if either source changed or merged doesn't exist
        if aa_changed || md_changed || !merged_path.exists() {
            if let Err(e) = parquet::write_llms_parquet(&merged, &merged_path) {
                eprintln!("Warning: Failed to write merged Parquet cache: {}", e);
            }
        }

        Ok((merged, quota))
    }

    /// Load merged LLM models from parquet cache.
    fn load_merged_cache(&self, path: &Path) -> Result<Vec<LlmModel>> {
        let conn = Connection::open_in_memory()
            .map_err(|e| crate::error::AppError::Cache(format!("DuckDB error: {}", e)))?;

        let path_str = path.to_string_lossy();
        // Use explicit column names to be resilient to schema changes
        let sql = format!(
            r#"SELECT
                id, name, slug, creator, creator_slug, release_date,
                intelligence, coding, math, mmlu_pro, gpqa, hle,
                livecodebench, scicode, math_500, aime,
                input_price, output_price, price, tps, latency,
                reasoning, tool_call, structured_output, attachment, temperature,
                context_window, max_input_tokens, max_output_tokens,
                input_modalities, output_modalities,
                knowledge_cutoff, open_weights, last_updated, models_dev_matched
            FROM read_parquet('{}')"#,
            path_str
        );
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| crate::error::AppError::Cache(format!("DuckDB error: {}", e)))?;

        let models: Vec<LlmModel> = stmt
            .query_map([], |row| {
                Ok(LlmModel {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    slug: row.get(2)?,
                    creator: row.get(3)?,
                    creator_slug: row.get(4)?,
                    release_date: row.get(5)?,
                    intelligence: row.get(6)?,
                    coding: row.get(7)?,
                    math: row.get(8)?,
                    mmlu_pro: row.get(9)?,
                    gpqa: row.get(10)?,
                    hle: row.get(11)?,
                    livecodebench: row.get(12)?,
                    scicode: row.get(13)?,
                    math_500: row.get(14)?,
                    aime: row.get(15)?,
                    input_price: row.get(16)?,
                    output_price: row.get(17)?,
                    price: row.get(18)?,
                    tps: row.get(19)?,
                    latency: row.get(20)?,
                    reasoning: row.get(21)?,
                    tool_call: row.get(22)?,
                    structured_output: row.get(23)?,
                    attachment: row.get(24)?,
                    temperature: row.get(25)?,
                    context_window: row.get::<_, Option<i64>>(26)?.map(|v| v as u64),
                    max_input_tokens: row.get::<_, Option<i64>>(27)?.map(|v| v as u64),
                    max_output_tokens: row.get::<_, Option<i64>>(28)?.map(|v| v as u64),
                    input_modalities: row
                        .get::<_, Option<String>>(29)?
                        .map(|s| s.split(',').map(String::from).collect()),
                    output_modalities: row
                        .get::<_, Option<String>>(30)?
                        .map(|s| s.split(',').map(String::from).collect()),
                    knowledge_cutoff: row.get(31)?,
                    open_weights: row.get(32)?,
                    last_updated: row.get(33)?,
                    models_dev_matched: row.get(34)?,
                })
            })
            .map_err(|e| crate::error::AppError::Cache(format!("DuckDB error: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| crate::error::AppError::Cache(format!("DuckDB error: {}", e)))?;

        Ok(models)
    }

    /// Fetch AA LLM models and write to cache.
    async fn fetch_aa_llms(&self, refresh: bool) -> Result<(Vec<AaLlmModel>, Option<QuotaInfo>)> {
        let (models, quota) = self.aa_client.fetch_llm_models(refresh).await?;

        // Write raw AA data to parquet
        let aa_path = self.cache.parquet_path("aa_llms");
        let rows: Vec<AaLlmRow> = models.iter().map(AaLlmRow::from).collect();
        if let Err(e) = parquet::write_aa_llms_parquet(&rows, &aa_path) {
            eprintln!("Warning: Failed to write AA Parquet cache: {}", e);
        }

        Ok((models, quota))
    }

    /// Fetch models.dev data if cache is expired or missing.
    /// Falls back to stale cache on API error.
    async fn fetch_models_dev_if_needed(&self) -> Result<ModelsDevResponse> {
        let md_path = self.cache.parquet_path("models_dev");
        let meta_path = self.models_dev_meta_path();

        // Check if cache is valid (not expired)
        if md_path.exists() && !self.is_models_dev_expired(&meta_path) {
            // Try to load from cached response (JSON)
            if let Some(cached) = self.load_models_dev_cache() {
                return Ok(cached);
            }
        }

        // Try to fetch fresh data
        match self.md_client.fetch().await {
            Ok(response) => {
                // Write raw parquet
                let rows: Vec<ModelsDevRow> = flatten_response(&response);
                if let Err(e) = parquet::write_models_dev_parquet(&rows, &md_path) {
                    eprintln!("Warning: Failed to write models.dev Parquet cache: {}", e);
                }

                // Save JSON cache for quick reload
                self.save_models_dev_cache(&response);

                // Update metadata timestamp
                self.update_models_dev_meta(&meta_path);

                Ok(response)
            }
            Err(e) => {
                // API failed - try to use stale cache as fallback
                if let Some(cached) = self.load_models_dev_cache() {
                    eprintln!("Warning: models.dev API failed ({}), using stale cache.", e);
                    return Ok(cached);
                }
                // No stale cache available, propagate the error
                Err(e)
            }
        }
    }

    /// Check if models.dev cache is expired.
    fn is_models_dev_expired(&self, meta_path: &PathBuf) -> bool {
        if !meta_path.exists() {
            return true;
        }

        if let Ok(content) = std::fs::read_to_string(meta_path) {
            if let Ok(timestamp) = content.trim().parse::<i64>() {
                let cached_at = chrono::DateTime::from_timestamp(timestamp, 0);
                if let Some(cached_at) = cached_at {
                    let age = Utc::now().signed_duration_since(cached_at);
                    return age > Duration::hours(MODELS_DEV_TTL_HOURS);
                }
            }
        }
        true
    }

    /// Check if models.dev was recently updated.
    fn is_models_dev_changed(&self, md_path: &Path) -> bool {
        // Simple check: if the file was modified in this session
        // For now, just check if it exists
        !md_path.exists()
    }

    /// Get path to models.dev metadata file.
    fn models_dev_meta_path(&self) -> PathBuf {
        self.cache.base_dir().join("models_dev_meta.txt")
    }

    /// Update models.dev metadata timestamp.
    fn update_models_dev_meta(&self, meta_path: &Path) {
        let timestamp = Utc::now().timestamp().to_string();
        let _ = std::fs::write(meta_path, timestamp);
    }

    /// Load models.dev response from JSON cache.
    fn load_models_dev_cache(&self) -> Option<ModelsDevResponse> {
        let cache_path = self.cache.base_dir().join("models_dev_cache.json");
        let content = std::fs::read_to_string(cache_path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Save models.dev response to JSON cache.
    fn save_models_dev_cache(&self, response: &ModelsDevResponse) {
        let cache_path = self.cache.base_dir().join("models_dev_cache.json");
        if let Ok(json) = serde_json::to_string(response) {
            let _ = std::fs::write(cache_path, json);
        }
    }

    // ========== Media Models (AA-only) ==========

    /// Fetch text-to-image models.
    pub async fn get_text_to_image(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        let (models, quota) = self.aa_client.fetch_text_to_image(refresh).await?;
        let parquet_path = self.cache.parquet_path("text_to_image");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok((models, quota))
    }

    /// Fetch image-editing models.
    pub async fn get_image_editing(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        let (models, quota) = self.aa_client.fetch_image_editing(refresh).await?;
        let parquet_path = self.cache.parquet_path("image_editing");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok((models, quota))
    }

    /// Fetch text-to-speech models.
    pub async fn get_text_to_speech(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        let (models, quota) = self.aa_client.fetch_text_to_speech(refresh).await?;
        let parquet_path = self.cache.parquet_path("text_to_speech");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok((models, quota))
    }

    /// Fetch text-to-video models.
    pub async fn get_text_to_video(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        let (models, quota) = self.aa_client.fetch_text_to_video(refresh).await?;
        let parquet_path = self.cache.parquet_path("text_to_video");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok((models, quota))
    }

    /// Fetch image-to-video models.
    pub async fn get_image_to_video(
        &self,
        refresh: bool,
    ) -> Result<(Vec<MediaModel>, Option<QuotaInfo>)> {
        let (models, quota) = self.aa_client.fetch_image_to_video(refresh).await?;
        let parquet_path = self.cache.parquet_path("image_to_video");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok((models, quota))
    }
}
