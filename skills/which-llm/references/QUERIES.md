# Common Query Patterns

Ready-to-use `which-llm query` commands for common model selection scenarios.

## Weighted Scoring

Use weighted scoring to rank models based on priorities. The formula:
```
Score = (intelligence × quality_weight) + (100/price × cost_weight) + (tps/10 × speed_weight)
```

### Balanced Scoring (0.4 quality, 0.4 cost, 0.2 speed)
```bash
which-llm query "SELECT name, intelligence, price, tps,
          ROUND((intelligence * 0.4) + (100/price * 0.4) + (tps/10 * 0.2), 1) as score
          FROM llms 
          WHERE intelligence >= 35 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"
```

### Quality Priority (0.7 quality, 0.2 cost, 0.1 speed)
```bash
which-llm query "SELECT name, intelligence, price, tps,
          ROUND((intelligence * 0.7) + (100/price * 0.2) + (tps/10 * 0.1), 1) as score
          FROM llms 
          WHERE intelligence >= 35 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"
```

### Cost Priority (0.2 quality, 0.7 cost, 0.1 speed)
```bash
which-llm query "SELECT name, intelligence, price, tps,
          ROUND((intelligence * 0.2) + (100/price * 0.7) + (tps/10 * 0.1), 1) as score
          FROM llms 
          WHERE intelligence >= 35 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"
```

### Speed Priority (0.2 quality, 0.2 cost, 0.6 speed)
```bash
which-llm query "SELECT name, intelligence, price, tps,
          ROUND((intelligence * 0.2) + (100/price * 0.2) + (tps/10 * 0.6), 1) as score
          FROM llms 
          WHERE intelligence >= 35 AND price > 0 AND tps > 0
          ORDER BY score DESC 
          LIMIT 10"
```

### Weighted Scoring with Capabilities
```bash
# Tool-using tasks with balanced scoring
which-llm query "SELECT name, intelligence, tool_call, price, tps,
          ROUND((intelligence * 0.4) + (100/price * 0.4) + (tps/10 * 0.2), 1) as score
          FROM llms 
          WHERE intelligence >= 35 AND tool_call = true AND price > 0
          ORDER BY score DESC 
          LIMIT 10"

# Agentic tasks with quality priority
which-llm query "SELECT name, intelligence, tool_call, context_window, price,
          ROUND((intelligence * 0.7) + (100/price * 0.2) + (COALESCE(tps,0)/10 * 0.1), 1) as score
          FROM llms 
          WHERE intelligence >= 40 AND tool_call = true AND context_window >= 100000 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"
```

## By Use Case

### Smartest Models (Regardless of Cost)
```bash
which-llm query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

### Cheap Models for High-Volume Tasks
```bash
which-llm query "SELECT name, creator, intelligence, price, tps 
          FROM llms 
          WHERE intelligence >= 20 
          ORDER BY price 
          LIMIT 10"
```

### Fast Models for Real-Time Chat
```bash
which-llm query "SELECT name, creator, intelligence, tps, latency, price 
          FROM llms 
          WHERE tps > 100 AND latency < 0.5 
          ORDER BY tps DESC"
```

### Best Coding Models Under Budget
```bash
which-llm query "SELECT name, creator, coding, intelligence, price 
          FROM llms 
          WHERE coding IS NOT NULL AND price < 10 
          ORDER BY coding DESC"
```

### Agentic/Orchestrator Models
```bash
which-llm query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE intelligence >= 48 AND coding >= 42 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

### Budget Worker Models (for pipelines)
```bash
which-llm query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE intelligence >= 20 
          ORDER BY price 
          LIMIT 10"
```

## By Capability

### Models with Tool Calling
```bash
which-llm query "SELECT name, creator, intelligence, tool_call, price 
          FROM llms 
          WHERE tool_call = true AND intelligence >= 35 
          ORDER BY price 
          LIMIT 10"
```

### Models with Structured Output (JSON Mode)
```bash
which-llm query "SELECT name, creator, intelligence, structured_output, price 
          FROM llms 
          WHERE structured_output = true 
          ORDER BY price 
          LIMIT 10"
```

### Large Context Window Models
```bash
which-llm query "SELECT name, creator, intelligence, context_window, price 
          FROM llms 
          WHERE context_window >= 128000 
          ORDER BY context_window DESC"
```

