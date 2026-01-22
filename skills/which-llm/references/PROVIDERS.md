# Provider Considerations

When users have constraints related to specific providers, use this guide to inform recommendations.

## Provider Overview

| Provider | Strengths | Considerations | Best For |
|----------|-----------|----------------|----------|
| **Anthropic** | Reasoning, code quality, safety, long context | Slightly higher latency | Complex agents, production code, safety-critical |
| **OpenAI** | Tool calling maturity, ecosystem, API stability | Higher cost at frontier tier | Function calling, integrations, enterprise |
| **Google** | Context length (1M+), multimodal, speed | API quirks, variable behavior | Long documents, video/image, batch processing |
| **DeepSeek** | Excellent cost/performance, strong coding | China-based (latency, compliance) | Budget-conscious, coding tasks |
| **Meta (Llama)** | Open weights, fine-tuning, community | Self-hosting overhead | Privacy, customization, on-prem |
| **Mistral** | European (GDPR), open weights options | Smaller model range | EU compliance, self-hosting |
| **Cohere** | Enterprise focus, RAG optimization | Narrower use case | Enterprise search, RAG pipelines |

## Provider-Specific Queries

### Filter by Provider

```bash
# Compare models from specific providers
which-llm query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE creator IN ('OpenAI', 'Anthropic', 'Google') 
          ORDER BY intelligence DESC"

# Single provider deep dive
which-llm query "SELECT name, intelligence, coding, price, tps, context_window 
          FROM llms 
          WHERE creator = 'Anthropic' 
          ORDER BY intelligence DESC"
```

### Exclude Providers (Compliance/Preference)

```bash
# Exclude specific providers
which-llm query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE creator NOT IN ('DeepSeek', 'Alibaba') 
            AND intelligence >= 40 
          ORDER BY price"
```

## When Provider Matters

### Data Privacy / Compliance

| Requirement | Recommended Approach |
|-------------|---------------------|
| **GDPR compliance** | Mistral, EU-hosted options, or self-hosted open weights |
| **Data sovereignty** | Self-hosted Llama/Mistral, or provider with regional hosting |
| **No external API** | Open weights models (Llama, Mistral, DeepSeek) |
| **SOC2/HIPAA** | OpenAI Enterprise, Anthropic, Azure OpenAI |

```bash
# Find open weight models for self-hosting
which-llm query "SELECT name, creator, intelligence, coding, open_weights 
          FROM llms 
          WHERE open_weights = true AND intelligence >= 35 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

### Enterprise Integration

| Requirement | Recommended Provider |
|-------------|---------------------|
| **Azure ecosystem** | OpenAI (via Azure OpenAI) |
| **Google Cloud** | Google (Vertex AI) |
| **AWS** | Anthropic (Bedrock), Llama (SageMaker) |
| **Existing OpenAI contracts** | OpenAI |

### Latency Constraints

| Region | Best Providers |
|--------|----------------|
| **US** | OpenAI, Anthropic, Google (lowest latency) |
| **Europe** | Mistral, Google, OpenAI (EU endpoints) |
| **Asia** | Google, DeepSeek (if compliance allows) |

```bash
# Find fastest models
which-llm query "SELECT name, creator, intelligence, latency, tps 
          FROM llms 
          WHERE latency < 0.5 
          ORDER BY latency"
```

## Provider Characteristics

### Anthropic (Claude)

**Strengths:**
- Excellent reasoning and instruction following
- Clean, well-structured code output
- Strong safety guardrails
- Large context windows (200k)

**Considerations:**
- Can be overly cautious for some use cases
- Slightly higher latency than competitors
- Less mature function calling (improving rapidly)

**Best models by tier:**
- Frontier: Claude Opus 4.5
- Strong: Claude Sonnet 4
- Budget: Claude Haiku 3.5

### OpenAI (GPT)

**Strengths:**
- Most mature tool/function calling
- Largest ecosystem and integrations
- Stable, predictable API
- Strong reasoning models (o-series)

**Considerations:**
- Higher pricing at frontier tier
- Data usage policies (check enterprise terms)

**Best models by tier:**
- Frontier: GPT-5.2, o3
- Strong: GPT-4.1
- Budget: GPT-4.1-mini

### Google (Gemini)

**Strengths:**
- Largest context windows (1M+ tokens)
- Best multimodal capabilities
- Very fast (especially Flash models)
- Competitive pricing

**Considerations:**
- API behavior can be less predictable
- Safety filters can be aggressive
- Tool calling less mature than OpenAI

**Best models by tier:**
- Frontier: Gemini 2.5 Pro
- Strong: Gemini 2.0 Pro
- Budget: Gemini Flash 2.0

### DeepSeek

**Strengths:**
- Exceptional cost/performance ratio
- Very strong coding benchmarks
- Open weights available
- Good reasoning (R1 series)

**Considerations:**
- China-based (compliance, latency for non-Asia)
- Less enterprise support
- Smaller ecosystem

**Best models by tier:**
- Frontier: DeepSeek-V3
- Reasoning: DeepSeek-R1

### Open Weights (Llama, Mistral, etc.)

**Strengths:**
- Full control over deployment
- Fine-tuning capability
- Data never leaves your infrastructure
- No per-token costs (after infrastructure)

**Considerations:**
- Infrastructure/hosting overhead
- Need ML ops expertise
- May lag behind closed models
- Support is community-based

**Best options:**
- General: Llama 4 (405B for frontier, 70B for strong)
- Coding: DeepSeek-V3
- European: Mistral Large

## Decision Framework

Use this when users mention provider preferences:

```
User mentions "privacy" or "self-hosted"?
├─ YES → Filter for open_weights = true
└─ NO ↓

User mentions specific cloud (Azure, GCP, AWS)?
├─ YES → Recommend that cloud's native options
└─ NO ↓

User mentions compliance (GDPR, HIPAA, SOC2)?
├─ YES → Recommend enterprise tiers or EU providers
└─ NO ↓

User mentions latency or region?
├─ YES → Check provider presence in that region
└─ NO → Use benchmark-based selection
```
