# Fallback Quick Reference

Use these heuristic recommendations when the `which-llm` CLI is unavailable.

> **Warning:** These recommendations are point-in-time snapshots (Jan 2026) and will become outdated. Always prefer using the CLI for current data when possible.

## Quick Picks by Task Type

| Task Type | Primary Pick | Fallback Pick | Why |
|-----------|--------------|---------------|-----|
| **Agentic/Orchestration** | Claude Sonnet 4 | Claude Opus 4.5, GPT-5.2 | Best reasoning + tool use reliability |
| **Coding** | Claude Sonnet 4, DeepSeek-V3 | GPT-4.1 | Strong code benchmarks, good tool integration |
| **High Volume/Cheap** | GPT-4.1-mini, Claude Haiku 3.5 | Gemini Flash 2.0 | Best $/intelligence ratio |
| **Long Context (>200k)** | Gemini 2.5 Pro | Claude Sonnet 4 | 1M+ context window |
| **Math/Reasoning** | o3, DeepSeek-R1 | Claude Opus 4.5 | Chain-of-thought optimized |
| **Real-time/Low Latency** | GPT-4.1-mini, Gemini Flash 2.0 | Claude Haiku 3.5 | Sub-second TTFT |
| **Multimodal (Vision)** | GPT-4o, Claude Sonnet 4 | Gemini 2.5 Pro | Strong vision understanding |
| **Self-hosted/Privacy** | Llama 4, DeepSeek-V3 | Mistral Large | Open weights, good capability |

## Cascade Pairs (Cost Optimization)

When budget matters, use these pre-validated cascade pairs:

### General Tasks
- **Primary:** GPT-4.1-mini (~$0.40/M)
- **Fallback:** Claude Sonnet 4 (~$3/M)
- **Expected savings:** ~60% vs always using fallback

### Coding Tasks
- **Primary:** DeepSeek-V3 (~$0.50/M)
- **Fallback:** Claude Sonnet 4 (~$3/M)
- **Expected savings:** ~55% vs always using fallback

### Agentic Tasks
- **Primary:** Claude Sonnet 4 (~$3/M)
- **Fallback:** Claude Opus 4.5 (~$15/M)
- **Expected savings:** ~65% vs always using fallback

## Provider Quick Reference

| Provider | Strengths | Best For |
|----------|-----------|----------|
| **Anthropic** | Reasoning, code quality, safety | Complex agents, production code |
| **OpenAI** | Tool calling maturity, ecosystem | Integrations, function calling |
| **Google** | Context length, multimodal | Long docs, video, images |
| **DeepSeek** | Cost/performance ratio | Budget-conscious, coding |
| **Meta (Llama)** | Open weights, fine-tuning | Self-hosting, customization |

## When to Use This Fallback

Use these heuristics when:
- CLI installation fails or isn't possible
- Working in an air-gapped environment
- Need a quick recommendation without querying
- CLI data is stale and can't be refreshed

## Limitations

These recommendations:
- **Are not real-time**: Prices, benchmarks, and model availability change frequently
- **May be biased**: Based on general patterns, not your specific use case
- **Skip nuance**: Don't account for specific capability requirements (context size, structured output, etc.)
- **Miss new models**: Won't include models released after Jan 2026

**Always validate** with the CLI when possible, and test on your actual use case before production deployment.

## Updating This Reference

If you have CLI access and want to update these recommendations:

```bash
# Get current top models by task type
which-llm query "SELECT name, intelligence, coding, price 
          FROM llms 
          WHERE intelligence >= 45 
          ORDER BY intelligence DESC 
          LIMIT 10"

# Get current best budget options
which-llm query "SELECT name, intelligence, price 
          FROM llms 
          WHERE intelligence >= 30 AND price < 1 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

Then update this file with current recommendations.
