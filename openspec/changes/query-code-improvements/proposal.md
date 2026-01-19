# Query Code Improvements

## Summary

Address code quality issues identified in the SQL query feature review:
1. Centralize schema definitions to prevent drift
2. Improve alias substitution robustness
3. Minor improvements: better table formatting, bulk inserts, error handling

## Motivation

The current implementation has schema definitions spread across multiple files:
- `src/parquet.rs`: `CREATE TABLE` SQL strings, `LlmRow`/`MediaRow` structs
- `src/query.rs`: `TableSchema` constants for `--tables` display

Adding a new column requires changes in 5+ places, risking inconsistency where `aa query --tables` shows different columns than what's actually in the Parquet file.

Additionally, the alias substitution is naive string replacement that breaks on edge cases like `SELECT 'FROM llms'`.

## Design

### 1. Centralized Schema Definitions

Create `src/schema.rs` with a single source of truth:

```rust
pub struct Column {
    pub name: &'static str,
    pub sql_type: &'static str,
    pub nullable: bool,
}

pub struct TableDef {
    pub name: &'static str,
    pub command: &'static str,  // CLI command to populate
    pub columns: &'static [Column],
}

pub const LLMS: TableDef = TableDef {
    name: "llms",
    command: "aa llms",
    columns: &[
        Column { name: "id", sql_type: "VARCHAR", nullable: false },
        Column { name: "name", sql_type: "VARCHAR", nullable: false },
        // ... etc
    ],
};
```

Generate from this:
- `CREATE TABLE` SQL for DuckDB
- `TableSchema` for `--tables` display
- Parquet schema validation

### 2. Robust Alias Substitution

Use the `sqlparser` crate to properly identify table references:
- Parse SQL into AST
- Walk AST to find table references
- Substitute only actual table names, not strings or identifiers

Fallback: If parsing fails (non-standard SQL), use current string replacement with a warning.

### 3. Minor Improvements

- **comfy-table**: Replace manual `format_ascii_table` with `comfy-table` crate
- **Bulk inserts**: Use DuckDB `Appender` instead of row-by-row INSERTs
- **Error handling**: Check for DuckDB error codes where available

## Files Changed

- `src/schema.rs` (new) - Centralized schema definitions
- `src/parquet.rs` - Use schema from `schema.rs`, use Appender
- `src/query.rs` - Use schema from `schema.rs`, use sqlparser, use comfy-table
- `src/lib.rs` - Export schema module
- `Cargo.toml` - Add `sqlparser`, `comfy-table` dependencies

## Testing

- Existing tests should continue to pass
- Add test for alias substitution edge cases (strings, quoted identifiers)
- Verify `--tables` output matches actual Parquet schema
