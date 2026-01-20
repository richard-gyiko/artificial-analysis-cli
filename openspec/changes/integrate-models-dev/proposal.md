# Change: Integrate models.dev Data for Enhanced LLM Capabilities

## Why

The current Artificial Analysis API provides excellent benchmark scores and performance metrics (intelligence, coding, math indices, TPS, latency), but lacks critical capability metadata needed for filtering LLMs in complex use cases. Users need to filter by:
- Reasoning support (chain-of-thought models like o3)
- Tool calling support
- Structured output support
- File/image attachment support
- Context window limits
- Input/output modalities

The [models.dev](https://github.com/anomalyco/models.dev) project maintains comprehensive capability data for AI models, making it an ideal complement to our benchmark data.

## What Changes

### Data Model Extensions
- Add new capability fields to `LlmModel`: `reasoning`, `tool_call`, `structured_output`, `attachment`, `temperature`
- Add new limit fields: `context_window`, `max_input_tokens`, `max_output_tokens`
- Add modality fields: `input_modalities`, `output_modalities`
- Add metadata: `knowledge_cutoff`, `open_weights`, `last_updated`

### New Data Source
- Add `models.dev` client to fetch from `https://models.dev/api.json`
- Store raw models.dev data as separate Parquet file
- Implement model matching algorithm using `{provider}/{model}` composite keys

### Three-Layer Cache Architecture
```
~/.config/aa/cache/
├── aa_llms.parquet           # Raw AA data (source 1)
├── models_dev.parquet        # Raw models.dev data (source 2)
└── llms.parquet              # Merged view (derived)
```

- **Raw sources**: Stored independently, fetched on their own schedule
- **Merged view**: Regenerated whenever either source updates
- **Trigger logic**: Any raw source change invalidates and rebuilds the merged view

### Schema Updates
- Create `models_dev` table schema for raw models.dev data
- Extend `llms` merged table schema with capability columns
- Ensure backward compatibility (new columns are nullable)

### CLI Enhancements
- New filter flags: `--reasoning`, `--tool-call`, `--structured-output`, `--attachment`
- New filter flags: `--min-context`, `--modality`
- Show capability columns in output

## Impact
- Affected specs: llm-data (new capability)
- Affected code: Major restructure to support multiple data sources

### New Code Structure
```
src/
├── sources/                    # Data source abstraction
│   ├── mod.rs                  # Source trait, common types
│   ├── artificial_analysis/    # AA-specific code
│   │   ├── mod.rs
│   │   ├── client.rs           # AA API client
│   │   ├── models.rs           # AA data structures (LlmModel, MediaModel)
│   │   └── schema.rs           # AA Parquet schema (aa_llms, media tables)
│   └── models_dev/             # models.dev-specific code
│       ├── mod.rs
│       ├── client.rs           # models.dev API client
│       ├── models.rs           # models.dev data structures
│       └── schema.rs           # models.dev Parquet schema
├── models/                     # Unified/merged models
│   ├── mod.rs
│   ├── llm.rs                  # Unified LlmModel (AA + models.dev merged)
│   └── media.rs                # Media models (AA-only for now)
├── merge/                      # Merge orchestration
│   ├── mod.rs
│   ├── matcher.rs              # Model matching algorithm
│   └── combiner.rs             # Field merge logic
└── ...existing files...
```

### Key Files Changed
- `src/sources/` - New module for data source abstraction
- `src/merge/` - New module for merge logic
- `src/models/llm.rs` - Unified model with fields from both sources
- `src/schema.rs` - Updated with all three table schemas
- `src/commands/llms.rs` - New filter flags
- `src/output/*.rs` - Display new columns

### Future Rebrand Note
This structure prepares for eventual rebrand to `which-llm-cli`. The `artificial_analysis/` folder becomes just one of multiple data sources, with the merged view being the primary user-facing data.

## Model Matching Strategy

### Primary Matching Algorithm
1. Create composite key: `{provider_slug}/{model_slug}`
   - AA: `model_creator.slug.lowercase()` + `/` + `slug.lowercase()`
   - models.dev: `provider_id` + `/` + `model_id`
2. Direct exact match on composite key

### Fallback Matching (for edge cases)
1. Fuzzy match on normalized model name + creator name
2. Handle versioned models (e.g., `claude-3-5-sonnet-20241022` -> `claude-3-5-sonnet`)
3. Provider name mapping where needed (e.g., `meta` -> `llama`)

### Provider Mapping
| AA Creator Slug | models.dev Provider |
|-----------------|---------------------|
| openai | openai |
| anthropic | anthropic |
| google | google |
| mistral | mistral |
| meta | llama |

## Data Complementarity

| Field | Source | Notes |
|-------|--------|-------|
| Benchmarks (intelligence, coding, math) | AA | Unique to AA |
| Performance (TPS, latency) | AA | Unique to AA |
| Capabilities (reasoning, tool_call) | models.dev | Unique to models.dev |
| Context limits | models.dev | Unique to models.dev |
| Modalities | models.dev | Unique to models.dev |
| Pricing | Both | Prefer AA, fallback to models.dev |
| Release date | Both | Prefer AA, fallback to models.dev |

## Non-Goals
- We will NOT replace AA data with models.dev data
- We will NOT support models that exist only in models.dev (AA is primary source)
- We will NOT fetch models.dev on every request (caching required)
