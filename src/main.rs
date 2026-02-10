//! which-llm - Query AI model benchmarks from the terminal.

use clap::Parser;
use which_llm::{
    cli::{get_output_format, CacheCommands, Cli, Commands, ProfileCommands, SkillCommands},
    client::HostedDataClient,
    commands,
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

    match &cli.command {
        // Query command - primary interface
        Commands::Query {
            sql,
            json,
            csv,
            table,
            plain,
        } => {
            let format = get_output_format(*json, *csv, *table, *plain);
            commands::query::run(sql.as_deref(), false, format)
        }

        // Refresh command - fetch fresh data
        Commands::Refresh => {
            commands::refresh::run(cli.quiet, cli.use_api, cli.profile.as_deref()).await
        }

        // Tables command - list available tables
        Commands::Tables { table } => commands::tables::run(table.as_deref()),

        // Compare command - side-by-side model comparison
        Commands::Compare {
            models,
            verbose,
            json,
            csv,
            table,
            plain,
        } => {
            let format = get_output_format(*json, *csv, *table, *plain);
            let client = HostedDataClient::new()?;
            let llm_models = client.get_llm_models(false).await?;
            commands::compare::run(&llm_models, models, *verbose, format)
        }

        // Cost command - token cost calculator
        Commands::Cost {
            models,
            input,
            output,
            requests,
            period,
            json,
            csv,
            table,
            plain,
        } => {
            let format = get_output_format(*json, *csv, *table, *plain);
            let client = HostedDataClient::new()?;
            let llm_models = client.get_llm_models(false).await?;
            commands::cost::run(
                &llm_models,
                models,
                input,
                output,
                *requests,
                period,
                format,
            )
        }

        // Info command
        Commands::Info => commands::info::run(),

        // Cache management
        Commands::Cache { command } => match command {
            CacheCommands::Clear => commands::cache::clear(),
            CacheCommands::Status => commands::cache::status(),
        },

        // Profile management
        Commands::Profile { command } => match command {
            ProfileCommands::Create { name, api_key } => {
                commands::profile::create(name, api_key.as_deref())
            }
            ProfileCommands::List => commands::profile::list(),
            ProfileCommands::Default { name } => commands::profile::set_default(name),
            ProfileCommands::Delete { name } => commands::profile::delete(name),
            ProfileCommands::Show { name } => commands::profile::show(name.as_deref()),
        },

        // Skill management
        Commands::Skill { command } => match command {
            SkillCommands::Install {
                tool,
                global,
                force,
                dry_run,
            } => commands::skill::install(tool, *global, *force, *dry_run).await,
            SkillCommands::Uninstall { tool, global } => commands::skill::uninstall(tool, *global),
            SkillCommands::List => commands::skill::list(),
        },
    }
}
