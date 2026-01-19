//! SQL query execution via DuckDB.
//!
//! Provides table name alias substitution and SQL execution against Parquet files.

use crate::error::{AppError, Result};
use crate::output::OutputFormat;
use crate::schema::{self, Column, ALL_TABLES};
use comfy_table::{presets::ASCII_BORDERS_ONLY_CONDENSED, Table};
use duckdb::arrow::array::Array;
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::Connection;
use sqlparser::ast::{ObjectName, Visit, Visitor};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::HashMap;
use std::ops::ControlFlow;
use std::path::PathBuf;

/// Schema information for a table (used for --tables display).
pub struct TableSchema {
    pub name: &'static str,
    pub columns: &'static [Column],
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

    /// Substitute table aliases with read_parquet() calls using AST parsing.
    /// Falls back to string replacement if parsing fails.
    pub fn substitute_aliases(&self, sql: &str) -> Result<String> {
        // Try AST-based substitution first
        match self.substitute_aliases_ast(sql) {
            Ok(result) => Ok(result),
            Err(_) => {
                // Fall back to string-based substitution
                self.substitute_aliases_string(sql)
            }
        }
    }

    /// AST-based table substitution - identifies table positions via AST, then does targeted replacement.
    fn substitute_aliases_ast(&self, sql: &str) -> Result<String> {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, sql)
            .map_err(|e| AppError::Query(format!("SQL parse error: {}", e)))?;

        // Collect table names that need substitution
        let mut table_names: Vec<String> = Vec::new();
        for statement in &ast {
            let mut visitor = TableNameCollector { tables: Vec::new() };
            let _ = statement.visit(&mut visitor);
            table_names.extend(visitor.tables);
        }

        // Check for missing tables and build replacement map
        let mut missing_tables = Vec::new();
        let mut replacements: HashMap<String, String> = HashMap::new();

        for table_name in &table_names {
            let lower = table_name.to_lowercase();
            if let Some(table_def) = schema::get_table_def(&lower) {
                let parquet_path = self.cache_dir.join(table_def.parquet_file);
                if !parquet_path.exists() {
                    missing_tables.push(table_def);
                } else {
                    let path_str = parquet_path.to_string_lossy().replace('\\', "/");
                    replacements.insert(lower.clone(), format!("read_parquet('{}')", path_str));
                }
            }
        }

        if !missing_tables.is_empty() {
            let table = missing_tables[0];
            return Err(AppError::Query(format!(
                "Table '{}' not found. Run '{}' first to fetch and cache the data.",
                table.name, table.command
            )));
        }

        // Now do targeted replacement using the AST to find exact positions
        // We regenerate SQL from AST with substitutions
        let mut result = sql.to_string();

        // Sort by length descending to avoid replacing substrings
        let mut sorted_replacements: Vec<_> = replacements.iter().collect();
        sorted_replacements.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        for (table_name, replacement) in sorted_replacements {
            // Use case-insensitive word boundary replacement
            result = replace_table_name_safe(&result, table_name, replacement);
        }

        Ok(result)
    }

    /// String-based table substitution (fallback).
    fn substitute_aliases_string(&self, sql: &str) -> Result<String> {
        let mut result = sql.to_string();
        let mut missing_tables = Vec::new();

        for table_def in ALL_TABLES {
            let alias = table_def.name;

            // Match table name as a whole word (case insensitive)
            let patterns = [
                format!(" FROM {} ", alias),
                format!(" FROM {}\n", alias),
                format!(" FROM {}\r", alias),
                format!(" FROM {}", alias),
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

            let parquet_path = self.cache_dir.join(table_def.parquet_file);
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
                missing_tables.push(table_def);
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
            return Err(AppError::Query(format!(
                "Table '{}' not found. Run '{}' first to fetch and cache the data.",
                table.name, table.command
            )));
        }

        Ok(result)
    }

    /// Execute a SQL query and return the results.
    pub fn execute(&self, sql: &str) -> Result<QueryResult> {
        // Validate SQL syntax first using sqlparser to catch syntax errors
        // before they reach DuckDB (which can throw uncatchable C++ exceptions)
        let dialect = GenericDialect {};
        Parser::parse_sql(&dialect, sql)
            .map_err(|e| AppError::Query(format!("SQL syntax error: {}", e)))?;

        // Substitute table aliases
        let transformed_sql = self.substitute_aliases(sql)?;

        // Execute the query
        let batches = execute_duckdb_query(&transformed_sql)?;

        if batches.is_empty() {
            return Ok(QueryResult {
                columns: vec![],
                rows: vec![],
            });
        }

        // Get column names from the first batch's schema
        let schema = batches[0].schema();
        let columns: Vec<String> = schema.fields().iter().map(|f| f.name().clone()).collect();

        // Convert Arrow arrays to strings
        let mut rows = Vec::new();
        for batch in &batches {
            let num_rows = batch.num_rows();
            for row_idx in 0..num_rows {
                let mut row_values = Vec::new();
                for col_idx in 0..batch.num_columns() {
                    let col = batch.column(col_idx);
                    let string_value = arrow_value_to_string(col.as_ref(), row_idx);
                    row_values.push(string_value);
                }
                rows.push(row_values);
            }
        }

        Ok(QueryResult { columns, rows })
    }

    /// List available tables with their schemas.
    pub fn list_tables(&self) -> Vec<TableInfo> {
        let mut tables = Vec::new();

        for table_def in ALL_TABLES {
            let parquet_path = self.cache_dir.join(table_def.parquet_file);
            let exists = parquet_path.exists();

            tables.push(TableInfo {
                name: table_def.name.to_string(),
                exists,
                schema: Some(
                    table_def
                        .columns
                        .iter()
                        .map(|col| ColumnInfo {
                            name: col.name.to_string(),
                            data_type: col.sql_type.to_string(),
                            nullable: col.nullable,
                        })
                        .collect(),
                ),
            });
        }

        tables
    }
}

