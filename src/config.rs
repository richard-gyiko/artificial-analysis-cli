//! Configuration management with profile support.

use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Profile configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub api_key: String,
}

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default_profile: Option<String>,
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

impl Config {
    /// Load configuration from the default path.
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to the default path.
    /// On Unix systems, sets restrictive permissions (600) to protect API keys.
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, &content)?;

        // Set restrictive permissions on Unix to protect API keys
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o600);
            std::fs::set_permissions(&path, permissions)?;
        }

        Ok(())
    }

    /// Get the configuration file path.
    /// Respects AA_CONFIG_DIR environment variable for testing/portability.
    pub fn config_path() -> Result<PathBuf> {
        // Allow override via environment variable (useful for testing)
        if let Ok(config_dir) = std::env::var("AA_CONFIG_DIR") {
            return Ok(PathBuf::from(config_dir).join("config.toml"));
        }

        dirs::config_dir()
            .map(|p| p.join("aa").join("config.toml"))
            .ok_or_else(|| AppError::Config("Could not determine config directory".into()))
    }

    /// Get a profile by name.
    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    /// Add or update a profile.
    pub fn set_profile(&mut self, name: String, profile: Profile) {
        self.profiles.insert(name, profile);
    }

    /// Remove a profile.
    pub fn remove_profile(&mut self, name: &str) -> Option<Profile> {
        self.profiles.remove(name)
    }

    /// Set the default profile.
    pub fn set_default(&mut self, name: String) {
        self.default_profile = Some(name);
    }

    /// Get the API key for the given profile, or the default profile.
    /// Environment variable AA_API_KEY takes precedence.
    pub fn get_api_key(&self, profile_name: Option<&str>) -> Result<String> {
        // Check environment variable first
        if let Ok(key) = std::env::var("AA_API_KEY") {
            return Ok(key);
        }

        // Determine which profile to use
        let profile_name = profile_name
            .map(String::from)
            .or_else(|| self.default_profile.clone())
            .ok_or(AppError::NoApiKey)?;

        // Get the profile
        let profile = self
            .profiles
            .get(&profile_name)
            .ok_or_else(|| AppError::ProfileNotFound(profile_name))?;

        Ok(profile.api_key.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.default_profile.is_none());
        assert!(config.profiles.is_empty());
    }

    #[test]
    fn test_config_profile_operations() {
        let mut config = Config::default();

        config.set_profile(
            "test".into(),
            Profile {
                api_key: "key123".into(),
            },
        );

        assert!(config.get_profile("test").is_some());
        assert_eq!(config.get_profile("test").unwrap().api_key, "key123");

        config.set_default("test".into());
        assert_eq!(config.default_profile, Some("test".into()));

        config.remove_profile("test");
        assert!(config.get_profile("test").is_none());
    }
}
