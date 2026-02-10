//! Remote data fetching from GitHub Releases.
//!
//! This module provides functionality to fetch pre-built Parquet data files
//! from GitHub Releases, eliminating the need for users to have API keys.

use crate::error::{AppError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;

/// GitHub repository for data releases.
const GITHUB_REPO: &str = "richard-gyiko/which-llm";

/// Release tag for latest data.
const DATA_RELEASE_TAG: &str = "data/latest";

/// TTL for remote data cache (24 hours).
const REMOTE_DATA_TTL_HOURS: i64 = 24;

/// Manifest file describing the data release.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataManifest {
    /// When the data was generated.
    pub generated_at: String,
    /// Manifest format version.
    pub version: String,
    /// Source information.
    pub source: ManifestSource,
    /// Files in the release.
    pub files: HashMap<String, FileInfo>,
    /// Attribution information.
    pub attribution: Attribution,
}

/// Source information in manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSource {
    pub artificial_analysis: String,
    pub models_dev: String,
}

/// File information in manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub size: u64,
    pub sha256: String,
}

/// Attribution information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    pub text: String,
    pub url: String,
}

/// Client for fetching data from GitHub Releases.
pub struct RemoteDataClient {
    http_client: reqwest::Client,
    cache_dir: PathBuf,
}

impl RemoteDataClient {
    /// Create a new remote data client.
    pub fn new(cache_dir: PathBuf) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .user_agent(format!("which-llm/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| AppError::Network(e.to_string()))?;

        Ok(Self {
            http_client,
            cache_dir,
        })
    }

    /// Check if local data is fresh (within TTL).
    pub fn is_data_fresh(&self) -> bool {
        let meta_path = self.cache_dir.join("remote_meta.json");
        if !meta_path.exists() {
            return false;
        }

        if let Ok(content) = std::fs::read_to_string(&meta_path) {
            if let Ok(meta) = serde_json::from_str::<RemoteMeta>(&content) {
                if let Ok(fetched_at) = DateTime::parse_from_rfc3339(&meta.fetched_at) {
                    let age = Utc::now().signed_duration_since(fetched_at.with_timezone(&Utc));
                    return age.num_hours() < REMOTE_DATA_TTL_HOURS;
                }
            }
        }
        false
    }

    /// Fetch the manifest from GitHub Releases.
    pub async fn fetch_manifest(&self) -> Result<DataManifest> {
        let url = format!(
            "https://github.com/{}/releases/download/{}/manifest.json",
            GITHUB_REPO, DATA_RELEASE_TAG
        );

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("Failed to fetch manifest: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Network(format!(
                "Failed to fetch manifest: HTTP {}",
                response.status()
            )));
        }

        let manifest: DataManifest = response
            .json()
            .await
            .map_err(|e| AppError::Network(format!("Failed to parse manifest: {}", e)))?;

        Ok(manifest)
    }

    /// Fetch a parquet file from GitHub Releases.
    pub async fn fetch_parquet(&self, filename: &str) -> Result<Vec<u8>> {
        let url = format!(
            "https://github.com/{}/releases/download/{}/{}",
            GITHUB_REPO, DATA_RELEASE_TAG, filename
        );

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("Failed to fetch {}: {}", filename, e)))?;

        if !response.status().is_success() {
            return Err(AppError::Network(format!(
                "Failed to fetch {}: HTTP {}",
                filename,
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| AppError::Network(format!("Failed to read {}: {}", filename, e)))?;

        Ok(bytes.to_vec())
    }

    /// Fetch and cache a parquet file if needed, with SHA256 verification.
    pub async fn ensure_parquet(&self, name: &str, force_refresh: bool) -> Result<PathBuf> {
        let filename = format!("{}.parquet", name);
        let local_path = self.cache_dir.join(&filename);

        // Check if we need to fetch
        if !force_refresh && local_path.exists() && self.is_data_fresh() {
            return Ok(local_path);
        }

        // Fetch manifest to get expected checksum
        let manifest = self.fetch_manifest().await?;
        let expected_hash = manifest.files.get(&filename).map(|f| f.sha256.clone());

        // Fetch the file
        let data = self.fetch_parquet(&filename).await?;

        // Verify SHA256 checksum if available
        if let Some(expected) = expected_hash {
            let mut hasher = Sha256::new();
            hasher.update(&data);
            let actual = format!("{:x}", hasher.finalize());
            if actual != expected {
                return Err(AppError::Cache(format!(
                    "SHA256 mismatch for {}: expected {}, got {}",
                    filename, expected, actual
                )));
            }
        }

        // Write to cache
        std::fs::create_dir_all(&self.cache_dir)?;
        std::fs::write(&local_path, data)?;

        // Update metadata
        self.update_meta()?;

        Ok(local_path)
    }

    /// Fetch all data files from GitHub Releases.
    pub async fn fetch_all_data(&self, force_refresh: bool) -> Result<()> {
        // Files to fetch
        let files = [
            "benchmarks",
            "models",
            "text_to_image",
            "image_editing",
            "text_to_speech",
            "text_to_video",
            "image_to_video",
        ];

        // Fetch manifest first
        let manifest = self.fetch_manifest().await?;

        // Fetch each file
        for name in &files {
            let filename = format!("{}.parquet", name);
            if manifest.files.contains_key(&filename) {
                self.ensure_parquet(name, force_refresh).await?;
            }
        }

        // Save manifest locally
        let manifest_path = self.cache_dir.join("manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        std::fs::write(manifest_path, manifest_json)?;

        Ok(())
    }

    /// Update the remote metadata file.
    fn update_meta(&self) -> Result<()> {
        let meta = RemoteMeta {
            fetched_at: Utc::now().to_rfc3339(),
        };
        let meta_path = self.cache_dir.join("remote_meta.json");
        let content = serde_json::to_string_pretty(&meta)?;
        std::fs::write(meta_path, content)?;
        Ok(())
    }

    /// Get the local manifest if available.
    pub fn get_local_manifest(&self) -> Option<DataManifest> {
        let manifest_path = self.cache_dir.join("manifest.json");
        if manifest_path.exists() {
            let content = std::fs::read_to_string(manifest_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    /// Check if a parquet file exists locally.
    pub fn has_local_parquet(&self, name: &str) -> bool {
        let path = self.cache_dir.join(format!("{}.parquet", name));
        path.exists()
    }

    /// Get the path to a local parquet file.
    pub fn local_parquet_path(&self, name: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.parquet", name))
    }
}

/// Metadata about when remote data was fetched.
#[derive(Debug, Serialize, Deserialize)]
struct RemoteMeta {
    fetched_at: String,
}
