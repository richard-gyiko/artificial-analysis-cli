//! Token count parsing utilities.
//!
//! Parses human-friendly token counts like "10k", "1.5M", "10000".

use crate::error::{AppError, Result};

/// Parse a token count string with optional unit suffix.
///
/// Supported formats:
/// - Plain numbers: "10000" -> 10,000
/// - Thousands: "10k" or "10K" -> 10,000
/// - Millions: "1m" or "1M" -> 1,000,000
/// - Decimals: "1.5M" -> 1,500,000
///
/// # Examples
///
/// ```
/// use which_llm::utils::parse_tokens;
///
/// assert_eq!(parse_tokens("10000").unwrap(), 10000);
/// assert_eq!(parse_tokens("10k").unwrap(), 10000);
/// assert_eq!(parse_tokens("1M").unwrap(), 1000000);
/// assert_eq!(parse_tokens("1.5M").unwrap(), 1500000);
/// ```
pub fn parse_tokens(s: &str) -> Result<u64> {
    let s = s.trim();

    if s.is_empty() {
        return Err(AppError::Config("Empty token count".into()));
    }

    // Check for suffix
    let (num_part, multiplier) = if let Some(num) = s.strip_suffix(['k', 'K']) {
        (num, 1_000.0)
    } else if let Some(num) = s.strip_suffix(['m', 'M']) {
        (num, 1_000_000.0)
    } else if let Some(num) = s.strip_suffix(['b', 'B']) {
        (num, 1_000_000_000.0)
    } else {
        (s, 1.0)
    };

    // Parse the numeric part
    let value: f64 = num_part.parse().map_err(|_| {
        AppError::Config(format!(
            "Invalid token count '{}': could not parse number",
            s
        ))
    })?;

    if value < 0.0 {
        return Err(AppError::Config(format!(
            "Invalid token count '{}': must be positive",
            s
        )));
    }

    let result = (value * multiplier).round() as u64;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_numbers() {
        assert_eq!(parse_tokens("0").unwrap(), 0);
        assert_eq!(parse_tokens("100").unwrap(), 100);
        assert_eq!(parse_tokens("10000").unwrap(), 10000);
        assert_eq!(parse_tokens("1000000").unwrap(), 1000000);
    }

    #[test]
    fn test_thousands() {
        assert_eq!(parse_tokens("1k").unwrap(), 1000);
        assert_eq!(parse_tokens("10k").unwrap(), 10000);
        assert_eq!(parse_tokens("10K").unwrap(), 10000);
        assert_eq!(parse_tokens("100k").unwrap(), 100000);
        assert_eq!(parse_tokens("1.5k").unwrap(), 1500);
    }

    #[test]
    fn test_millions() {
        assert_eq!(parse_tokens("1m").unwrap(), 1000000);
        assert_eq!(parse_tokens("1M").unwrap(), 1000000);
        assert_eq!(parse_tokens("10M").unwrap(), 10000000);
        assert_eq!(parse_tokens("1.5M").unwrap(), 1500000);
        assert_eq!(parse_tokens("0.5M").unwrap(), 500000);
    }

    #[test]
    fn test_billions() {
        assert_eq!(parse_tokens("1b").unwrap(), 1000000000);
        assert_eq!(parse_tokens("1B").unwrap(), 1000000000);
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(parse_tokens(" 10k ").unwrap(), 10000);
        assert_eq!(parse_tokens("  1M  ").unwrap(), 1000000);
    }

    #[test]
    fn test_invalid() {
        assert!(parse_tokens("").is_err());
        assert!(parse_tokens("abc").is_err());
        assert!(parse_tokens("10x").is_err());
        assert!(parse_tokens("-10k").is_err());
    }
}
