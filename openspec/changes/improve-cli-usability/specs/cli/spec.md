## ADDED Requirements

### Requirement: Refresh Command
The CLI SHALL provide a `refresh` command to fetch fresh data for all tables.

#### Scenario: Refresh all data
- **WHEN** user runs `which-llm refresh`
- **THEN** fetch fresh data from Artificial Analysis and models.dev
- **AND** display progress for each table: "Refreshing benchmarks... done (N models)"

#### Scenario: Refresh with quiet mode
- **WHEN** user runs `which-llm refresh --quiet`
- **THEN** fetch fresh data without printing progress messages

### Requirement: Tables Command
The CLI SHALL provide a `tables` command to list available tables and their schemas.

#### Scenario: List all tables
- **WHEN** user runs `which-llm tables`
- **THEN** display all available tables with their columns and cache status

#### Scenario: Show specific table
- **WHEN** user runs `which-llm tables benchmarks`
- **THEN** display columns and sample data for the specified table

### Requirement: Improved Error Messages
The CLI SHALL provide actionable error messages for query failures.

#### Scenario: Table not cached
- **WHEN** user queries a table that is not cached
- **THEN** display error suggesting `which-llm refresh`

#### Scenario: Unknown table
- **WHEN** user queries a table that does not exist
- **THEN** display error listing available tables

## REMOVED Requirements

### Requirement: LLMs Command
**Reason**: Replaced by SQL-first interface. Users query `benchmarks` table directly.
**Migration**: Use `which-llm query "SELECT * FROM benchmarks"`

### Requirement: Media Commands
**Reason**: Low usage, adds complexity. Data still available via SQL.
**Migration**: Use `which-llm query "SELECT * FROM text_to_image"` etc.
