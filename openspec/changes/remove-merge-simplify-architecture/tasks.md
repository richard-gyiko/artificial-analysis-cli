# Implementation Tasks

## 1. Remove Merge Module
- [x] 1.1 Delete `src/merge/matcher.rs`
- [x] 1.2 Delete `src/merge/combiner.rs`
- [x] 1.3 Delete `src/merge/mod.rs`
- [x] 1.4 Remove `mod merge;` from `src/lib.rs` or `src/main.rs`
- [x] 1.5 Fix any compilation errors from removed imports

## 2. Simplify LlmModel Struct
- [x] 2.1 Remove capability fields from `LlmModel` in `src/models/llm.rs`:
  - `reasoning`, `tool_call`, `structured_output`, `attachment`, `temperature`
  - `context_window`, `max_input_tokens`, `max_output_tokens`
  - `input_modalities`, `output_modalities`
  - `knowledge_cutoff`, `open_weights`, `last_updated`
  - `models_dev_matched`
- [x] 2.2 Update `LlmModel` documentation to clarify it's AA-only
- [x] 2.3 Fix any code that references removed fields

## 3. Update Parquet Writer
- [x] 3.1 Simplify parquet writer to write only AA fields
- [x] 3.2 Remove merge logic from parquet generation
- [x] 3.3 Rename to `benchmarks.parquet` (from `llms.parquet`)
- [x] 3.4 Update parquet tests

## 4. Simplify Cache Management
- [x] 4.1 Remove `models_dev_cache.json` handling from `src/client/mod.rs`
- [x] 4.2 Remove `models_dev_meta.txt` handling
- [x] 4.3 Update cache refresh logic to write two independent files:
  - `benchmarks.parquet` (from AA API)
  - `models.parquet` (from models.dev API)
- [x] 4.4 Use file mtime for staleness checks instead of meta file
- [x] 4.5 Remove merge step from update flow

## 5. Update Schema
- [x] 5.1 Rename `LLMS` → `BENCHMARKS` table definition
- [x] 5.2 Rename `PROVIDERS` → `MODELS` table definition
- [x] 5.3 Update `get_table_def()` in `src/schema.rs`
- [x] 5.4 Update table documentation/comments

## 6. Update CLI Commands
- [x] 6.1 Remove capability filter flags from `llms` command:
  - `--reasoning`
  - `--tool-call`
  - `--structured-output`
  - `--attachment`
  - `--min-context`
  - `--modality`
- [x] 6.2 Update help text to reference `models` table for capabilities
- [x] 6.3 Update query hints to use `benchmarks` table name

## 7. Update Skill Documentation
- [x] 7.1 Update `skills/which-llm/SKILL.md` with:
  - Clear separation of table purposes (`benchmarks` vs `models`)
  - When to use each table
  - Examples of cross-table queries
  - Guidance on fuzzy matching model names
- [x] 7.2 Add example queries for common scenarios
- [x] 7.3 Document that LLM should handle name matching

## 8. Testing
- [x] 8.1 Update tests in `src/query.rs` (table name references)
- [x] 8.2 Update tests in `tests/cli.rs` (table name references)
- [x] 8.3 Update parquet writer tests
- [x] 8.4 Verify `which-llm query "SELECT * FROM benchmarks LIMIT 5"` works
- [x] 8.5 Verify `which-llm query "SELECT * FROM models LIMIT 5"` works
- [x] 8.6 Verify `which-llm llms --refresh` creates `benchmarks.parquet`
- [ ] 8.7 Verify no JSON cache files are created

## 9. Cleanup
- [x] 9.1 Remove any unused imports across codebase
- [ ] 9.2 Run `cargo clippy` and fix warnings
- [x] 9.3 Run `cargo fmt`
- [ ] 9.4 Run full test suite

## 10. Documentation
- [x] 10.1 Update README to document two-table architecture
- [ ] 10.2 Add migration notes for users with existing cache
- [x] 10.3 Document removed CLI flags

---

## Notes

**Table Naming (Updated):**
- `benchmarks` - Artificial Analysis data (intelligence, coding, price, tps, latency)
- `models` - models.dev data (capabilities, context limits, provider info)

**Compilation:** Run `cargo build --release` after major changes. The libduckdb-sys crate takes a very long time to compile from source.

**Testing locally:**
```bash
cargo test
which-llm llms --refresh
which-llm query "SELECT COUNT(*) FROM benchmarks"
which-llm query "SELECT COUNT(*) FROM models"
```

**Expected results:**
- `benchmarks` table: ~200 models (AA data only)
- `models` table: ~2,500 entries (models.dev data)
