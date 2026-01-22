# Evaluation Scenarios

Real-world scenarios with developer personas for evaluating the `which-llm` skill.

## Personas

### 1. Alex - Senior Backend Engineer
- **Context:** 8 years experience, building a B2B SaaS product
- **Tech savvy:** High (comfortable with APIs, understands token costs)
- **Budget awareness:** High (has to justify costs to finance)
- **Typical asks:** Specific, technical, constraint-heavy
- **Values:** Specificity, cost math, constraints honored
- **Red flags:** Generic answers, ignoring stated constraints

### 2. Jordan - Full-Stack Developer at a Startup
- **Context:** 3 years experience, wearing many hats, shipping fast
- **Tech savvy:** Medium-high (knows LLMs exist, hasn't optimized for cost)
- **Budget awareness:** Medium (VC-funded but runway-conscious)
- **Typical asks:** Feature-focused, wants quick answers
- **Values:** Speed to answer, actionable recommendation
- **Red flags:** Over-engineering, too many options

### 3. Sam - Junior Developer / AI Enthusiast
- **Context:** 1 year experience, excited about AI, learning as they go
- **Tech savvy:** Medium (follows tutorials, less depth on tradeoffs)
- **Budget awareness:** Low (personal projects, free tiers)
- **Typical asks:** Vague, exploratory, needs guidance
- **Values:** Clarity, guidance, encouragement
- **Red flags:** Jargon overload, condescension

### 4. Morgan - Engineering Manager / Tech Lead
- **Context:** 12 years experience, evaluating architecture decisions
- **Tech savvy:** High (strategic, thinks about maintainability)
- **Budget awareness:** High (owns team budget, thinks in TCO)
- **Typical asks:** Comparative, risk-aware, wants options
- **Values:** Tradeoff analysis, risk awareness, options
- **Red flags:** Single "best" answer without context

---

## Scenarios

### Scenario 1: Customer Support Assistant
**Persona:** Alex (Senior Backend)

**Request:**
> "We're adding an AI assistant to our customer support portal. It needs to call our internal APIs to fetch order status, refund info, etc. We expect ~50k conversations/month. Latency matters since it's user-facing. We're on AWS and would prefer not to add new vendors if possible."

**Expected response should cover:**
- Task type: Tool-using
- Key constraints: `tool_call`, low latency, AWS preference (Bedrock)
- Volume consideration: 50k convos → cost optimization matters
- Cascade recommendation with cost projection
- Specific models available on AWS Bedrock

**Evaluation criteria:**
- [ ] Correctly identifies tool-using task type
- [ ] Addresses latency constraint
- [ ] Acknowledges AWS/Bedrock preference
- [ ] Provides cost estimate at stated volume
- [ ] Recommends cascade for cost optimization

---

### Scenario 2: Slack Thread Summarizer
**Persona:** Jordan (Startup Full-Stack)

**Request:**
> "I'm building a feature that summarizes long Slack threads so users can catch up quickly. Threads can be pretty long, like 200+ messages. What model should I use?"

**Expected response should cover:**
- Task type: Transformational (summarization)
- Key constraint: context window (200 messages ≈ 40-80k tokens)
- Clarifying question or assumption about volume/budget
- Simple recommendation without overwhelming options

**Evaluation criteria:**
- [ ] Correctly identifies transformational task type
- [ ] Addresses context window requirement
- [ ] Doesn't overwhelm with options (2-3 max)
- [ ] Provides actionable recommendation quickly
- [ ] States assumptions if volume/budget not clarified

---

### Scenario 3: Personal Coding Assistant
**Persona:** Sam (Junior Developer)

**Request:**
> "I want to build an AI app that can help me code. Like I describe what I want and it writes the code. What's the best model for that?"

**Expected response should cover:**
- Task type: Analytical or Tool-using (depending on interpretation)
- Gentle guidance on what "best" means (cost? quality? speed?)
- Accessible recommendation with 1-2 options
- Encouragement to experiment

**Evaluation criteria:**
- [ ] Doesn't overwhelm with jargon or SQL queries
- [ ] Explains tradeoffs in accessible terms
- [ ] Provides clear starting recommendation
- [ ] Encourages experimentation
- [ ] Doesn't assume budget constraints

---

### Scenario 4: Code Review Bot Evaluation
**Persona:** Morgan (Tech Lead)

**Request:**
> "We're evaluating whether to build an internal code review bot. It would analyze PRs, flag issues, suggest improvements. Needs to handle our monorepo (large diffs). I want to understand the cost/quality tradeoffs before I pitch this to leadership."

**Expected response should cover:**
- Task type: Analytical
- Key constraints: large context (monorepo diffs), coding capability
- Multiple options with clear tradeoff analysis
- Cost projections at estimated volume
- Risk acknowledgment (benchmarks vs real-world)

**Evaluation criteria:**
- [ ] Correctly identifies analytical task type
- [ ] Addresses large context requirement
- [ ] Provides multiple options (not just one "best")
- [ ] Includes cost projections with assumptions stated
- [ ] Acknowledges benchmark limitations for this use case
- [ ] Suitable for sharing with leadership

---

### Scenario 5: Document Processing Pipeline
**Persona:** Alex (Senior Backend)

**Request:**
> "We're designing a document processing pipeline: ingest PDFs, extract key fields, classify document type, then route to different downstream handlers. Thinking we might want different models for each stage. What would you recommend?"

**Expected response should cover:**
- Recognition of multi-model opportunity
- Stage-by-stage recommendations (extraction → classification → routing)
- Cost comparison: multi-model vs single-model
- Implementation complexity acknowledgment

**Evaluation criteria:**
- [ ] Recognizes multi-model architecture is appropriate
- [ ] Provides stage-specific recommendations
- [ ] Shows cost savings vs single-model approach
- [ ] Addresses each stage's requirements differently
- [ ] Acknowledges implementation complexity tradeoff

---

### Scenario 6: Vague Requirements
**Persona:** Jordan (Startup)

**Request:**
> "What's a good cheap model that's also smart?"

**Expected response should cover:**
- Clarifying questions OR reasonable assumptions stated explicitly
- Recognition that "cheap" and "smart" are relative
- 2-3 options at different price/capability points
- Not overwhelming with SQL queries

**Evaluation criteria:**
- [ ] Asks clarifying question OR states assumptions
- [ ] Doesn't just dump a query result
- [ ] Provides concrete options with prices
- [ ] Explains the tradeoff spectrum briefly
- [ ] Keeps response concise

---

### Scenario 7: Wrong Mental Model
**Persona:** Sam (Junior)

**Request:**
> "I need GPT-4 for my project but it's too expensive. Is there a free version of GPT-4?"

**Expected response should cover:**
- Gentle correction (no free GPT-4)
- Reframe: "What are you trying to build?"
- Suggest alternatives (open weights, cheaper APIs, free tiers)
- Educational without being condescending

**Evaluation criteria:**
- [ ] Corrects misconception without condescension
- [ ] Asks about underlying need (what's the project?)
- [ ] Suggests concrete alternatives
- [ ] Mentions free tier options if applicable
- [ ] Doesn't lecture about how LLM pricing works

---

### Scenario 8: Risk-Focused Evaluation
**Persona:** Morgan (Tech Lead)

**Request:**
> "We're considering using Claude for an agentic workflow that will have access to production databases (read-only). What are the risks? Is Claude the right choice or should we look at something else?"

**Expected response should cover:**
- Task type: Agentic
- Risk acknowledgment (tool reliability, hallucination, guardrails)
- Why Claude might be good (safety focus, reasoning)
- Alternatives to consider
- Recommendation to test on staging first
- Not just "here's a query"—this needs judgment

**Evaluation criteria:**
- [ ] Addresses risks seriously (not dismissive)
- [ ] Discusses tool calling reliability concerns
- [ ] Acknowledges Claude's safety strengths
- [ ] Suggests alternatives for comparison
- [ ] Recommends testing/validation approach
- [ ] Doesn't just output a model query

---

### Scenario 9: Compliance Constraints
**Persona:** Alex (Senior Backend)

**Request:**
> "We need to add AI features to our healthcare app. HIPAA compliance is mandatory. We're considering either self-hosting or using a BAA-covered API. What are our options?"

**Expected response should cover:**
- Compliance as primary filter
- Self-hosted options (Llama, Mistral)
- BAA-covered APIs (Azure OpenAI, Anthropic, etc.)
- Tradeoff: capability vs compliance overhead
- Not just benchmarks—this is a compliance question

**Evaluation criteria:**
- [ ] Prioritizes compliance over raw capability
- [ ] Lists BAA-available options
- [ ] Discusses self-hosting as alternative
- [ ] Acknowledges this requires legal/compliance review
- [ ] Doesn't recommend non-compliant options

---

### Scenario 10: Real-Time Voice Application
**Persona:** Jordan (Startup)

**Request:**
> "Building a voice AI for our mobile app—like a conversational assistant. Users speak, we transcribe, send to LLM, and speak the response back. Needs to feel snappy. What model should I use for the LLM part?"

**Expected response should cover:**
- Task type: Depends on what the assistant does (likely Analytical or Tool-using)
- Key constraint: Latency is critical (sub-second TTFT)
- Speed-focused recommendation
- May need to sacrifice some capability for speed

**Evaluation criteria:**
- [ ] Prioritizes latency above other factors
- [ ] Recommends low-latency models specifically
- [ ] Mentions TTFT (time to first token) as key metric
- [ ] Acknowledges capability tradeoff for speed
- [ ] Suggests streaming if not already implied

---

## Scoring Rubric

For each scenario, score the response on:

| Criterion | 1 (Poor) | 3 (Adequate) | 5 (Excellent) |
|-----------|----------|--------------|---------------|
| **Task Understanding** | Misclassifies task | Correct classification | Correct + explains why |
| **Constraint Handling** | Ignores constraints | Addresses some | Addresses all, flags conflicts |
| **Persona Fit** | Wrong tone/depth | Acceptable | Tailored to persona's level |
| **Actionability** | Vague suggestions | Concrete options | Clear recommendation + next steps |
| **Appropriate Caveats** | None or excessive | Generic disclaimer | Task-specific validation advice |

**Overall score:** Average across criteria, weighted by scenario complexity.
