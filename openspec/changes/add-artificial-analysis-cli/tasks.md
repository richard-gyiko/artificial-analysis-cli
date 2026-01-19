# Implementation Tasks

## 1. Project Setup
- [x] 1.1 Initialize Rust project with `cargo init`
- [x] 1.2 Add dependencies to Cargo.toml:
  - [x] clap (CLI parsing with derive feature)
  - [x] reqwest (HTTP client with rustls-tls feature)
  - [x] serde, serde_json (serialization)
  - [x] tokio (async runtime with rt-multi-thread feature)
  - [x] dirs (XDG-compliant paths)
  - [x] toml (config file parsing)
  - [x] thiserror (error derivation)
  - [x] tabled (ASCII table formatting)
  - [x] csv (CSV output)
  - [x] dialoguer (interactive prompts, secret input)
  - [x] chrono (timestamp handling)
- [x] 1.3 Set up project structure (src/main.rs, src/lib.rs, modules)
- [x] 1.4 Configure rustfmt.toml and clippy.toml
- [x] 1.5 Set up CI workflow (GitHub Actions) for build/test/lint

## 2. Core Infrastructure
- [x] 2.1 Implement configuration module (config.rs)
  - [x] Profile-based config file parsing (~/.config/aa/config.toml)
  - [x] Profile struct with api_key and optional settings
  - [x] Default profile selection logic
  - [x] Environment variable override (AA_API_KEY)
- [x] 2.2 Implement API client module (client.rs)
  - [x] HTTP client with reqwest
  - [x] Authentication header injection from active profile (x-api-key)
  - [x] Response header extraction (rate limit info)
  - [x] Error response handling
- [x] 2.3 Implement caching module (cache.rs)
  - [x] File-based cache storage (~/.cache/aa/)
  - [x] TTL-based invalidation (default 1 hour)
  - [x] Cache key generation from requests
  - [x] Quota info caching from response headers
- [x] 2.4 Implement error types (error.rs)
  - [x] Custom error enum with thiserror
  - [x] User-friendly error messages

## 3. Data Models
- [x] 3.1 Define LLM model structs (models/llm.rs)
  - [x] LlmModel with evaluations, pricing, speed metrics
  - [x] ModelCreator
  - [x] Evaluations struct
  - [x] Pricing struct
- [x] 3.2 Define media model structs (models/media.rs)
  - [x] MediaModel base struct (id, name, slug, creator, elo, rank)
  - [x] Category breakdown struct
- [x] 3.3 Define API response wrappers (models/response.rs)
  - [x] Generic API response with status and data
  - [x] Error response struct
  - [x] Rate limit info struct

## 4. CLI Commands
- [x] 4.1 Set up clap CLI structure (cli/mod.rs)
  - [x] Define Cli struct with derive macro
  - [x] Define Commands enum for subcommands
  - [x] Global flags (--json, --csv, --table, --plain, --refresh, --profile)
- [x] 4.2 Implement `profile` command (commands/profile.rs)
  - [x] create subcommand (interactive and --api-key flag)
  - [x] list subcommand
  - [x] default subcommand
  - [x] delete subcommand
  - [x] show subcommand
- [x] 4.3 Implement `llms` command (commands/llms.rs)
  - [x] Fetch and display LLM models
  - [x] --model filter
  - [x] --creator filter
  - [x] --sort option
- [x] 4.4 Implement `text-to-image` command (commands/text_to_image.rs)
  - [x] Fetch and display rankings
  - [x] --categories flag
- [x] 4.5 Implement `image-editing` command (commands/image_editing.rs)
- [x] 4.6 Implement `text-to-speech` command (commands/text_to_speech.rs)
- [x] 4.7 Implement `text-to-video` command (commands/text_to_video.rs)
  - [x] --categories flag
- [x] 4.8 Implement `image-to-video` command (commands/image_to_video.rs)
  - [x] --categories flag
- [x] 4.9 Implement `quota` command (commands/quota.rs)
  - [x] Display cached quota info
  - [x] Show last updated time
  - [x] Show reset time
- [x] 4.10 Implement `cache` command (commands/cache.rs)
  - [x] clear subcommand
  - [x] status subcommand

## 5. Output Formatting
- [x] 5.1 Implement markdown formatter (output/markdown.rs) - DEFAULT
  - [x] Markdown table generation
  - [x] Column alignment
- [x] 5.2 Implement table formatter (output/table.rs)
  - [x] ASCII box-drawing tables
- [x] 5.3 Implement JSON formatter (output/json.rs)
  - [x] Pretty-print by default
- [x] 5.4 Implement CSV formatter (output/csv.rs)
- [x] 5.5 Implement plain formatter (output/plain.rs)
  - [x] Tab-separated values, no headers
- [x] 5.6 Implement output dispatcher (output/mod.rs)
  - [x] Select formatter based on flags
  - [x] Default to markdown

## 6. Quota Tracking
- [x] 6.1 Extract rate limit headers from API responses (X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset)
- [x] 6.2 Store quota info in profile-namespaced cache file (~/.cache/aa/quota-{profile}.json)
- [x] 6.3 Implement low-quota warning (< 10%)
- [x] 6.4 Display quota info in `aa quota` command
- [x] 6.5 Handle first-run case (no quota data yet)

## 7. Testing
- [x] 7.1 Unit tests for config/profile parsing
- [x] 7.2 Unit tests for data model deserialization
- [ ] 7.3 Integration tests with mock HTTP server (deferred - optional)
- [x] 7.4 CLI integration tests (command parsing)
- [x] 7.5 Snapshot tests for output formatting (especially markdown)

## 8. Documentation & Polish
- [ ] 8.1 Add README.md with installation and usage (out of scope per conventions)
- [x] 8.2 Add --help text for all commands
- [ ] 8.3 Add shell completion generation (clap_complete) - future enhancement
- [ ] 8.4 Add man page generation (optional) - future enhancement

## 9. Release Preparation
- [x] 9.1 Test on Windows (verified via current development)
- [ ] 9.2 Set up release workflow (cross-compilation) - future enhancement
- [ ] 9.3 Create initial release with binaries - future enhancement
