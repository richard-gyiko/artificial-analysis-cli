//! Unified API client for fetching data from multiple sources.
//!
//! This module provides two client types:
//! - `HostedDataClient`: Uses pre-built data from GitHub Releases (no API key needed)
//! - `Client`: Uses direct API access (requires API key)
//!
//! Architecture: Two independent tables with no pre-computed merge.
//! - `benchmarks`: Pure Artificial Analysis data (benchmarks, performance, pricing)
//! - `models`: Pure models.dev data (capabilities, limits, provider info)

use crate::cache::Cache;
use crate::error::{AppError, Result};
use crate::models::{LlmModel, MediaModel};
use crate::parquet;
use crate::remote::RemoteDataClient;
use crate::sources::artificial_analysis::models::{AaLlmModel, AaLlmRow};
use crate::sources::artificial_analysis::AaClient;
use crate::sources::models_dev::models::{flatten_response, ModelsDevRow};
use crate::sources::models_dev::ModelsDevClient;
use duckdb::Connection;
use std::path::Path;

/// Client for fetching data using hosted GitHub Releases.
/// This is the default mode that requires no API key.
pub struct HostedDataClient {
    remote: RemoteDataClient,
    cache: Cache,
}

impl HostedDataClient {
    /// Create a new hosted data client.
    pub fn new() -> Result<Self> {
        let cache = Cache::new()?;
        let remote = RemoteDataClient::new(cache.base_dir().to_path_buf())?;
        Ok(Self { remote, cache })
    }

    /// Get the cache instance.
    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    /// Get the remote client for manifest access.
    pub fn remote(&self) -> &RemoteDataClient {
        &self.remote
    }

    /// Fetch LLM models from hosted data.
    pub async fn get_llm_models(&self, refresh: bool) -> Result<Vec<LlmModel>> {
        // Ensure we have the data
        let parquet_path = self.remote.ensure_parquet("benchmarks", refresh).await?;

        // Load from parquet
        self.load_llms_from_parquet(&parquet_path)
    }

    /// Load LLM models from a parquet file.
    fn load_llms_from_parquet(&self, path: &Path) -> Result<Vec<LlmModel>> {
        let conn = Connection::open_in_memory()
            .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

        let path_str = path.to_string_lossy();
        let sql = format!(
            r#"SELECT
                id, name, slug, creator, creator_slug, release_date,
                intelligence, coding, math, mmlu_pro, gpqa, hle,
                livecodebench, scicode, math_500, aime,
                input_price, output_price, price, tps, latency
            FROM read_parquet('{}')"#,
            path_str
        );
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

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
                })
            })
            .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

        Ok(models)
    }

    /// Load media models from a parquet file.
    fn load_media_from_parquet(&self, path: &Path) -> Result<Vec<MediaModel>> {
        use crate::models::media::MediaCreator;

        let conn = Connection::open_in_memory()
            .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

        let path_str = path.to_string_lossy();
        let sql = format!(
            r#"SELECT
                id, name, slug, creator,
                elo, rank, release_date
            FROM read_parquet('{}')"#,
            path_str
        );
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

        let models: Vec<MediaModel> = stmt
            .query_map([], |row| {
                let creator_name: String = row.get(3)?;
                Ok(MediaModel {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    slug: row.get(2)?,
                    model_creator: MediaCreator {
                        id: String::new(),
                        name: creator_name,
                        slug: None,
                        extra: serde_json::Value::Null,
                    },
                    elo: row.get(4)?,
                    rank: row.get::<_, Option<i32>>(5)?.map(|r| r as u32),
                    ci95: None,
                    appearances: None,
                    release_date: row.get(6)?,
                    categories: None,
                    extra: serde_json::Value::Null,
                })
            })
            .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

        Ok(models)
    }

    /// Fetch text-to-image models from hosted data.
    pub async fn get_text_to_image(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let parquet_path = self.remote.ensure_parquet("text_to_image", refresh).await?;
        self.load_media_from_parquet(&parquet_path)
    }

    /// Fetch image-editing models from hosted data.
    pub async fn get_image_editing(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let parquet_path = self.remote.ensure_parquet("image_editing", refresh).await?;
        self.load_media_from_parquet(&parquet_path)
    }

    /// Fetch text-to-speech models from hosted data.
    pub async fn get_text_to_speech(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let parquet_path = self
            .remote
            .ensure_parquet("text_to_speech", refresh)
            .await?;
        self.load_media_from_parquet(&parquet_path)
    }

    /// Fetch text-to-video models from hosted data.
    pub async fn get_text_to_video(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let parquet_path = self.remote.ensure_parquet("text_to_video", refresh).await?;
        self.load_media_from_parquet(&parquet_path)
    }

    /// Fetch image-to-video models from hosted data.
    pub async fn get_image_to_video(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let parquet_path = self
            .remote
            .ensure_parquet("image_to_video", refresh)
            .await?;
        self.load_media_from_parquet(&parquet_path)
    }

    /// Refresh models.dev data.
    pub async fn refresh_models(&self) -> Result<()> {
        let _ = self.remote.ensure_parquet("models", true).await?;
        Ok(())
    }
}

