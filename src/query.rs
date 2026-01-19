//! SQL query execution via DuckDB.
//!
//! Provides table name alias substitution and SQL execution against Parquet files.

use crate::error::{AppError, Result};
use crate::output::OutputFormat;
use duckdb::types::Value;
use duckdb::Connection;
use std::collections::HashMap;
use std::path::PathBuf;

/// Known table aliases and their corresponding Parquet filenames.
pub const TABLE_ALIASES: &[(&str, &str)] = &[
    ("llms", "llms.parquet"),
    ("text_to_image", "text_to_image.parquet"),
    ("image_editing", "image_editing.parquet"),
    ("text_to_speech", "text_to_speech.parquet"),
    ("text_to_video", "text_to_video.parquet"),
    ("image_to_video", "image_to_video.parquet"),
];

/// Schema information for a table.
pub struct TableSchema {
    pub name: &'static str,
    pub columns: &'static [(&'static str, &'static str, bool)], // (name, type, nullable)
}

/// LLM table schema.
pub const LLM_SCHEMA: TableSchema = TableSchema {
    name: "llms",
    columns: &[
        ("id", "VARCHAR", false),
        ("name", "VARCHAR", false),
        ("slug", "VARCHAR", false),
        ("creator", "VARCHAR", false),
        ("creator_slug", "VARCHAR", true),
        ("release_date", "VARCHAR", true),
        ("intelligence", "DOUBLE", true),
        ("coding", "DOUBLE", true),
        ("math", "DOUBLE", true),
        ("mmlu_pro", "DOUBLE", true),
        ("gpqa", "DOUBLE", true),
        ("input_price", "DOUBLE", true),
        ("output_price", "DOUBLE", true),
        ("price", "DOUBLE", true),
        ("tps", "DOUBLE", true),
        ("latency", "DOUBLE", true),
    ],
};

/// Media table schema (used by all media tables).
pub const MEDIA_SCHEMA: TableSchema = TableSchema {
    name: "media",
    columns: &[
        ("id", "VARCHAR", false),
        ("name", "VARCHAR", false),
        ("slug", "VARCHAR", false),
        ("creator", "VARCHAR", false),
        ("elo", "DOUBLE", true),
        ("rank", "INTEGER", true),
        ("release_date", "VARCHAR", true),
    ],
};

/// Get the schema for a table alias.
pub fn get_schema(table_name: &str) -> Option<&'static TableSchema> {
    match table_name {
        "llms" => Some(&LLM_SCHEMA),
        "text_to_image" | "image_editing" | "text_to_speech" | "text_to_video"
        | "image_to_video" => Some(&MEDIA_SCHEMA),
        _ => None,
    }
}

/// Result of a SQL query.
pub struct QueryResult {
    /// Column names.
    pub columns: Vec<String>,
    /// Rows of values.
    pub rows: Vec<Vec<String>>,
}

impl QueryResult {
    /// Check if the result is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Get the number of rows.
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/// SQL query executor.
pub struct QueryExecutor {
    cache_dir: PathBuf,
}

impl QueryExecutor {
    /// Create a new query executor.
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }

