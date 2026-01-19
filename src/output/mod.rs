//! Output formatting module.

pub mod csv;
pub mod json;
pub mod markdown;
pub mod plain;
pub mod table;

use serde::Serialize;
use tabled::Tabled;

/// Output format selection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OutputFormat {
    #[default]
    Markdown,
    Json,
    Csv,
    Table,
    Plain,
}

/// Trait for types that can be formatted for output.
pub trait Formattable: Serialize + Tabled {
    /// Get the headers for table/markdown/csv output.
    fn headers() -> &'static [&'static str];

    /// Convert to a row of strings.
    fn to_row(&self) -> Vec<String>;
}

/// Format a collection of items.
pub fn format_output<T: Formattable>(data: &[T], format: OutputFormat) -> String {
    match format {
        OutputFormat::Markdown => {
            let rows: Vec<Vec<String>> = data.iter().map(|item| item.to_row()).collect();
            markdown::format_table(<T as Formattable>::headers(), &rows)
        }
        OutputFormat::Json => json::format_json(&data),
        OutputFormat::Csv => {
            let rows: Vec<Vec<String>> = data.iter().map(|item| item.to_row()).collect();
            csv::format_csv(<T as Formattable>::headers(), &rows)
        }
        OutputFormat::Table => table::format_table(data),
        OutputFormat::Plain => {
            let rows: Vec<Vec<String>> = data.iter().map(|item| item.to_row()).collect();
            plain::format_plain(&rows)
        }
    }
}
