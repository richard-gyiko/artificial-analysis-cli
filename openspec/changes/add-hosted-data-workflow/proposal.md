# Change: Add Hosted Data Workflow

## Why
Currently, users must register for Artificial Analysis API keys to use the CLI. This creates significant adoption friction. By hosting pre-built Parquet data files on GitHub and fetching them at runtime, users can query LLM data without any API key registration.

## What Changes
- Add GitHub Actions workflow to periodically fetch data from Artificial Analysis and models.dev APIs
- Store the merged Parquet files as GitHub Release assets
- Modify CLI to fetch pre-built Parquet from GitHub Releases by default
- **BREAKING**: API key is no longer required for basic usage (still optional for real-time data)
- Add proper attribution to Artificial Analysis as required by their terms

## Impact
- Affected specs: `cli`
- Affected code:
  - `.github/workflows/` - New data update workflow
  - `src/client/` - Add GitHub data fetching
  - `src/cache.rs` - Handle remote Parquet caching
  - `README.md` - Update setup instructions, add attribution
  - CLI help text - Add attribution notice