### Reasoning Models (Chain-of-Thought)
```bash
which-llm query "SELECT name, creator, intelligence, reasoning, price 
          FROM llms 
          WHERE reasoning = true 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

### Multimodal Models (Vision)
```bash
which-llm query "SELECT name, creator, intelligence, input_modalities, price 
          FROM llms 
          WHERE input_modalities LIKE '%image%' 
          ORDER BY intelligence DESC"
```

### Open Weight Models (Self-Hosting)
```bash
which-llm query "SELECT name, creator, intelligence, coding, open_weights 
          FROM llms 
          WHERE open_weights = true 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

### Models with Full Capability Data
```bash
which-llm query "SELECT name, creator, intelligence, context_window, tool_call, structured_output 
          FROM llms 
          WHERE models_dev_matched = true 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

## By Constraint

### With Budget Constraint
```bash
which-llm query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE price < 5 
          ORDER BY intelligence DESC"
```

### With Speed Constraint
```bash
which-llm query "SELECT name, creator, intelligence, tps, price 
          FROM llms 
          WHERE tps > 150 
          ORDER BY intelligence DESC"
```

### With Latency Constraint
```bash
which-llm query "SELECT name, creator, intelligence, latency, price 
          FROM llms 
          WHERE latency < 0.3 
          ORDER BY intelligence DESC"
```

### With Context Window Constraint
```bash
which-llm query "SELECT name, creator, intelligence, context_window, price 
          FROM llms 
          WHERE context_window >= 200000 
          ORDER BY price"
```

## By Creator

### Compare Specific Creators
```bash
which-llm query "SELECT name, creator, intelligence, coding, price, tps 
          FROM llms 
          WHERE creator IN ('OpenAI', 'Anthropic', 'Google') 
          ORDER BY intelligence DESC"
```

### Single Creator Deep Dive
```bash
which-llm query "SELECT name, intelligence, coding, math, price, tps 
          FROM llms 
          WHERE creator = 'OpenAI' 
          ORDER BY intelligence DESC"
```

## Cascade Selection

### Find Primary + Fallback Pair (General)
```bash
# Primary: Cheap model meeting minimum requirements
which-llm query "SELECT name, intelligence, price 
          FROM llms 
          WHERE intelligence >= 35 AND price < 2 
          ORDER BY price 
          LIMIT 3"

# Fallback: High-capability model
which-llm query "SELECT name, intelligence, price 
          FROM llms 
          WHERE intelligence >= 48 
          ORDER BY price 
          LIMIT 3"
```

### Find Primary + Fallback Pair (Tool-Using)
```bash
# Primary: Capable + cheap + tool calling
which-llm query "SELECT name, intelligence, tool_call, price 
          FROM llms 
          WHERE intelligence >= 35 AND tool_call = true AND price < 3 
          ORDER BY price 
          LIMIT 3"

# Fallback: Frontier model with tool calling
which-llm query "SELECT name, intelligence, tool_call, price 
          FROM llms 
          WHERE intelligence >= 48 AND tool_call = true 
          ORDER BY intelligence DESC 
          LIMIT 3"
```

### Find Primary + Fallback Pair (Coding)
```bash
# Primary: Good coding, cheap
which-llm query "SELECT name, coding, price 
          FROM llms 
          WHERE coding >= 30 AND price < 2 
          ORDER BY coding DESC 
          LIMIT 3"

# Fallback: Best coding model
which-llm query "SELECT name, coding, price 
          FROM llms 
          WHERE coding >= 42 
          ORDER BY coding DESC 
          LIMIT 3"
```

### Find Primary + Fallback Pair (Long Context)
```bash
# Primary: Large context, affordable
which-llm query "SELECT name, intelligence, context_window, price 
          FROM llms 
          WHERE context_window >= 128000 AND price < 5 
          ORDER BY price 
          LIMIT 3"

# Fallback: Largest context, high capability
which-llm query "SELECT name, intelligence, context_window, price 
          FROM llms 
          WHERE context_window >= 200000 AND intelligence >= 40 
          ORDER BY context_window DESC 
          LIMIT 3"
