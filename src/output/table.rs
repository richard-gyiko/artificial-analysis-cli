//! ASCII table formatter.

use tabled::{Table, Tabled};

/// Format data as an ASCII table.
pub fn format_table<T: Tabled>(data: &[T]) -> String {
    if data.is_empty() {
        return "No data available.".into();
    }
    Table::new(data).to_string()
}
