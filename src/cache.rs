//! File-based caching for API responses.

use crate::error::{AppError, Result};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Default cache TTL (1 hour).
const DEFAULT_TTL_SECS: u64 = 3600;

/// Cached entry wrapper.
#[derive(Debug, Serialize, Deserialize)]
struct CacheEntry<T> {
    data: T,
    cached_at: DateTime<Utc>,
}

/// Cache manager for API responses.
pub struct Cache {
    base_dir: PathBuf,
    ttl: Duration,
}

impl Cache {
    /// Create a new cache with default settings.
    /// Respects WHICH_LLM_CACHE_DIR environment variable for testing/portability.
    pub fn new() -> Result<Self> {
        let base_dir = if let Ok(cache_dir) = std::env::var("WHICH_LLM_CACHE_DIR") {
            PathBuf::from(cache_dir)
        } else {
            dirs::cache_dir()
                .map(|p| p.join("which-llm"))
                .ok_or_else(|| AppError::Cache("Could not determine cache directory".into()))?
        };

        std::fs::create_dir_all(&base_dir)?;

        Ok(Self {
            base_dir,
            ttl: Duration::from_secs(DEFAULT_TTL_SECS),
        })
    }

    /// Get the cache base directory.
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// Get the path to a Parquet cache file.
    pub fn parquet_path(&self, name: &str) -> PathBuf {
        self.base_dir.join(format!("{}.parquet", name))
    }

    /// Generate a cache key from endpoint and params.
    pub fn cache_key(endpoint: &str, params: &[(&str, &str)]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(endpoint.as_bytes());
        for (key, value) in params {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }
        let hash = hex::encode(hasher.finalize());
        format!("{}-{}", endpoint.replace('/', "-"), &hash[..16])
    }

    /// Get cached data if valid.
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let path = self.base_dir.join(format!("{key}.json"));
        if !path.exists() {
            return None;
        }

        let content = std::fs::read_to_string(&path).ok()?;
        let entry: CacheEntry<T> = serde_json::from_str(&content).ok()?;

        // Check TTL
        let age = Utc::now().signed_duration_since(entry.cached_at);
        if age.num_seconds() > self.ttl.as_secs() as i64 {
            // Expired, remove the file
            let _ = std::fs::remove_file(&path);
            return None;
        }

        Some(entry.data)
    }

    /// Store data in cache.
    pub fn set<T: Serialize>(&self, key: &str, data: &T) -> Result<()> {
        let entry = CacheEntry {
            data,
            cached_at: Utc::now(),
        };

        let path = self.base_dir.join(format!("{key}.json"));
        let content = serde_json::to_string_pretty(&entry)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Clear all cached data.
    pub fn clear(&self) -> Result<()> {
        for entry in std::fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str());
            if ext == Some("json") || ext == Some("parquet") {
                std::fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    /// Get cache statistics.
    pub fn stats(&self) -> Result<CacheStats> {
        let mut count = 0;
        let mut size = 0;

        for entry in std::fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str());
            if ext == Some("json") || ext == Some("parquet") {
                count += 1;
                size += entry.metadata()?.len();
            }
        }

        Ok(CacheStats {
            location: self.base_dir.clone(),
            entry_count: count,
            total_size: size,
        })
    }
}

/// Cache statistics.
#[derive(Debug)]
pub struct CacheStats {
    pub location: PathBuf,
    pub entry_count: u32,
    pub total_size: u64,
}

impl CacheStats {
    /// Format size as human-readable string.
    pub fn size_human(&self) -> String {
        if self.total_size < 1024 {
            format!("{} B", self.total_size)
        } else if self.total_size < 1024 * 1024 {
            format!("{:.1} KB", self.total_size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", self.total_size as f64 / (1024.0 * 1024.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_generation() {
        let key1 = Cache::cache_key("llms", &[]);
        let key2 = Cache::cache_key("llms", &[("model", "gpt-4")]);
        let key3 = Cache::cache_key("llms", &[]);

        assert_ne!(key1, key2);
        assert_eq!(key1, key3);
    }
}