/// Unified client that fetches from both AA and models.dev APIs.
/// Requires an API key for Artificial Analysis.
///
/// This client maintains two independent caches:
/// - `benchmarks.parquet`: Artificial Analysis data
/// - `models.parquet`: models.dev data
///
/// No merge is performed - each table is independent.
pub struct Client {
    aa_client: AaClient,
    md_client: ModelsDevClient,
    cache: Cache,
}

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

    // ========== LLM Data (Pure AA) ==========

    /// Fetch LLM models from Artificial Analysis.
    ///
    /// Returns pure AA data without any models.dev merge.
    /// For capability data, query the `models` table separately.
    ///
    /// When `refresh=true`, also refreshes the models.dev data for the `models` table.
    pub async fn get_llm_models(&self, refresh: bool) -> Result<Vec<LlmModel>> {
        let benchmarks_path = self.cache.parquet_path("benchmarks");

        // Quick path: if not refreshing and cache exists, load from cache
        if !refresh && benchmarks_path.exists() {
            if let Ok(models) = self.load_llms_from_parquet(&benchmarks_path) {
                return Ok(models);
            }
        }

        // Fetch AA data
        let aa_models = self.aa_client.fetch_llm_models(refresh).await?;

        // Convert to rows and write to parquet
        let rows: Vec<AaLlmRow> = aa_models.iter().map(AaLlmRow::from).collect();
        if let Err(e) = parquet::write_benchmarks_parquet(&rows, &benchmarks_path) {
            eprintln!("Warning: Failed to write benchmarks Parquet cache: {}", e);
        }

        // Also refresh models.dev data when refreshing
        if refresh {
            if let Err(e) = self.refresh_models().await {
                eprintln!("Warning: Failed to refresh models.dev data: {}", e);
            }
        }

        // Convert to LlmModel for return
        let models: Vec<LlmModel> = aa_models.iter().map(aa_to_llm_model).collect();

        Ok(models)
    }

    /// Load LLM models from parquet cache.
    fn load_llms_from_parquet(&self, path: &Path) -> Result<Vec<LlmModel>> {
        let conn = Connection::open_in_memory()
            .map_err(|e| crate::error::AppError::Cache(format!("DuckDB error: {}", e)))?;

        let path_str = path.to_string_lossy();
        let sql = format!(
            r#"SELECT
                id, name, slug, creator, creator_slug, release_date,
                intelligence, coding, math, mmlu_pro, gpqa, hle,
                livecodebench, scicode, math_500, aime,
                input_price, output_price, price, tps, latency
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
                })
            })
            .map_err(|e| crate::error::AppError::Cache(format!("DuckDB error: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| crate::error::AppError::Cache(format!("DuckDB error: {}", e)))?;

        Ok(models)
    }

    // ========== Models Data (Pure models.dev) ==========

    /// Fetch and cache models data from models.dev.
    ///
    /// This is called during `update` to refresh the models table.
    pub async fn refresh_models(&self) -> Result<()> {
        let models_path = self.cache.parquet_path("models");

        match self.md_client.fetch().await {
            Ok(response) => {
                let rows: Vec<ModelsDevRow> = flatten_response(&response);
                if let Err(e) = parquet::write_models_parquet(&rows, &models_path) {
                    eprintln!("Warning: Failed to write models Parquet cache: {}", e);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Warning: Could not fetch models.dev data: {}", e);
                Err(e)
            }
        }
    }

    // ========== Media Models (AA-only) ==========

    /// Fetch text-to-image models.
    pub async fn get_text_to_image(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let models = self.aa_client.fetch_text_to_image(refresh).await?;
        let parquet_path = self.cache.parquet_path("text_to_image");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok(models)
    }

    /// Fetch image-editing models.
    pub async fn get_image_editing(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let models = self.aa_client.fetch_image_editing(refresh).await?;
        let parquet_path = self.cache.parquet_path("image_editing");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok(models)
    }

    /// Fetch text-to-speech models.
    pub async fn get_text_to_speech(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let models = self.aa_client.fetch_text_to_speech(refresh).await?;
        let parquet_path = self.cache.parquet_path("text_to_speech");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok(models)
    }

    /// Fetch text-to-video models.
    pub async fn get_text_to_video(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let models = self.aa_client.fetch_text_to_video(refresh).await?;
        let parquet_path = self.cache.parquet_path("text_to_video");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok(models)
    }

    /// Fetch image-to-video models.
    pub async fn get_image_to_video(&self, refresh: bool) -> Result<Vec<MediaModel>> {
        let models = self.aa_client.fetch_image_to_video(refresh).await?;
        let parquet_path = self.cache.parquet_path("image_to_video");
        if let Err(e) = parquet::write_media_parquet(&models, &parquet_path) {
            eprintln!("Warning: Failed to write Parquet cache: {}", e);
        }
        Ok(models)
    }
}

/// Convert AA model to simplified LlmModel.
fn aa_to_llm_model(aa: &AaLlmModel) -> LlmModel {
    let evaluations = aa.evaluations.as_ref();
    let pricing = aa.pricing.as_ref();

    LlmModel {
        id: aa.id.clone(),
        name: aa.name.clone(),
        slug: aa.slug.clone(),
        release_date: aa.release_date.clone(),
        creator: aa.model_creator.name.clone(),
        creator_slug: aa.model_creator.slug.clone(),
        intelligence: evaluations.and_then(|e| e.artificial_analysis_intelligence_index),
        coding: evaluations.and_then(|e| e.artificial_analysis_coding_index),
        math: evaluations.and_then(|e| e.artificial_analysis_math_index),
        mmlu_pro: evaluations.and_then(|e| e.mmlu_pro),
        gpqa: evaluations.and_then(|e| e.gpqa),
        hle: evaluations.and_then(|e| e.hle),
        livecodebench: evaluations.and_then(|e| e.livecodebench),
        scicode: evaluations.and_then(|e| e.scicode),
        math_500: evaluations.and_then(|e| e.math_500),
        aime: evaluations.and_then(|e| e.aime),
        input_price: pricing.and_then(|p| p.price_1m_input_tokens),
        output_price: pricing.and_then(|p| p.price_1m_output_tokens),
        price: pricing.and_then(|p| p.price_1m_blended_3_to_1),
        tps: aa.median_output_tokens_per_second,
        latency: aa.median_time_to_first_token_seconds,
    }
}
