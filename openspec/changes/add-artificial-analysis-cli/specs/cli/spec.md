# CLI Capability Specification

## ADDED Requirements

### Requirement: CLI Application Structure
The system SHALL provide a command-line interface named `aa` (artificial-analysis) with subcommands organized by resource type.

#### Scenario: Display help information
- **WHEN** user runs `aa --help`
- **THEN** display available commands and global options

#### Scenario: Display version
- **WHEN** user runs `aa --version`
- **THEN** display the CLI version number

---

### Requirement: Profile-Based Configuration
The system SHALL support multiple named profiles for managing different API keys and settings.

#### Scenario: Use default profile
- **WHEN** user runs a command without `--profile` flag
- **AND** a default profile exists in config
- **THEN** use the default profile's API key and settings

#### Scenario: Use named profile
- **WHEN** user runs a command with `--profile <name>` flag
- **THEN** use the specified profile's API key and settings

#### Scenario: Override with environment variable
- **WHEN** `AA_API_KEY` environment variable is set
- **THEN** use the environment variable value regardless of profile settings

#### Scenario: Missing API key error
- **WHEN** no API key is configured in active profile or environment
- **AND** user runs a command requiring authentication
- **THEN** display error message explaining how to configure a profile
- **AND** exit with non-zero status code

#### Scenario: Profile configuration file location
- **WHEN** loading profiles
- **THEN** read from `~/.config/aa/config.toml`

---

### Requirement: Profile Management Command
The system SHALL provide commands to manage configuration profiles.

#### Scenario: Create new profile
- **WHEN** user runs `aa profile create <name>`
- **THEN** prompt for API key input (hidden)
- **AND** save the new profile to config file

#### Scenario: Create profile with key flag
- **WHEN** user runs `aa profile create <name> --api-key <key>`
- **THEN** save the new profile with the provided API key

#### Scenario: List profiles
- **WHEN** user runs `aa profile list`
- **THEN** display all configured profiles
- **AND** indicate which profile is the default

#### Scenario: Set default profile
- **WHEN** user runs `aa profile default <name>`
- **THEN** set the specified profile as the default

#### Scenario: Delete profile
- **WHEN** user runs `aa profile delete <name>`
- **THEN** remove the profile from config file
- **AND** prompt for confirmation if it's the default profile

#### Scenario: Show profile details
- **WHEN** user runs `aa profile show <name>`
- **THEN** display profile configuration (with API key masked)

---

### Requirement: LLM Models Command
The system SHALL provide a command to list and query LLM model data from the Artificial Analysis API.

#### Scenario: List all LLM models
- **WHEN** user runs `aa llms`
- **THEN** display LLM models with name, creator, intelligence index, and pricing

#### Scenario: Get specific model details
- **WHEN** user runs `aa llms --model <slug>`
- **THEN** display detailed information for the specified model including all evaluations and metrics

#### Scenario: Filter by creator
- **WHEN** user runs `aa llms --creator <creator-slug>`
- **THEN** display only models from the specified creator

#### Scenario: Sort by metric
- **WHEN** user runs `aa llms --sort <field>`
- **THEN** display models sorted by the specified field (e.g., price, speed, intelligence)

---

### Requirement: Text-to-Image Models Command
The system SHALL provide a command to list text-to-image model rankings.

#### Scenario: List text-to-image models
- **WHEN** user runs `aa text-to-image`
- **THEN** display text-to-image models with name, creator, ELO rating, and rank

#### Scenario: Include category breakdown
- **WHEN** user runs `aa text-to-image --categories`
- **THEN** display ELO scores broken down by style and subject categories

---

### Requirement: Image Editing Models Command
The system SHALL provide a command to list image editing model rankings.

#### Scenario: List image editing models
- **WHEN** user runs `aa image-editing`
- **THEN** display image editing models with name, creator, ELO rating, and rank

---

### Requirement: Text-to-Speech Models Command
The system SHALL provide a command to list text-to-speech model rankings.

#### Scenario: List text-to-speech models
- **WHEN** user runs `aa text-to-speech`
- **THEN** display text-to-speech models with name, creator, ELO rating, and rank

---

### Requirement: Text-to-Video Models Command
The system SHALL provide a command to list text-to-video model rankings.