    /// Substitute table aliases with read_parquet() calls.
    pub fn substitute_aliases(&self, sql: &str) -> Result<String> {
        let mut result = sql.to_string();
        let mut missing_tables = Vec::new();

        for (alias, filename) in TABLE_ALIASES {
            // Match table name as a whole word (case insensitive)
            // Use regex-like pattern matching with word boundaries
            let patterns = [
                format!(" FROM {} ", alias),
                format!(" FROM {}\n", alias),
                format!(" FROM {}\r", alias),
                format!(" FROM {}", alias), // At end
                format!(" JOIN {} ", alias),
                format!(" JOIN {}\n", alias),
                format!(" JOIN {}\r", alias),
                format!(" JOIN {}", alias),
                format!(",{} ", alias),
                format!(",{},", alias),
            ];

            let alias_upper = alias.to_uppercase();
            let patterns_upper = [
                format!(" FROM {} ", alias_upper),
                format!(" FROM {}\n", alias_upper),
                format!(" FROM {}\r", alias_upper),
                format!(" FROM {}", alias_upper),
                format!(" JOIN {} ", alias_upper),
                format!(" JOIN {}\n", alias_upper),
                format!(" JOIN {}\r", alias_upper),
                format!(" JOIN {}", alias_upper),
                format!(",{} ", alias_upper),
                format!(",{},", alias_upper),
            ];

            let parquet_path = self.cache_dir.join(filename);
            let replacement = format!(
                "read_parquet('{}')",
                parquet_path.to_string_lossy().replace('\\', "/")
            );

            // Check if this table is referenced
            let is_referenced = patterns.iter().any(|p| result.contains(p))
                || patterns_upper.iter().any(|p| result.contains(p))
                || result.to_lowercase().ends_with(&format!(" from {}", alias))
                || result.to_lowercase().ends_with(&format!(" join {}", alias));

            if is_referenced && !parquet_path.exists() {
                missing_tables.push(*alias);
            }

            // Perform substitutions (case-insensitive, whole word)
            for (pattern, pattern_upper) in patterns.iter().zip(patterns_upper.iter()) {
                if result.contains(pattern) {
                    let new_pattern = pattern.replace(alias, &replacement);
                    result = result.replace(pattern, &new_pattern);
                }
                if result.contains(pattern_upper) {
                    let new_pattern = pattern_upper.replace(&alias_upper, &replacement);
                    result = result.replace(pattern_upper, &new_pattern);
                }
            }

            // Handle end-of-string case
            let end_pattern = format!(" from {}", alias);
            let end_pattern_upper = format!(" FROM {}", alias_upper);
            if result.to_lowercase().ends_with(&end_pattern) {
                let len = result.len();
                let prefix = &result[..len - end_pattern.len()];
                result = format!("{} FROM {}", prefix, replacement);
            } else if result.ends_with(&end_pattern_upper) {
                let len = result.len();
                let prefix = &result[..len - end_pattern_upper.len()];
                result = format!("{} FROM {}", prefix, replacement);
            }
        }

        if !missing_tables.is_empty() {
            let table = missing_tables[0];
            let command = match table {
                "llms" => "aa llms",
                "text_to_image" => "aa text-to-image",
                "image_editing" => "aa image-editing",
                "text_to_speech" => "aa text-to-speech",
                "text_to_video" => "aa text-to-video",
                "image_to_video" => "aa image-to-video",
                _ => "aa <command>",
            };
            return Err(AppError::Query(format!(
                "Table '{}' not found. Run '{}' first to fetch and cache the data.",
                table, command
            )));
        }

        Ok(result)
    }

    /// Execute a SQL query and return the results.
    pub fn execute(&self, sql: &str) -> Result<QueryResult> {
        // Substitute table aliases
        let transformed_sql = self.substitute_aliases(sql)?;

        // Open in-memory connection
        let conn = Connection::open_in_memory()
            .map_err(|e| AppError::Query(format!("DuckDB error: {}", e)))?;

        // Execute the query
        let mut stmt = conn.prepare(&transformed_sql).map_err(|e| {
            // Provide helpful error messages for common mistakes
            let msg = e.to_string();
            if msg.contains("syntax error") {
                AppError::Query(format!("SQL syntax error: {}", msg))
            } else if msg.contains("does not exist") || msg.contains("not found") {
                AppError::Query(format!(
                    "Table or column not found. Use 'aa query --tables' to see available tables and columns.\nError: {}",
                    msg
                ))
            } else {
                AppError::Query(format!("SQL error: {}", msg))
            }
        })?;

        // Get column names
        let columns: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        // Fetch rows
        let mut rows = Vec::new();
        let mut query_rows = stmt
            .query([])
            .map_err(|e| AppError::Query(format!("Query error: {}", e)))?;

        while let Some(row) = query_rows
            .next()
            .map_err(|e| AppError::Query(format!("Row error: {}", e)))?
        {
            let mut row_values = Vec::new();
            for i in 0..columns.len() {
                let value: Value = row.get(i).unwrap_or(Value::Null);
                let string_value = match value {
                    Value::Null => String::new(),
                    Value::Boolean(b) => b.to_string(),
                    Value::TinyInt(n) => n.to_string(),
                    Value::SmallInt(n) => n.to_string(),
                    Value::Int(n) => n.to_string(),
                    Value::BigInt(n) => n.to_string(),
                    Value::HugeInt(n) => n.to_string(),
                    Value::UTinyInt(n) => n.to_string(),
                    Value::USmallInt(n) => n.to_string(),
                    Value::UInt(n) => n.to_string(),
                    Value::UBigInt(n) => n.to_string(),
                    Value::Float(f) => format!("{:.2}", f),
                    Value::Double(f) => format!("{:.2}", f),
                    Value::Text(s) => s,
                    _ => format!("{:?}", value),
                };
                row_values.push(string_value);
            }
            rows.push(row_values);
        }

        Ok(QueryResult { columns, rows })
    }

