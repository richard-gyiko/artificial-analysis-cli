# Implementation Tasks

## 1. Code Restructure - Source Isolation
- [x] 1.1 Create `src/sources/mod.rs` with module exports
- [x] 1.2 Create `src/sources/artificial_analysis/` folder
- [x] 1.3 Move/refactor AA client code to `sources/artificial_analysis/client.rs`
- [x] 1.4 Move/refactor AA models to `sources/artificial_analysis/models.rs`
- [x] 1.5 Create `sources/artificial_analysis/schema.rs` for AA-specific Parquet schema
- [x] 1.6 Create `src/sources/models_dev/` folder structure
- [x] 1.7 Update imports throughout codebase to use new paths

## 2. Models.dev Data Structures
- [x] 2.1 Create `sources/models_dev/models.rs` with data structures
- [x] 2.2 Define `ModelsDevProvider` struct (id, name, env, npm, doc, models)
- [x] 2.3 Define `ModelsDevModel` struct (capabilities, limits, modalities, cost)
- [x] 2.4 Define `Limits` struct (context, input, output)
- [x] 2.5 Define `Modalities` struct (input: Vec, output: Vec)
- [x] 2.6 Add unit tests for JSON deserialization

## 3. Models.dev Client
- [x] 3.1 Create `sources/models_dev/client.rs`
- [x] 3.2 Implement fetch from `https://models.dev/api.json`
- [x] 3.3 Add endpoint constant
- [x] 3.4 Handle API errors gracefully with fallback to stale cache

## 4. Schema Updates for Raw Tables
- [x] 4.1 Create `sources/artificial_analysis/schema.rs` with `AA_LLMS` table
- [x] 4.2 Create `sources/models_dev/schema.rs` with `MODELS_DEV` table
- [x] 4.3 Update main `schema.rs` to import and expose all tables
- [x] 4.4 Create merged `LLMS` table schema with all fields
- [x] 4.5 Update `ALL_TABLES` to include all three LLM tables

## 5. Three-Layer Cache Architecture
- [x] 5.1 Update cache module to support multiple Parquet files per data type
- [x] 5.2 Implement `aa_llms.parquet` storage (raw AA data)
- [x] 5.3 Implement `models_dev.parquet` storage (raw models.dev data)
- [x] 5.4 Implement `llms.parquet` as derived merged view
- [x] 5.5 Add cache metadata to track last-modified timestamps
- [x] 5.6 Implement 24h TTL check for models.dev cache

## 6. Merge Module
- [x] 6.1 Create `src/merge/mod.rs` with merge entry point
- [x] 6.2 Create `src/merge/matcher.rs` with matching algorithm
- [x] 6.3 Implement composite key generation (`provider/model`)
- [x] 6.4 Implement exact match algorithm (case-insensitive)
- [x] 6.5 Implement fuzzy fallback matching (strip version suffixes)
- [x] 6.6 Create provider name mapping table (meta→llama, etc.)
- [x] 6.7 Create `src/merge/combiner.rs` with field merge logic
- [x] 6.8 Implement merge trigger logic (rebuild on source change)
- [x] 6.9 Write comprehensive matching tests with known model pairs

## 7. Unified LlmModel
- [x] 7.1 Update `src/models/llm.rs` with unified model struct
- [x] 7.2 Add capability fields (reasoning, tool_call, structured_output, etc.)
- [x] 7.3 Add limit fields (context_window, max_input, max_output)
- [x] 7.4 Add modality fields (input_modalities, output_modalities)
- [x] 7.5 Add metadata fields (knowledge_cutoff, open_weights, last_updated)
- [x] 7.6 Add accessor methods for new fields
- [x] 7.7 Add `source` field to track data origin (for debugging)

## 8. Parquet Serialization
- [x] 8.1 Update Parquet schema builder for `aa_llms` table
- [x] 8.2 Create Parquet schema for `models_dev` table
- [x] 8.3 Create Parquet schema for merged `llms` table
- [x] 8.4 Handle array columns (modalities) as List type

## 9. CLI Filter Enhancements
- [x] 9.1 Add `--reasoning` flag (bool filter)
- [x] 9.2 Add `--tool-call` flag (bool filter)
- [x] 9.3 Add `--structured-output` flag (bool filter)
- [x] 9.4 Add `--attachment` flag (bool filter)
- [x] 9.5 Add `--min-context <tokens>` filter
- [x] 9.6 Add `--modality <input|output>:<type>` filter

## 10. Output Formatting
- [x] 10.1 Update table output to show capability columns (compact: `+`/`-`/`?`)
- [x] 10.2 Update JSON output to include merged data
- [x] 10.3 Update markdown output with capability columns
- [ ] 10.4 Add capability summary in detailed view

## 11. Testing
- [x] 11.1 Unit tests for models.dev data parsing
- [x] 11.2 Unit tests for model matching algorithm
- [x] 11.3 Unit tests for merge logic
- [ ] 11.4 Integration tests with mock APIs for both sources
- [ ] 11.5 Snapshot tests for new output formats
- [ ] 11.6 Test cache invalidation and rebuild logic
- [ ] 11.7 Test filter combinations with capability flags

## 12. Documentation
- [ ] 12.1 Update README with new capabilities
- [ ] 12.2 Add examples for capability filtering
- [ ] 12.3 Document data sources and attribution
- [ ] 12.4 Document cache architecture and raw table queries
