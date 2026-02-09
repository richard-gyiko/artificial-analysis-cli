---
name: which-llm
description: Select optimal LLM(s) for a task based on skill requirements, budget, and constraints. Uses the `which-llm` CLI to query Artificial Analysis benchmarks enriched with capability data from models.dev.
license: MIT
compatibility: Requires `which-llm` CLI installed and configured with API key
metadata:
  author: richard-gyiko
  version: "0.5.0"
  category: "ai"
allowed-tools: Bash(which-llm:*) Read
---

# Skill: which-llm

Select the right LLM(s) for a task using real benchmark and capability data.

## When to Use

- User needs to pick a model for a specific task
- User wants to compare models by capability/price/speed
- User is designing a multi-agent system and needs model recommendations

## Preflight Check

Before proceeding, verify the CLI is operational:

```bash
# 1. Check CLI exists and show version
which-llm --version

# 2. Check data freshness (should be < 7 days old)
which-llm cache status

# 3. If stale or missing, refresh data
which-llm llms --refresh
```

**If CLI is unavailable:** See [references/INSTALL.md](references/INSTALL.md) for installation, or use [references/FALLBACK.md](references/FALLBACK.md) for heuristic recommendations without the CLI.

## Quick Start

1. **Run preflight check** to verify CLI is ready
2. **Classify the task** using the decision tree below
3. **Query with `which-llm`** to find matching models
4. **Recommend Primary + Fallback** models with tradeoffs

## Task Classification

Use this decision tree to classify the user's task:

```
Is the task primarily about FORMAT conversion (summarize, extract, reformat)?
├─ YES → Transformational (intelligence ≥ 20)
└─ NO ↓

Does it require EXTERNAL ACTIONS (API calls, DB queries, file ops, code execution)?
├─ YES → Does it need to PLAN multiple steps autonomously?
│        ├─ YES → Agentic (intelligence ≥ 48, coding ≥ 42)
│        └─ NO  → Tool-using (intelligence ≥ 35, coding ≥ 35)
└─ NO ↓

Does it require JUDGMENT, COMPARISON, or ANALYSIS?
├─ YES → Analytical (intelligence ≥ 38)
└─ NO  → Transformational (intelligence ≥ 20)
```

### Skill Type Reference

> **Note:** Thresholds calibrated for Intelligence Index v4.0 (Jan 2026), SOTA ~50. Scores within ±2 points are effectively equivalent. See [references/BENCHMARKS.md](references/BENCHMARKS.md) for dynamic threshold calculation.

| Skill Type | Examples | Min Intelligence | Min Coding | Consider Also |
|------------|----------|------------------|------------|---------------|
| **Transformational** | summarize, extract, reformat | 20 | - | `tps` for high volume |
| **Analytical** | compare, analyze, justify | 38 | - | `context_window` for long docs |
| **Tool-using** | API calls, DB queries, code execution | 35 | 35 | `tool_call = true` |
| **Agentic** | plan, decompose, orchestrate, self-critique | 48 | 42 | `tool_call`, `reasoning`, large `context_window` |

## Additional Selection Factors

Beyond skill type thresholds, consider these constraints when relevant:

| Factor | Column | When to Use |
|--------|--------|-------------|
| Context window | `context_window` | Long documents (>32k tokens), RAG with large chunks |
| Tool calling | `tool_call` | Function calling, MCP servers, API integration |
| Structured output | `structured_output` | JSON responses, typed outputs, schema validation |
| Reasoning | `reasoning` | Complex multi-step problems, chain-of-thought |
| Latency | `latency` | Real-time chat, streaming UIs (want < 0.5s) |
| Throughput | `tps` | Batch processing, high volume (want > 100 tps) |
| Open weights | `open_weights` | Self-hosting, fine-tuning, data privacy |

> **Note:** Capability fields require `models_dev_matched = true` (~53% of models have this data). Models without capability data will have NULL values for these fields.

## Weighted Scoring

Instead of just filtering by thresholds, use weighted scoring to rank models based on user priorities.

### Scoring Formula

```
Score = (intelligence × quality_weight) 
      + (100/price × cost_weight) 
      + (tps/10 × speed_weight)
```

### Priority Presets

| Preset | Quality | Cost | Speed | Best For |
|--------|---------|------|-------|----------|
| **Balanced** | 0.4 | 0.4 | 0.2 | General use, no strong preference |
| **Quality** | 0.7 | 0.2 | 0.1 | Critical tasks, accuracy matters most |
| **Cost** | 0.2 | 0.7 | 0.1 | High volume, budget-sensitive |
| **Speed** | 0.2 | 0.2 | 0.6 | Real-time, latency-sensitive |

### Weighted Query Example

```bash
# Balanced scoring for analytical tasks
which-llm query "SELECT name, intelligence, price, tps,
          ROUND((intelligence * 0.4) + (100/price * 0.4) + (tps/10 * 0.2), 1) as score
          FROM llms 
          WHERE intelligence >= 38 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"

# Cost-priority scoring
which-llm query "SELECT name, intelligence, price, tps,
          ROUND((intelligence * 0.2) + (100/price * 0.7) + (tps/10 * 0.1), 1) as score
          FROM llms 
          WHERE intelligence >= 38 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"
```

See [references/QUERIES.md](references/QUERIES.md) for more weighted scoring patterns.

## Core Queries

