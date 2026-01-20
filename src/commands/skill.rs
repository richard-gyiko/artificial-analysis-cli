//! Skill installation commands for AI coding tools.

use crate::error::{AppError, Result};
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Component, Path, PathBuf};
use std::time::{Duration, SystemTime};
use zip::ZipArchive;

/// URL for downloading skill zip from GitHub releases.
const SKILLS_ZIP_URL: &str =
    "https://github.com/richard-gyiko/which-llm/releases/latest/download/which-llm-skill.zip";

/// Cache TTL for skills.zip (24 hours).
const CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60);

/// Supported AI coding tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Cursor,
    Claude,
    Codex,
    OpenCode,
    Windsurf,
    Copilot,
    Antigravity,
}

impl Tool {
    /// Parse tool name from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "cursor" => Some(Self::Cursor),
            "claude" => Some(Self::Claude),
            "codex" => Some(Self::Codex),
            "opencode" => Some(Self::OpenCode),
            "windsurf" => Some(Self::Windsurf),
            "copilot" => Some(Self::Copilot),
            "antigravity" => Some(Self::Antigravity),
            _ => None,
        }
    }

    /// Get the tool name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Cursor => "cursor",
            Self::Claude => "claude",
            Self::Codex => "codex",
            Self::OpenCode => "opencode",
            Self::Windsurf => "windsurf",
            Self::Copilot => "copilot",
            Self::Antigravity => "antigravity",
        }
    }

    /// Get all supported tools.
    pub fn all() -> &'static [Self] {
        &[
            Self::Cursor,
            Self::Claude,
            Self::Codex,
            Self::OpenCode,
            Self::Windsurf,
            Self::Copilot,
            Self::Antigravity,
        ]
    }

    /// Get the project-level skill directory path (relative to current directory).
    pub fn project_path(&self) -> PathBuf {
        match self {
            Self::Cursor => PathBuf::from(".cursor/skills/which-llm"),
            Self::Claude => PathBuf::from(".claude/skills/which-llm"),
            Self::Codex => PathBuf::from(".codex/skills/which-llm"),
            Self::OpenCode => PathBuf::from(".opencode/skills/which-llm"),
            Self::Windsurf => PathBuf::from(".windsurf/skills/which-llm"),
            Self::Copilot => PathBuf::from(".github/skills/which-llm"),
            Self::Antigravity => PathBuf::from(".antigravity/skills/which-llm"),
        }
    }

    /// Get the global (user-level) skill directory path.
    pub fn global_path(&self) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| AppError::Config("Could not determine home directory".into()))?;

        let path = match self {
            Self::Cursor => home.join(".cursor/skills/which-llm"),
            Self::Claude => home.join(".claude/skills/which-llm"),
            Self::Codex => home.join(".codex/skills/which-llm"),
            Self::OpenCode => home.join(".config/opencode/skills/which-llm"),
            Self::Windsurf => home.join(".codeium/windsurf/skills/which-llm"),
            Self::Copilot => home.join(".copilot/skills/which-llm"),
            Self::Antigravity => home.join(".antigravity/skills/which-llm"),
        };

        Ok(path)
    }
}

/// Get the cache directory for skills.
fn cache_dir() -> Result<PathBuf> {
    let cache = dirs::cache_dir()
        .ok_or_else(|| AppError::Config("Could not determine cache directory".into()))?;
    Ok(cache.join("which-llm/skills"))
}

/// Get the path to the cached skills.zip.
fn cached_zip_path() -> Result<PathBuf> {
    Ok(cache_dir()?.join("skills.zip"))
}

/// Check if the cached skills.zip is fresh (within TTL).
fn is_cache_fresh() -> Result<bool> {
    let zip_path = cached_zip_path()?;
    if !zip_path.exists() {
        return Ok(false);
    }

    let metadata = fs::metadata(&zip_path)?;
    let modified = metadata.modified()?;
    let age = SystemTime::now()
        .duration_since(modified)
        .unwrap_or(Duration::MAX);

    Ok(age < CACHE_TTL)
}

/// Download skills.zip from GitHub releases.
async fn download_skills_zip() -> Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .user_agent("which-llm-cli")
        .build()
        .map_err(|e| AppError::Network(e.to_string()))?;

    let response = client
        .get(SKILLS_ZIP_URL)
        .send()
        .await
        .map_err(|e| AppError::Network(format!("Failed to download skills.zip: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::Network(format!(
            "Failed to download skills.zip: HTTP {}",
            response.status()
        )));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| AppError::Network(format!("Failed to read skills.zip: {}", e)))?;

    Ok(bytes.to_vec())
}

