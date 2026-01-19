//! Profile management commands.

use crate::config::{Config, Profile};
use crate::error::Result;
use dialoguer::{Input, Password};

/// Create a new profile.
pub fn create(name: &str, api_key: Option<&str>) -> Result<()> {
    let mut config = Config::load()?;

    // Check if profile already exists
    if config.get_profile(name).is_some() {
        eprintln!("Profile '{}' already exists. Use a different name.", name);
        return Ok(());
    }

    // Get API key
    let key = match api_key {
        Some(k) => k.to_string(),
        None => Password::new()
            .with_prompt("Enter API key")
            .interact()
            .map_err(|e| crate::error::AppError::Config(e.to_string()))?,
    };

    // Create and save profile
    config.set_profile(name.to_string(), Profile { api_key: key });

    // Set as default if first profile
    if config.profiles.len() == 1 {
        config.set_default(name.to_string());
        println!("Created profile '{}' and set as default.", name);
    } else {
        println!("Created profile '{}'.", name);
    }

    config.save()?;
    Ok(())
}

/// List all profiles.
pub fn list() -> Result<()> {
    let config = Config::load()?;

    if config.profiles.is_empty() {
        println!("No profiles configured. Run 'aa profile create <name>' to create one.");
        return Ok(());
    }

    println!("Profiles:");
    for name in config.profiles.keys() {
        let is_default = config.default_profile.as_deref() == Some(name.as_str());
        let marker = if is_default { " (default)" } else { "" };
        println!("  - {}{}", name, marker);
    }

    Ok(())
}

/// Set the default profile.
pub fn set_default(name: &str) -> Result<()> {
    let mut config = Config::load()?;

    if config.get_profile(name).is_none() {
        eprintln!("Profile '{}' not found.", name);
        return Ok(());
    }

    config.set_default(name.to_string());
    config.save()?;
    println!("Set '{}' as the default profile.", name);

    Ok(())
}

/// Delete a profile.
pub fn delete(name: &str) -> Result<()> {
    let mut config = Config::load()?;

    if config.get_profile(name).is_none() {
        eprintln!("Profile '{}' not found.", name);
        return Ok(());
    }

    // Warn if deleting default profile
    let is_default = config.default_profile.as_deref() == Some(name);
    if is_default {
        let confirm: String = Input::new()
            .with_prompt(format!(
                "'{}' is the default profile. Delete anyway? (yes/no)",
                name
            ))
            .interact_text()
            .map_err(|e| crate::error::AppError::Config(e.to_string()))?;

        if confirm.to_lowercase() != "yes" {
            println!("Cancelled.");
            return Ok(());
        }
        config.default_profile = None;
    }

    config.remove_profile(name);
    config.save()?;
    println!("Deleted profile '{}'.", name);

    Ok(())
}

/// Show profile details.
pub fn show(name: Option<&str>) -> Result<()> {
    let config = Config::load()?;

    let profile_name = name
        .map(String::from)
        .or(config.default_profile.clone())
        .ok_or_else(|| {
            crate::error::AppError::Config(
                "No profile specified and no default profile set.".into(),
            )
        })?;

    let profile = config
        .get_profile(&profile_name)
        .ok_or_else(|| crate::error::AppError::ProfileNotFound(profile_name.clone()))?;

    let is_default = config.default_profile.as_deref() == Some(profile_name.as_str());
    let default_marker = if is_default { " (default)" } else { "" };

    println!("Profile: {}{}", profile_name, default_marker);
    println!(
        "  API Key: {}...{}",
        &profile.api_key[..8.min(profile.api_key.len())],
        if profile.api_key.len() > 12 {
            &profile.api_key[profile.api_key.len() - 4..]
        } else {
            ""
        }
    );

    Ok(())
}
