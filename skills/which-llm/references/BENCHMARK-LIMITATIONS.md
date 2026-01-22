# Benchmark Limitations

Understanding what benchmarks can and cannot tell you about model performance.

## Key Limitations

### 1. Aggregate Scores Hide Specialization

The `intelligence` score is a composite of 10+ evaluations across coding, reasoning, agents, and knowledge. A model scoring 45 could be:
- Excellent at coding (55) but weak at reasoning (35)
- Mediocre at everything (45 across the board)

**Implication:** For specialized tasks, check the specific benchmark (e.g., `coding`, `math`) rather than relying on `intelligence` alone.

### 2. Benchmark Contamination

Many public benchmarks have been contaminated—models may have been trained on test data, inflating scores.

| Benchmark | Contamination Risk | Notes |
|-----------|-------------------|-------|
| MMLU | High | Widely used in training data |
| HumanEval | Medium-High | Commonly seen in code training sets |
| GSM8K | Medium | Popular math benchmark |
| GPQA | Low | Graduate-level, less exposed |
| HLE (Humanity's Last Exam) | Low | Recent, expert-level |
| LiveCodeBench | Low | Continuously updated with new problems |
| SWE-bench | Low | Real GitHub issues, harder to memorize |

**Implication:** Prefer benchmarks with low contamination risk for reliable comparisons. Task-specific benchmarks (SWE-bench, LiveCodeBench) are generally more trustworthy than general knowledge tests.

### 3. Benchmarks ≠ Real-World Performance

Benchmarks measure performance on standardized tests. Your actual task may differ in:
- **Domain specificity**: Medical, legal, or niche technical domains
- **Input format**: Messy real-world data vs. clean benchmark prompts
- **Output requirements**: Specific formats, length constraints, tone
- **Edge cases**: Unusual inputs that benchmarks don't cover

**Research finding:** General benchmark scores have only moderate correlation (~0.6-0.7) with success on specific production tasks.

**Implication:** Always validate on your specific use case. A model that scores lower on benchmarks may outperform on your particular task.

### 4. Boolean Capabilities Lack Nuance

Fields like `tool_call` and `structured_output` are boolean (true/false). They indicate:
- ✅ The model **supports** the capability
- ❌ The model's **reliability** at that capability

A model with `tool_call = true` might:
- Call tools correctly 95% of the time (excellent)
- Call tools correctly 70% of the time (problematic for production)

**Implication:** Test tool calling and structured output reliability on your actual use case. Don't assume boolean support equals reliable performance.

### 5. Speed Metrics Vary by Provider

`tps` (tokens per second) and `latency` measurements:
- Come from specific API endpoints at specific times
- May vary significantly based on load, time of day, and region
- Don't account for rate limits or throttling

**Implication:** Benchmark speed metrics are indicative, not guaranteed. Test actual performance under your expected load patterns.

### 6. Price Doesn't Include Hidden Costs

The `price` metric covers token costs but not:
- **Rate limit overhead**: Retry costs when hitting limits
- **Prompt engineering**: Longer prompts may be needed for some models
- **Error handling**: Failed requests that still cost tokens
- **Caching efficiency**: Some models work better with caching

**Implication:** Total cost of ownership may differ significantly from raw token pricing.

## When Benchmarks ARE Reliable

Benchmarks work well for:

| Use Case | Why |
|----------|-----|
| **Rough capability tiers** | Distinguishing a 45 from a 25 is meaningful |
| **Price/performance ratios** | Comparing similar-capability models |
| **Minimum thresholds** | Filtering out clearly inadequate models |
| **Trend analysis** | Tracking model improvements over time |

## When to Distrust Benchmarks

Be skeptical when:

| Situation | Risk |
|-----------|------|
| Scores are within ±2 points | Effectively equivalent given confidence intervals |
| New model claims dramatic improvement | May be benchmark-optimized |
| Your task is highly specialized | General benchmarks may not apply |
| You need reliability, not just capability | Benchmarks measure best-case, not consistency |

## Recommended Approach

1. **Use benchmarks for shortlisting**: Filter to 3-5 candidates
2. **Check task-specific benchmarks**: Use `coding` for code, `math` for math, etc.
3. **Create a golden set**: 10-20 representative examples from your actual use case
4. **Run your own eval**: Test candidates on your golden set
5. **Measure what matters**: Accuracy, latency, cost, and consistency for YOUR task

See [SPECIALIZATION.md](SPECIALIZATION.md) for guidance on which models excel at specific task types.
