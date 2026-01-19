//! Plain text formatter (tab-separated values).

/// Format data as tab-separated values without headers.
pub fn format_plain(rows: &[Vec<String>]) -> String {
    rows.iter()
        .map(|row| row.join("\t"))
        .collect::<Vec<_>>()
        .join("\n")
}