```

### Cascade Cost Savings Estimation

Calculate potential savings for a cascade pair:

```bash
# Get prices for primary and fallback candidates
which-llm query "SELECT 
          'Primary' as role, name, intelligence, price
          FROM llms 
          WHERE intelligence >= 35 AND price < 2 
          ORDER BY price 
          LIMIT 1
          
          UNION ALL
          
          SELECT 
          'Fallback' as role, name, intelligence, price
          FROM llms 
          WHERE intelligence >= 48 
          ORDER BY price 
          LIMIT 1"
```

Then calculate savings manually:
```
Cascade cost = (0.70 × primary_price) + (0.30 × fallback_price)
Savings = (1 - cascade_cost / fallback_price) × 100
```

**Example with $0.50 primary and $5.00 fallback:**
```
Cascade = (0.70 × $0.50) + (0.30 × $5.00) = $1.85/M
Savings = (1 - 1.85/5.00) × 100 = 63%
```

### Quick Savings Reference Table

Use this to estimate savings based on your cascade pair's price ratio:

| Price Ratio | 20% Escalation | 30% Escalation | 50% Escalation |
|-------------|----------------|----------------|----------------|
| 3x | 53% savings | 40% savings | 13% savings |
| 5x | 68% savings | 58% savings | 40% savings |
| 10x | 82% savings | 73% savings | 55% savings |
| 20x | 91% savings | 86% savings | 77% savings |

See [CASCADE-PATTERNS.md](CASCADE-PATTERNS.md) for detailed cost estimation formulas.

## Skill-Type Specific Queries

### Transformational Tasks (summarize, extract, reformat)
```bash
which-llm query "SELECT name, creator, intelligence, price, tps 
          FROM llms 
          WHERE intelligence >= 20 
          ORDER BY price 
          LIMIT 10"
```

### Analytical Tasks (compare, analyze, justify)
```bash
which-llm query "SELECT name, creator, intelligence, context_window, price 
          FROM llms 
          WHERE intelligence >= 38 
          ORDER BY price 
          LIMIT 10"
```

### Tool-Using Tasks (API calls, code execution)
```bash
which-llm query "SELECT name, creator, intelligence, coding, tool_call, price 
          FROM llms 
          WHERE intelligence >= 35 AND coding >= 35 AND tool_call = true 
          ORDER BY price 
          LIMIT 10"
```

### Agentic Tasks (planning, orchestration)
```bash
which-llm query "SELECT name, creator, intelligence, coding, tool_call, context_window, price 
          FROM llms 
          WHERE intelligence >= 48 AND coding >= 42 
            AND tool_call = true 
          ORDER BY price 
          LIMIT 10"
```

## Aggregations

### Average Intelligence by Creator
```bash
which-llm query "SELECT creator, COUNT(*) as models, 
                 ROUND(AVG(intelligence), 1) as avg_intel,
                 ROUND(MIN(price), 2) as min_price
          FROM llms 
          WHERE intelligence IS NOT NULL 
          GROUP BY creator 
          ORDER BY avg_intel DESC"
```

### Price Tiers
```bash
which-llm query "SELECT 
            CASE 
              WHEN price < 1 THEN 'Budget (<\$1)'
              WHEN price < 5 THEN 'Mid (\$1-5)'
              WHEN price < 20 THEN 'Premium (\$5-20)'
              ELSE 'Enterprise (\$20+)'
            END as tier,
            COUNT(*) as models,
            ROUND(AVG(intelligence), 1) as avg_intel
          FROM llms
          WHERE price IS NOT NULL
          GROUP BY tier
          ORDER BY avg_intel DESC"
```

### Capability Coverage
```bash
which-llm query "SELECT 
            COUNT(*) as total_models,
            SUM(CASE WHEN models_dev_matched THEN 1 ELSE 0 END) as with_capabilities,
            SUM(CASE WHEN tool_call THEN 1 ELSE 0 END) as with_tool_call,
            SUM(CASE WHEN structured_output THEN 1 ELSE 0 END) as with_structured_output,
            SUM(CASE WHEN reasoning THEN 1 ELSE 0 END) as with_reasoning
          FROM llms"
```

## Utilities

### List All Available Columns
```bash
which-llm query --tables
```

### Refresh Data
```bash
which-llm llms --refresh
```

### Check Cache Status
```bash
which-llm cache status
```

### Export to JSON (for further processing)
```bash
which-llm llms --json > models.json
```
