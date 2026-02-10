//! Tables command - list available tables and their schemas.

use crate::cache::Cache;
use crate::error::Result;
use crate::query::{format_tables_list, QueryExecutor};

/// Run the tables command.
pub fn run(table_name: Option<&str>) -> Result<()> {
    let cache = Cache::new()?;
    let executor = QueryExecutor::new(cache.base_dir().to_path_buf());

    let table_info = executor.list_tables();

    if let Some(name) = table_name {
        // Show details for a specific table
        if let Some(info) = table_info.iter().find(|t| t.name == name) {
            println!("Table: {}", info.name);
            println!(
                "Status: {}",
                if info.exists { "cached" } else { "not cached" }
            );
            if let Some(columns) = &info.schema {
                println!();
                println!("Columns:");
                for col in columns {
                    println!(
                        "  - {} {} {}",
                        col.name,
                        col.data_type,
                        if col.nullable { "NULL" } else { "NOT NULL" }
                    );
                }
            }
            if !info.exists {
                println!();
                println!("Run 'which-llm refresh' to cache this table.");
            }
        } else {
            eprintln!("Table '{}' not found.", name);
            eprintln!();
            eprintln!("Available tables:");
            for info in &table_info {
                eprintln!("  - {}", info.name);
            }
        }
    } else {
        // List all tables
        println!("{}", format_tables_list(&table_info));
    }

    Ok(())
}
