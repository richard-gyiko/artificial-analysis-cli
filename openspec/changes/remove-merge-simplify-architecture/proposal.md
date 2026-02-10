# Change: Remove Merge Logic and Simplify to Two Independent Tables

## Why

The current architecture attempts to merge Artificial Analysis (AA) and models.dev data at refresh time using complex fuzzy matching logic. This approach has significant problems:

### Problem 1: Complex Matching Logic (~600 lines)

The `src/merge/matcher.rs` file contains 9 different matching strategies:
1. Exact composite key match
2. Normalized provider names (meta → llama)
3. Fuzzy version suffix stripping
4. Version separator normalization (dots ↔ dashes)
5. Provider prefix stripping
6. Reasoning variant matching
7. Compressed version expansion (35 → 3-5)
8. Gemma -it suffix handling
9. Effort level suffix stripping

Despite this complexity, the match rate is only ~53%.

### Problem 2: Naming Inconsistencies Are Unfixable

| Issue | AA Example | models.dev Example |
|-------|------------|-------------------|
| Dot vs Dash | `kimi-k2-5` | `kimi-k2.5` |
| Provider prefixes | `kimi-k2.5` | `moonshotai/kimi-k2.5` |
| Date suffixes | `claude-3-5-sonnet` | `claude-3-5-sonnet-20241022` |
| Reasoning modes | `gemini-2-5-flash-reasoning` | `gemini-2.5-flash` |
| Custom aliases | `gpt-4o` | `gpt-4o-2024-08-06` |

These are not bugs - the sources genuinely use different naming conventions.

### Problem 3: Unmatched Models Lose Data

When a model can't be matched (47% of cases), users see `null` for capabilities even though the data exists in models.dev - just under a different name.

### Problem 4: Redundant Cache Files

Current cache structure has redundancy:
```
~/.cache/which-llm/
├── aa_llms.parquet          # Raw AA data
├── providers.parquet        # Raw models.dev data  
├── llms.parquet             # Merged result
├── models_dev_cache.json    # Duplicate of providers.parquet in JSON
└── models_dev_meta.txt      # Could use file mtime instead
```

### The Insight: LLMs Are Better at Fuzzy Matching

An LLM using the skill can easily understand that:
- "Claude 3.5 Sonnet" = "claude-3-5-sonnet" = "claude-3.5-sonnet-20241022"
- "GPT-4o" = "gpt-4o-2024-08-06" = "chatgpt-4o-latest"

The LLM has context about what the user is asking and can intelligently match model names across tables when needed - which is rarely.

## What Changes

### Remove Pre-Computed Merge

**Before:**
```
AA API → aa_llms.parquet ─┐
                          ├─→ llms.parquet (merged)
models.dev API → JSON ────┘
              ↓
         providers.parquet
```

**After:**
```
AA API → llms.parquet (pure AA data)
models.dev API → providers.parquet (pure models.dev data)
```

### Simplified Cache Structure

```
~/.cache/which-llm/
├── llms.parquet         # Pure AA data (~200 models)
└── providers.parquet    # Pure models.dev data (~2,500 entries)
```

Remove:
- `aa_llms.parquet` (rename to `llms.parquet`)
- `models_dev_cache.json` (no longer needed)
- `models_dev_meta.txt` (use file mtime)

### Code Removal

Delete or significantly simplify:
- `src/merge/matcher.rs` - All fuzzy matching logic
- `src/merge/combiner.rs` - Merge orchestration
- `src/merge/mod.rs` - Merge module

### Simplified LlmModel Struct

Remove capability fields from `LlmModel` - they live only in `providers` table:

```rust
// Before: LlmModel had 30+ fields from both sources
// After: LlmModel has ~20 fields from AA only

pub struct LlmModel {
    // Identity
    pub id: String,
    pub name: String,
    pub slug: String,
    pub creator: String,
    pub creator_slug: Option<String>,
    pub release_date: Option<String>,
    
    // Benchmarks (AA exclusive)
    pub intelligence: Option<f64>,
    pub coding: Option<f64>,
    pub math: Option<f64>,
    pub mmlu_pro: Option<f64>,
    pub gpqa: Option<f64>,
    pub hle: Option<f64>,
    pub livecodebench: Option<f64>,
    pub scicode: Option<f64>,
    pub math_500: Option<f64>,
    pub aime: Option<f64>,
    
    // Pricing (AA canonical)
    pub input_price: Option<f64>,
    pub output_price: Option<f64>,
    pub price: Option<f64>,
    
    // Performance (AA exclusive)
    pub tps: Option<f64>,
    pub latency: Option<f64>,
    
    // REMOVED: reasoning, tool_call, structured_output, attachment,
    //          temperature, context_window, max_input_tokens,
    //          max_output_tokens, input_modalities, output_modalities,
    //          knowledge_cutoff, open_weights, last_updated,
    //          models_dev_matched
}
```

### Two-Table Query Pattern

The skill teaches the LLM to use both tables appropriately:

**Benchmark questions → `llms` table:**
```sql
SELECT name, coding, price FROM llms 
WHERE coding > 60 ORDER BY coding DESC LIMIT 5
```

**Capability questions → `providers` table:**
```sql
SELECT DISTINCT model_name, tool_call, reasoning 
FROM providers WHERE tool_call = true
```

**Cross-reference questions → Two queries + LLM matching:**
```sql
-- Step 1: Find best coding model
SELECT name, coding FROM llms ORDER BY coding DESC LIMIT 1
-- Result: "Claude 3.5 Sonnet"

-- Step 2: Find providers (LLM fuzzy matches the name)
SELECT provider_name, cost_input FROM providers 
WHERE model_name LIKE '%Claude%Sonnet%' OR family LIKE '%claude%sonnet%'
ORDER BY cost_input
```

## Impact

### Affected Code

| File | Change |
|------|--------|
| `src/merge/` | **DELETE** entire module |
| `src/models/llm.rs` | Remove capability fields, simplify struct |
| `src/client/mod.rs` | Remove merge orchestration, simplify cache handling |
| `src/parquet.rs` | Remove `write_llms_parquet` merge logic |
| `src/schema.rs` | Update `llms` table schema (fewer columns) |
| `src/commands/llms.rs` | Remove capability filter flags |
| `skills/which-llm/SKILL.md` | Update to document two-table pattern |

### Affected Specs

- `data-architecture` - Modify to remove merge requirement
- `cli` - Modify to remove capability filter flags from `llms` command

### Breaking Changes

1. **`llms` table schema changes** - Capability columns removed
2. **CLI flags removed** - `--reasoning`, `--tool-call`, `--structured-output`, `--attachment` removed from `llms` command
3. **`models_dev_matched` column removed** - No longer relevant

### Migration

Users with existing cache will have old `llms.parquet` with capability columns. On next `which-llm update`:
1. Old cache files are replaced
2. New schema takes effect
3. Users query `providers` table for capabilities

## Benefits

1. **~600 lines of complex code removed** - Simpler maintenance
2. **100% data visibility** - No models "lost" due to matching failures
3. **Transparent data sources** - Users see exactly what each source provides
4. **Better accuracy** - LLM fuzzy matching with context beats static rules
5. **Faster updates** - No merge step during refresh
6. **Smaller cache** - Remove redundant JSON file

## Non-Goals

- We will NOT create any automated cross-table matching
- We will NOT add capability columns back to `llms` table
- We will NOT deprecate the `providers` table
- We will NOT change the CLI command structure (just remove some flags)
