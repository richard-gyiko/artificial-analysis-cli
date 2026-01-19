# Design: Integrate models.dev Data

## Context

The CLI fetches LLM data from Artificial Analysis API which provides benchmark scores but lacks capability metadata. Users need to filter by capabilities (reasoning, tool calling, context limits, modalities) for complex use cases like agent orchestration or multi-modal applications.

This change also restructures the codebase to support multiple data sources, preparing for eventual rebrand to `which-llm-cli`.

**Stakeholders**: CLI users who need to select models for specific use cases

## Goals / Non-Goals

### Goals
- Enrich AA model data with capability metadata from models.dev
- Enable filtering by capabilities (reasoning, tool_call, etc.)
- Maintain AA as the primary/authoritative data source
- Minimize API calls via effective caching
- **Restructure code to isolate source-specific logic**
- **Prepare for future rebrand and additional data sources**

## Non-Goals (Explicit)
- Exposing raw tables (`aa_llms`, `models_dev`) to CLI users - these are internal
- Replace AA benchmarks with models.dev data
- Show models that only exist in models.dev
- Real-time sync between sources
- Support for non-LLM models from models.dev
- Complete rebrand (future work)

## Decisions

### Decision 1: models.dev as Secondary Data Source
**What**: Treat AA as primary, models.dev as enrichment only
**Why**: AA provides unique benchmark data we can't get elsewhere; models.dev provides complementary capability data
**Alternatives considered**:
- Make models.dev primary: Rejected - loses benchmark data
- Separate tables: Rejected - increases query complexity

### Decision 2: Composite Key Matching
**What**: Match using `{provider_slug}/{model_slug}` composite key
**Why**: Both sources use similar naming conventions; slug is more stable than display name
**Alternatives considered**:
- UUID matching: Not possible - different ID systems
- Name fuzzy matching: Too error-prone, needed only as fallback

### Decision 3: Three-Layer Cache Architecture
**What**: Store raw AA and models.dev as separate Parquet files; derive merged view on demand
**Why**: 
- Clear data lineage - always trace where each field came from
- Independent refresh - update either source without losing the other
- Debugging - inspect raw sources when matching fails
- Resilience - if one source fails, other's raw data persists
- Schema evolution - new fields don't require re-fetching both sources
**Alternatives considered**:
- Single merged cache: Rejected - loses raw data lineage, harder to debug matching
- Fetch-and-merge on every request: Rejected - slow, unnecessary API load

### Decision 4: Nullable Capability Fields
**What**: All models.dev fields are Option<T> in the model
**Why**: Not all AA models will have models.dev matches; graceful degradation
**Alternatives considered**:
- Required fields with defaults: Rejected - misleading (false != unknown)

### Decision 5: Source-Isolated Code Structure
**What**: Organize code into `src/sources/{source_name}/` subfolders
**Why**:
- Clear separation of source-specific logic (client, models, schema)
- Easy to add new data sources in the future
- Testable - each source can be mocked independently
- Prepares for rebrand to `which-llm-cli` where AA becomes just one source
**Alternatives considered**:
- Flat structure: Rejected - harder to maintain as sources grow
- Single client with adapters: Rejected - too much abstraction for current needs

## Code Structure

```
src/
├── sources/                    # Data source abstraction
│   ├── mod.rs                  # Source trait, re-exports
│   ├── artificial_analysis/    # AA-specific code
│   │   ├── mod.rs              # Module exports
│   │   ├── client.rs           # AA API client (moved from src/client/)
│   │   ├── models.rs           # AA raw data structures
│   │   └── schema.rs           # AA Parquet schema (aa_llms table)
│   └── models_dev/             # models.dev-specific code
│       ├── mod.rs              # Module exports
│       ├── client.rs           # models.dev API client
│       ├── models.rs           # models.dev data structures
│       └── schema.rs           # models.dev Parquet schema
├── models/                     # Unified/merged models
│   ├── mod.rs
│   ├── llm.rs                  # Unified LlmModel (merged view)
│   └── media.rs                # Media models (AA-only)
├── merge/                      # Merge orchestration
│   ├── mod.rs                  # Merge entry point
│   ├── matcher.rs              # Model matching algorithm
│   └── combiner.rs             # Field merge logic
├── schema.rs                   # Unified schema (imports from sources)
├── cache.rs                    # Cache management (all layers)
├── commands/                   # CLI commands (unchanged structure)
└── output/                     # Output formatters (unchanged structure)
```

### Source Trait (Future Extensibility)
```rust
// src/sources/mod.rs
pub trait DataSource {
    type RawModel;
    type Error;
    
    fn name(&self) -> &'static str;
    fn fetch(&self) -> Result<Vec<Self::RawModel>, Self::Error>;
    fn cache_path(&self) -> PathBuf;
    fn ttl(&self) -> Option<Duration>;
}
```

This trait isn't strictly required for the initial implementation but provides a pattern for adding future sources.

## Data Structures

### models.dev Response Shape
```rust
// Top-level: HashMap<ProviderId, Provider>
struct ModelsDevProvider {
    id: String,
    name: String,
    env: Vec<String>,
    npm: Option<String>,
    doc: Option<String>,
    models: HashMap<String, ModelsDevModel>,
}

struct ModelsDevModel {
    id: String,
    name: String,
    family: Option<String>,
    // Capabilities
    attachment: Option<bool>,
    reasoning: Option<bool>,
    tool_call: Option<bool>,
    structured_output: Option<bool>,
    temperature: Option<bool>,
    // Metadata
    knowledge: Option<String>,
    release_date: Option<String>,
    last_updated: Option<String>,
    open_weights: Option<bool>,
    status: Option<String>,  // "alpha", "beta", "deprecated"
    // Limits
    limit: Option<Limits>,
    // Cost
    cost: Option<Cost>,
    // Modalities
    modalities: Option<Modalities>,
}

struct Limits {
    context: Option<u64>,
    input: Option<u64>,
    output: Option<u64>,
}

struct Modalities {
    input: Vec<String>,   // ["text", "image", "pdf"]
    output: Vec<String>,  // ["text"]
}
```

