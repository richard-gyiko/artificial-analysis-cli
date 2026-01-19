# Artificial Analysis CLI

A command-line interface for querying AI model benchmarks from [Artificial Analysis](https://artificialanalysis.ai).

## Installation

```bash
cargo install --path .
```

## Setup

1. Create an account at [Artificial Analysis](https://artificialanalysis.ai/login) and generate an API key
2. Create a profile with your API key:

```bash
aa profile create default --api-key YOUR_API_KEY
```

Or set the `AA_API_KEY` environment variable.

## Usage

### Query LLM Models

```bash
# List all LLM models (default: markdown table)
aa llms

# Filter by creator and sort by intelligence
aa llms --creator openai --sort intelligence

# Output as JSON for scripting
aa llms --json

# Output as CSV
aa llms --csv
```

### Query Media Models

```bash
# Text-to-image rankings
aa text-to-image

# With category breakdown
aa text-to-image --categories

# Other media endpoints
aa image-editing
aa text-to-speech
aa text-to-video
aa image-to-video
```

### Other Commands

```bash
# Check API quota
aa quota

# Manage cache
aa cache status
aa cache clear

# Manage profiles
aa profile list
aa profile create work --api-key KEY
aa profile default work
```

### Output Formats

- **Markdown** (default): AI-agent friendly tables
- `--json`: Full JSON response
- `--csv`: CSV format
- `--table`: ASCII table
- `--plain`: Tab-separated values

### Options

- `-p, --profile <NAME>`: Use a specific profile
- `--refresh`: Bypass cache and fetch fresh data
- `-q, --quiet`: Suppress attribution notice (for scripting)

## Attribution

Data provided by [Artificial Analysis](https://artificialanalysis.ai).

This CLI uses the [Artificial Analysis API](https://artificialanalysis.ai/documentation). Per the API terms, attribution is required for all use of the data.

## License

MIT
