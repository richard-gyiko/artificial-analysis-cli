# Installing the `which-llm` CLI

This skill requires the `which-llm` CLI to be installed and configured.

## Quick Check

```bash
which-llm --version
```

If this works, you're ready. If not, follow the installation steps below.

## Installation

### macOS / Linux (Homebrew)

```bash
brew tap richard-gyiko/tap
brew install which-llm
```

### Windows (Scoop)

```powershell
scoop bucket add richard-gyiko https://github.com/richard-gyiko/scoop-bucket
scoop install which-llm
```

### Manual Download

Download from [GitHub Releases](https://github.com/richard-gyiko/which-llm-cli/releases):

```bash
# macOS (Apple Silicon)
curl -LO https://github.com/richard-gyiko/which-llm-cli/releases/latest/download/which-llm-aarch64-apple-darwin.tar.gz
tar -xzf which-llm-aarch64-apple-darwin.tar.gz
sudo mv which-llm /usr/local/bin/

# macOS (Intel)
curl -LO https://github.com/richard-gyiko/which-llm-cli/releases/latest/download/which-llm-x86_64-apple-darwin.tar.gz
tar -xzf which-llm-x86_64-apple-darwin.tar.gz
sudo mv which-llm /usr/local/bin/

# Linux
curl -LO https://github.com/richard-gyiko/which-llm-cli/releases/latest/download/which-llm-x86_64-unknown-linux-gnu.tar.gz
tar -xzf which-llm-x86_64-unknown-linux-gnu.tar.gz
sudo mv which-llm /usr/local/bin/
```

### From Source (requires Rust)

```bash
git clone https://github.com/richard-gyiko/which-llm-cli.git
cd which-llm-cli
cargo install --path .
```

## Setup

1. Create an account at [Artificial Analysis](https://artificialanalysis.ai/login)
2. Generate an API key
3. Configure the CLI:

```bash
which-llm profile create default --api-key YOUR_API_KEY
```

Or set the environment variable:

```bash
export ARTIFICIAL_ANALYSIS_API_KEY=YOUR_API_KEY
```

## Verify Installation

```bash
# Should show available models
which-llm llms --quiet | head -20
```

If you see a table of models, you're all set!

## Data Sources

The `which-llm` CLI combines data from two sources:

- **[Artificial Analysis](https://artificialanalysis.ai)** - Benchmark scores (intelligence, coding, math, etc.) and performance metrics (price, latency, tps)
- **[models.dev](https://models.dev)** - Capability metadata (context_window, tool_call, structured_output, reasoning, etc.)

About 53% of models have enriched capability data from models.dev. Use `models_dev_matched = true` in queries to filter for models with full capability information.
