//! Cache management command.

use crate::cache::Cache;
use crate::error::Result;

/// Clear the cache.
pub fn clear() -> Result<()> {
    let cache = Cache::new()?;
    cache.clear()?;
    println!("Cache cleared.");
    Ok(())
}

/// Show cache status.
pub fn status() -> Result<()> {
    let cache = Cache::new()?;
    let stats = cache.stats()?;

    println!("Cache Status");
    println!("============");
    println!("Location: {}", stats.location.display());
    println!("Entries:  {}", stats.entry_count);
    println!("Size:     {}", stats.size_human());

    Ok(())
}
