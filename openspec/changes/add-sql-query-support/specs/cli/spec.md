## ADDED Requirements

### Requirement: SQL Query Command
The CLI SHALL provide a `query` command that executes SQL queries against cached API data.

#### Scenario: Basic SELECT query
- **GIVEN** LLM data has been cached
- **WHEN** user runs `aa query "SELECT name, intelligence FROM llms LIMIT 5"`
- **THEN** results are displayed in markdown table format

#### Scenario: Filtered query
- **GIVEN** LLM data has been cached
- **WHEN** user runs `aa query "SELECT * FROM llms WHERE intelligence > 40"`
- **THEN** only models with intelligence > 40 are returned

#### Scenario: Aggregation query
- **GIVEN** LLM data has been cached
- **WHEN** user runs `aa query "SELECT creator, AVG(intelligence) FROM llms GROUP BY creator"`
- **THEN** aggregated results by creator are displayed

#### Scenario: Query with JSON output
- **GIVEN** LLM data has been cached
- **WHEN** user runs `aa query --json "SELECT name, price FROM llms LIMIT 3"`
- **THEN** results are output as JSON array

#### Scenario: Invalid SQL error
- **GIVEN** user provides malformed SQL
- **WHEN** user runs `aa query "SELEC * FROM llms"`
- **THEN** CLI displays helpful error message about SQL syntax

#### Scenario: Missing table error
- **GIVEN** user references uncached data
- **WHEN** user runs `aa query "SELECT * FROM llms"` without having fetched LLM data
- **THEN** CLI displays message suggesting to run `aa llms` first

### Requirement: Table Name Aliases
The CLI SHALL support table name aliases that resolve to cached Parquet file paths. The CLI transforms SQL queries by replacing table names with `read_parquet()` calls.

| Alias | Parquet File | Source Command |
|-------|--------------|----------------|
| `llms` | `llms.parquet` | `aa llms` |
| `text_to_image` | `text_to_image.parquet` | `aa text-to-image` |
| `image_editing` | `image_editing.parquet` | `aa image-editing` |
| `text_to_speech` | `text_to_speech.parquet` | `aa text-to-speech` |
| `text_to_video` | `text_to_video.parquet` | `aa text-to-video` |
| `image_to_video` | `image_to_video.parquet` | `aa image-to-video` |

#### Scenario: Table alias resolves to Parquet file
- **GIVEN** LLM data has been cached as `llms.parquet`
- **WHEN** user runs `aa query "SELECT * FROM llms"`
- **THEN** CLI transforms query to use `read_parquet('/cache/path/llms.parquet')`

#### Scenario: List available tables
- **GIVEN** user wants to know available tables
- **WHEN** user runs `aa query --tables`
- **THEN** CLI displays list of table aliases with their column schemas

#### Scenario: Query media table
- **GIVEN** text-to-image data has been cached
- **WHEN** user runs `aa query "SELECT name, elo FROM text_to_image WHERE elo > 1200"`
- **THEN** filtered image model results are displayed

### Requirement: LLMs Table Schema
The `llms` table SHALL have the following columns with flattened structure:

| Column | Type | Nullable |
|--------|------|----------|
| `id` | VARCHAR | NO |
| `name` | VARCHAR | NO |
| `slug` | VARCHAR | NO |
| `creator` | VARCHAR | NO |
| `creator_slug` | VARCHAR | YES |
| `release_date` | VARCHAR | YES |
| `intelligence` | DOUBLE | YES |
| `coding` | DOUBLE | YES |
| `math` | DOUBLE | YES |
| `mmlu_pro` | DOUBLE | YES |
| `gpqa` | DOUBLE | YES |
| `input_price` | DOUBLE | YES |
| `output_price` | DOUBLE | YES |
| `price` | DOUBLE | YES |
| `tps` | DOUBLE | YES |
| `latency` | DOUBLE | YES |

#### Scenario: Query using flattened column names
- **GIVEN** LLM data has been cached
- **WHEN** user runs `aa query "SELECT name, creator, intelligence, price FROM llms WHERE creator = 'OpenAI'"`
- **THEN** results show flattened columns without nested access syntax

### Requirement: Media Tables Schema
Media tables (`text_to_image`, `image_editing`, `text_to_speech`, `text_to_video`, `image_to_video`) SHALL have the following columns:

| Column | Type | Nullable |
|--------|------|----------|
| `id` | VARCHAR | NO |
| `name` | VARCHAR | NO |
| `slug` | VARCHAR | NO |
| `creator` | VARCHAR | NO |
| `elo` | DOUBLE | YES |
| `rank` | INTEGER | YES |
| `release_date` | VARCHAR | YES |

#### Scenario: Query media rankings
- **GIVEN** text-to-video data has been cached
- **WHEN** user runs `aa query "SELECT name, elo, rank FROM text_to_video ORDER BY elo DESC LIMIT 5"`
- **THEN** top 5 video models by ELO are displayed

### Requirement: Parquet Cache Format
Cached API responses SHALL be stored in Parquet format for efficient columnar queries.

#### Scenario: Cache stored as Parquet
- **GIVEN** user runs `aa llms` to fetch data
- **WHEN** data is cached
- **THEN** cache file is stored as Parquet (e.g., `llms.parquet`)

#### Scenario: Parquet cache is queryable
- **GIVEN** Parquet cache file exists
- **WHEN** user runs `aa query "SELECT * FROM llms"`
- **THEN** DuckDB reads directly from Parquet file

### Requirement: Query Output Formats
The `query` command SHALL respect output format flags consistent with other commands.

#### Scenario: Markdown output (default)
- **WHEN** user runs `aa query "SELECT name FROM llms LIMIT 3"`
- **THEN** output is markdown table

#### Scenario: JSON output
- **WHEN** user runs `aa query --json "SELECT name FROM llms LIMIT 3"`
- **THEN** output is JSON array

#### Scenario: CSV output
- **WHEN** user runs `aa query --csv "SELECT name FROM llms LIMIT 3"`
- **THEN** output is CSV format

### Requirement: Advanced Query Hint
Data listing commands (`llms`, `text-to-image`, etc.) SHALL display a hint about `aa query` for advanced filtering when not in quiet mode.

#### Scenario: Hint displayed after llms output
- **GIVEN** user runs `aa llms`
- **WHEN** output is displayed
- **THEN** a hint is shown: "Tip: Use 'aa query \"SELECT * FROM llms WHERE ...\"' for advanced filtering"

#### Scenario: Hint suppressed in quiet mode
- **GIVEN** user runs `aa llms --quiet`
- **WHEN** output is displayed
- **THEN** no hint is shown

## MODIFIED Requirements

### Requirement: Response Caching
The CLI SHALL cache API responses locally to reduce redundant requests and respect rate limits.

Cache behavior:
- Cache location: Platform-specific cache directory (`~/.cache/aa/` on Linux, `~/Library/Caches/aa/` on macOS, `%LOCALAPPDATA%\aa\cache\` on Windows), or `AA_CACHE_DIR` if set
- Cache format: Parquet for query support, with JSON metadata
- Cache TTL: 1 hour (configurable in future)
- Cache key: Based on endpoint and query parameters
- `--refresh` flag bypasses cache and fetches fresh data

#### Scenario: First request caches as Parquet
- **GIVEN** no cached data exists for LLMs
- **WHEN** user runs `aa llms`
- **THEN** response is fetched from API and cached as Parquet

#### Scenario: Cached Parquet is queryable
- **GIVEN** LLM data is cached as Parquet
- **WHEN** user runs `aa query "SELECT * FROM llms"`
- **THEN** query executes against cached Parquet file
