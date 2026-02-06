## Context
The which-llm CLI requires users to have Artificial Analysis API keys to function. This creates a significant barrier to adoption since users must:
1. Register at artificialanalysis.ai
2. Create an API key
3. Configure the CLI with the key

This proposal introduces a "hosted data" approach where pre-built Parquet files are published to GitHub, eliminating the API key requirement for most users.

## Goals / Non-Goals

### Goals
- Remove API key requirement for basic CLI usage
- Maintain data freshness with daily/weekly automated updates
- Comply with Artificial Analysis attribution requirements
- Keep existing API-based flow as an option for real-time data

### Non-Goals
- Real-time data updates (daily freshness is acceptable)
- Hosting our own infrastructure beyond GitHub
- Supporting offline-first workflows (network still required)

## Decisions

### Decision: Use GitHub Releases for data hosting
**What**: Store Parquet files as GitHub Release assets rather than committing to the repo.

**Why**:
- Releases have higher size limits than repo files
- Cleaner git history (no data churn)
- Easy to version and tag data updates
- Simple to fetch via `gh` CLI or raw URLs

**Alternatives considered**:
- Git LFS: Adds complexity, bandwidth costs
- S3/Cloud storage: Requires infrastructure, costs
- Commit to repo: Bloats repo size over time

### Decision: Daily scheduled workflow
**What**: Run the data update workflow daily via cron.

**Why**:
- LLM benchmarks don't change frequently
- Daily is fresh enough for most use cases
- Respects AA's 1,000 requests/day rate limit
- Can trigger manually for urgent updates

### Decision: Fallback chain for data fetching
**What**: CLI tries sources in order: local cache -> GitHub Release -> API (if key available)

**Why**:
- Maximizes availability
- Graceful degradation
- Users with API keys can still get real-time data

### Decision: Attribution in multiple locations
**What**: Add Artificial Analysis attribution to:
- CLI `--version` output
- README.md
- `which-llm info` command output

**Why**:
- Required by AA's terms of use
- Multiple touchpoints ensure visibility
- Professional acknowledgment of data source

## Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    GitHub Actions (Daily)                       │
├─────────────────────────────────────────────────────────────────┤
│  1. Fetch from Artificial Analysis API (using repo secret)     │
│  2. Fetch from models.dev API (public)                         │
│  3. Merge data sources                                          │
│  4. Write Parquet files                                         │
│  5. Create/update GitHub Release with assets                    │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                    GitHub Release                               │
├─────────────────────────────────────────────────────────────────┤
│  Assets:                                                        │
│  - llms.parquet                                                 │
│  - text_to_image.parquet                                        │
│  - image_editing.parquet                                        │
│  - text_to_speech.parquet                                       │
│  - text_to_video.parquet                                        │
│  - image_to_video.parquet                                       │
│  - manifest.json (timestamps, versions)                         │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                    which-llm CLI                                │
├─────────────────────────────────────────────────────────────────┤
│  1. Check local cache (TTL: 24 hours)                          │
│  2. If stale/missing: fetch from GitHub Release                │
│  3. Fallback: use API directly (if key configured)             │
│  4. Query data with DuckDB                                      │
└─────────────────────────────────────────────────────────────────┘
```

## Risks / Trade-offs

### Risk: Data staleness
**Mitigation**: Daily updates are sufficient for benchmark data. Users needing real-time data can configure API keys.

### Risk: GitHub rate limiting
**Mitigation**: Cache aggressively (24h TTL). GitHub Release downloads are generous.

### Risk: GitHub availability
**Mitigation**: Fallback to API if GitHub is unreachable and API key is available.

### Risk: Breaking existing workflows
**Mitigation**: API-based flow remains available. Users with API keys configured continue to work.

## Release Strategy

### Phase 1: GitHub Actions workflow
- Create workflow that fetches and publishes data
- Test with manual triggers
- Set up repo secret for AA API key

### Phase 2: CLI changes
- Add GitHub Release fetching to client
- Update cache logic for remote data
- Add attribution notices

### Phase 3: Documentation
- Update README with new setup (simplified)
- Add attribution section
- Update CONTRIBUTING guide

## Open Questions

1. **Release tag naming**: Use `data/YYYY-MM-DD` or `data/latest`?
   - Recommendation: Both. Update `data/latest` tag on each run, keep dated tags for history.

2. **Manifest format**: What metadata to include?
   - Recommendation: `manifest.json` with timestamps, source versions, record counts.

3. **Cache invalidation**: How does CLI know when to refresh?
   - Recommendation: Check `manifest.json` etag/last-modified headers, compare with local.
