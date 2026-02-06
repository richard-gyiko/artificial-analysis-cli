## ADDED Requirements

### Requirement: Hosted Data Source
The CLI SHALL fetch pre-built Parquet data from GitHub Releases by default, eliminating the need for users to configure API keys.

#### Scenario: First run without API key
- **WHEN** user runs `which-llm llms` without any API key configured
- **THEN** CLI fetches data from GitHub Releases and displays results

#### Scenario: Cached data available
- **WHEN** user runs `which-llm llms` with valid cached data (less than 24 hours old)
- **THEN** CLI uses local cache without network requests

#### Scenario: GitHub unavailable with API key
- **WHEN** GitHub Releases are unreachable AND user has API key configured
- **THEN** CLI falls back to direct API calls

#### Scenario: GitHub unavailable without API key
- **WHEN** GitHub Releases are unreachable AND no API key is configured
- **THEN** CLI displays error with instructions to configure API key or retry later

### Requirement: Data Refresh Control
The CLI SHALL provide flags to control data source selection.

#### Scenario: Force refresh from GitHub
- **WHEN** user runs `which-llm llms --refresh`
- **THEN** CLI fetches fresh data from GitHub Releases, ignoring local cache

#### Scenario: Use API directly
- **WHEN** user runs `which-llm llms --use-api`
- **THEN** CLI fetches data directly from Artificial Analysis API (requires API key)

### Requirement: Data Attribution
The CLI SHALL display attribution to Artificial Analysis as required by their terms of use.

#### Scenario: Version output includes attribution
- **WHEN** user runs `which-llm --version`
- **THEN** output includes "Data provided by Artificial Analysis (https://artificialanalysis.ai)"

#### Scenario: Info command shows attribution
- **WHEN** user runs `which-llm info`
- **THEN** output shows data source, last update time, and attribution notice

### Requirement: Data Update Workflow
The project SHALL maintain a GitHub Actions workflow that updates hosted data regularly.

#### Scenario: Scheduled data update
- **WHEN** the scheduled workflow runs (daily)
- **THEN** fresh data is fetched from APIs and published to GitHub Releases

#### Scenario: Manual data update
- **WHEN** maintainer triggers workflow manually via workflow_dispatch
- **THEN** fresh data is fetched and published immediately

## MODIFIED Requirements

### Requirement: Environment Variable Configuration
The CLI SHALL support configuration via environment variables.

#### Scenario: API key from environment
- **WHEN** `ARTIFICIAL_ANALYSIS_API_KEY` environment variable is set
- **THEN** CLI uses that API key for authentication when `--use-api` flag is used or as fallback

#### Scenario: Config directory override
- **WHEN** `WHICH_LLM_CONFIG_DIR` environment variable is set
- **THEN** CLI uses that directory for configuration files

#### Scenario: No API key configured
- **WHEN** user runs `which-llm llms` without API key configured
- **THEN** CLI uses hosted data from GitHub Releases (no error)

### Requirement: Profile Management
The CLI SHALL provide profile management commands using the new binary name.

#### Scenario: Create profile
- **WHEN** user runs `which-llm profile create default --api-key KEY`
- **THEN** profile is created in `~/.config/which-llm/config.toml`

#### Scenario: No API key usage
- **WHEN** user runs `which-llm llms` without API key configured
- **THEN** CLI uses hosted data (no error, no suggestion to create profile)

#### Scenario: API key for real-time data
- **WHEN** user wants real-time data instead of hosted data
- **THEN** user can run `which-llm profile create` to configure API key and use `--use-api` flag
