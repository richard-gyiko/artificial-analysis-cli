# Common Workflows

End-to-end examples showing how to use the skill for real-world model selection scenarios.

## Workflow 1: RAG Chatbot

**User request:** "I'm building a RAG chatbot for customer support. It needs to handle tool calls for retrieval and be cost-effective since it's user-facing with high volume."

### Step 1: Classify the Task

```
External actions required? YES (retrieval tool calls)
Multi-step autonomous planning? NO (single retrieval + response)
→ Task Type: Tool-using
```

### Step 2: Identify Requirements

- **Skill type:** Tool-using (intelligence >= 35, coding >= 35)
- **Required capability:** `tool_call = true`
- **Context:** 32k+ for retrieved chunks
- **Priority:** Cost (high volume, budget-sensitive)

### Step 3: Query for Candidates

```bash
# Cost-priority weighted scoring for tool-using task
which-llm query "SELECT name, intelligence, tool_call, context_window, price, tps,
          ROUND((intelligence * 0.2) + (100/price * 0.7) + (tps/10 * 0.1), 1) as score
          FROM llms 
          WHERE intelligence >= 35 AND tool_call = true 
            AND context_window >= 32000 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"
```

### Step 4: Select Cascade Pair

```bash
# Primary: Cheapest meeting requirements
which-llm query "SELECT name, intelligence, tool_call, price 
          FROM llms 
          WHERE intelligence >= 35 AND tool_call = true AND price < 1 
          ORDER BY price 
          LIMIT 3"

# Fallback: Higher capability for complex queries
which-llm query "SELECT name, intelligence, tool_call, price 
          FROM llms 
          WHERE intelligence >= 45 AND tool_call = true 
          ORDER BY price 
          LIMIT 3"
```

### Step 5: Format Recommendation

```markdown
## Task Classification
- **Skill Type:** Tool-using
- **Key Constraints:** tool_call required, context >= 32k
- **Priority:** Cost (high volume)

## Recommendations

### Primary: GPT-4.1-mini ($0.40/M) - Score: 38.2
- Intelligence: 38, tool_call: true, context: 128k
- Why: Best cost/capability ratio for tool-using tasks

### Fallback: Claude Sonnet 4 ($3.00/M) - Score: 42.1
- Intelligence: 48, tool_call: true, context: 200k
- Use if: Complex multi-turn conversations or primary fails

## Cost Estimate
- **Primary only:** $0.40/M tokens
- **Fallback only:** $3.00/M tokens (7.5x primary)
- **Cascade (30% fallback):** $1.18/M tokens
- **Savings vs always fallback:** 61%
```

---

## Workflow 2: Code Review Agent

**User request:** "I need a model to review pull requests in our CI pipeline. It should understand code well and provide actionable feedback. We can afford some cost for quality."

### Step 1: Classify the Task

```
External actions required? NO (just reading code, outputting review)
Judgment/analysis required? YES (code quality assessment)
→ Task Type: Analytical
```

### Step 2: Identify Requirements

- **Skill type:** Analytical (intelligence >= 38)
- **Additional:** High `coding` score for code understanding
- **Context:** Large enough for full PR diffs (64k+)
- **Priority:** Quality (accuracy matters more than cost)

### Step 3: Query for Candidates

```bash
# Quality-priority scoring, filtered for coding capability
which-llm query "SELECT name, intelligence, coding, context_window, price,
          ROUND((intelligence * 0.5) + (coding * 0.3) + (100/price * 0.2), 1) as score
          FROM llms 
          WHERE intelligence >= 38 AND coding >= 40 
            AND context_window >= 64000 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"
```

### Step 4: Format Recommendation

```markdown
## Task Classification
- **Skill Type:** Analytical (code review)
- **Key Constraints:** coding >= 40, context >= 64k
- **Priority:** Quality

## Recommendations

### Primary: Claude Sonnet 4 ($3.00/M) - Score: 51.2
- Intelligence: 48, Coding: 47, Context: 200k
- Why: Best code understanding, produces clean feedback

### Alternative: DeepSeek-V3 ($0.50/M) - Score: 45.8
- Intelligence: 44, Coding: 48, Context: 128k
- Consider if: Budget is tight; excellent code understanding at lower cost
```

---

## Workflow 3: Agentic Research Assistant

**User request:** "I'm building an AI research assistant that can browse the web, take notes, synthesize findings, and write reports autonomously."

### Step 1: Classify the Task

```
External actions required? YES (web browsing, note-taking tools)
Multi-step autonomous planning? YES (plan research, gather, synthesize)
→ Task Type: Agentic
```

### Step 2: Identify Requirements

- **Skill type:** Agentic (intelligence >= 48, coding >= 42)
- **Required capabilities:** `tool_call = true`, `reasoning = true`
- **Context:** Very large (200k+) for synthesizing multiple sources
- **Priority:** Quality (autonomy requires high reliability)

### Step 3: Query for Candidates

```bash
# Quality-priority for agentic task
which-llm query "SELECT name, intelligence, coding, tool_call, reasoning, context_window, price,
          ROUND((intelligence * 0.7) + (100/price * 0.2) + (COALESCE(tps,0)/10 * 0.1), 1) as score
          FROM llms 
          WHERE intelligence >= 48 AND coding >= 42 
            AND tool_call = true AND context_window >= 128000 AND price > 0
          ORDER BY score DESC 
          LIMIT 10"
```

