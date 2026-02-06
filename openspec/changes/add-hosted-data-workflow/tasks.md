## 1. GitHub Actions Workflow

- [x] 1.1 Create `.github/workflows/update-data.yml` workflow file
- [x] 1.2 Add scheduled trigger (daily cron) and manual dispatch
- [x] 1.3 Implement data fetching step using existing Rust code
- [x] 1.4 Add step to create/update GitHub Release with Parquet assets
- [x] 1.5 Add manifest.json generation with timestamps and metadata
- [x] 1.6 Document required repository secret (`ARTIFICIAL_ANALYSIS_API_KEY`)
- [ ] 1.7 Test workflow with manual trigger

## 2. CLI Data Fetching

- [x] 2.1 Add `remote` module for GitHub Release fetching
- [x] 2.2 Implement Parquet download from GitHub Release assets
- [x] 2.3 Add manifest.json fetching and parsing
- [x] 2.4 Implement cache freshness check using manifest timestamps
- [x] 2.5 Add fallback chain: local cache -> GitHub -> API
- [x] 2.6 Handle GitHub rate limiting gracefully

## 3. Cache Updates

- [x] 3.1 Add remote data caching with configurable TTL
- [x] 3.2 Store manifest.json locally for freshness comparison
- [x] 3.3 Add `--refresh` flag to force re-download from GitHub
- [x] 3.4 Add `--use-api` flag to bypass hosted data and use API directly

## 4. Attribution

- [x] 4.1 Add attribution notice to `--version` output (in footer)
- [x] 4.2 Add `which-llm info` command showing data source and attribution
- [x] 4.3 Update README.md with attribution section
- [x] 4.4 Link to Artificial Analysis methodology page

## 5. Documentation

- [x] 5.1 Update README.md with simplified setup (no API key needed)
- [x] 5.2 Document optional API key configuration for real-time data
- [x] 5.3 Add section on data freshness and update schedule
- [ ] 5.4 Update CONTRIBUTING.md if needed

## 6. Testing

- [ ] 6.1 Add unit tests for remote data fetching
- [ ] 6.2 Add integration tests for fallback chain
- [ ] 6.3 Test workflow in fork before merging
- [ ] 6.4 Verify attribution displays correctly
