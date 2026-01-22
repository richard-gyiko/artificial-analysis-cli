# Benchmark Reference

Detailed explanation of metrics and what scores mean.

> **Note:** These benchmarks measure general capabilities across standardized tests. They are useful for narrowing down candidates but do not guarantee performance on your specific task. Always validate with your own evaluations.

## Dynamic Threshold Calculation

Instead of using hardcoded thresholds that rot over time, derive thresholds from the current leaderboard:

```bash
# Step 1: Find current SOTA score
which-llm query "SELECT MAX(intelligence) as sota FROM llms"

# Step 2: Calculate tier thresholds relative to SOTA
# - Frontier (Agentic): >= 95% of SOTA
# - Strong (Analytical): >= 75% of SOTA
# - Capable (Tool-using): >= 70% of SOTA
# - Basic (Transformational): >= 40% of SOTA
```

### Example Calculation

If current SOTA = 52:
- **Frontier (Agentic):** >= 49 (52 × 0.95)
- **Strong (Analytical):** >= 39 (52 × 0.75)
- **Capable (Tool-using):** >= 36 (52 × 0.70)
- **Basic (Transformational):** >= 21 (52 × 0.40)

### Query to Get Current Thresholds

```bash
which-llm query "SELECT 
          MAX(intelligence) as sota,
          ROUND(MAX(intelligence) * 0.95, 0) as frontier_threshold,
          ROUND(MAX(intelligence) * 0.75, 0) as strong_threshold,
          ROUND(MAX(intelligence) * 0.70, 0) as capable_threshold,
          ROUND(MAX(intelligence) * 0.40, 0) as basic_threshold
          FROM llms"
```

This makes the skill self-calibrating as models improve.

## Core Benchmark Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `intelligence` | DOUBLE | Composite score (0-100) across reasoning, knowledge, coding, agents |
| `coding` | DOUBLE | Code generation and terminal-based task completion |
| `math` | DOUBLE | Mathematical reasoning (competition level) |
| `mmlu_pro` | DOUBLE | MMLU-Pro benchmark score |
| `gpqa` | DOUBLE | Graduate-level science Q&A |

## Additional Benchmarks

For more targeted selection, the CLI provides specialized benchmark scores:

