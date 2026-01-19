//! CSV formatter.

/// Format data as CSV.
pub fn format_csv(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut wtr = csv::Writer::from_writer(vec![]);

    // Write headers
    let _ = wtr.write_record(headers);

    // Write rows
    for row in rows {
        let _ = wtr.write_record(row);
    }

    wtr.into_inner()
        .map(|v| String::from_utf8_lossy(&v).to_string())
        .unwrap_or_else(|_| "Error formatting CSV".into())
}