### Step 4: Format Recommendation

```markdown
## Task Classification
- **Skill Type:** Agentic
- **Key Constraints:** tool_call, reasoning, context >= 128k
- **Priority:** Quality (reliability critical for autonomy)

## Recommendations

### Primary: Claude Opus 4.5 ($15.00/M) - Score: 52.1
- Intelligence: 52, Coding: 48, Context: 200k
- Reasoning: true, Tool call: true
- Why: Highest reliability for autonomous multi-step tasks

### Fallback: GPT-5.2 ($12.00/M) - Score: 51.8
- Intelligence: 51, Coding: 46, Context: 128k
- Use if: Claude unavailable or need different "reasoning style"

## Cost Estimate
For agentic tasks, avoid cascade patterns—use the best model consistently.
Agentic failures are expensive (wasted tool calls, incorrect research paths).
```

---

## Workflow 4: High-Volume Data Extraction

**User request:** "I need to extract structured data from 100,000 invoices. Speed and cost are critical."

### Step 1: Classify the Task

```
Format conversion? YES (invoice → structured JSON)
→ Task Type: Transformational
```

### Step 2: Identify Requirements

- **Skill type:** Transformational (intelligence >= 20)
- **Capability:** `structured_output = true` for reliable JSON
- **Priority:** Cost and Speed (high volume)

### Step 3: Query for Candidates

```bash
# Cost + Speed priority for transformational task
which-llm query "SELECT name, intelligence, structured_output, price, tps,
          ROUND((intelligence * 0.1) + (100/price * 0.5) + (tps/10 * 0.4), 1) as score
          FROM llms 
          WHERE intelligence >= 20 AND structured_output = true 
            AND price > 0 AND tps > 0
          ORDER BY score DESC 
          LIMIT 10"
```

### Step 4: Format Recommendation

```markdown
## Task Classification
- **Skill Type:** Transformational
- **Key Constraints:** structured_output required
- **Priority:** Cost + Speed

## Recommendations

### Primary: Gemini Flash 2.0 ($0.10/M) - Score: 62.3
- Intelligence: 28, TPS: 250, structured_output: true
- Why: Extremely fast and cheap, sufficient for extraction

### Alternative: GPT-4.1-mini ($0.40/M) - Score: 55.1
- Intelligence: 38, TPS: 180, structured_output: true
- Consider if: Extraction requires more reasoning

## Volume Cost Estimate (100k invoices, ~2k tokens each)
- **Total tokens:** ~200M tokens
- **Gemini Flash:** $20
- **GPT-4.1-mini:** $80
```

---

## Workflow 5: Multi-Model Pipeline

**User request:** "I'm building a document processing pipeline: extract entities, analyze sentiment, generate summary. Different complexity at each stage."

### Step 1: Design Pipeline Roles

| Stage | Task Type | Requirements |
|-------|-----------|--------------|
| Extract | Transformational | intelligence >= 20, fast |
| Analyze | Analytical | intelligence >= 38 |
| Summarize | Transformational | intelligence >= 25, good writing |

### Step 2: Query for Each Role

```bash
# Extraction worker (cheap, fast)
which-llm query "SELECT name, intelligence, price, tps 
          FROM llms 
          WHERE intelligence >= 20 AND price < 0.5 
          ORDER BY tps DESC 
          LIMIT 3"

# Analysis worker (balanced)
which-llm query "SELECT name, intelligence, price 
          FROM llms 
          WHERE intelligence >= 38 
          ORDER BY price 
          LIMIT 3"

# Summarization worker (good output quality, affordable)
which-llm query "SELECT name, intelligence, price 
          FROM llms 
          WHERE intelligence >= 25 AND price < 2 
          ORDER BY intelligence DESC 
          LIMIT 3"
```

### Step 3: Format Recommendation

```markdown
## Pipeline Architecture

| Stage | Model | Price | Why |
|-------|-------|-------|-----|
| **Extract** | Gemini Flash 2.0 | $0.10/M | Fastest, handles simple extraction |
| **Analyze** | GPT-4.1-mini | $0.40/M | Good reasoning at low cost |
| **Summarize** | Claude Haiku 3.5 | $0.25/M | Clean, readable output |

## Cost vs Single Model
- **Single model (Claude Sonnet 4):** $3.00/M × 3 stages = $9.00/M effective
- **Multi-model pipeline:** $0.10 + $0.40 + $0.25 = $0.75/M effective
- **Savings:** 92%
```

---

## Quick Reference: Priority Weights

| Priority | Quality | Cost | Speed | Use When |
|----------|---------|------|-------|----------|
| **Balanced** | 0.4 | 0.4 | 0.2 | No strong preference |
| **Quality** | 0.7 | 0.2 | 0.1 | Accuracy critical, errors expensive |
| **Cost** | 0.2 | 0.7 | 0.1 | High volume, budget-sensitive |
| **Speed** | 0.2 | 0.2 | 0.6 | Real-time, user-facing latency |
| **Quality+Cost** | 0.5 | 0.4 | 0.1 | Good results, reasonable budget |
| **Cost+Speed** | 0.1 | 0.5 | 0.4 | Batch processing, maximize throughput |
