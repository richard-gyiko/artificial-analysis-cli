//! Refresh command - fetch fresh data for all tables.

use crate::client::{Client, HostedDataClient};
use crate::config::Config;
use crate::error::Result;

/// Run the refresh command using hosted data client with API fallback.
pub async fn run(quiet: bool, use_api: bool, profile: Option<&str>) -> Result<()> {
    if use_api {
        run_with_api(quiet, profile).await
    } else {
        match run_with_hosted(quiet).await {
            Ok(()) => Ok(()),
            Err(e) => {
                // Fallback to API if hosted data fails and API key is available
                let config = Config::load()?;
                if config.get_api_key(profile).is_ok() {
                    if !quiet {
                        eprintln!(
                            "Warning: Could not fetch hosted data ({}). Falling back to API.",
                            e
                        );
                    }
                    run_with_api(quiet, profile).await
                } else {
                    Err(e)
                }
            }
        }
    }
}

/// Refresh using hosted data.
async fn run_with_hosted(quiet: bool) -> Result<()> {
    let client = HostedDataClient::new()?;

    // Refresh benchmarks
    if !quiet {
        eprint!("Refreshing benchmarks... ");
    }
    let models = client.get_llm_models(true).await?;
    if !quiet {
        eprintln!("done ({} models)", models.len());
    }

    // Refresh models.dev data
    if !quiet {
        eprint!("Refreshing models... ");
    }
    client.refresh_models().await?;
    if !quiet {
        eprintln!("done");
    }

    // Refresh media tables
    if !quiet {
        eprint!("Refreshing media tables... ");
    }
    let _ = client.get_text_to_image(true).await;
    let _ = client.get_image_editing(true).await;
    let _ = client.get_text_to_speech(true).await;
    let _ = client.get_text_to_video(true).await;
    let _ = client.get_image_to_video(true).await;
    if !quiet {
        eprintln!("done");
    }

    if !quiet {
        eprintln!();
        eprintln!("All tables refreshed. Use 'which-llm tables' to see available data.");
    }

    Ok(())
}

/// Refresh using API client.
async fn run_with_api(quiet: bool, profile: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let api_key = config.get_api_key(profile)?;
    let profile_name = profile
        .map(String::from)
        .or(config.default_profile.clone())
        .unwrap_or_else(|| "default".into());

    let client = Client::new(api_key, profile_name)?;

    // Refresh benchmarks
    if !quiet {
        eprint!("Refreshing benchmarks... ");
    }
    let models = client.get_llm_models(true).await?;
    if !quiet {
        eprintln!("done ({} models)", models.len());
    }

    // Refresh models.dev data
    if !quiet {
        eprint!("Refreshing models... ");
    }
    client.refresh_models().await?;
    if !quiet {
        eprintln!("done");
    }

    // Refresh media tables
    if !quiet {
        eprint!("Refreshing media tables... ");
    }
    let _ = client.get_text_to_image(true).await;
    let _ = client.get_image_editing(true).await;
    let _ = client.get_text_to_speech(true).await;
    let _ = client.get_text_to_video(true).await;
    let _ = client.get_image_to_video(true).await;
    if !quiet {
        eprintln!("done");
    }

    if !quiet {
        eprintln!();
        eprintln!("All tables refreshed. Use 'which-llm tables' to see available data.");
    }

    Ok(())
}
