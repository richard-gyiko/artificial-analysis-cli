//! which-llm - Query AI model benchmarks from the terminal.

use clap::Parser;
use which_llm::{
    cli::{CacheCommands, Cli, Commands, ProfileCommands, SkillCommands},
    client::{Client, HostedDataClient},
    commands,
    config::Config,
    error::Result,
};

/// Attribution text for Artificial Analysis.
const ATTRIBUTION: &str = "Data provided by Artificial Analysis (https://artificialanalysis.ai)";
const MODELS_DEV_ATTRIBUTION: &str = "Capability data from models.dev (https://models.dev)";

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    let format = cli.output_format();

    // Handle commands that don't need data access
    match &cli.command {
        Commands::Profile { command } => {
            return match command {
                ProfileCommands::Create { name, api_key } => {
                    commands::profile::create(name, api_key.as_deref())
                }
                ProfileCommands::List => commands::profile::list(),
                ProfileCommands::Default { name } => commands::profile::set_default(name),
                ProfileCommands::Delete { name } => commands::profile::delete(name),
                ProfileCommands::Show { name } => commands::profile::show(name.as_deref()),
            };
        }
        Commands::Cache { command } => {
            return match command {
                CacheCommands::Clear => commands::cache::clear(),
                CacheCommands::Status => commands::cache::status(),
            };
        }
        Commands::Query { sql, tables } => {
            return commands::query::run(sql.as_deref(), *tables, format);
        }
        Commands::Skill { command } => {
            return match command {
                SkillCommands::Install {
                    tool,
                    global,
                    force,
                    dry_run,
                } => commands::skill::install(tool, *global, *force, *dry_run).await,
                SkillCommands::Uninstall { tool, global } => {
                    commands::skill::uninstall(tool, *global)
                }
                SkillCommands::List => commands::skill::list(),
            };
        }
        Commands::Info => {
            return commands::info::run();
        }
        _ => {}
    }

    // Determine data source: API if --use-api flag or if we need to try API as fallback
    let use_api = cli.use_api;

    // Handle commands that need data access
    let show_hint = if use_api {
        // Use direct API access (requires API key)
        let config = Config::load()?;
        let api_key = config.get_api_key(cli.profile.as_deref())?;
        let profile_name = cli
            .profile
            .clone()
            .or(config.default_profile.clone())
            .unwrap_or_else(|| "default".into());

        let client = Client::new(api_key, profile_name)?;
        run_with_api_client(&cli, &client, format).await?
    } else {
        // Use hosted data (default, no API key needed)
        let hosted_client = HostedDataClient::new()?;
        match run_with_hosted_client(&cli, &hosted_client, format).await {
            Ok(hint) => hint,
            Err(e) => {
                // Fallback to API if hosted data fails and API key is available
                let config = Config::load()?;
                if let Ok(api_key) = config.get_api_key(cli.profile.as_deref()) {
                    eprintln!(
                        "Warning: Could not fetch hosted data ({}). Falling back to API.",
                        e
                    );
                    let profile_name = cli
                        .profile
                        .clone()
                        .or(config.default_profile.clone())
                        .unwrap_or_else(|| "default".into());
                    let client = Client::new(api_key, profile_name)?;
                    run_with_api_client(&cli, &client, format).await?
                } else {
                    // No API key and hosted data failed
                    return Err(e);
                }
            }
        }
    };

    // Print attribution (required by API terms) unless --quiet
    if !cli.quiet {
        println!();
        println!("{}", ATTRIBUTION);
        println!("{}", MODELS_DEV_ATTRIBUTION);

        // Show hint about which-llm query for advanced filtering
        if let Some(table) = show_hint {
            println!();
            println!(
                "Tip: Use 'which-llm query \"SELECT * FROM {} WHERE ...\"' for advanced filtering",
                table
            );
        }
    }

    Ok(())
}

