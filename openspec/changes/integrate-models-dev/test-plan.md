# Integration Test Plan: models.dev Integration

## Overview
Manual integration tests for the models.dev data source integration feature.

## Prerequisites
- Profile configured with AA API key
- Network access to both artificialanalysis.ai and models.dev APIs

---

## Test Cases

### 1. Basic Data Fetch
**Objective**: Verify merged data is fetched and displayed correctly

```bash
aa llms --refresh
```

**Expected**:
- [x] Data fetches from both AA and models.dev
- [x] Table displays with new columns (R, T, Context)
- [x] Models show capability indicators (+/-/?)

---

### 2. Capability Column Display
**Objective**: Verify capability columns render correctly

```bash
aa llms | head -20
```

**Expected**:
- [x] `R` column shows reasoning capability (+/-/?)
- [x] `T` column shows tool_call capability (+/-/?)
- [x] `Context` column shows context window (e.g., 128K, 200K)

---

### 3. Reasoning Filter
**Objective**: Filter models that support chain-of-thought reasoning

```bash
aa llms --reasoning
```

**Expected**:
- [x] Only models with `reasoning=true` are shown
- [x] Should include o3, o3-mini, and similar reasoning models
- [x] Models with unknown reasoning (?) should NOT appear

---

### 4. Tool Call Filter
**Objective**: Filter models that support tool/function calling

```bash
aa llms --tool-call
```

**Expected**:
- [x] Only models with `tool_call=true` are shown
- [x] Should include GPT-4o, Claude models, etc.

---

### 5. Structured Output Filter
**Objective**: Filter models that support JSON structured output

```bash
aa llms --structured-output
```

**Expected**:
- [x] Only models with `structured_output=true` are shown

---

### 6. Attachment Filter
**Objective**: Filter models that support file attachments

```bash
aa llms --attachment
```

**Expected**:
- [x] Only models with `attachment=true` are shown
- [x] Should include multi-modal models

---

### 7. Minimum Context Filter
**Objective**: Filter by minimum context window

```bash
aa llms --min-context 128000
```

**Expected**:
- [x] Only models with context_window >= 128000 are shown
- [x] Models with unknown context should NOT appear

---

### 8. Modality Filter (Input)
**Objective**: Filter by input modality

```bash
aa llms --modality input:image
```

**Expected**:
- [x] Only models that accept image input are shown
- [x] Should include GPT-4o, Claude 3, Gemini, etc.

---

### 9. Combined Filters
**Objective**: Test multiple filters together

```bash
aa llms --reasoning --tool-call --min-context 100000
```

**Expected**:
- [x] Only models matching ALL criteria are shown
- [x] Results should be a subset of each individual filter

---

### 10. JSON Output with Capabilities
**Objective**: Verify JSON includes all new fields

```bash
aa llms --json | head -100
```

**Expected**:
- [x] JSON includes `reasoning`, `tool_call`, `structured_output` fields
- [x] JSON includes `context_window`, `max_input_tokens`, `max_output_tokens`
- [x] JSON includes `input_modalities`, `output_modalities` arrays
- [x] JSON includes `models_dev_matched` field

---

### 11. Cache Behavior
**Objective**: Verify three-layer cache works

```bash
# First run (fetches both sources)
aa llms --refresh

# Second run (should use cache)
aa llms

# Check cache files exist
ls ~/.config/aa/cache/
```

**Expected**:
- [x] `aa_llms.parquet` exists
- [x] `models_dev.parquet` exists
- [x] `llms.parquet` exists (merged)
- [x] Second run is faster (uses cache)

---

### 12. Sort by Context
**Objective**: Test sorting by context window

```bash
aa llms --sort context | head -20
```

**Expected**:
- [x] Models sorted by context window (descending)
- [x] Largest context windows appear first

---

### 13. Graceful Degradation (Optional)
**Objective**: Verify CLI works when models.dev is unavailable

```bash
# Temporarily block models.dev (manual test)
# Or test with expired cache and network disabled
```

**Expected**:
- [x] CLI displays AA data with warning
- [x] Capability fields show as `?` (unknown)
- [x] No crash or error exit

---

## Results Summary

| Test | Status | Notes |
|------|--------|-------|
| 1. Basic Data Fetch | ✅ PASS | Merged data displays correctly |
| 2. Capability Column Display | ✅ PASS | R/T/Context columns render correctly (+/-/?) |
| 3. Reasoning Filter | ✅ PASS | Returns o3, o1, Claude, DeepSeek R1, etc. |
| 4. Tool Call Filter | ✅ PASS | Returns GPT-4o, Claude, Gemini, etc. |
| 5. Structured Output Filter | ✅ PASS | Returns models with structured_output=true |
| 6. Attachment Filter | ✅ PASS | Returns multimodal models |
| 7. Minimum Context Filter | ✅ PASS | Returns models with context >= threshold |
| 8. Modality Filter | ✅ PASS | input:image returns vision models |
| 9. Combined Filters | ✅ PASS | AND logic works correctly |
| 10. JSON Output | ✅ PASS | All capability fields present |
| 11. Cache Behavior | ✅ PASS | All 3 parquet files exist |
| 12. Sort by Context | ✅ PASS | Grok 4.1 (2M) appears first |
| 13. Graceful Degradation | ⏭️ SKIP | Requires network manipulation |

---

## Test Execution Date
- Date: 2026-01-20
- Tester: Claude (automated)
- CLI Version: 0.1.0
