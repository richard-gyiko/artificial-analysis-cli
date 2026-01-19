# Change: Add SQL Query Support

## Why

The current CLI has limited filtering (`--model`, `--creator`, `--sort`) which doesn't scale as users need to filter by more fields (intelligence, coding, math, pricing, speed, latency). Users comparing LLMs need expressive queries to slice data by multiple dimensions. SQL is universal - everyone knows it, AI agents can generate it, and it solves all filtering/sorting/aggregation needs without custom syntax.

## Problem Statement

Current state:
```bash
aa llms --creator openai --sort intelligence  # Limited filtering
```

Flag explosion approach (doesn't scale):
```bash
aa llms --min-intelligence 40 --min-coding 35 --max-output-price 5 \
        --min-tps 100 --max-latency 2 --sort coding --limit 10
# 10+ flags, grows with every new API field
```

Custom expression parser (reinventing the wheel):
```bash
aa llms --where "intelligence > 40 AND coding > 35"
# Need to implement: LIKE, regex, grouping, aggregations, column selection...
```

**Solution - SQL via DuckDB:**
```bash
aa query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE intelligence > 40 AND creator = 'OpenAI'
          ORDER BY price 
          LIMIT 10"
```

## What Changes

- **ADDED**: `aa query` command for SQL queries against cached data
- **ADDED**: DuckDB integration for SQL execution
- **ADDED**: Parquet caching for efficient columnar storage
- **ADDED**: Table name aliases (`llms`, `text_to_image`, etc.) that resolve to Parquet file paths
- **MODIFIED**: Cache format from JSON to Parquet

## Why DuckDB

| Concern | Reality |
|---------|---------|
| 25MB binary | Acceptable in 2026. Users download 100MB+ apps routinely. |
| Build time | One-time cost. CI handles it. |
| Learning curve | Zero - everyone knows SQL |
| Maintenance | Battle-tested vs custom parser bugs |
| Future-proofing | Aggregations, joins, window functions come free |

## Capabilities

### Filtering
```bash
aa query "SELECT * FROM llms WHERE intelligence > 40"
aa query "SELECT * FROM llms WHERE creator ILIKE '%openai%'"
aa query "SELECT * FROM llms WHERE name LIKE 'GPT%'"
```

### Column Selection
```bash
aa query "SELECT name, intelligence, price FROM llms"
```

### Sorting & Limiting
```bash
aa query "SELECT * FROM llms ORDER BY intelligence DESC LIMIT 10"
```

### Aggregations
```bash
aa query "SELECT creator, COUNT(*) as models, AVG(intelligence) as avg_intel 
          FROM llms GROUP BY creator ORDER BY avg_intel DESC"
```

### Cross-table Queries (future)
```bash
aa query "SELECT l.name, l.intelligence, t.elo 
          FROM llms l JOIN text_to_image t ON l.creator = t.creator"
```

## Impact

- Affected specs: `cli`
- Affected code:
  - New: `src/query.rs` (DuckDB integration)
  - Modified: `src/cache.rs` (Parquet storage)
  - Modified: `src/cli/mod.rs` (new `query` command)
  - Modified: `Cargo.toml` (DuckDB dependency)
- Binary size: ~5MB → ~25-30MB
- Build time: Increases due to DuckDB compilation

## Implementation Approach

DuckDB queries Parquet files directly via `read_parquet()`. The CLI provides table name aliases for convenience:

```
User writes:     SELECT * FROM llms WHERE intelligence > 40
CLI transforms:  SELECT * FROM read_parquet('/cache/llms.parquet') WHERE intelligence > 40
```

No views or table registration needed - just simple string substitution of known table names to their Parquet file paths.

## Table Schemas

### `llms`
| Column | Type | Source |
|--------|------|--------|
| `id` | VARCHAR | `id` |
| `name` | VARCHAR | `name` |
| `slug` | VARCHAR | `slug` |
| `creator` | VARCHAR | `model_creator.name` |
| `creator_slug` | VARCHAR | `model_creator.slug` |
| `release_date` | VARCHAR | `release_date` |
| `intelligence` | DOUBLE | `evaluations.artificial_analysis_intelligence_index` |
| `coding` | DOUBLE | `evaluations.artificial_analysis_coding_index` |
| `math` | DOUBLE | `evaluations.artificial_analysis_math_index` |
| `mmlu_pro` | DOUBLE | `evaluations.mmlu_pro` |
| `gpqa` | DOUBLE | `evaluations.gpqa` |
| `input_price` | DOUBLE | `pricing.price_1m_input_tokens` |
| `output_price` | DOUBLE | `pricing.price_1m_output_tokens` |
| `price` | DOUBLE | `pricing.price_1m_blended_3_to_1` |
| `tps` | DOUBLE | `median_output_tokens_per_second` |
| `latency` | DOUBLE | `median_time_to_first_token_seconds` |

### `text_to_image`, `text_to_video`, etc.
| Column | Type | Source |
|--------|------|--------|
| `id` | VARCHAR | `id` |
| `name` | VARCHAR | `name` |
| `slug` | VARCHAR | `slug` |
| `creator` | VARCHAR | `model_creator.name` |
| `elo` | DOUBLE | `elo` |
| `rank` | INTEGER | `rank` |
| `release_date` | VARCHAR | `release_date` |

## Example Queries

```bash
# Best coding models under $5/M output price
aa query "SELECT name, creator, coding, output_price 
          FROM llms 
          WHERE coding > 40 AND output_price < 5 
          ORDER BY coding DESC"

# Fastest models with good intelligence
aa query "SELECT name, intelligence, tps 
          FROM llms 
          WHERE intelligence > 35 AND tps > 100 
          ORDER BY tps DESC 
          LIMIT 10"

# Compare creators by average intelligence
aa query "SELECT creator, COUNT(*) as models, ROUND(AVG(intelligence), 1) as avg_intel 
          FROM llms 
          WHERE intelligence IS NOT NULL
          GROUP BY creator 
          ORDER BY avg_intel DESC"

# Top image generation models
aa query "SELECT name, creator, elo, rank 
          FROM text_to_image 
          WHERE elo > 1200 
          ORDER BY elo DESC"

# Budget-friendly models with decent quality
aa query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE price < 1 AND intelligence > 25 
          ORDER BY intelligence DESC"
```

## Output Formats

The `query` command respects existing output flags:
- Default: Markdown table
- `--json`: JSON array
- `--csv`: CSV format
- `--table`: ASCII table
- `--plain`: Tab-separated

## Migration

- Existing JSON cache files will be migrated to Parquet on first use
- Old `--model`, `--creator`, `--sort` flags remain for simple use cases
- `aa query` is additive - doesn't break existing commands

## Decisions

1. **No JSON cache** - Parquet only. Keeps implementation simple.
2. **Add hint** - `aa llms` (and other data commands) will show a hint about `aa query` for advanced filtering.
