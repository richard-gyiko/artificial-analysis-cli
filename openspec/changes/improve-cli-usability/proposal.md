# Change: Simplify CLI to SQL-First for Agentic Usage

## Why

The current CLI has overlapping commands that confuse both humans and agents:
- `which-llm llms` - lists benchmarks but command name says "llms"
- `which-llm query "SELECT * FROM benchmarks"` - same data, different interface
- No command for models.dev data except SQL

For agentic usage, SQL is the primary interface. The skill already teaches agents to write SQL queries. Having wrapper commands adds confusion without value.

## What Changes

1. **Remove `llms` command** - SQL is the primary interface
2. **Add `refresh` command** - Refreshes all data (benchmarks + models)
3. **Add `tables` command** - Shows available tables (currently `query --tables`)
4. **Keep `compare` and `cost`** - These add value beyond raw SQL
5. **Simplify help** - Focus on SQL workflow

## New CLI Structure

```bash
# Primary interface: SQL
which-llm query "SELECT * FROM benchmarks WHERE coding > 40"
which-llm query "SELECT * FROM models WHERE reasoning = true"

# Data management
which-llm refresh              # Fetch fresh data for all tables
which-llm tables               # List available tables and columns

# Value-add commands (keep)
which-llm compare "gpt-4o" "claude-3.5"   # Side-by-side comparison
which-llm cost "gpt-4o" --input 1M        # Cost calculator

# Utility (keep)
which-llm info                 # Data source attribution
which-llm cache status|clear   # Cache management
which-llm skill install        # Skill installation
```

## Impact

- Affected code: `src/cli/mod.rs`, `src/commands/`, `src/main.rs`
- **Breaking**: Removes `llms` command and all media commands
- Simplifies codebase significantly
