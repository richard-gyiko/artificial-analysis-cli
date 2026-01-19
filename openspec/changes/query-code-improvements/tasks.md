## 1. Centralized Schema Definitions
- [x] 1.1 Create `src/schema.rs` with `Column` and `TableDef` structs
- [x] 1.2 Define all table schemas (llms, text_to_image, etc.) as constants
- [x] 1.3 Add helper methods: `create_table_sql()`, `to_table_schema()`
- [x] 1.4 Update `src/parquet.rs` to use schemas from `schema.rs`
- [x] 1.5 Update `src/query.rs` to use schemas from `schema.rs`
- [x] 1.6 Remove duplicated schema definitions

## 2. Robust Alias Substitution
- [x] 2.1 Add `sqlparser` dependency to Cargo.toml
- [x] 2.2 Implement AST-based table substitution in `query.rs`
- [x] 2.3 Fallback to string replacement if parsing fails (with warning)
- [x] 2.4 Add tests for edge cases (strings, quoted identifiers, subqueries)

## 3. Better Table Formatting
- [x] 3.1 Add `comfy-table` dependency to Cargo.toml
- [x] 3.2 Replace manual `format_ascii_table` with comfy-table
- [x] 3.3 Verify output format matches or improves on current

## 4. Bulk Insert Performance
- [x] 4.1 Replace row-by-row INSERT with DuckDB Appender in `parquet.rs`
- [x] 4.2 Verify Parquet files are written correctly

## 5. Error Handling
- [x] 5.1 Review DuckDB error types for better error categorization
- [x] 5.2 Replace string matching with error code checks where possible
  - Note: DuckDB Rust crate does not expose detailed error codes; string matching is the appropriate approach.
    The current implementation categorizes errors by content (syntax errors, not found, etc.) which provides
    good user experience.

## 6. Testing & Verification
- [x] 6.1 Run existing tests - all should pass
- [x] 6.2 Add tests for alias substitution edge cases
- [x] 6.3 Verify `aa query --tables` matches actual schema
- [x] 6.4 Manual testing of query command
