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
- [ ] `R` column shows reasoning capability (+/-/?)
- [ ] `T` column shows tool_call capability (+/-/?)
- [ ] `Context` column shows context window (e.g., 128K, 200K)

---

### 3. Reasoning Filter
**Objective**: Filter models that support chain-of-thought reasoning

```bash
aa llms --reasoning
```

**Expected**:
- [ ] Only models with `reasoning=true` are shown
- [ ] Should include o3, o3-mini, and similar reasoning models
- [ ] Models with unknown reasoning (?) should NOT appear

---

### 4. Tool Call Filter
**Objective**: Filter models that support tool/function calling

```bash
aa llms --tool-call
```

**Expected**:
- [ ] Only models with `tool_call=true` are shown
- [ ] Should include GPT-4o, Claude models, etc.

---

### 5. Structured Output Filter
**Objective**: Filter models that support JSON structured output

```bash
aa llms --structured-output
```

**Expected**:
- [ ] Only models with `structured_output=true` are shown

---

### 6. Attachment Filter
**Objective**: Filter models that support file attachments

```bash
aa llms --attachment
```

**Expected**:
- [ ] Only models with `attachment=true` are shown
- [ ] Should include multi-modal models

---

### 7. Minimum Context Filter
**Objective**: Filter by minimum context window

```bash
aa llms --min-context 128000
```

**Expected**:
- [ ] Only models with context_window >= 128000 are shown
- [ ] Models with unknown context should NOT appear

---

### 8. Modality Filter (Input)
**Objective**: Filter by input modality

```bash
aa llms --modality input:image
```

**Expected**:
- [ ] Only models that accept image input are shown
- [ ] Should include GPT-4o, Claude 3, Gemini, etc.

---

### 9. Combined Filters
**Objective**: Test multiple filters together

```bash
aa llms --reasoning --tool-call --min-context 100000
```

**Expected**:
- [ ] Only models matching ALL criteria are shown
- [ ] Results should be a subset of each individual filter

---

### 10. JSON Output with Capabilities
**Objective**: Verify JSON includes all new fields

```bash
aa llms --json | head -100
```

**Expected**:
- [ ] JSON includes `reasoning`, `tool_call`, `structured_output` fields
- [ ] JSON includes `context_window`, `max_input_tokens`, `max_output_tokens`
- [ ] JSON includes `input_modalities`, `output_modalities` arrays
- [ ] JSON includes `models_dev_matched` field

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
- [ ] `aa_llms.parquet` exists
- [ ] `models_dev.parquet` exists
- [ ] `llms.parquet` exists (merged)
- [ ] Second run is faster (uses cache)

---

### 12. Sort by Context
**Objective**: Test sorting by context window

```bash
aa llms --sort context | head -20
```

**Expected**:
- [ ] Models sorted by context window (descending)
- [ ] Largest context windows appear first

---

### 13. Graceful Degradation (Optional)
**Objective**: Verify CLI works when models.dev is unavailable

```bash
# Temporarily block models.dev (manual test)
# Or test with expired cache and network disabled
```

**Expected**:
- [ ] CLI displays AA data with warning
- [ ] Capability fields show as `?` (unknown)
- [ ] No crash or error exit

---

## Results Summary

| Test | Status | Notes |
|------|--------|-------|
| 1. Basic Data Fetch | | |
| 2. Capability Column Display | | |
| 3. Reasoning Filter | | |
| 4. Tool Call Filter | | |
| 5. Structured Output Filter | | |
| 6. Attachment Filter | | |
| 7. Minimum Context Filter | | |
| 8. Modality Filter | | |
| 9. Combined Filters | | |
| 10. JSON Output | | |
| 11. Cache Behavior | | |
| 12. Sort by Context | | |
| 13. Graceful Degradation | | |

---

## Test Execution Date
- Date: ___________
- Tester: ___________
- CLI Version: ___________
