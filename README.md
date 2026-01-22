# which-llm

> **Experimental:** This skill is under active development. APIs and recommendations may change.

An AI agent skill for selecting optimal LLMs based on task requirements, benchmarks, and cost constraints.

Uses real benchmark data from [Artificial Analysis](https://artificialanalysis.ai/) enriched with capability metadata from [models.dev](https://models.dev) via the [`which-llm` CLI](https://github.com/richard-gyiko/which-llm-cli).

## Quick Start

### Installation

```bash
# Using degit (recommended)
degit richard-gyiko/which-llm/skills/which-llm skills/which-llm

# Or clone and copy
git clone https://github.com/richard-gyiko/which-llm.git
cp -r which-llm/skills/which-llm /path/to/your/skills/
```

### Prerequisites

This skill requires the `which-llm` CLI. See [installation instructions](skills/which-llm/references/INSTALL.md).

```bash
# Verify which-llm is installed
which-llm --version
```

## What It Does

Helps AI agents select the right LLM for a task by:

1. **Classifying tasks** into skill types (Transformational, Analytical, Tool-using, Agentic)
2. **Checking constraints** (context window, tool calling, structured output, latency)
3. **Querying benchmarks** to find models meeting requirements
4. **Recommending Primary + Fallback** models for cost-optimized cascades

## Skill Types

| Skill Type | Examples | Min Intelligence | Min Coding | Consider Also |
|------------|----------|------------------|------------|---------------|
| **Transformational** | summarize, extract, reformat | 20 | - | `tps` for volume |
| **Analytical** | compare, analyze, justify | 38 | - | `context_window` |
| **Tool-using** | API calls, DB queries, code execution | 35 | 35 | `tool_call = true` |
| **Agentic** | plan, decompose, orchestrate | 48 | 42 | `tool_call`, `reasoning` |

> Thresholds calibrated for Intelligence Index v4.0 (Jan 2026), where SOTA models score ~50.

## Benchmark Guidance & Specialization

Understand what benchmarks can and can't tell you:

- **[BENCHMARK-LIMITATIONS.md](skills/which-llm/references/BENCHMARK-LIMITATIONS.md)** - Contamination risks, aggregate score issues, when to distrust benchmarks
- **[SPECIALIZATION.md](skills/which-llm/references/SPECIALIZATION.md)** - Domain-specific model guidance (code, math, long context, vision)
- **Enhanced task-to-benchmark mapping** - Know which specific benchmark matters for your task

## Weighted Scoring

Rank models using weighted scores based on your priorities:

| Preset | Quality | Cost | Speed | Best For |
|--------|---------|------|-------|----------|
| **Balanced** | 0.4 | 0.4 | 0.2 | General use |
| **Quality** | 0.7 | 0.2 | 0.1 | Critical tasks |
| **Cost** | 0.2 | 0.7 | 0.1 | High volume |
| **Speed** | 0.2 | 0.2 | 0.6 | Real-time apps |

## Capability Filtering

Beyond benchmark scores, you can now filter by model capabilities:

- **`context_window`** - For long document processing
- **`tool_call`** - For function calling and API integration
- **`structured_output`** - For JSON mode and typed outputs
- **`reasoning`** - For chain-of-thought models
- **`open_weights`** - For self-hosting requirements

## Example Usage

```
User: Which LLM should I use for an agent that calls external APIs?

Agent: [loads which-llm skill, classifies as Tool-using, checks tool_call support]

       ## Task Classification
       - Skill Type: Tool-using
       - Key Constraints: Requires tool_call = true
       
       ## Recommendations
       
       ### Primary: GPT-4o-mini ($0.30/M)
       - intelligence 38, coding 36, tool_call: yes
       - Why: Cheapest model with reliable tool calling
       
       ### Fallback: Claude 3.5 Sonnet ($3.00/M)
       - intelligence 45, coding 43, tool_call: yes
       - Why: Higher capability for complex tool orchestration
       
       ## Cost Estimate
       - With 30% fallback rate: $1.11/M (63% savings vs always using fallback)
```

## Documentation

- [SKILL.md](skills/which-llm/SKILL.md) - Main skill definition
- [BENCHMARKS.md](skills/which-llm/references/BENCHMARKS.md) - What the scores mean
- [BENCHMARK-LIMITATIONS.md](skills/which-llm/references/BENCHMARK-LIMITATIONS.md) - What benchmarks can't tell you
- [SPECIALIZATION.md](skills/which-llm/references/SPECIALIZATION.md) - Domain-specific model guidance
- [QUERIES.md](skills/which-llm/references/QUERIES.md) - Common query patterns
- [MULTI-MODEL.md](skills/which-llm/references/MULTI-MODEL.md) - Multi-model architecture guidance
- [CASCADE-PATTERNS.md](skills/which-llm/references/CASCADE-PATTERNS.md) - Cost-optimized cascade patterns
- [INSTALL.md](skills/which-llm/references/INSTALL.md) - Installing the `which-llm` CLI

## License

MIT
