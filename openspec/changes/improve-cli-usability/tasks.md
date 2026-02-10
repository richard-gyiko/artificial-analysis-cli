# Implementation Tasks

## 1. Add `refresh` Command
- [x] 1.1 Create `src/commands/refresh.rs`
- [x] 1.2 Refresh benchmarks (AA data)
- [x] 1.3 Refresh models (models.dev data)
- [x] 1.4 Print progress: "Refreshing benchmarks... done (N models)"
- [x] 1.5 Print progress: "Refreshing models... done (N entries)"
- [x] 1.6 Add to CLI in `src/cli/mod.rs`

## 2. Add `tables` Command
- [x] 2.1 Move `query --tables` logic to standalone `tables` command
- [x] 2.2 Show table name, row count, and column list
- [x] 2.3 Indicate which tables are cached vs not cached
- [ ] ~~2.4 Keep `query --tables` as alias for backward compat~~ (Removed - not needed for SQL-first approach)

## 3. Remove `llms` Command
- [x] 3.1 Delete `src/commands/llms.rs`
- [x] 3.2 Remove from CLI enum in `src/cli/mod.rs`
- [x] 3.3 Remove from `src/main.rs`

## 4. Remove Media Commands
- [x] 4.1 Delete `src/commands/media.rs`
- [x] 4.2 Remove media commands from CLI: text-to-image, image-editing, text-to-speech, text-to-video, image-to-video
- [x] 4.3 Keep media tables queryable via SQL

## 5. Improve Error Messages
- [x] 5.1 When table not cached, suggest `which-llm refresh`
- [x] 5.2 When query fails, show available tables

## 6. Update `compare` and `cost` Commands
- [x] 6.1 Keep using hosted client (still works with new CLI structure)
- [x] 6.2 Ensure they still work with fuzzy model name matching

## 7. Documentation
- [x] 7.1 Update README with new CLI structure
- [x] 7.2 Update skill documentation (SKILL.md)
- [x] 7.3 Update `--help` text for all commands

## 8. Testing
- [ ] 8.1 Run `cargo test` to verify all tests pass
- [x] 8.2 Update tests for deleted commands (tests/cli.rs)
- [x] 8.3 Manually verify `tables`, `query`, `refresh` commands work

## 9. Finalize
- [ ] 9.1 Run `cargo test` (blocked by slow DuckDB compilation)
- [ ] 9.2 Commit changes
- [ ] 9.3 Create PR or update existing PR #7
