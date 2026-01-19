## 1. DuckDB Integration
- [x] 1.1 Add DuckDB dependency to Cargo.toml with `bundled` and `parquet` features
- [x] 1.2 Create `src/query.rs` module for DuckDB operations
- [x] 1.3 Implement in-memory DuckDB connection management
- [x] 1.4 Implement table name alias substitution (replace `llms` with `read_parquet('/path/llms.parquet')`)
- [x] 1.5 Add unit tests for basic DuckDB operations and alias substitution

## 2. Parquet Cache Storage
- [x] 2.1 Add Parquet serialization for LLM models (flatten nested structure)
- [x] 2.2 Add Parquet serialization for media models
- [x] 2.3 Update `Cache` to write Parquet files instead of JSON
- [x] 2.4 Implement schema for `llms` table (see proposal for columns)
- [x] 2.5 Implement schema for media tables (`text_to_image`, etc.)
- [x] 2.6 ~~Add migration logic~~ N/A - Parquet written fresh on each fetch, no migration needed

## 3. Query Command
- [x] 3.1 Add `query` subcommand to CLI
- [x] 3.2 Execute user SQL query with table alias substitution
- [x] 3.3 Format results according to output flags (markdown, json, csv, table, plain)
- [x] 3.4 Handle SQL errors with user-friendly messages
- [x] 3.5 Handle missing Parquet files gracefully (suggest fetching data first)
- [x] 3.6 Add `--tables` flag to list available table aliases and schemas

## 4. Testing
- [x] 4.1 Integration tests for `aa query` with various SQL queries
- [x] 4.2 Test output format flags with query results
- [x] 4.3 Test error handling (invalid SQL, missing tables)
- [x] 4.4 ~~Test cache migration~~ N/A - no migration needed
- [x] 4.5 Verify existing commands still work (`aa llms`, `aa text-to-image`, etc.)

## 5. Documentation
- [x] 5.1 Update README with query command documentation
- [x] 5.2 Document available table aliases and their schemas
- [x] 5.3 Add example queries for common use cases
- [x] 5.4 Document output format options for query command

## 6. Polish
- [x] 6.1 Add `aa query --tables` to show available tables with schemas
- [x] 6.2 Add helpful error messages for common SQL mistakes
- [x] 6.3 Add hint about `aa query` after data command output (unless --quiet)