/// Execute a DuckDB query and return the results as Arrow RecordBatches.
fn execute_duckdb_query(sql: &str) -> Result<Vec<RecordBatch>> {
    // Open in-memory connection
    let conn = Connection::open_in_memory()
        .map_err(|e| AppError::Query(format!("DuckDB error: {}", e)))?;

    // Execute the query and get results as Arrow RecordBatches
    let mut stmt = conn.prepare(sql).map_err(|e| {
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

    // Use query_arrow to get results as Arrow RecordBatches
    let batches: Vec<RecordBatch> = stmt
        .query_arrow([])
        .map_err(|e| AppError::Query(format!("Query error: {}", e)))?
        .collect();

    Ok(batches)
}

/// Convert an Arrow array value at a given row index to a string.
/// Uses Arrow's built-in display formatting to handle all types safely.
fn arrow_value_to_string(array: &dyn Array, row: usize) -> String {
    use duckdb::arrow::array::*;
    use duckdb::arrow::datatypes::*;

    if array.is_null(row) {
        return String::new();
    }

    match array.data_type() {
        DataType::Null => String::new(),
        DataType::Boolean => {
            let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
            arr.value(row).to_string()
        }
        DataType::Int8 => {
            let arr = array.as_any().downcast_ref::<Int8Array>().unwrap();
            arr.value(row).to_string()
        }
        DataType::Int16 => {
            let arr = array.as_any().downcast_ref::<Int16Array>().unwrap();
            arr.value(row).to_string()
        }
        DataType::Int32 => {
            let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
            arr.value(row).to_string()
        }
        DataType::Int64 => {
            let arr = array.as_any().downcast_ref::<Int64Array>().unwrap();
            arr.value(row).to_string()
        }
        DataType::UInt8 => {
            let arr = array.as_any().downcast_ref::<UInt8Array>().unwrap();
            arr.value(row).to_string()
        }
        DataType::UInt16 => {
            let arr = array.as_any().downcast_ref::<UInt16Array>().unwrap();
            arr.value(row).to_string()
        }
        DataType::UInt32 => {
            let arr = array.as_any().downcast_ref::<UInt32Array>().unwrap();
            arr.value(row).to_string()
        }
        DataType::UInt64 => {
            let arr = array.as_any().downcast_ref::<UInt64Array>().unwrap();
            arr.value(row).to_string()
        }
        DataType::Float16 | DataType::Float32 => {
            let arr = array.as_any().downcast_ref::<Float32Array>().unwrap();
            format!("{:.2}", arr.value(row))
        }
        DataType::Float64 => {
            let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
            format!("{:.2}", arr.value(row))
        }
        DataType::Utf8 => {
            let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
            arr.value(row).to_string()
        }
        DataType::LargeUtf8 => {
            let arr = array.as_any().downcast_ref::<LargeStringArray>().unwrap();
            arr.value(row).to_string()
        }
        DataType::Binary => {
            let arr = array.as_any().downcast_ref::<BinaryArray>().unwrap();
            format!("<binary:{} bytes>", arr.value(row).len())
        }
        DataType::LargeBinary => {
            let arr = array.as_any().downcast_ref::<LargeBinaryArray>().unwrap();
            format!("<binary:{} bytes>", arr.value(row).len())
        }
        DataType::Date32 => {
            let arr = array.as_any().downcast_ref::<Date32Array>().unwrap();
            // Date32 is days since epoch
            let days = arr.value(row);
            format!("{}", days)
        }
        DataType::Date64 => {
            let arr = array.as_any().downcast_ref::<Date64Array>().unwrap();
            let ms = arr.value(row);
            format!("{}", ms)
        }
        DataType::Timestamp(unit, _) => match unit {
            TimeUnit::Second => {
                let arr = array
                    .as_any()
                    .downcast_ref::<TimestampSecondArray>()
                    .unwrap();
                arr.value(row).to_string()
            }
            TimeUnit::Millisecond => {
                let arr = array
                    .as_any()
                    .downcast_ref::<TimestampMillisecondArray>()
                    .unwrap();
                arr.value(row).to_string()
            }
            TimeUnit::Microsecond => {
                let arr = array
                    .as_any()
                    .downcast_ref::<TimestampMicrosecondArray>()
                    .unwrap();
                arr.value(row).to_string()
            }
            TimeUnit::Nanosecond => {
                let arr = array
                    .as_any()
                    .downcast_ref::<TimestampNanosecondArray>()
                    .unwrap();
                arr.value(row).to_string()
            }
        },
        DataType::Decimal128(_, scale) => {
            let arr = array.as_any().downcast_ref::<Decimal128Array>().unwrap();
            let value = arr.value(row);
            if *scale == 0 {
                value.to_string()
            } else {
                // Format with decimal places
                let divisor = 10i128.pow(*scale as u32);
                let int_part = value / divisor;
                let frac_part = (value % divisor).abs();
                format!(
                    "{}.{:0>width$}",
                    int_part,
                    frac_part,
                    width = *scale as usize
                )
            }
        }
        // For complex types, use Arrow's display formatting
        _ => {
            // Use the ArrayFormatter for complex types
            use duckdb::arrow::util::display::ArrayFormatter;
            let options = duckdb::arrow::util::display::FormatOptions::default();
            match ArrayFormatter::try_new(array, &options) {
                Ok(formatter) => formatter.value(row).to_string(),
                Err(_) => format!("<{}>", array.data_type()),
            }
        }
    }
}

/// Replace table name in SQL, being careful not to replace inside string literals.
fn replace_table_name_safe(sql: &str, table_name: &str, replacement: &str) -> String {
    let mut result = String::new();
    let chars = sql.chars();
    let mut in_string = false;
    let mut string_char = '"';
    let mut current_word = String::new();

    for ch in chars {
        // Track string literals
        if !in_string && (ch == '\'' || ch == '"') {
            in_string = true;
            string_char = ch;
            // Flush any accumulated word
            if !current_word.is_empty() {
                if current_word.to_lowercase() == table_name {
                    result.push_str(replacement);
                } else {
                    result.push_str(&current_word);
                }
                current_word.clear();
            }
            result.push(ch);
        } else if in_string && ch == string_char {
            in_string = false;
            result.push(ch);
        } else if in_string {
            result.push(ch);
        } else if ch.is_alphanumeric() || ch == '_' {
            current_word.push(ch);
        } else {
            // End of word, check if it's a table name
            if !current_word.is_empty() {
                if current_word.to_lowercase() == table_name {
                    result.push_str(replacement);
                } else {
                    result.push_str(&current_word);
                }
                current_word.clear();
            }
            result.push(ch);
        }
    }

    // Handle any remaining word
    if !current_word.is_empty() {
        if current_word.to_lowercase() == table_name {
            result.push_str(replacement);
        } else {
            result.push_str(&current_word);
        }
    }

    result
}

/// Visitor to collect table names from SQL AST.
struct TableNameCollector {
    tables: Vec<String>,
}

impl Visitor for TableNameCollector {
    type Break = ();

    fn pre_visit_relation(&mut self, relation: &ObjectName) -> ControlFlow<Self::Break> {
        if let Some(ident) = relation.0.first() {
            self.tables.push(ident.value.clone());
        }
        ControlFlow::Continue(())
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
    let mut table = Table::new();
    table.load_preset(ASCII_BORDERS_ONLY_CONDENSED);
    table.set_header(&result.columns);

    for row in &result.rows {
        table.add_row(row);
    }

    table.to_string()
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
    for table_def in ALL_TABLES {
        writeln!(output, "  {} -> {}", table_def.command, table_def.name).unwrap();
    }

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
    fn test_substitute_aliases_string_literal_not_replaced() {
        let temp_dir = TempDir::new().unwrap();
        let executor = QueryExecutor::new(temp_dir.path().to_path_buf());

        // Create a dummy parquet file
        std::fs::write(temp_dir.path().join("llms.parquet"), b"dummy").unwrap();

        // String literal should NOT be replaced
        let sql = "SELECT 'llms' AS table_name FROM llms";
        let result = executor.substitute_aliases(sql).unwrap();

        // The string literal 'llms' should be preserved
        assert!(result.contains("'llms'"));
        // But the table reference should be replaced
        assert!(result.contains("read_parquet("));
    }

    #[test]
    fn test_replace_table_name_safe() {
        // Test that string literals are preserved
        let sql = "SELECT 'llms' FROM llms";
        let result = replace_table_name_safe(sql, "llms", "REPLACED");
        assert_eq!(result, "SELECT 'llms' FROM REPLACED");

        // Test case insensitivity
        let sql = "SELECT * FROM LLMS";
        let result = replace_table_name_safe(sql, "llms", "REPLACED");
        assert_eq!(result, "SELECT * FROM REPLACED");
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

    #[test]
    fn test_format_query_result_table() {
        let result = QueryResult {
            columns: vec!["name".to_string(), "score".to_string()],
            rows: vec![vec!["Model A".to_string(), "100".to_string()]],
        };

        let output = format_query_result(&result, OutputFormat::Table);
        assert!(output.contains("name"));
        assert!(output.contains("score"));
        assert!(output.contains("Model A"));
        assert!(output.contains("100"));
    }
}