/// Run data commands using the hosted data client.
async fn run_with_hosted_client(
    cli: &Cli,
    client: &HostedDataClient,
    format: which_llm::output::OutputFormat,
) -> Result<Option<&'static str>> {
    let show_hint = match &cli.command {
        Commands::Llms {
            model,
            creator,
            sort,
        } => {
            commands::llms::run_hosted(
                client,
                cli.refresh,
                format,
                model.as_deref(),
                creator.as_deref(),
                sort.as_deref(),
            )
            .await?;
            Some("benchmarks")
        }
        Commands::TextToImage { categories } => {
            let models = client.get_text_to_image(cli.refresh).await?;
            commands::media::display_media_models(&models, format, "text_to_image", *categories);
            Some("text_to_image")
        }
        Commands::ImageEditing => {
            let models = client.get_image_editing(cli.refresh).await?;
            commands::media::display_media_models(&models, format, "image_editing", false);
            Some("image_editing")
        }
        Commands::TextToSpeech => {
            let models = client.get_text_to_speech(cli.refresh).await?;
            commands::media::display_media_models(&models, format, "text_to_speech", false);
            Some("text_to_speech")
        }
        Commands::TextToVideo { categories } => {
            let models = client.get_text_to_video(cli.refresh).await?;
            commands::media::display_media_models(&models, format, "text_to_video", *categories);
            Some("text_to_video")
        }
        Commands::ImageToVideo { categories } => {
            let models = client.get_image_to_video(cli.refresh).await?;
            commands::media::display_media_models(&models, format, "image_to_video", *categories);
            Some("image_to_video")
        }
        Commands::Compare { models, verbose } => {
            let llm_models = client.get_llm_models(cli.refresh).await?;
            commands::compare::run(&llm_models, models, *verbose, format)?;
            None
        }
        Commands::Cost {
            models,
            input,
            output,
            requests,
            period,
        } => {
            let llm_models = client.get_llm_models(cli.refresh).await?;
            commands::cost::run(
                &llm_models,
                models,
                input,
                output,
                *requests,
                period,
                format,
            )?;
            None
        }
        _ => unreachable!(),
    };

    Ok(show_hint)
}

/// Run data commands using the API client.
async fn run_with_api_client(
    cli: &Cli,
    client: &Client,
    format: which_llm::output::OutputFormat,
) -> Result<Option<&'static str>> {
    let show_hint = match &cli.command {
        Commands::Llms {
            model,
            creator,
            sort,
        } => {
            commands::llms::run(
                client,
                cli.refresh,
                format,
                model.as_deref(),
                creator.as_deref(),
                sort.as_deref(),
            )
            .await?;
            Some("benchmarks")
        }
        Commands::TextToImage { categories } => {
            commands::media::run_text_to_image(client, cli.refresh, format, *categories).await?;
            Some("text_to_image")
        }
        Commands::ImageEditing => {
            commands::media::run_image_editing(client, cli.refresh, format).await?;
            Some("image_editing")
        }
        Commands::TextToSpeech => {
            commands::media::run_text_to_speech(client, cli.refresh, format).await?;
            Some("text_to_speech")
        }
        Commands::TextToVideo { categories } => {
            commands::media::run_text_to_video(client, cli.refresh, format, *categories).await?;
            Some("text_to_video")
        }
        Commands::ImageToVideo { categories } => {
            commands::media::run_image_to_video(client, cli.refresh, format, *categories).await?;
            Some("image_to_video")
        }
        Commands::Compare { models, verbose } => {
            let llm_models = client.get_llm_models(cli.refresh).await?;
            commands::compare::run(&llm_models, models, *verbose, format)?;
            None
        }
        Commands::Cost {
            models,
            input,
            output,
            requests,
            period,
        } => {
            let llm_models = client.get_llm_models(cli.refresh).await?;
            commands::cost::run(
                &llm_models,
                models,
                input,
                output,
                *requests,
                period,
                format,
            )?;
            None
        }
        _ => unreachable!(),
    };

    Ok(show_hint)
}
