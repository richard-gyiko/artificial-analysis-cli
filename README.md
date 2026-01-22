# which-llm

**Stop guessing which LLM to use.** Get data-driven model recommendations based on your task requirements, budget, and performance needs.

With 100+ LLMs available—each with different strengths, pricing, and capabilities—choosing the right one is overwhelming. `which-llm` queries real benchmark data and gives you actionable recommendations.

> **Note:** This tool provides best-effort suggestions based on benchmark scores and capability metadata. It does not substitute proper evaluation on your specific use case. Benchmarks have known limitations and may not reflect real-world performance for your domain.

## Quick Start

The easiest way to use `which-llm` is through the **agent skill**—your AI coding assistant (Cursor, Claude Code, Copilot, etc.) learns how to recommend models for you automatically.

### 1. Install the CLI

```bash
# macOS / Linux
brew tap richard-gyiko/tap
brew install which-llm

# Windows
scoop bucket add richard-gyiko https://github.com/richard-gyiko/scoop-bucket
scoop install which-llm
```

<details>
<summary>Other installation methods</summary>

**Manual download** from [GitHub Releases](https://github.com/richard-gyiko/which-llm/releases):

```bash
# macOS (Apple Silicon)
curl -LO https://github.com/richard-gyiko/which-llm/releases/latest/download/which-llm-aarch64-apple-darwin.tar.gz
tar -xzf which-llm-aarch64-apple-darwin.tar.gz
sudo mv which-llm /usr/local/bin/

# macOS (Intel)
curl -LO https://github.com/richard-gyiko/which-llm/releases/latest/download/which-llm-x86_64-apple-darwin.tar.gz
tar -xzf which-llm-x86_64-apple-darwin.tar.gz
sudo mv which-llm /usr/local/bin/

# Linux
curl -LO https://github.com/richard-gyiko/which-llm/releases/latest/download/which-llm-x86_64-unknown-linux-gnu.tar.gz
tar -xzf which-llm-x86_64-unknown-linux-gnu.tar.gz
sudo mv which-llm /usr/local/bin/
```

**From source** (requires Rust):

```bash
cargo install --path .
```

</details>

### 2. Configure API Access

The CLI fetches benchmark data from [Artificial Analysis](https://artificialanalysis.ai), which requires a free API key.

1. Create an account at [artificialanalysis.ai/login](https://artificialanalysis.ai/login)
2. Generate an API key
3. Configure the CLI:

```bash
which-llm profile create default --api-key YOUR_API_KEY
```

Or set the `ARTIFICIAL_ANALYSIS_API_KEY` environment variable.

### 3. Install the Skill

```bash
# Pick your AI coding tool
which-llm skill install cursor      # Cursor
which-llm skill install claude      # Claude Code
which-llm skill install opencode    # OpenCode
which-llm skill install codex       # Codex CLI
which-llm skill install windsurf    # Windsurf
which-llm skill install copilot     # GitHub Copilot
which-llm skill install antigravity # Antigravity

# Or install globally (available in all projects)
which-llm skill install cursor --global
```

Now just ask your AI assistant: *"Which LLM should I use for [your task]?"*

The skill follows the [Agent Skills](https://agentskills.io) open standard. See the [full skill documentation](skills/which-llm/SKILL.md) for details on how it classifies tasks and selects models.

## How It Works

The skill teaches your AI assistant to:

1. **Classify your task** — Is it transformational (summarize, extract), analytical (compare, justify), tool-using (API calls), or agentic (planning, orchestration)?
2. **Derive requirements** — Map task complexity to minimum benchmark thresholds
3. **Query real data** — Filter models by capability (tool calling, context window, structured output)
4. **Recommend Primary + Fallback** — Suggest a cost-optimized cascade (cheap model first, escalate when needed)

Data sources:
- **[Artificial Analysis](https://artificialanalysis.ai)** — Benchmark scores (intelligence, coding, math) and performance metrics (price, latency, throughput)
- **[models.dev](https://models.dev)** — Capability metadata (context window, tool calling, structured output, reasoning)

## CLI Reference

For power users, scripting, or debugging, you can query the data directly.

### Basic Queries

```bash
# List all LLM models
which-llm llms

# Filter and sort
which-llm llms --creator openai --sort intelligence

# Output formats: --json, --csv, --table, --plain
which-llm llms --json
```

### SQL Queries

Use full SQL expressiveness on the cached benchmark data:

```bash
# Best coding models under $5/M
which-llm query "SELECT name, creator, coding, output_price FROM llms WHERE coding > 40 AND output_price < 5 ORDER BY coding DESC"

# Models with tool calling and large context
which-llm query "SELECT name, context_window, tool_call FROM llms WHERE tool_call = true AND context_window > 100000"

# List available tables
which-llm query --tables
```

<details>
<summary>Available tables and columns</summary>

#### Tables

| Table | Description |
|-------|-------------|
| `llms` | LLM models with benchmarks and capabilities |
| `text_to_image` | Text-to-image models |
| `image_editing` | Image editing models |
| `text_to_speech` | Text-to-speech models |
| `text_to_video` | Text-to-video models |
| `image_to_video` | Image-to-video models |

#### LLMs Table — Core Fields

| Column | Type | Description |
|--------|------|-------------|
| `name` | VARCHAR | Model name |
| `creator` | VARCHAR | Creator (OpenAI, Anthropic, etc.) |
| `intelligence` | DOUBLE | Intelligence index |
| `coding` | DOUBLE | Coding index |
| `math` | DOUBLE | Math index |
| `input_price` | DOUBLE | Price per 1M input tokens |
| `output_price` | DOUBLE | Price per 1M output tokens |
| `tps` | DOUBLE | Tokens per second |
| `latency` | DOUBLE | Time to first token (seconds) |

#### LLMs Table — Capability Fields

| Column | Type | Description |
|--------|------|-------------|
| `context_window` | BIGINT | Maximum context window |
| `tool_call` | BOOLEAN | Supports function calling |
| `structured_output` | BOOLEAN | Supports JSON mode |
| `reasoning` | BOOLEAN | Chain-of-thought model |
| `open_weights` | BOOLEAN | Weights publicly available |

> **Note:** Capability fields are `NULL` for ~47% of models not matched to models.dev. Use `models_dev_matched = true` to filter for complete data.

</details>

### Other Commands

```bash
# Manage cache
which-llm cache status
which-llm cache clear

# Manage profiles
which-llm profile list
which-llm profile create work --api-key KEY
which-llm profile default work

# Skill management
which-llm skill list
which-llm skill uninstall cursor
```

## Attribution

- Benchmark data provided by [Artificial Analysis](https://artificialanalysis.ai)
- Capability metadata provided by [models.dev](https://models.dev)

This tool uses the [Artificial Analysis API](https://artificialanalysis.ai/documentation). Per the API terms, attribution is required for all use of the data.

## License

MIT