| Metric | Type | Description |
|--------|------|-------------|
| `hle` | DOUBLE | HLE (Humanity's Last Exam) score |
| `livecodebench` | DOUBLE | LiveCodeBench - real-world coding tasks |
| `scicode` | DOUBLE | SciCode - scientific code generation |
| `math_500` | DOUBLE | MATH-500 benchmark score |
| `aime` | DOUBLE | AIME competition math problems |

### Task-Specific Benchmark Guidance

Use task-specific benchmarks instead of relying solely on aggregate `intelligence` scores.

| Task Type | Primary Metric | Secondary | Reliability | Why |
|-----------|---------------|-----------|-------------|-----|
| **Production coding** | `coding` | `livecodebench` | High | LiveCodeBench uses fresh problems, low contamination |
| **Bug fixing / PRs** | `coding` | - | Medium | SWE-bench is ideal but not in CLI; `coding` correlates |
| **Scientific code** | `coding` | `scicode` | High | SciCode tests domain-specific code generation |
| **Competition math** | `math` | `aime` | High | AIME is uncontaminated competition problems |
| **Applied math** | `math` | `math_500` | Medium | MATH-500 covers broader problem types |
| **Scientific Q&A** | `gpqa` | `hle` | High | Graduate-level, low contamination |
| **Expert knowledge** | `hle` | `gpqa` | High | HLE is Humanity's Last Exam - hardest available |
| **General knowledge** | `mmlu_pro` | `intelligence` | Medium | MMLU-Pro is harder than original MMLU |
| **Instruction following** | `intelligence` | - | Low | No direct IF-Eval in CLI; `intelligence` includes it |
| **Long document analysis** | `intelligence` | - | Medium | Check `context_window` as primary filter |
| **Tool use / Agents** | `intelligence` | `coding` | Medium | No direct tool-use benchmark; test on your tasks |

**Key insight:** For coding and math tasks, use the specific benchmarks. For agentic/tool-use tasks, benchmarks are less reliable—always validate on your actual use case.

See [BENCHMARK-LIMITATIONS.md](BENCHMARK-LIMITATIONS.md) for understanding what benchmarks can't tell you.

## Capability Fields (from models.dev)

These fields provide capability metadata beyond benchmark scores:

| Field | Type | Description |
|-------|------|-------------|
| `context_window` | BIGINT | Maximum context window (tokens) |
| `max_input_tokens` | BIGINT | Maximum input tokens allowed |
| `max_output_tokens` | BIGINT | Maximum output tokens allowed |
| `tool_call` | BOOLEAN | Supports function/tool calling |
| `structured_output` | BOOLEAN | Supports structured JSON output |
| `reasoning` | BOOLEAN | Supports chain-of-thought reasoning |
| `input_modalities` | VARCHAR | Input types (e.g., "text,image") |
| `output_modalities` | VARCHAR | Output types (e.g., "text") |
| `open_weights` | BOOLEAN | Model weights publicly available |
| `knowledge_cutoff` | VARCHAR | Training data cutoff date |
| `models_dev_matched` | BOOLEAN | Whether model was matched to models.dev |

> **Coverage:** ~53% of models have capability data from models.dev. Use `models_dev_matched = true` to filter for models with full capability information. Models without matches will have NULL values for capability fields.

### Handling Missing Capability Data

When filtering by capabilities (`tool_call`, `structured_output`, etc.), you're limiting results to the ~53% of models matched to models.dev. High-scoring models without capability data may be missed.

**To see what you might be missing:**

```bash
# Models with high intelligence but NO capability data
which-llm query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE intelligence >= 40 AND models_dev_matched = false
          ORDER BY intelligence DESC 
          LIMIT 10"
```

**Recommendation:** If a high-scoring model appears without capability data, check its documentation manually before ruling it out. The model may support the capability you need even though it's not in the database.

**To include models with unknown capability status:**

```bash
# Include models where tool_call is true OR unknown
which-llm query "SELECT name, intelligence, tool_call, price 
          FROM llms 
          WHERE intelligence >= 40 AND (tool_call = true OR tool_call IS NULL)
          ORDER BY intelligence DESC 
          LIMIT 10"
```

### Capability Field Limitations

- `tool_call` is a **boolean**, not a reliability score. It indicates whether the model supports tool calling, not how well it performs.
- `structured_output` indicates JSON mode support, but doesn't measure accuracy of structured outputs.
- For reliability assessment, test candidate models on your specific use case.

## Pricing Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `price` | DOUBLE | Blended cost per 1M tokens (3:1 input:output ratio) |
| `input_price` | DOUBLE | Cost per 1M input tokens |
| `output_price` | DOUBLE | Cost per 1M output tokens |

### Price Considerations

- `price` is blended assuming 3:1 input:output ratio
- For **output-heavy tasks** (generation), check `output_price` directly
- For **input-heavy tasks** (analysis of large docs), check `input_price`

```bash
# Compare input vs output pricing
which-llm query "SELECT name, input_price, output_price, price 
          FROM llms 
          WHERE price < 5 
          ORDER BY output_price"
```

## Performance Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `tps` | DOUBLE | Tokens per second (output generation speed) |
| `latency` | DOUBLE | Time to first token in seconds |

### Speed vs Latency

| Use Case | Priority |
|----------|----------|
| Batch processing | High TPS, latency doesn't matter |
| Real-time chat | Both TPS and low latency matter |
| Streaming UI | Low latency most important |

## Intelligence Index (v4.0)

The intelligence score is a weighted composite of 10 evaluations across 4 categories:

| Category (25% each) | What it measures |
|---------------------|------------------|
| **Agents** | Real-world task completion (GDPval-AA), agentic workflows (τ²-Bench) |
| **Coding** | Terminal tasks (Terminal-Bench Hard), scientific code (SciCode) |
| **General** | Long-context reasoning, knowledge/hallucination, instruction following |
| **Scientific Reasoning** | Graduate-level science (GPQA, HLE), physics (CritPt) |

**Scale Calibration (Jan 2026):** The v4.0 index was recalibrated to create headroom for future improvements. Current SOTA models (GPT-5.2, Claude Opus 4.5) score ~50. Legacy models from 2024 (e.g., Llama 3.1 405B) score ~15. This is intentional—the scale measures real-world economic work capability, not academic test performance.

**Confidence interval:** ±1% (so 45 vs 46 is effectively equivalent)

## Intelligence Score Interpretation

> **Note:** These tiers are calibrated for Intelligence Index v4.0 (Jan 2026). Current SOTA is ~50.

| Score | Tier | Capabilities |
|-------|------|--------------|
| **48+** | Frontier | Complex autonomous agents, research-level problems, PhD-level reasoning |
| **40-48** | Very Strong | Professional coding, complex analysis, multi-step agentic tasks |
| **30-40** | Strong | Most business tasks, standard coding, document Q&A, analysis |
| **20-30** | Capable | Simple coding, basic Q&A, content generation, straightforward tasks |
| **10-20** | Basic | Chat, simple queries, drafting text, basic assistance |
| **<10** | Limited | Very simple tasks only |

## Coding Score Interpretation

| Score | Capability |
|-------|------------|
| **45+** | Exceptional - complex systems, debugging, architecture |
| **35-45** | Strong - professional software development |
| **25-35** | Capable - standard coding tasks, scripts |
| **<25** | Limited - simple code only |

## Math Score Interpretation

| Score | Capability |
|-------|------------|
| **70+** | Competition-level (AIME, IMO problems) |
| **50-70** | University-level mathematics |
| **30-50** | High school / AP level |
| **<30** | Basic arithmetic, simple algebra |

## Data Freshness

- Data is cached locally by the `which-llm` CLI
- Use `which-llm llms --refresh` to fetch fresh data
- Use `which-llm cache status` to check cache age
