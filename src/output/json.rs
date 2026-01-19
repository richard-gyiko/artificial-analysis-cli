//! JSON formatter.

use serde::Serialize;

/// Format data as pretty-printed JSON.
pub fn format_json<T: Serialize + ?Sized>(data: &T) -> String {
    serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting JSON".into())
}
