# Cascade Patterns

When and how to use model cascades for cost optimization.

## What is a Cascade?

A cascade uses a cheap model first, escalating to an expensive model only when needed. This approach, popularized by research like FrugalGPT, can reduce costs by 50-70% compared to always using the best model.

```
User Request
    │
    ▼
┌─────────────┐
│   Primary   │ (cheap, fast)
│   Model     │
└─────┬───────┘
      │
      ├── Confident/Valid ──► Return Response
      │
      ▼
┌─────────────┐
│  Fallback   │ (expensive, capable)
│   Model     │
└─────────────┘
      │
      ▼
  Return Response
```

## When to Use Cascades

**Good candidates for cascading:**
- High-volume workloads where most requests are straightforward
- Variable complexity tasks (mix of easy and hard)
- Cost-sensitive applications
- When you can detect or verify failures

**Not ideal for cascading:**
- Low volume (complexity not worth the overhead)
- Uniformly difficult tasks (everything escalates anyway)
- Latency-critical applications (cascade adds roundtrip)
- No reliable way to detect primary model failures

## Escalation Triggers

| Trigger | How to Detect | Example |
|---------|---------------|---------|
| Low confidence | Model outputs uncertainty markers | "I'm not sure", hedging language |
| Validation failure | Output doesn't match expected format | JSON parse error, missing fields |
| Verification failure | Verifier model flags issues | Fact-check fails, logic errors |
| Complexity signal | Input characteristics | Long input, multiple sub-questions |
| Explicit failure | Model refuses or errors | "I can't help with that" |

## Finding Cascade Pairs

When selecting cascade pairs, use **relaxed thresholds** for the primary model and **strict thresholds** for the fallback:

| Role | Threshold Strategy | Example (Agentic task) |
|------|-------------------|------------------------|
| Primary | ~20% below skill type minimum | intelligence ≥ 40 (vs 48) |
| Fallback | Skill type minimum or higher | intelligence ≥ 48 |

> **Note:** Intelligence scores have a ±1% confidence interval. Don't over-optimize between adjacent scores (e.g., 47 vs 48 are effectively equivalent).

### Score-Based Cascade Selection

Use weighted scoring to find the best primary/fallback pair based on priorities:

```bash
# Primary: Cost-optimized (relaxed threshold, cost priority)
which-llm query "SELECT name, intelligence, price, tps,
          ROUND((intelligence * 0.2) + (100/price * 0.7) + (tps/10 * 0.1), 1) as score
          FROM llms 
          WHERE intelligence >= 35 AND price > 0 AND price < 3
          ORDER BY score DESC 
          LIMIT 5"

# Fallback: Quality-optimized (strict threshold, quality priority)
which-llm query "SELECT name, intelligence, price, tps,
          ROUND((intelligence * 0.7) + (100/price * 0.2) + (tps/10 * 0.1), 1) as score
          FROM llms 
          WHERE intelligence >= 48 AND price > 0
          ORDER BY score DESC 
          LIMIT 5"
```

### General Tasks

```bash
# Primary: Budget model with decent intelligence
which-llm query "SELECT name, intelligence, price 
          FROM llms 
          WHERE intelligence >= 30 AND price < 1 
          ORDER BY intelligence DESC 
          LIMIT 5"

# Fallback: Frontier model
which-llm query "SELECT name, intelligence, price 
          FROM llms 
          WHERE intelligence >= 48 
          ORDER BY price 
          LIMIT 5"
```

### Tool-Using Tasks

```bash
# Primary: Cheap with tool calling
which-llm query "SELECT name, intelligence, tool_call, price 
          FROM llms 
          WHERE tool_call = true AND intelligence >= 35 AND price < 3 
          ORDER BY price 
          LIMIT 5"

# Fallback: Best tool-calling model
which-llm query "SELECT name, intelligence, tool_call, price 
          FROM llms 
          WHERE tool_call = true AND intelligence >= 45 
          ORDER BY intelligence DESC 
          LIMIT 5"
```

### Coding Tasks

```bash
# Primary: Good coding, cheap
which-llm query "SELECT name, coding, price 
          FROM llms 
          WHERE coding >= 30 AND price < 2 
          ORDER BY coding DESC 
          LIMIT 5"

# Fallback: Best coding model
which-llm query "SELECT name, coding, price 
          FROM llms 
          WHERE coding >= 42 
          ORDER BY coding DESC 
          LIMIT 5"
```

### Structured Output Tasks

```bash
# Primary: Cheap with structured output support
which-llm query "SELECT name, intelligence, structured_output, price 
          FROM llms 
          WHERE structured_output = true AND intelligence >= 25 AND price < 2 
          ORDER BY price 
          LIMIT 5"

# Fallback: High capability with structured output
which-llm query "SELECT name, intelligence, structured_output, price 
          FROM llms 
          WHERE structured_output = true AND intelligence >= 40 
          ORDER BY intelligence DESC 
          LIMIT 5"
```

### Long Context Tasks

```bash
# Primary: Large context, affordable
which-llm query "SELECT name, intelligence, context_window, price 
          FROM llms 
          WHERE context_window >= 128000 AND price < 5 
          ORDER BY price 
          LIMIT 5"

# Fallback: Largest context, high capability
which-llm query "SELECT name, intelligence, context_window, price 
          FROM llms 
          WHERE context_window >= 200000 AND intelligence >= 40 
          ORDER BY context_window DESC 
          LIMIT 5"
```

## Cost Savings Estimation

### Quick Formula

