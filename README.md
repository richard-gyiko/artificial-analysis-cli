# Artificial Analysis CLI

A command-line interface for querying AI model benchmarks from [Artificial Analysis](https://artificialanalysis.ai).

## Installation

### Windows (Scoop)

```powershell
scoop bucket add aa https://github.com/richard-gyiko/scoop-aa
scoop install aa
```

### macOS / Linux

Download the latest release from [GitHub Releases](https://github.com/richard-gyiko/artificial-analysis-cli/releases):

```bash
# macOS (Apple Silicon)
curl -LO https://github.com/richard-gyiko/artificial-analysis-cli/releases/latest/download/aa-aarch64-apple-darwin.tar.gz
tar -xzf aa-aarch64-apple-darwin.tar.gz
sudo mv aa /usr/local/bin/

# macOS (Intel)
curl -LO https://github.com/richard-gyiko/artificial-analysis-cli/releases/latest/download/aa-x86_64-apple-darwin.tar.gz
tar -xzf aa-x86_64-apple-darwin.tar.gz
sudo mv aa /usr/local/bin/

# Linux
curl -LO https://github.com/richard-gyiko/artificial-analysis-cli/releases/latest/download/aa-x86_64-unknown-linux-gnu.tar.gz
tar -xzf aa-x86_64-unknown-linux-gnu.tar.gz
sudo mv aa /usr/local/bin/
```

### From Source

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

### SQL Queries

Use SQL to filter, sort, and aggregate cached data with full expressiveness:

```bash
# Best coding models under $5/M output price
aa query "SELECT name, creator, coding, output_price FROM llms WHERE coding > 40 AND output_price < 5 ORDER BY coding DESC"

# Fastest models with good intelligence
aa query "SELECT name, intelligence, tps FROM llms WHERE intelligence > 35 AND tps > 100 ORDER BY tps DESC LIMIT 10"

# Compare creators by average intelligence
aa query "SELECT creator, COUNT(*) as models, ROUND(AVG(intelligence), 1) as avg_intel FROM llms WHERE intelligence IS NOT NULL GROUP BY creator ORDER BY avg_intel DESC"

# Top image generation models
aa query "SELECT name, creator, elo, rank FROM text_to_image WHERE elo > 1200 ORDER BY elo DESC"

# List available tables and their schemas
aa query --tables
```

#### Available Tables

| Table | Source Command |
|-------|----------------|
| `llms` | `aa llms` |
| `text_to_image` | `aa text-to-image` |
| `image_editing` | `aa image-editing` |
| `text_to_speech` | `aa text-to-speech` |
| `text_to_video` | `aa text-to-video` |
| `image_to_video` | `aa image-to-video` |

#### LLMs Table Columns

| Column | Type | Description |
|--------|------|-------------|
| `id` | VARCHAR | Model ID |
| `name` | VARCHAR | Model name |
| `slug` | VARCHAR | URL slug |
| `creator` | VARCHAR | Creator name |
| `creator_slug` | VARCHAR | Creator slug |
| `release_date` | VARCHAR | Release date |
| `intelligence` | DOUBLE | Intelligence index |
| `coding` | DOUBLE | Coding index |
| `math` | DOUBLE | Math index |
| `mmlu_pro` | DOUBLE | MMLU-Pro score |
| `gpqa` | DOUBLE | GPQA score |
| `input_price` | DOUBLE | Input price per 1M tokens |
| `output_price` | DOUBLE | Output price per 1M tokens |
| `price` | DOUBLE | Blended price (3:1 ratio) |
| `tps` | DOUBLE | Tokens per second |
| `latency` | DOUBLE | Time to first token (seconds) |

#### Media Tables Columns

All media tables (`text_to_image`, `image_editing`, etc.) share this schema:

| Column | Type | Description |
|--------|------|-------------|
| `id` | VARCHAR | Model ID |
| `name` | VARCHAR | Model name |
| `slug` | VARCHAR | URL slug |
| `creator` | VARCHAR | Creator name |
| `elo` | DOUBLE | ELO score |
| `rank` | INTEGER | Rank |
| `release_date` | VARCHAR | Release date |

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
