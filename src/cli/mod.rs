//! CLI argument definitions.

use crate::output::OutputFormat;
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

    /// Output as JSON
    #[arg(long, global = true, conflicts_with_all = ["csv", "table", "plain"])]
    pub json: bool,

    /// Output as CSV
    #[arg(long, global = true, conflicts_with_all = ["json", "table", "plain"])]
    pub csv: bool,

    /// Output as ASCII table
    #[arg(long, global = true, conflicts_with_all = ["json", "csv", "plain"])]
    pub table: bool,

    /// Output as plain text (tab-separated)
    #[arg(long, global = true, conflicts_with_all = ["json", "csv", "table"])]
    pub plain: bool,

    /// Bypass cache and fetch fresh data
    #[arg(long, global = true)]
    pub refresh: bool,

    /// Use direct API access instead of hosted data (requires API key)
    #[arg(long, global = true)]
    pub use_api: bool,

    /// Suppress attribution notice (for scripting)
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,
}

impl Cli {
    /// Get the selected output format.
    pub fn output_format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else if self.csv {
            OutputFormat::Csv
        } else if self.table {
            OutputFormat::Table
        } else if self.plain {
            OutputFormat::Plain
        } else {
            OutputFormat::Markdown
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List and query LLM models
    Llms {
        /// Filter by model slug
        #[arg(long, short = 'm')]
        model: Option<String>,

        /// Filter by creator slug
        #[arg(long, short = 'c')]
        creator: Option<String>,

        /// Sort by field (intelligence, price, speed, context)
        #[arg(long, short = 's')]
        sort: Option<String>,

        // Capability filters
        /// Filter by reasoning support (chain-of-thought models)
        #[arg(long)]
        reasoning: bool,

        /// Filter by tool/function calling support
        #[arg(long)]
        tool_call: bool,

        /// Filter by structured JSON output support
        #[arg(long)]
        structured_output: bool,

        /// Filter by file attachment support
        #[arg(long)]
        attachment: bool,

        /// Filter by minimum context window (tokens)
        #[arg(long)]
        min_context: Option<u64>,

        /// Filter by modality (e.g., "input:image", "output:text")
        #[arg(long)]
        modality: Option<String>,
    },

    /// List text-to-image model rankings
    TextToImage {
        /// Include category breakdown
        #[arg(long)]
        categories: bool,
    },

    /// List image editing model rankings
    ImageEditing,

    /// List text-to-speech model rankings
    TextToSpeech,

    /// List text-to-video model rankings
    TextToVideo {
        /// Include category breakdown
        #[arg(long)]
        categories: bool,
    },

    /// List image-to-video model rankings
    ImageToVideo {
        /// Include category breakdown
        #[arg(long)]
        categories: bool,
    },

    /// Execute SQL queries against cached data
    Query {
        /// SQL query to execute (e.g., "SELECT * FROM llms WHERE intelligence > 40")
        sql: Option<String>,

        /// List available tables and their schemas
        #[arg(long)]
        tables: bool,
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