```
Cascade Cost = (1 - escalation_rate) × primary_price + escalation_rate × fallback_price
Savings % = (1 - cascade_cost / fallback_price) × 100
```

### Simplified Calculation

For quick estimates, use the **price ratio** (fallback_price / primary_price):

| Price Ratio | 20% Escalation | 30% Escalation | 50% Escalation |
|-------------|----------------|----------------|----------------|
| 3x (e.g. $1 vs $3) | 53% savings | 40% savings | 13% savings |
| 5x (e.g. $1 vs $5) | 68% savings | 58% savings | 40% savings |
| 10x (e.g. $0.5 vs $5) | 82% savings | 73% savings | 55% savings |
| 20x (e.g. $0.25 vs $5) | 91% savings | 86% savings | 77% savings |

**Rule of thumb:** Cascades need at least 3x price ratio and <50% escalation to be worthwhile.

### Detailed Formula

```
Average Cost = (primary_rate × primary_price) + (fallback_rate × fallback_price)
```

Where `primary_rate + fallback_rate = 1.0`

### Example Calculation

**Models:**
- Primary: $0.50/M tokens
- Fallback: $5.00/M tokens
- Price ratio: 10x

**Scenario: 70% handled by primary, 30% escalate**
```
Cascade avg = (0.70 × $0.50) + (0.30 × $5.00)
            = $0.35 + $1.50
            = $1.85/M tokens

vs Always Fallback = $5.00/M tokens

Savings = (1 - 1.85/5.00) × 100 = 63%
```

### Escalation Rate Impact

| Escalation Rate | Primary ($0.50) | Fallback ($5.00) | Avg Cost | Savings |
|-----------------|-----------------|------------------|----------|---------|
| 10% | 90% | 10% | $0.95/M | 81% |
| 20% | 80% | 20% | $1.40/M | 72% |
| 30% | 70% | 30% | $1.85/M | 63% |
| 50% | 50% | 50% | $2.75/M | 45% |
| 70% | 30% | 70% | $3.65/M | 27% |

**Key insight:** Cascades are most effective when escalation rate stays below 40%.

### Break-Even Analysis

When does a cascade stop being worth it?

```
Break-even escalation rate = 1 - (overhead / (fallback_price - primary_price))
```

Where `overhead` = implementation/monitoring cost per request

**Example:** If overhead is $0.10/M and prices are $0.50 (primary) vs $5.00 (fallback):
```
Break-even = 1 - (0.10 / 4.50) = 97.8%
```

In this case, cascade is worthwhile even at very high escalation rates because the price gap is large.

### Estimating Your Escalation Rate

Before committing to a cascade, estimate your escalation rate:

| Method | How | Accuracy |
|--------|-----|----------|
| **Sample test** | Run 100 requests through primary, manually check quality | High |
| **Benchmark proxy** | Compare model scores (larger gap = lower escalation) | Medium |
| **Historical data** | If migrating from single model, check failure rate | High |
| **Industry baseline** | Start with 30% assumption, measure and adjust | Low |

**Recommended approach:**
1. Start with 30% escalation assumption
2. Implement logging to track actual escalation rate
3. Adjust thresholds to optimize cost/quality tradeoff

## Implementation Patterns

### Pattern 1: Validation-Based Escalation

Escalate when output fails validation:

```python
# Pseudocode
response = primary_model.generate(prompt)

if validate_output(response):
    return response
else:
    return fallback_model.generate(prompt)
```

**Best for:** Structured output, JSON responses, specific formats

### Pattern 2: Confidence-Based Escalation

Escalate when model expresses uncertainty:

```python
# Pseudocode
response = primary_model.generate(prompt)

if contains_uncertainty(response):  # "I'm not sure", "might be", etc.
    return fallback_model.generate(prompt)
else:
    return response
```

**Best for:** Factual questions, knowledge retrieval

### Pattern 3: Complexity-Based Routing

Route based on input characteristics before calling any model:

```python
# Pseudocode
complexity = estimate_complexity(prompt)

if complexity > THRESHOLD:
    return fallback_model.generate(prompt)
else:
    return primary_model.generate(prompt)
```

**Best for:** When you can reliably estimate task difficulty upfront

### Pattern 4: Verifier-Based Escalation

Use a third model to verify output quality:

```python
# Pseudocode
response = primary_model.generate(prompt)
verification = verifier_model.check(prompt, response)

if verification.passed:
    return response
else:
    return fallback_model.generate(prompt)
```

**Best for:** High-stakes applications where verification is cheaper than fallback

## Implementation Tips

1. **Start simple**: Begin with validation failures only, add more triggers later

2. **Log everything**: Track which requests escalate and why
   - Escalation rate over time
   - Reasons for escalation
   - Quality difference between primary and fallback

3. **Tune thresholds**: Adjust based on quality/cost tradeoff
   - Too aggressive → poor quality
   - Too conservative → no cost savings

4. **Consider latency**: Cascade adds latency for escalated requests
   - Primary attempt time + Fallback time
   - May need timeout on primary

5. **Monitor quality**: Ensure primary model quality is acceptable
   - Sample and review primary-only responses
   - Compare against fallback on same prompts

6. **Handle edge cases**: 
   - What if both models fail?
   - Rate limiting on fallback?
   - Graceful degradation?

## When NOT to Cascade

- **Latency-critical**: User-facing chat with strict SLAs
- **Uniformly hard tasks**: Research, complex reasoning (everything escalates)
- **Low volume**: Implementation overhead not worth it
- **No validation possible**: Can't tell if primary succeeded
- **Fallback not much better**: Small capability gap doesn't justify complexity
