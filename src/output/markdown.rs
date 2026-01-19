//! Markdown table formatter (default output).

use std::fmt::Write;

/// Generate a markdown table from headers and rows.
pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return "No data available.".into();
    }

    let mut output = String::new();

    // Header row
    write!(output, "| {} |", headers.join(" | ")).unwrap();
    writeln!(output).unwrap();

    // Separator row
    let separators: Vec<&str> = headers.iter().map(|_| "---").collect();
    write!(output, "| {} |", separators.join(" | ")).unwrap();
    writeln!(output).unwrap();

    // Data rows
    for row in rows {
        let cells: Vec<&str> = row.iter().map(|s| s.as_str()).collect();
        write!(output, "| {} |", cells.join(" | ")).unwrap();
        writeln!(output).unwrap();
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_table() {
        let headers = &["Name", "Score"];
        let rows = vec![
            vec!["Model A".into(), "100".into()],
            vec!["Model B".into(), "95".into()],
        ];

        let result = format_table(headers, &rows);
        assert!(result.contains("| Name | Score |"));
        assert!(result.contains("| --- | --- |"));
        assert!(result.contains("| Model A | 100 |"));
    }

    #[test]
    fn test_format_table_empty() {
        let headers = &["Name"];
        let rows: Vec<Vec<String>> = vec![];

        let result = format_table(headers, &rows);
        assert_eq!(result, "No data available.");
    }
}
