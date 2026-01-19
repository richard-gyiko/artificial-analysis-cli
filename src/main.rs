//! Artificial Analysis CLI - Query AI model benchmarks from the terminal.

use aa::{
    cli::{CacheCommands, Cli, Commands, ProfileCommands},
    client::Client,
    commands,
    config::Config,
    error::Result,
};
use clap::Parser;

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

    // Handle commands that don't need API access
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
        _ => {}
    }

    // Load config and get API key
    let config = Config::load()?;
    let api_key = config.get_api_key(cli.profile.as_deref())?;
    let profile_name = cli
        .profile
        .clone()
        .or(config.default_profile.clone())
        .unwrap_or_else(|| "default".into());

    // Create client
    let client = Client::new(api_key, profile_name)?;

    // Handle commands that need API access
    let quota = match &cli.command {
        Commands::Llms {
            model,
            creator,
            sort,
        } => {
            commands::llms::run(
                &client,
                cli.refresh,
                format,
                model.as_deref(),
                creator.as_deref(),
                sort.as_deref(),
            )
            .await?
        }
        Commands::TextToImage { categories } => {
            commands::media::run_text_to_image(&client, cli.refresh, format, *categories).await?
        }
        Commands::ImageEditing => {
            commands::media::run_image_editing(&client, cli.refresh, format).await?
        }
        Commands::TextToSpeech => {
            commands::media::run_text_to_speech(&client, cli.refresh, format).await?
        }
        Commands::TextToVideo { categories } => {
            commands::media::run_text_to_video(&client, cli.refresh, format, *categories).await?
        }
        Commands::ImageToVideo { categories } => {
            commands::media::run_image_to_video(&client, cli.refresh, format, *categories).await?
        }
        Commands::Quota => {
            commands::quota::run(&client)?;
            None
        }
        // Profile and Cache are handled above
        Commands::Profile { .. } | Commands::Cache { .. } => unreachable!(),
    };

    // Print attribution (required by API terms) unless --quiet
    if !cli.quiet {
        println!();
        println!("Data provided by Artificial Analysis (https://artificialanalysis.ai)");
    }

    // Show low quota warning if applicable
    if let Some(q) = quota {
        if q.is_low() {
            eprintln!();
            eprintln!(
                "WARNING: API quota is low ({} of {} requests remaining, {:.1}%)",
                q.remaining,
                q.limit,
                q.percentage_remaining()
            );
        }
    }

    Ok(())
}
