## MODIFIED Requirements

### Requirement: CLI Binary Name
The CLI binary SHALL be named `which-llm`.

#### Scenario: Running the CLI
- **WHEN** user runs `which-llm --help`
- **THEN** the help text is displayed

#### Scenario: Running commands
- **WHEN** user runs `which-llm llms`
- **THEN** LLM data is fetched and displayed

### Requirement: Environment Variable Configuration
The CLI SHALL support configuration via environment variables.

#### Scenario: API key from environment
- **WHEN** `ARTIFICIAL_ANALYSIS_API_KEY` environment variable is set
- **THEN** CLI uses that API key for authentication

#### Scenario: Config directory override
- **WHEN** `WHICH_LLM_CONFIG_DIR` environment variable is set
- **THEN** CLI uses that directory for configuration files

### Requirement: Configuration Directory
The CLI SHALL store configuration in the user's config directory under `which-llm/`.

#### Scenario: Default config location (Unix)
- **WHEN** user creates a profile on Unix/macOS
- **THEN** config is stored at `~/.config/which-llm/config.toml`

#### Scenario: Default config location (Windows)
- **WHEN** user creates a profile on Windows
- **THEN** config is stored at `%APPDATA%\which-llm\config.toml`

### Requirement: Cache Directory
The CLI SHALL store cached data in the system cache directory under `which-llm/`.

#### Scenario: Cache file location
- **WHEN** user runs `which-llm llms` and data is cached
- **THEN** cache files are stored in `~/.cache/which-llm/`

### Requirement: Profile Management
The CLI SHALL provide profile management commands using the new binary name.

#### Scenario: Create profile
- **WHEN** user runs `which-llm profile create default --api-key KEY`
- **THEN** profile is created in `~/.config/which-llm/config.toml`

#### Scenario: No API key error
- **WHEN** user runs `which-llm llms` without API key configured
- **THEN** error message suggests running `which-llm profile create`