### Extended LlmModel
```rust
struct LlmModel {
    // Existing AA fields...
    
    // New capability fields (from models.dev)
    #[serde(default)]
    pub reasoning: Option<bool>,
    #[serde(default)]
    pub tool_call: Option<bool>,
    #[serde(default)]
    pub structured_output: Option<bool>,
    #[serde(default)]
    pub attachment: Option<bool>,
    #[serde(default)]
    pub temperature: Option<bool>,
    
    // Limits
    #[serde(default)]
    pub context_window: Option<u64>,
    #[serde(default)]
    pub max_input_tokens: Option<u64>,
    #[serde(default)]
    pub max_output_tokens: Option<u64>,
    
    // Modalities
    #[serde(default)]
    pub input_modalities: Option<Vec<String>>,
    #[serde(default)]
    pub output_modalities: Option<Vec<String>>,
    
    // Additional metadata
    #[serde(default)]
    pub knowledge_cutoff: Option<String>,
    #[serde(default)]
    pub open_weights: Option<bool>,
    #[serde(default)]
    pub last_updated: Option<String>,
}
```

## Matching Algorithm

```
fn match_model(aa_model: &LlmModel, providers: &HashMap<String, ModelsDevProvider>) 
    -> Option<&ModelsDevModel> 
{
    // 1. Build AA composite key
    let aa_provider = aa_model.model_creator.slug
        .as_ref()
        .map(|s| normalize_provider(s))
        .unwrap_or_default();
    let aa_slug = aa_model.slug.to_lowercase();
    
    // 2. Try direct match
    if let Some(provider) = providers.get(&aa_provider) {
        if let Some(model) = provider.models.get(&aa_slug) {
            return Some(model);
        }
    }
    
    // 3. Try fuzzy matching (remove version suffixes)
    let base_slug = strip_version_suffix(&aa_slug);
    for (_, provider) in providers {
        for (model_id, model) in &provider.models {
            if strip_version_suffix(model_id) == base_slug {
                return Some(model);
            }
        }
    }
    
    None
}

fn normalize_provider(slug: &str) -> String {
    match slug.to_lowercase().as_str() {
        "meta" => "llama".to_string(),
        other => other.to_string(),
    }
}

fn strip_version_suffix(s: &str) -> String {
    // Remove date suffixes like -20241022
    let re = regex::Regex::new(r"-\d{8}$").unwrap();
    re.replace(s, "").to_string()
}
```

## Cache Strategy

### Three-Layer Architecture
```
~/.config/aa/cache/
├── aa_llms.parquet           # Raw AA data (source 1)
├── models_dev.parquet        # Raw models.dev data (source 2)  
└── llms.parquet              # Merged view (derived)
```

### Layer Responsibilities

| Layer | Source | TTL | Contains |
|-------|--------|-----|----------|
| `aa_llms.parquet` | AA API | User-controlled (`--refresh`) | Raw benchmarks, pricing, performance |
| `models_dev.parquet` | models.dev API | 24h auto-refresh | Raw capabilities, limits, modalities |
| `llms.parquet` | Derived | Rebuilt on source change | Merged data for queries |

### Merge Trigger Logic
```
on fetch_llms():
    aa_changed = fetch_aa_if_needed() → save aa_llms.parquet
    md_changed = fetch_models_dev_if_expired() → save models_dev.parquet
    
    if aa_changed OR md_changed OR !exists(llms.parquet):
        merged = merge(aa_llms.parquet, models_dev.parquet)
        save(merged, llms.parquet)
    
    return read(llms.parquet)
```

### Benefits
1. **Data lineage**: Always know where each field came from (internal debugging)
2. **Independent refresh**: `--refresh` updates AA only; models.dev refreshes on its own schedule
3. **Debugging**: Developers can inspect raw Parquet files directly when matching fails
4. **Resilience**: If models.dev fails, stale `models_dev.parquet` used; AA data unaffected

**Note**: Raw tables (`aa_llms`, `models_dev`) are internal implementation details. Users only interact with the merged `llms` table.

**Cache invalidation**:
- `aa_llms.parquet`: Manual via `--refresh` flag
- `models_dev.parquet`: 24h TTL (auto-refresh)
- `llms.parquet`: Invalidated when either source changes

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| models.dev API unavailable | Graceful degradation - show AA data only, log warning |
| Match rate too low | Implement fuzzy matching, maintain provider mapping table |
| Data staleness | Clear cache command, configurable TTL |
| API breaking changes | Version pin models.dev schema, add validation |

## Migration Plan

1. **Phase 1**: Add models.dev client and caching (no user-visible changes)
2. **Phase 2**: Add capability fields to schema and output
3. **Phase 3**: Add filter flags to CLI

**Rollback**: New fields are optional; removing them doesn't break existing queries.

## Open Questions

- [ ] Should we expose a `--refresh-capabilities` flag to force models.dev refresh?
- [ ] Should unmatched models show `null` or `unknown` for capability columns?
- [ ] Do we need to handle models.dev `status: deprecated` specially?
