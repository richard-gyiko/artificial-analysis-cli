# Project Context

## Purpose
A command-line interface (CLI) tool for interacting with the Artificial Analysis API.
Provides access to AI model benchmarks, performance metrics, pricing data, and media model rankings.

## Tech Stack
- Rust (primary language)
- clap - CLI argument parsing
- reqwest - HTTP client with async support
- serde/serde_json - JSON serialization/deserialization
- tokio - Async runtime
- tabled or comfy-table - Table formatting for output

## Project Conventions

### Code Style
- Follow Rust standard style guidelines (rustfmt)
- Use clippy for linting with pedantic warnings enabled
- Prefer explicit error types over anyhow in library code
- Use thiserror for error derivation
- Module organization: one file per logical unit

### Architecture Patterns
- Command pattern for CLI subcommands
- Client abstraction for API interactions
- Structured output (table, JSON, plain text)
- Configuration via environment variables and config files

### Testing Strategy
- Unit tests for parsing and data transformation
- Integration tests with mock server for API client
- Snapshot tests for CLI output formatting

### Git Workflow
- Conventional commits (feat:, fix:, docs:, refactor:)
- Feature branches merged via PR
- CI must pass before merge

## Domain Context
Artificial Analysis provides independent benchmarks and metrics for AI models:
- **LLMs**: Intelligence scores, speed metrics, pricing
- **Media models**: ELO ratings for text-to-image, image-editing, text-to-speech, text-to-video, image-to-video
- **CritPt**: Code generation benchmark evaluation

Key concepts:
- Models have IDs (stable), slugs (may change), and names
- Evaluations include proprietary indices and standard benchmarks
- Pricing is per million tokens (input/output/blended)
- Media models ranked by ELO with confidence intervals

## Important Constraints
- API rate limit: 1,000 requests/day on free tier
- API key must not be exposed in client-side code
- Attribution to Artificial Analysis required when using data
- Cache responses where appropriate to avoid rate limiting

## External Dependencies
- Artificial Analysis API: https://artificialanalysis.ai/api/v2/
- Authentication: x-api-key header