/// Get skills.zip data, using cache if available and fresh.
async fn get_skills_zip(force_refresh: bool) -> Result<Vec<u8>> {
    let zip_path = cached_zip_path()?;

    // Use cache if fresh and not forcing refresh
    if !force_refresh && is_cache_fresh()? {
        return fs::read(&zip_path)
            .map_err(|e| AppError::Cache(format!("Failed to read cached skills.zip: {}", e)));
    }

    // Download fresh copy
    println!("Downloading skills from GitHub releases...");
    let data = download_skills_zip().await?;

    // Cache the downloaded zip
    let cache_dir = cache_dir()?;
    fs::create_dir_all(&cache_dir)?;
    fs::write(&zip_path, &data)
        .map_err(|e| AppError::Cache(format!("Failed to cache skills.zip: {}", e)))?;

    Ok(data)
}

/// Extract the which-llm skill directory from the zip to the target path.
fn extract_skill_to(zip_data: &[u8], target_dir: &PathBuf, dry_run: bool) -> Result<Vec<PathBuf>> {
    let cursor = Cursor::new(zip_data);
    let mut archive = ZipArchive::new(cursor)
        .map_err(|e| AppError::Config(format!("Invalid skills.zip: {}", e)))?;

    let mut extracted_files = Vec::new();

    // Look for files under "which-llm/" prefix in the zip
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| AppError::Config(format!("Failed to read zip entry: {}", e)))?;

        let file_path = file.name().to_string();

        // Skip files not under which-llm/ directory
        if !file_path.starts_with("which-llm/") {
            continue;
        }

        // Get relative path within which-llm/
        let relative_path = &file_path["which-llm/".len()..];
        if relative_path.is_empty() {
            continue; // Skip the directory entry itself
        }

        // Security: Reject path traversal attempts (Zip Slip vulnerability)
        // A malicious zip could contain entries like "../../../etc/passwd"
        let path = Path::new(relative_path);
        if path.components().any(|c| matches!(c, Component::ParentDir)) {
            return Err(AppError::Config(format!(
                "Invalid path in skills.zip (path traversal attempt): {}",
                file_path
            )));
        }

        let target_path = target_dir.join(relative_path);
        extracted_files.push(target_path.clone());

        if dry_run {
            continue;
        }

        // Create parent directories
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Skip directories (they're handled by create_dir_all)
        if file.is_dir() {
            continue;
        }

        // Extract file
        let mut content = Vec::new();
        file.read_to_end(&mut content)
            .map_err(|e| AppError::Config(format!("Failed to read {}: {}", file_path, e)))?;

        fs::write(&target_path, &content)?;

        // Preserve Unix file permissions if available
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                let permissions = std::fs::Permissions::from_mode(mode);
                let _ = std::fs::set_permissions(&target_path, permissions);
            }
        }
    }

    Ok(extracted_files)
}

/// Install skill for a tool.
pub async fn install(tool_name: &str, global: bool, force: bool, dry_run: bool) -> Result<()> {
    let tool = Tool::from_str(tool_name).ok_or_else(|| {
        AppError::Config(format!(
            "Unknown tool '{}'. Run 'which-llm skill list' to see supported tools.",
            tool_name
        ))
    })?;

    let target_dir = if global {
        tool.global_path()?
    } else {
        std::env::current_dir()?.join(tool.project_path())
    };

    // Check if already installed
    if target_dir.exists() && !force && !dry_run {
        return Err(AppError::Config(format!(
            "Skill already installed at {}. Use --force to overwrite.",
            target_dir.display()
        )));
    }

    // Get skills.zip (from cache or download)
    let zip_data = get_skills_zip(false).await?;

    if dry_run {
        println!("Dry run: would install skill to {}", target_dir.display());
        let files = extract_skill_to(&zip_data, &target_dir, true)?;
        println!("Files that would be created:");
        for file in files {
            // Only show files, not directories
            if !file.to_string_lossy().ends_with('/') {
                println!("  {}", file.display());
            }
        }
        return Ok(());
    }

    // Remove existing directory if force
    if target_dir.exists() && force {
        fs::remove_dir_all(&target_dir)?;
    }

    // Create target directory
    fs::create_dir_all(&target_dir)?;

    // Extract skill files
    let files = extract_skill_to(&zip_data, &target_dir, false)?;

    let scope = if global {
        "globally"
    } else {
        "for this project"
    };
    println!("Installed which-llm skill for {} {}", tool.name(), scope);
    println!("Location: {}", target_dir.display());
    println!("Files installed: {}", files.len());

    Ok(())
}

