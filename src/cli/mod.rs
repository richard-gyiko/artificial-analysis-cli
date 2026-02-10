//! CLI argument definitions.

use clap::{Parser, Subcommand};

/// which-llm - Query AI model benchmarks from the terminal.
#[derive(Parser, Debug)]
#[command(name = "which-llm", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Use a specific profile
    #[arg(long, short = 'p', global = true)]
    pub profile: Option<String>,

    /// Use direct API access instead of hosted data (requires API key)
    #[arg(long, global = true)]
    pub use_api: bool,

    /// Suppress output messages (for scripting)
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Execute SQL queries against cached data
    Query {
        /// SQL query to execute (e.g., "SELECT * FROM benchmarks WHERE intelligence > 40")
        sql: Option<String>,

        /// Output as JSON
        #[arg(long, conflicts_with_all = ["csv", "table", "plain"])]
        json: bool,

        /// Output as CSV
        #[arg(long, conflicts_with_all = ["json", "table", "plain"])]
        csv: bool,

        /// Output as ASCII table
        #[arg(long, conflicts_with_all = ["json", "csv", "plain"])]
        table: bool,

        /// Output as plain text (tab-separated)
        #[arg(long, conflicts_with_all = ["json", "csv", "table"])]
        plain: bool,
    },

    /// Refresh all cached data from sources
    Refresh,

    /// List available tables and their schemas
    Tables {
        /// Show details for a specific table
        table: Option<String>,
    },

    /// Compare multiple models side by side
    Compare {
        /// Model names to compare (fuzzy matched)
        #[arg(required = true)]
        models: Vec<String>,

        /// Show all available fields
        #[arg(long, short = 'v')]
        verbose: bool,

        /// Output as JSON
        #[arg(long, conflicts_with_all = ["csv", "table", "plain"])]
        json: bool,

        /// Output as CSV
        #[arg(long, conflicts_with_all = ["json", "table", "plain"])]
        csv: bool,

        /// Output as ASCII table
        #[arg(long, conflicts_with_all = ["json", "csv", "plain"])]
        table: bool,

        /// Output as plain text (tab-separated)
        #[arg(long, conflicts_with_all = ["json", "csv", "table"])]
        plain: bool,
    },

    /// Calculate token costs for models
    Cost {
        /// Model names to calculate costs for (fuzzy matched)
        #[arg(required = true)]
        models: Vec<String>,

        /// Input tokens per request (supports units: 10k, 1M)
        #[arg(long, short = 'i')]
        input: String,

        /// Output tokens per request (supports units: 10k, 1M)
        #[arg(long, short = 'o')]
        output: String,

        /// Number of requests per period (used with --period for projections)
        #[arg(long, short = 'r', default_value = "1")]
        requests: u64,

        /// Time period for cost projection: once (single calculation), daily, or monthly (30 days)
        #[arg(long, default_value = "once")]
        period: String,

        /// Output as JSON
        #[arg(long, conflicts_with_all = ["csv", "table", "plain"])]
        json: bool,

        /// Output as CSV
        #[arg(long, conflicts_with_all = ["json", "table", "plain"])]
        csv: bool,

        /// Output as ASCII table
        #[arg(long, conflicts_with_all = ["json", "csv", "plain"])]
        table: bool,

        /// Output as plain text (tab-separated)
        #[arg(long, conflicts_with_all = ["json", "csv", "table"])]
        plain: bool,
    },

    /// Show data source information and attribution
    Info,

    /// Manage response cache
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },

    /// Manage configuration profiles
    Profile {
        #[command(subcommand)]
        command: ProfileCommands,
    },

    /// Install skills for AI coding tools
    Skill {
        #[command(subcommand)]
        command: SkillCommands,
    },
}

/// Get output format from command-specific flags.
pub fn get_output_format(
    json: bool,
    csv: bool,
    table: bool,
    plain: bool,
) -> crate::output::OutputFormat {
    if json {
        crate::output::OutputFormat::Json
    } else if csv {
        crate::output::OutputFormat::Csv
    } else if table {
        crate::output::OutputFormat::Table
    } else if plain {
        crate::output::OutputFormat::Plain
    } else {
        crate::output::OutputFormat::Markdown
    }
}

#[derive(Subcommand, Debug)]
pub enum CacheCommands {
    /// Clear all cached data
    Clear,
    /// Show cache status
    Status,
}

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// Create a new profile
    Create {
        /// Profile name
        name: String,
        /// API key (will prompt if not provided)
        #[arg(long)]
        api_key: Option<String>,
    },
    /// List all profiles
    List,
    /// Set the default profile
    Default {
        /// Profile name to set as default
        name: String,
    },
    /// Delete a profile
    Delete {
        /// Profile name to delete
        name: String,
    },
    /// Show profile details
    Show {
        /// Profile name (uses default if not specified)
        name: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum SkillCommands {
    /// Install skill for an AI coding tool
    Install {
        /// Tool name (cursor, claude, codex, opencode, windsurf, copilot, antigravity)
        tool: String,

        /// Install globally (user-level, available in all projects)
        #[arg(long, short = 'g')]
        global: bool,

        /// Overwrite existing skill directory
        #[arg(long, short = 'f')]
        force: bool,

        /// Preview without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Uninstall skill for an AI coding tool
    Uninstall {
        /// Tool name (cursor, claude, codex, opencode, windsurf, copilot, antigravity)
        tool: String,

        /// Uninstall from global location
        #[arg(long, short = 'g')]
        global: bool,
    },
    /// List supported tools and their paths
    List,
}
