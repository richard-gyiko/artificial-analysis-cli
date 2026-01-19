//! Quota command.

use crate::client::Client;
use crate::error::Result;

/// Run the quota command.
pub fn run(client: &Client) -> Result<()> {
    match client.get_cached_quota() {
        Some(quota) => {
            println!("API Quota Status");
            println!("================");
            println!("Limit:     {} requests/day", quota.limit);
            println!("Remaining: {} requests", quota.remaining);
            println!(
                "Used:      {} requests ({:.1}%)",
                quota.limit - quota.remaining,
                100.0 - quota.percentage_remaining()
            );
            println!("Resets:    {}", quota.reset);
            println!(
                "Updated:   {}",
                quota.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
            );

            if quota.is_low() {
                println!();
                println!(
                    "WARNING: Quota is low ({:.1}% remaining)",
                    quota.percentage_remaining()
                );
            }
        }
        None => {
            println!("No quota data available.");
            println!("Run a data command (e.g., 'aa llms') to initialize quota tracking.");
        }
    }

    Ok(())
}
