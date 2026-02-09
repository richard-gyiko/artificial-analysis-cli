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

### 2. Start Using It

**No API key required!** The CLI fetches pre-built benchmark data from GitHub Releases, updated daily.

```bash
# List all LLM models
which-llm llms

# Filter by creator
which-llm llms --creator openai

# Check data source info
which-llm info
```

<details>
<summary>Optional: Configure API access for real-time data</summary>

For the freshest data (instead of daily snapshots), you can configure direct API access to [Artificial Analysis](https://artificialanalysis.ai):

1. Create an account at [artificialanalysis.ai/login](https://artificialanalysis.ai/login)
2. Generate an API key
3. Configure the CLI:

```bash
which-llm profile create default --api-key YOUR_API_KEY
```

Or set the `ARTIFICIAL_ANALYSIS_API_KEY` environment variable.

Then use the `--use-api` flag to fetch directly from the API:

```bash
which-llm llms --use-api
```

</details>

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

> **Tip:** Most AI assistants will automatically load the skill when you mention "which llm" in your question—no need to explicitly tell it to load the skill.

> **Experimental:** The skill is under active development. Recommendations and output format may change as we refine the task classification and benchmark interpretation.

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

## Example

**You:** *"I'm building a feature that summarizes long Slack threads so users can catch up quickly. Threads can be pretty long, like 200+ messages. Which LLM should I use?"*

**Assistant (with skill loaded):**

> ## Task Classification
> - **Skill Type:** Transformational (summarization)
> - **Key Constraints:** Large context window (200+ messages)
> - **Priority:** Balanced (quality + cost + speed matter equally)
> 
> ## Recommendations
> 
> ### Primary: gpt-oss-20B ($0.10/M)
> - Intelligence: 24.7 | Context: 131k tokens | Throughput: 305 tps
> - Why: Excellent cost-capability ratio. 131k context easily handles 200+ messages.
> 
> ### Fallback: MiMo-V2-Flash ($0.15/M)
> - Intelligence: 30-39 | Context: 256k tokens
> - Use if: Primary struggles with nuanced summaries or threads exceed 300+ messages
> 
> ## Cost Estimate
> - **Cascade (70/30 split):** $0.115/M tokens
> - **Savings vs always using fallback:** 23%
>
> **Validation step:** Before deploying, test both models on 5-10 representative Slack threads from your workspace.

[View full transcript](examples/opencode_haiku_45_transcript.md) — shows the complete flow including CLI queries and scoring.

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

### Compare Models

Compare models side-by-side with highlighted winners:

```bash
# Compare two or more models
which-llm compare "gpt-5 (high)" "claude 4.5 sonnet" "gemini 2.5 pro"

# Show additional fields
which-llm compare "gpt-5" "claude-4.5" --verbose

# Output formats: --json, --csv, --table, --plain
which-llm compare "gpt-5" "claude-4.5" --json
```

The compare command uses fuzzy matching on model names and displays a transposed table with models as columns and metrics as rows. Winners for each metric are marked with `*`.

### Calculate Token Costs

Estimate token costs with projections:

```bash
# Single model cost calculation
which-llm cost "gpt-5 (high)" --input 10k --output 5k

# Compare costs across models
which-llm cost "gpt-5" "claude 4.5" --input 1M --output 500k

# Daily/monthly projections with request volume
which-llm cost "gpt-5 (high)" --input 2k --output 1k --requests 1000 --period daily

# Supports token units: k (thousands), M (millions), B (billions)
which-llm cost "claude-4.5" --input 1.5M --output 750k
```

### Other Commands

```bash
# Force refresh data from GitHub
which-llm llms --refresh

# View data source and attribution info
which-llm info

# Manage cache
which-llm cache status
which-llm cache clear

# Manage profiles (for API access)
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

This tool uses data from the [Artificial Analysis API](https://artificialanalysis.ai/documentation). Per the API terms, attribution is required for all use of the data.

## Data Freshness

The CLI uses pre-built benchmark data hosted on GitHub Releases, updated daily via automated workflows. This means:

- **No API key required** for basic usage
- Data is typically **less than 24 hours old**
- Use `which-llm info` to see when data was last updated
- Use `--refresh` to force a fresh download from GitHub
- Use `--use-api` with an API key for real-time data

## License

MIT
