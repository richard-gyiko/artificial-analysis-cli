//! Query command implementation.

use crate::cache::Cache;
use crate::error::Result;
use crate::output::OutputFormat;
use crate::query::{format_query_result, format_tables_list, QueryExecutor};

/// Run the query command.
pub fn run(sql: Option<&str>, tables: bool, format: OutputFormat) -> Result<()> {
    let cache = Cache::new()?;
    let executor = QueryExecutor::new(cache.base_dir().to_path_buf());

    if tables {
        // List available tables
        let table_info = executor.list_tables();
        println!("{}", format_tables_list(&table_info));
        return Ok(());
    }

    // Execute SQL query
    let sql = match sql {
        Some(s) => s,
        None => {
            eprintln!("Error: No SQL query provided.");
            eprintln!();
            eprintln!(
                "Usage: which-llm query \"SELECT * FROM benchmarks WHERE intelligence > 40\""
            );
            eprintln!();
            eprintln!("Use 'which-llm tables' to see available tables.");
            return Ok(());
        }
    };

    let result = executor.execute(sql)?;
    println!("{}", format_query_result(&result, format));

    Ok(())
}
