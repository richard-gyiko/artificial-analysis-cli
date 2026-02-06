//! Info command - shows data source and attribution information.

use crate::cache::Cache;
use crate::error::Result;
use crate::remote::RemoteDataClient;

/// Attribution text.
const ATTRIBUTION: &str = "Data provided by Artificial Analysis (https://artificialanalysis.ai)";
const MODELS_DEV_ATTRIBUTION: &str = "Capability data from models.dev (https://models.dev)";
const METHODOLOGY_URL: &str = "https://artificialanalysis.ai/methodology";

/// Run the info command.
pub fn run() -> Result<()> {
    let cache = Cache::new()?;
    let remote = RemoteDataClient::new(cache.base_dir().to_path_buf())?;

    println!("which-llm v{}", env!("CARGO_PKG_VERSION"));
    println!();

    // Show data source status
    println!("Data Source:");
    if let Some(manifest) = remote.get_local_manifest() {
        println!("  Type: Hosted (GitHub Releases)");
        println!("  Generated: {}", manifest.generated_at);
        println!("  Version: {}", manifest.version);
        println!();
        println!("  Files:");
        for (name, info) in &manifest.files {
            println!("    {} ({} bytes)", name, info.size);
        }
    } else if remote.is_data_fresh() {
        println!("  Type: Hosted (cached)");
        println!("  Status: Fresh (within 24h TTL)");
    } else {
        println!("  Type: Not yet fetched");
        println!("  Run 'which-llm llms' to fetch data");
    }

    println!();

    // Show cache status
    if let Ok(stats) = cache.stats() {
        println!("Cache:");
        println!("  Location: {}", stats.location.display());
        println!("  Files: {}", stats.entry_count);
        println!("  Size: {}", stats.size_human());
    }

    println!();

    // Show attribution
    println!("Attribution:");
    println!("  {}", ATTRIBUTION);
    println!("  {}", MODELS_DEV_ATTRIBUTION);
    println!();
    println!("Methodology: {}", METHODOLOGY_URL);
    println!();
    println!("Attribution is required when using this data.");
    println!("See: https://artificialanalysis.ai/documentation#attribution");

    Ok(())
}