    /// List available tables with their schemas.
    pub fn list_tables(&self) -> Vec<TableInfo> {
        let mut tables = Vec::new();

        for (alias, filename) in TABLE_ALIASES {
            let parquet_path = self.cache_dir.join(filename);
            let exists = parquet_path.exists();
            let schema = get_schema(alias);

            tables.push(TableInfo {
                name: alias.to_string(),
                exists,
                schema: schema.map(|s| {
                    s.columns
                        .iter()
                        .map(|(name, typ, nullable)| ColumnInfo {
                            name: name.to_string(),
                            data_type: typ.to_string(),
                            nullable: *nullable,
                        })
                        .collect()
                }),
            });
        }

        tables
    }
}

/// Information about a table.
pub struct TableInfo {
    pub name: String,
    pub exists: bool,
    pub schema: Option<Vec<ColumnInfo>>,
}

/// Information about a column.
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

/// Format query results according to the output format.
pub fn format_query_result(result: &QueryResult, format: OutputFormat) -> String {
    if result.is_empty() {
        return "No results.".to_string();
    }

    match format {
        OutputFormat::Json => format_json(result),
        OutputFormat::Csv => format_csv(result),
        OutputFormat::Table => format_ascii_table(result),
        OutputFormat::Plain => format_plain(result),
        OutputFormat::Markdown => format_markdown(result),
    }
}

fn format_json(result: &QueryResult) -> String {
    let mut rows_json = Vec::new();
    for row in &result.rows {
        let mut obj = HashMap::new();
        for (i, col) in result.columns.iter().enumerate() {
            obj.insert(col.clone(), row.get(i).cloned().unwrap_or_default());
        }
        rows_json.push(obj);
    }
    serde_json::to_string_pretty(&rows_json).unwrap_or_else(|_| "Error formatting JSON".to_string())
}

fn format_csv(result: &QueryResult) -> String {
    let mut wtr = csv::Writer::from_writer(vec![]);
    let _ = wtr.write_record(&result.columns);
    for row in &result.rows {
        let _ = wtr.write_record(row);
    }
    wtr.into_inner()
        .map(|v| String::from_utf8_lossy(&v).to_string())
        .unwrap_or_else(|_| "Error formatting CSV".to_string())
}

fn format_ascii_table(result: &QueryResult) -> String {
    use std::fmt::Write;

    // Calculate column widths
    let mut widths: Vec<usize> = result.columns.iter().map(|c| c.len()).collect();
    for row in &result.rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() && cell.len() > widths[i] {
                widths[i] = cell.len();
            }
        }
    }

    let mut output = String::new();

    // Top border
    write!(output, "+").unwrap();
    for w in &widths {
        write!(output, "-{}-+", "-".repeat(*w)).unwrap();
    }
    writeln!(output).unwrap();

    // Header row
    write!(output, "|").unwrap();
    for (i, col) in result.columns.iter().enumerate() {
        write!(output, " {:width$} |", col, width = widths[i]).unwrap();
    }
    writeln!(output).unwrap();

    // Header separator
    write!(output, "+").unwrap();
    for w in &widths {
        write!(output, "-{}-+", "-".repeat(*w)).unwrap();
    }
    writeln!(output).unwrap();

    // Data rows
    for row in &result.rows {
        write!(output, "|").unwrap();
        for (i, cell) in row.iter().enumerate() {
            let w = widths.get(i).copied().unwrap_or(0);
            write!(output, " {:width$} |", cell, width = w).unwrap();
        }
        writeln!(output).unwrap();
    }

    // Bottom border
    write!(output, "+").unwrap();
    for w in &widths {
        write!(output, "-{}-+", "-".repeat(*w)).unwrap();
    }
    writeln!(output).unwrap();

    output
}

fn format_plain(result: &QueryResult) -> String {
    let mut output = String::new();
    for row in &result.rows {
        output.push_str(&row.join("\t"));
        output.push('\n');
    }
    output
}