/// Uninstall skill for a tool.
pub fn uninstall(tool_name: &str, global: bool) -> Result<()> {
    let tool = Tool::from_str(tool_name).ok_or_else(|| {
        AppError::Config(format!(
            "Unknown tool '{}'. Run 'which-llm skill list' to see supported tools.",
            tool_name
        ))
    })?;

    let target_dir = if global {
        tool.global_path()?
    } else {
        std::env::current_dir()?.join(tool.project_path())
    };

    if !target_dir.exists() {
        println!(
            "No skill installed for {} at {}",
            tool.name(),
            target_dir.display()
        );
        return Ok(());
    }

    fs::remove_dir_all(&target_dir)?;

    let scope = if global {
        "globally"
    } else {
        "for this project"
    };
    println!("Uninstalled which-llm skill for {} {}", tool.name(), scope);

    Ok(())
}

/// List supported tools and their paths.
pub fn list() -> Result<()> {
    println!("Supported tools for skill installation:\n");
    println!("{:<15} {:<40} {}", "TOOL", "PROJECT PATH", "GLOBAL PATH");
    println!("{}", "-".repeat(100));

    for tool in Tool::all() {
        let global_path = tool
            .global_path()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "N/A".to_string());

        // Shorten home path for display
        let global_display = if let Some(home) = dirs::home_dir() {
            global_path.replace(&home.display().to_string(), "~")
        } else {
            global_path
        };

        println!(
            "{:<15} {:<40} {}",
            tool.name(),
            tool.project_path().display(),
            global_display
        );
    }

    println!("\nUsage:");
    println!("  which-llm skill install <tool>          Install for current project");
    println!("  which-llm skill install <tool> --global Install globally");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_from_str() {
        assert_eq!(Tool::from_str("cursor"), Some(Tool::Cursor));
        assert_eq!(Tool::from_str("CURSOR"), Some(Tool::Cursor));
        assert_eq!(Tool::from_str("Claude"), Some(Tool::Claude));
        assert_eq!(Tool::from_str("opencode"), Some(Tool::OpenCode));
        assert_eq!(Tool::from_str("unknown"), None);
    }

    #[test]
    fn test_tool_name() {
        assert_eq!(Tool::Cursor.name(), "cursor");
        assert_eq!(Tool::Claude.name(), "claude");
        assert_eq!(Tool::OpenCode.name(), "opencode");
    }

    #[test]
    fn test_project_paths() {
        assert_eq!(
            Tool::Cursor.project_path(),
            PathBuf::from(".cursor/skills/which-llm")
        );
        assert_eq!(
            Tool::Copilot.project_path(),
            PathBuf::from(".github/skills/which-llm")
        );
        assert_eq!(
            Tool::OpenCode.project_path(),
            PathBuf::from(".opencode/skills/which-llm")
        );
    }

    #[test]
    fn test_all_tools() {
        let tools = Tool::all();
        assert_eq!(tools.len(), 7);
        assert!(tools.contains(&Tool::Cursor));
        assert!(tools.contains(&Tool::Claude));
        assert!(tools.contains(&Tool::Codex));
        assert!(tools.contains(&Tool::OpenCode));
        assert!(tools.contains(&Tool::Windsurf));
        assert!(tools.contains(&Tool::Copilot));
        assert!(tools.contains(&Tool::Antigravity));
    }

    #[test]
    fn test_extract_rejects_path_traversal() {
        // Create a zip with a malicious path traversal entry
        let mut zip_buffer = Vec::new();
        {
            use std::io::Write;
            let cursor = Cursor::new(&mut zip_buffer);
            let mut zip = zip::ZipWriter::new(cursor);

            // Add a legitimate file first
            let options = zip::write::SimpleFileOptions::default();
            zip.start_file("which-llm/legitimate.txt", options).unwrap();
            zip.write_all(b"safe content").unwrap();

            // Add a malicious path traversal entry
            zip.start_file("which-llm/../../../etc/passwd", options)
                .unwrap();
            zip.write_all(b"malicious content").unwrap();

            zip.finish().unwrap();
        }

        let target_dir = PathBuf::from("/tmp/test-extract");
        let result = extract_skill_to(&zip_buffer, &target_dir, true);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("path traversal"),
            "Expected path traversal error, got: {}",
            err_msg
        );
    }
}
