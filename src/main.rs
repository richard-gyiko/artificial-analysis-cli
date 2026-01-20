//! which-llm - Query AI model benchmarks from the terminal.

use clap::Parser;
use which_llm::{
    cli::{CacheCommands, Cli, Commands, ProfileCommands},
    client::Client,
    commands::{self, llms::CapabilityFilters},
    config::Config,
    error::Result,
};

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
        Commands::Query { sql, tables } => {
            return commands::query::run(sql.as_deref(), *tables, format);
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
    let (quota, show_hint) = match &cli.command {
        Commands::Llms {
            model,
            creator,
            sort,
            reasoning,
            tool_call,
            structured_output,
            attachment,
            min_context,
            modality,
        } => {
            let capability_filters = CapabilityFilters {
                reasoning: *reasoning,
                tool_call: *tool_call,
                structured_output: *structured_output,
                attachment: *attachment,
                min_context: *min_context,
                modality: modality.clone(),
            };

            let q = commands::llms::run(
                &client,
                cli.refresh,
                format,
                model.as_deref(),
                creator.as_deref(),
                sort.as_deref(),
                capability_filters,
            )
            .await?;
            (q, Some("llms"))
        }
        Commands::TextToImage { categories } => {
            let q = commands::media::run_text_to_image(&client, cli.refresh, format, *categories)
                .await?;
            (q, Some("text_to_image"))
        }
        Commands::ImageEditing => {
            let q = commands::media::run_image_editing(&client, cli.refresh, format).await?;
            (q, Some("image_editing"))
        }
        Commands::TextToSpeech => {
            let q = commands::media::run_text_to_speech(&client, cli.refresh, format).await?;
            (q, Some("text_to_speech"))
        }
        Commands::TextToVideo { categories } => {
            let q = commands::media::run_text_to_video(&client, cli.refresh, format, *categories)
                .await?;
            (q, Some("text_to_video"))
        }
        Commands::ImageToVideo { categories } => {
            let q = commands::media::run_image_to_video(&client, cli.refresh, format, *categories)
                .await?;
            (q, Some("image_to_video"))
        }
        Commands::Quota => {
            commands::quota::run(&client)?;
            (None, None)
        }
        // Profile, Cache, and Query are handled above
        Commands::Profile { .. } | Commands::Cache { .. } | Commands::Query { .. } => {
            unreachable!()
        }
    };

    // Print attribution (required by API terms) unless --quiet
    if !cli.quiet {
        println!();
        println!("Data provided by Artificial Analysis (https://artificialanalysis.ai)");
        println!("Capability data from models.dev (https://models.dev)");

        // Show hint about which-llm query for advanced filtering
        if let Some(table) = show_hint {
            println!();
            println!(
                "Tip: Use 'which-llm query \"SELECT * FROM {} WHERE ...\"' for advanced filtering",
                table
            );
        }
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