fn format_markdown(result: &QueryResult) -> String {
    use std::fmt::Write;

    let mut output = String::new();

    // Header row
    write!(output, "| {} |", result.columns.join(" | ")).unwrap();
    writeln!(output).unwrap();

    // Separator row
    let separators: Vec<&str> = result.columns.iter().map(|_| "---").collect();
    write!(output, "| {} |", separators.join(" | ")).unwrap();
    writeln!(output).unwrap();

    // Data rows
    for row in &result.rows {
        write!(output, "| {} |", row.join(" | ")).unwrap();
        writeln!(output).unwrap();
    }

    output
}

/// Format table list for display.
pub fn format_tables_list(tables: &[TableInfo]) -> String {
    use std::fmt::Write;

    let mut output = String::new();

    writeln!(output, "Available tables:\n").unwrap();

    for table in tables {
        let status = if table.exists {
            "(cached)"
        } else {
            "(not cached)"
        };
        writeln!(output, "  {} {}", table.name, status).unwrap();

        if let Some(ref schema) = table.schema {
            writeln!(output, "    Columns:").unwrap();
            for col in schema {
                let nullable = if col.nullable { "NULL" } else { "NOT NULL" };
                writeln!(
                    output,
                    "      - {} {} {}",
                    col.name, col.data_type, nullable
                )
                .unwrap();
            }
        }
        writeln!(output).unwrap();
    }

    writeln!(output, "To cache a table, run the corresponding command:").unwrap();
    writeln!(output, "  aa llms           -> llms").unwrap();
    writeln!(output, "  aa text-to-image  -> text_to_image").unwrap();
    writeln!(output, "  aa image-editing  -> image_editing").unwrap();
    writeln!(output, "  aa text-to-speech -> text_to_speech").unwrap();
    writeln!(output, "  aa text-to-video  -> text_to_video").unwrap();
    writeln!(output, "  aa image-to-video -> image_to_video").unwrap();

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_substitute_aliases_basic() {
        let temp_dir = TempDir::new().unwrap();
        let executor = QueryExecutor::new(temp_dir.path().to_path_buf());

        // Create a dummy parquet file
        std::fs::write(temp_dir.path().join("llms.parquet"), b"dummy").unwrap();

        let sql = "SELECT * FROM llms WHERE intelligence > 40";
        let result = executor.substitute_aliases(sql).unwrap();

        assert!(result.contains("read_parquet("));
        assert!(result.contains("llms.parquet"));
        assert!(!result.contains("FROM llms"));
    }

    #[test]
    fn test_substitute_aliases_missing_table() {
        let temp_dir = TempDir::new().unwrap();
        let executor = QueryExecutor::new(temp_dir.path().to_path_buf());

        let sql = "SELECT * FROM llms";
        let result = executor.substitute_aliases(sql);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("not found"));
        assert!(err.contains("aa llms"));
    }

    #[test]
    fn test_list_tables() {
        let temp_dir = TempDir::new().unwrap();

        // Create one parquet file
        std::fs::write(temp_dir.path().join("llms.parquet"), b"dummy").unwrap();

        let executor = QueryExecutor::new(temp_dir.path().to_path_buf());
        let tables = executor.list_tables();

        assert_eq!(tables.len(), 6);

        let llms = tables.iter().find(|t| t.name == "llms").unwrap();
        assert!(llms.exists);

        let text_to_image = tables.iter().find(|t| t.name == "text_to_image").unwrap();
        assert!(!text_to_image.exists);
    }

    #[test]
    fn test_format_query_result_markdown() {
        let result = QueryResult {
            columns: vec!["name".to_string(), "score".to_string()],
            rows: vec![
                vec!["Model A".to_string(), "100".to_string()],
                vec!["Model B".to_string(), "95".to_string()],
            ],
        };

        let output = format_query_result(&result, OutputFormat::Markdown);
        assert!(output.contains("| name | score |"));
        assert!(output.contains("| Model A | 100 |"));
    }

    #[test]
    fn test_format_query_result_json() {
        let result = QueryResult {
            columns: vec!["name".to_string()],
            rows: vec![vec!["Test".to_string()]],
        };

        let output = format_query_result(&result, OutputFormat::Json);
        assert!(output.contains("\"name\""));
        assert!(output.contains("\"Test\""));
    }
}
