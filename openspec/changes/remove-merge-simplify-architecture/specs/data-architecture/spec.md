## MODIFIED Requirements

### Requirement: Two-Table Architecture
The system SHALL maintain two independent data tables that are NOT merged, allowing each source to be queried directly.

#### Scenario: Tables are independent
- **WHEN** user queries `llms` table
- **THEN** only Artificial Analysis data is returned (benchmarks, performance, canonical pricing)

#### Scenario: No pre-computed merge
- **WHEN** user runs `which-llm update`
- **THEN** `llms.parquet` and `providers.parquet` are created independently without merging

#### Scenario: Query llms for benchmarks
- **WHEN** user needs model quality comparison
- **THEN** user queries the `llms` table for benchmarks, pricing, and performance metrics

#### Scenario: Query providers for capabilities
- **WHEN** user needs capability information (tool_call, reasoning, context_window)
- **THEN** user queries the `providers` table

#### Scenario: Cross-reference via LLM
- **WHEN** user needs both benchmark data and capability data for the same model
- **THEN** the LLM/skill handles fuzzy matching between table results at query time

### Requirement: LLMs Table Schema
The system SHALL store only Artificial Analysis data in the `llms` table.

#### Scenario: Benchmark fields present
- **WHEN** querying `llms` table
- **THEN** columns include: `id`, `name`, `slug`, `creator`, `creator_slug`, `release_date`, `intelligence`, `coding`, `math`, `mmlu_pro`, `gpqa`, `hle`, `livecodebench`, `scicode`, `math_500`, `aime`, `input_price`, `output_price`, `price`, `tps`, `latency`

#### Scenario: Capability fields absent
- **WHEN** querying `llms` table
- **THEN** there are NO columns for: `reasoning`, `tool_call`, `structured_output`, `attachment`, `temperature`, `context_window`, `max_input_tokens`, `max_output_tokens`, `input_modalities`, `output_modalities`, `knowledge_cutoff`, `open_weights`, `models_dev_matched`

### Requirement: Simplified Cache Structure
The system SHALL maintain a minimal cache without redundant files.

#### Scenario: Cache contains two parquet files
- **WHEN** cache is populated
- **THEN** only `llms.parquet` and `providers.parquet` exist in `~/.cache/which-llm/`

#### Scenario: No JSON cache files
- **WHEN** models.dev data is fetched
- **THEN** no `models_dev_cache.json` file is created

#### Scenario: No meta files
- **WHEN** checking cache staleness
- **THEN** file modification time is used instead of separate meta files

## REMOVED Requirements

### Requirement: Model Matching Algorithm
**Reason**: Matching is now handled at query time by the LLM/skill, not at data refresh time.
**Migration**: Users query both tables separately; the skill guides cross-referencing.

### Requirement: Merged View Generation
**Reason**: The merge added complexity (~600 lines) for limited benefit (~53% match rate).
**Migration**: Capability data is now queried directly from `providers` table.

### Requirement: models_dev_matched Flag
**Reason**: With no merge, there's no concept of "matched" vs "unmatched" models.
**Migration**: Field is simply removed from schema.
