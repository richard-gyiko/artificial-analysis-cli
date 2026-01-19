# Change: Add Artificial Analysis CLI

## Why
Developers, researchers, and **AI agents** need a command-line tool to quickly query AI model 
benchmarks, compare performance metrics, and retrieve pricing data from Artificial Analysis 
without leaving their terminal. A Rust-based CLI provides fast execution, cross-platform 
support, and easy distribution as a single binary.

## What Changes
- **ADDED**: New Rust CLI application for Artificial Analysis API
- **ADDED**: Commands for querying LLM benchmarks and metrics
- **ADDED**: Commands for querying media model rankings (image, video, speech)
- **ADDED**: Markdown as default output format (AI-agent friendly)
- **ADDED**: Multiple output formats (JSON, CSV, ASCII table, plain)
- **ADDED**: Profile-based API key management
- **ADDED**: Quota tracking and low-quota warnings
- **ADDED**: Response caching to respect rate limits

## Impact
- Affected specs: `cli` (new capability)
- Affected code: New project - all Rust source files

## Scope

### In Scope
- LLM data endpoint: models, evaluations, pricing, speed metrics
- Media model endpoints: text-to-image, image-editing, text-to-speech, text-to-video, image-to-video
- Output formatting: **markdown (default)**, JSON, CSV, ASCII table, plain
- Profile management: multiple API keys, default profile, environment override
- Quota tracking: usage display, low-quota warnings
- Basic filtering and sorting options
- Response caching with configurable TTL

### Out of Scope (Future Work)
- CritPt benchmark evaluation (POST endpoint - complex submission workflow)
- Interactive/TUI mode
- Webhook integrations
- Real-time monitoring