```bash
# Find models meeting requirements, sorted by price
which-llm query "SELECT name, creator, intelligence, coding, price, tps 
          FROM llms 
          WHERE intelligence >= 38 
          ORDER BY price 
          LIMIT 10"

# Tool-using tasks: require tool_call support
which-llm query "SELECT name, creator, intelligence, coding, tool_call, price 
          FROM llms 
          WHERE intelligence >= 35 AND coding >= 35 AND tool_call = true
          ORDER BY price 
          LIMIT 10"

# Agentic tasks: high capability + tool calling + large context
which-llm query "SELECT name, creator, intelligence, coding, context_window, price 
          FROM llms 
          WHERE intelligence >= 48 AND coding >= 42 
            AND tool_call = true AND context_window >= 128000
          ORDER BY price 
          LIMIT 10"

# Speed-critical (real-time chat)
which-llm query "SELECT name, intelligence, tps, latency, price 
          FROM llms 
          WHERE tps > 100 AND latency < 0.5 
          ORDER BY tps DESC"
```

## Compare Models

Use the `compare` command for side-by-side model comparison with winner highlighting:

```bash
# Compare candidate models directly
which-llm compare "gpt-5 (high)" "claude 4.5 sonnet" "gemini 2.5 pro"

# Include additional metrics with --verbose
which-llm compare "gpt-5" "claude-4.5" --verbose

# Output as JSON for programmatic use
which-llm compare "gpt-5" "claude-4.5" --json
```

Winners for each metric are marked with `*`. This is useful when presenting trade-offs to users.

## Calculate Token Costs

Use the `cost` command to estimate costs and project usage:

```bash
# Calculate cost for a single model
which-llm cost "gpt-5 (high)" --input 10k --output 5k

# Compare costs across multiple models
which-llm cost "gpt-5" "claude 4.5" --input 1M --output 500k

# Project daily/monthly costs with request volume
which-llm cost "gpt-5 (high)" --input 2k --output 1k --requests 1000 --period daily
```

Token units: `k` (thousands), `M` (millions), `B` (billions). Decimals supported (e.g., `1.5M`).

## Cascade Recommendations

For cost optimization, recommend a **Primary + Fallback** pair instead of a single model:

1. **Primary model**: Cheapest model meeting *relaxed* requirements (e.g., 5-10 points below strict threshold)
2. **Fallback model**: Higher-capability model meeting *strict* requirements for when primary fails

**Relaxed vs Strict thresholds:**
- Strict: Use the skill type thresholds directly (e.g., Agentic: intelligence ≥ 48)
- Relaxed: Lower by ~20% for primary selection (e.g., intelligence ≥ 40)

This approach can reduce costs by 50-70% compared to always using the best model. See [references/CASCADE-PATTERNS.md](references/CASCADE-PATTERNS.md) for detailed implementation guidance.

## Output Format

```markdown
## Task Classification
- **Skill Type:** [type]
- **Key Constraints:** [e.g., needs tool_call, context > 100k]
- **Priority:** [Balanced/Quality/Cost/Speed]
- **Reasoning:** [why this classification]

## Recommendations

### Primary: [Model] ($X.XX/M) - Score: Y
- Meets requirements: intelligence X, coding Y
- Capabilities: [relevant capabilities like tool_call, context_window]
- Why: Best cost/capability ratio for this task

### Fallback: [Model] ($X.XX/M) - Score: Y
- Use if: Primary fails or task is unusually complex
- Capabilities: [relevant capabilities]
- Why: Higher capability ceiling

### Other Options
| Rank | Model | Score | Intelligence | Price | Key Capability |
|------|-------|-------|--------------|-------|----------------|
| 3 | ... | ... | ... | ... | ... |
| 4 | ... | ... | ... | ... | ... |

## Cost Estimate
- **Primary only:** $X.XX/M tokens
- **Fallback only:** $Y.YY/M tokens  
- **Cascade (30% fallback):** $Z.ZZ/M tokens
- **Savings vs always fallback:** NN%

### Calculation
Cascade cost = (0.70 × $X.XX) + (0.30 × $Y.YY) = $Z.ZZ/M
Savings = (1 - Z.ZZ/Y.YY) × 100 = NN%

## Query Used
[the which-llm query command]
```

### Cost Estimate Guidance

When presenting cost estimates:

1. **Always show the cascade calculation** - helps users understand the math
2. **Use 30% as default escalation rate** - reasonable starting assumption
3. **Note the price ratio** - larger ratios mean more potential savings
4. **Include the caveat** - actual savings depend on task complexity mix

Example:
```markdown
## Cost Estimate
- **Primary only:** $0.50/M tokens
- **Fallback only:** $5.00/M tokens (10x primary)
- **Cascade (30% fallback):** $1.85/M tokens
- **Savings vs always fallback:** 63%

*Assumes 30% of requests escalate. Actual rate depends on task complexity.*
```

## Important Disclaimer

These recommendations are **indicative starting points**, not definitive answers. Benchmark scores measure general capabilities but may not reflect performance on your specific task.

**Always validate** by testing candidate models on representative examples from your actual use case before committing to a model choice.

## References

For detailed information, see:
- [references/FALLBACK.md](references/FALLBACK.md) - Quick recommendations when CLI is unavailable
- [references/INSTALL.md](references/INSTALL.md) - Installing and configuring the `which-llm` CLI
- [references/BENCHMARKS.md](references/BENCHMARKS.md) - What the scores mean + dynamic thresholds
- [references/BENCHMARK-LIMITATIONS.md](references/BENCHMARK-LIMITATIONS.md) - What benchmarks can't tell you
- [references/QUERIES.md](references/QUERIES.md) - Common query patterns
- [references/WORKFLOWS.md](references/WORKFLOWS.md) - End-to-end workflow examples
- [references/MULTI-MODEL.md](references/MULTI-MODEL.md) - Multi-model architecture guidance
- [references/CASCADE-PATTERNS.md](references/CASCADE-PATTERNS.md) - Cascade and fallback patterns
- [references/SPECIALIZATION.md](references/SPECIALIZATION.md) - Domain-specific model guidance
- [references/PROVIDERS.md](references/PROVIDERS.md) - Provider-specific considerations
