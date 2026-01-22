# Model Specialization Patterns

Some models excel at specific domains. Use this guide to pick the right tool for the job.

> **Note:** This guidance reflects patterns as of Jan 2026. The landscape changes rapidly—verify with current benchmarks using `which-llm query`.

## Domain-Specific Recommendations

### Coding Tasks

| Need | Recommended Approach | Why |
|------|---------------------|-----|
| Production code | Filter by `coding >= 40` | High reliability needed |
| Code review | Filter by `coding >= 35`, prioritize `intelligence` | Needs reasoning + code understanding |
| Quick scripts | `coding >= 25` is sufficient | Lower stakes, can iterate |
| Specialized languages | Test specifically | Benchmarks favor Python/JS |

**Query pattern:**
```bash
which-llm query "SELECT name, coding, livecodebench, price 
          FROM llms 
          WHERE coding >= 40 
          ORDER BY coding DESC 
          LIMIT 10"
```

**Known specialists (verify current scores):**
- DeepSeek models often excel at code despite lower general intelligence
- Claude models tend to produce cleaner, more idiomatic code
- OpenAI models have strong tool integration for code execution

### Long Context Tasks

| Context Needed | Filter | Use Case |
|----------------|--------|----------|
| < 32K tokens | Any modern model | Standard documents |
| 32K - 128K | `context_window >= 128000` | Long docs, codebases |
| 128K - 1M | `context_window >= 500000` | Books, large repos |
| > 1M | `context_window >= 1000000` | Massive analysis |

**Query pattern:**
```bash
which-llm query "SELECT name, context_window, intelligence, price 
          FROM llms 
          WHERE context_window >= 200000 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

**Known specialists:**
- Gemini models lead on context length (1M+ tokens)
- Claude models handle long context well with good recall
- GPT models have shorter context but strong reasoning

**Warning:** Large context window doesn't guarantee good recall. Test "needle in haystack" performance for your specific use case.

### Math & Reasoning

| Task | Recommended Filter | Notes |
|------|-------------------|-------|
| Competition math | `math >= 60` | AIME/IMO level |
| University math | `math >= 45` | Calculus, linear algebra |
| Applied math | `math >= 30` | Statistics, business math |
| Logic puzzles | `intelligence >= 40` | General reasoning |

**Query pattern:**
```bash
which-llm query "SELECT name, math, aime, intelligence, price 
          FROM llms 
          WHERE math >= 50 
          ORDER BY math DESC 
          LIMIT 10"
```

**Known specialists:**
- Models with `reasoning = true` (chain-of-thought) excel at multi-step math
- DeepSeek-R1 variants are strong on mathematical reasoning
- OpenAI o-series models designed for reasoning tasks

### Scientific & Technical

| Domain | Metrics | Notes |
|--------|---------|-------|
| Graduate science | `gpqa >= 50` | PhD-level Q&A |
| Frontier research | `hle >= 30` | Humanity's Last Exam |
| Scientific coding | `scicode` + `coding` | Combine both |

**Query pattern:**
```bash
which-llm query "SELECT name, gpqa, hle, intelligence, price 
          FROM llms 
          WHERE gpqa >= 45 
          ORDER BY gpqa DESC 
          LIMIT 10"
```

### Agentic / Tool Use

No direct benchmark for tool reliability. Use this proxy approach:

1. **Filter:** `tool_call = true AND intelligence >= 45`
2. **Prefer:** Models with `reasoning = true` for complex planning
3. **Test:** Always validate on your specific tool schemas

**Query pattern:**
```bash
which-llm query "SELECT name, intelligence, coding, tool_call, reasoning, price 
          FROM llms 
          WHERE tool_call = true AND intelligence >= 45 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

**Known patterns:**
- OpenAI models have mature function calling
- Claude models handle complex multi-turn tool use well
- Open-weight models vary significantly—test thoroughly

### Real-Time / Streaming

| Requirement | Filter | Use Case |
|-------------|--------|----------|
| Interactive chat | `latency < 0.5` | User-facing |
| Batch processing | `tps > 100` | Background jobs |
| Voice/real-time | `latency < 0.3 AND tps > 150` | Conversational AI |

**Query pattern:**
```bash
which-llm query "SELECT name, latency, tps, intelligence, price 
          FROM llms 
          WHERE latency < 0.5 AND tps > 100 
          ORDER BY latency 
          LIMIT 10"
```

### Multimodal (Vision)

Filter for vision capability:
```bash
which-llm query "SELECT name, intelligence, price, input_modalities 
          FROM llms 
          WHERE input_modalities LIKE '%image%' 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

**Known specialists:**
- GPT-4o and Claude 3.5 Sonnet for general vision
- Gemini for video and long visual context

## Anti-Patterns

Avoid these common mistakes:

| Mistake | Why It's Wrong | Better Approach |
|---------|----------------|-----------------|
| Always picking highest `intelligence` | Overkill for simple tasks, expensive | Match capability to task complexity |
| Ignoring `coding` for code tasks | `intelligence` doesn't guarantee code quality | Use `coding` as primary filter |
| Assuming `tool_call = true` means reliable | Boolean doesn't indicate reliability | Test on your actual tool schemas |
| Picking by context window alone | Large context ≠ good recall | Test retrieval accuracy |
| Using same model for everything | Different tasks have different needs | Specialize per task type |

## Specialization Strategy

For a multi-model system, consider this architecture:

```
┌─────────────────────────────────────────────────────┐
│                    Task Router                       │
└─────────────────────────────────────────────────────┘
          │           │           │           │
          ▼           ▼           ▼           ▼
    ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐
    │  Code   │ │  Math   │ │ General │ │ Cheap   │
    │ Expert  │ │ Expert  │ │ Strong  │ │ Simple  │
    │ coding  │ │ math    │ │ intel   │ │ intel   │
    │  >=45   │ │  >=60   │ │  >=45   │ │  >=25   │
    └─────────┘ └─────────┘ └─────────┘ └─────────┘
```

This allows:
- **Cost optimization**: Simple tasks go to cheap models
- **Quality optimization**: Hard tasks go to specialists
- **Flexibility**: Easy to swap individual models

See [MULTI-MODEL.md](MULTI-MODEL.md) for implementation patterns.
