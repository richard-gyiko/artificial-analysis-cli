# Multi-Model Architecture Guide

When and how to use different models for different roles in a system.

## When to Use Multi-Model

Consider multi-model architecture when:

- Task has both "thinking" and "doing" components
- High volume workload where cost matters
- Pipeline with distinct stages (extract → process → format)
- User explicitly mentions agents or orchestration

## The Pattern

> **Note:** Thresholds calibrated for Intelligence Index v4.0 (Jan 2026), where current SOTA is ~50.

```
┌─────────────────────────────────────────────────────┐
│                    ORCHESTRATOR                      │
│              (intelligence 48+, coding 42+)          │
│              tool_call, reasoning, large context     │
│         Planning, decomposition, decisions           │
└─────────────────────┬───────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        ▼             ▼             ▼
   ┌─────────┐  ┌─────────┐  ┌─────────┐
   │ Extract │  │ Execute │  │ Format  │
   │  (20+)  │  │  (35+)  │  │  (15+)  │
   │         │  │tool_call│  │         │
   └─────────┘  └─────────┘  └─────────┘
     Cheap        Medium       Cheapest
```

## Role Requirements

| Role | Min Intelligence | Min Coding | Capabilities | Priority |
|------|------------------|------------|--------------|----------|
| **Orchestrator** | 48 | 42 | `tool_call`, `reasoning` | Capability over cost |
| **Reasoning Worker** | 38 | - | - | Balance |
| **Tool Worker** | 35 | 35 | `tool_call = true` | Reliability |
| **Extraction Worker** | 20 | - | - | Cost |
| **Formatting Worker** | 15 | - | `structured_output` helpful | Cost |

## Finding Models by Role

### Orchestrator (Don't Skimp)
```bash
which-llm query "SELECT name, creator, intelligence, coding, tool_call, context_window, price 
          FROM llms 
          WHERE intelligence >= 48 AND coding >= 42 AND tool_call = true 
          ORDER BY intelligence DESC 
          LIMIT 5"
```

### Reasoning Workers
```bash
which-llm query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE intelligence >= 38 
          ORDER BY price 
          LIMIT 5"
```

### Tool/Code Workers
```bash
which-llm query "SELECT name, creator, intelligence, coding, tool_call, price 
          FROM llms 
          WHERE intelligence >= 35 AND coding >= 35 AND tool_call = true 
          ORDER BY price 
          LIMIT 5"
```

### Cheap Extraction/Formatting Workers
```bash
which-llm query "SELECT name, creator, intelligence, price, tps 
          FROM llms 
          WHERE intelligence >= 15 
          ORDER BY price 
          LIMIT 5"
```

## Cost Optimization Example

**Scenario:** Document processing pipeline
- Extract entities from 10,000 documents
- Analyze extracted data for insights
- Generate summary report

**Single Model Approach:**
- Use GPT-4 class model for everything
- Cost: ~$50/M tokens × high volume = expensive

**Multi-Model Approach:**
| Stage | Model Tier | Est. Price | Volume |
|-------|------------|------------|--------|
| Extract | Budget ($0.50/M) | Low | 10,000 docs |
| Analyze | Mid ($5/M) | Medium | Aggregated |
| Report | Mid ($5/M) | Medium | 1 report |

**Savings:** Often 5-10x cheaper for high-volume pipelines

## Architecture Patterns

### Router Pattern
Use a cheap/fast model to classify incoming requests, route to appropriate specialist:

```
Input → Router (cheap, fast) → Specialist A (expensive)
                            → Specialist B (medium)
                            → Specialist C (cheap)
```

**Router model requirements:**
- Very fast (high TPS, low latency)
- Cheap (handles every request)
- Decent intelligence for classification (~25+)

```bash
# Find router candidates
which-llm query "SELECT name, intelligence, tps, latency, price 
          FROM llms 
          WHERE tps > 150 AND latency < 0.3 AND price < 1 
          ORDER BY price"
```

### Cascade Pattern (FrugalGPT-style)
Start with cheap model, escalate to expensive only when needed:

```
Input → Primary (cheap) → [confident?] → Output
                       → [uncertain?] → Fallback (expensive) → Output
```

**When to escalate:**
- Model returns low-confidence response
- Output fails validation/verification
- Task complexity exceeds primary's capability

**Cost savings:** Research shows 50-70% cost reduction vs always using the best model.

**Example cascade pair:**
| Role | Requirements | Query |
|------|--------------|-------|
| Primary | intelligence ≥ 35, price < $2/M | `WHERE intelligence >= 35 AND price < 2` |
| Fallback | intelligence ≥ 48 | `WHERE intelligence >= 48` |

See [CASCADE-PATTERNS.md](CASCADE-PATTERNS.md) for detailed cascade implementation guidance.

### Verifier Pattern
Use a second model to check the first model's work:

```
Input → Generator (medium) → Output → Verifier (cheap) → [OK?] → Final
                                                      → [Bad?] → Retry
```

**Verifier model requirements:**
- Can evaluate quality of generated output
- Cheaper than generator (runs on every output)
- Fast (adds to total latency)

### Ensemble Pattern
Query multiple models, use voting or judge to pick best:

```
Input → Model A ─┐
      → Model B ─┼→ Judge/Vote → Best Output
      → Model C ─┘
```

**Use when:**
- Extremely high stakes (worth extra cost)
- Need to compare different model "opinions"
- Building evaluation datasets

**Caution:** Increases both cost and latency significantly.

## Capability-Based Role Assignment

When assigning models to roles, match capabilities to requirements:

| Role Needs | Required Capability | Query Filter |
|------------|---------------------|--------------|
| Call external APIs | Tool calling | `tool_call = true` |
| Return structured data | JSON mode | `structured_output = true` |
| Process long documents | Large context | `context_window >= 128000` |
| Complex reasoning | Chain-of-thought | `reasoning = true` |
| Self-hosted/private | Open weights | `open_weights = true` |

## Key Insight

> Intelligence scores matter most for **orchestrator/planning** roles.
> They matter least for **well-scoped execution** roles.

If you decompose your system into narrow skills:
- Each skill requires far less intelligence
- You can use cheaper models for most work
- Failures become local and diagnosable
- Total cost drops significantly