#### Scenario: List text-to-video models
- **WHEN** user runs `aa text-to-video`
- **THEN** display text-to-video models with name, creator, ELO rating, and rank

#### Scenario: Include category breakdown
- **WHEN** user runs `aa text-to-video --categories`
- **THEN** display ELO scores broken down by style, subject, and format categories

---

### Requirement: Image-to-Video Models Command
The system SHALL provide a command to list image-to-video model rankings.

#### Scenario: List image-to-video models
- **WHEN** user runs `aa image-to-video`
- **THEN** display image-to-video models with name, creator, ELO rating, and rank

#### Scenario: Include category breakdown
- **WHEN** user runs `aa image-to-video --categories`
- **THEN** display ELO scores broken down by style, subject, and format categories

---

### Requirement: Output Format Options
The system SHALL support multiple output formats for all data commands, with markdown as the default for AI agent and human readability.

#### Scenario: Markdown output (default)
- **WHEN** user runs a command without format flag
- **THEN** display results as a formatted markdown table

#### Scenario: JSON output
- **WHEN** user runs a command with `--json` flag
- **THEN** output results as formatted JSON to stdout

#### Scenario: CSV output
- **WHEN** user runs a command with `--csv` flag
- **THEN** output results as CSV to stdout

#### Scenario: Table output
- **WHEN** user runs a command with `--table` flag
- **THEN** display results in a formatted ASCII table with box-drawing characters

#### Scenario: Plain output
- **WHEN** user runs a command with `--plain` flag
- **THEN** output tab-separated values without headers or formatting

---

### Requirement: Quota Status Command
The system SHALL provide a command to check API quota usage and remaining requests for the active profile.

#### Scenario: Check quota status
- **WHEN** user runs `aa quota`
- **THEN** display current API usage statistics for the active profile
- **AND** display remaining requests for the day
- **AND** display rate limit reset time

#### Scenario: Quota from cached data
- **WHEN** user runs `aa quota`
- **AND** rate limit headers were received from a recent API call for the active profile
- **THEN** display the cached quota information
- **AND** indicate when the data was last updated

#### Scenario: No quota data available
- **WHEN** user runs `aa quota`
- **AND** no cached quota data exists for the active profile
- **THEN** display "No quota data available. Run a data command (e.g., 'aa llms') to initialize quota tracking."

#### Scenario: Quota warning on commands
- **WHEN** user runs any data command
- **AND** remaining quota is below 10% (less than 100 requests)
- **THEN** display a warning about low remaining quota

---

### Requirement: Response Caching
The system SHALL cache API responses to reduce unnecessary requests and respect rate limits.

#### Scenario: Cache valid response
- **WHEN** a successful API response is received
- **THEN** cache the response with a configurable TTL (default: 1 hour)

#### Scenario: Use cached response
- **WHEN** user runs a command
- **AND** a valid cached response exists for the request
- **THEN** use the cached response instead of making an API call

#### Scenario: Force refresh
- **WHEN** user runs a command with `--refresh` flag
- **THEN** bypass cache and fetch fresh data from the API

#### Scenario: Clear cache
- **WHEN** user runs `aa cache clear`
- **THEN** remove all cached responses

#### Scenario: Show cache status
- **WHEN** user runs `aa cache status`
- **THEN** display cache location, size, and entry count

---

### Requirement: Error Handling
The system SHALL provide clear error messages for API and user errors.

#### Scenario: Invalid API key
- **WHEN** API returns 401 status
- **THEN** display "Invalid API key. Run 'aa profile create' to configure."
- **AND** exit with status code 1

#### Scenario: Rate limit exceeded
- **WHEN** API returns 429 status
- **THEN** display "Rate limit exceeded. Free tier allows 1,000 requests/day."
- **AND** display reset time if available
- **AND** exit with status code 1

#### Scenario: Network error
- **WHEN** network request fails
- **THEN** display "Network error: <details>"
- **AND** suggest checking internet connection
- **AND** exit with status code 1

#### Scenario: API server error
- **WHEN** API returns 500 status
- **THEN** display "Artificial Analysis API error. Please try again later."
- **AND** exit with status code 1
