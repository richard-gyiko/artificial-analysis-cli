# Design Document: Artificial Analysis CLI

## Context

This CLI provides access to the Artificial Analysis API, which offers AI model benchmarks, 
performance metrics, and rankings. The tool targets developers, researchers, and **AI agents** 
who need quick terminal access to this data.

**Stakeholders**: Developers, researchers, AI practitioners, AI agents  
**Constraints**: 
- Rate limit of 1,000 requests/day on free tier
- Must provide attribution to Artificial Analysis
- Cross-platform support required (Windows, macOS, Linux)

## Goals / Non-Goals

### Goals
- Fast, single-binary CLI with minimal dependencies
- Intuitive command structure following CLI conventions
- Respect API rate limits through caching
- **AI-agent friendly output** (markdown default, structured formats)
- Support scripting with JSON/CSV output
- Multi-profile support for different API keys/environments

### Non-Goals
- GUI or TUI interface
- Real-time data streaming
- CritPt evaluation submission (complex POST workflow)
- Data aggregation or historical tracking

## Decisions

### Decision 1: CLI Framework - clap v4
**Choice**: clap with derive macros  
**Why**: 
- Industry standard for Rust CLIs
- Derive macros reduce boilerplate
- Built-in shell completion and help generation
- Type-safe argument parsing

**Alternatives considered**:
- `structopt`: Merged into clap v3+
- `argh`: Lighter but less ecosystem support
- Manual parsing: Too error-prone

### Decision 2: HTTP Client - reqwest
**Choice**: reqwest with rustls  
**Why**:
- Most popular async HTTP client in Rust
- rustls avoids OpenSSL dependency for easier cross-compilation
- Built-in JSON support with serde

**Alternatives considered**:
- `ureq`: Simpler but blocking-only
- `hyper`: Lower-level, more code required

### Decision 3: Async Runtime - tokio
**Choice**: tokio with `rt-multi-thread` feature  
**Why**:
- Required by reqwest
- Standard in async Rust ecosystem
- Excellent performance

### Decision 4: Cache Strategy - File-based with TTL
**Choice**: JSON files in `~/.cache/aa/` with 1-hour default TTL  
**Why**:
- Simple implementation
- Inspectable by users
- Survives process restarts
- No external dependencies (no Redis, SQLite)

**Cache key format**: `{endpoint}-{hash(params)}.json`  
**Metadata**: Store `cached_at` timestamp in file

**Alternatives considered**:
- SQLite: Overkill for this use case
- In-memory only: Lost between invocations

### Decision 5: Default Output Format - Markdown
**Choice**: Markdown tables as default output  
**Why**:
- **Primary use case**: AI agents consuming CLI output need structured, readable data
- Markdown is universally parsed by LLMs and humans alike
- GitHub/GitLab/docs render markdown tables natively
- Easy to copy-paste into documentation or chat

**Format options** (in order of priority):
1. Markdown table (default) - AI-agent and human friendly
2. JSON (`--json`) - Machine parsing, full data fidelity
3. CSV (`--csv`) - Spreadsheet import, data analysis
4. ASCII table (`--table`) - Terminal aesthetics
5. Plain (`--plain`) - Tab-separated for scripting

### Decision 6: Profile-Based Configuration
**Choice**: Named profiles in TOML config file  
**Why**:
- Users may have multiple API keys (personal, work, testing)
- Profiles allow switching contexts easily
- Environment variable override for CI/CD pipelines
- Similar UX to AWS CLI profiles

**Config structure**:
```toml
default_profile = "personal"

[profiles.personal]
api_key = "aa_key_xxx"

[profiles.work]
api_key = "aa_key_yyy"
```

**Precedence** (highest to lowest):
1. `AA_API_KEY` environment variable
2. `--profile <name>` flag
3. `default_profile` from config
4. Error if none configured

### Decision 7: Configuration Location
**Choice**: XDG-compliant paths via `dirs` crate
- Config: `~/.config/aa/config.toml`
- Cache: `~/.cache/aa/`

**Why**: Follows platform conventions, `dirs` handles Windows/macOS differences

### Decision 8: Quota Tracking
**Choice**: Track quota from response headers, display on demand  
**Why**:
- API returns rate limit info in response headers
- Avoids wasting requests on quota-check-only calls
- Cached quota info is good enough for warnings

**Rate Limit Headers** (from API documentation):
- `X-RateLimit-Limit`: Maximum requests allowed in the time window
- `X-RateLimit-Remaining`: Number of requests remaining in current window
- `X-RateLimit-Reset`: Human-readable date string when the rate limit window resets

**Implementation**:
- Store last-known quota in profile-namespaced cache file: `~/.cache/aa/quota-{profile_name}.json`
- Update after each API call from response headers
- `aa quota` command displays cached info for active profile
- Warn when remaining < 10%
- If no quota data exists (first run), inform user to run a data command first

## Architecture

```
src/
├── main.rs              # Entry point, CLI parsing
├── lib.rs               # Library exports
├── cli/
│   ├── mod.rs           # CLI structure (clap)
│   └── args.rs          # Shared argument types
├── commands/
│   ├── mod.rs           # Command dispatcher
│   ├── llms.rs          # LLM models command
│   ├── text_to_image.rs
│   ├── image_editing.rs
│   ├── text_to_speech.rs
│   ├── text_to_video.rs
│   ├── image_to_video.rs
│   ├── profile.rs       # Profile management
│   ├── quota.rs         # Quota status
│   └── cache.rs
├── client/
│   ├── mod.rs           # API client
│   └── endpoints.rs     # Endpoint definitions
├── models/
│   ├── mod.rs
│   ├── llm.rs           # LLM data structures
│   ├── media.rs         # Media model structures
│   └── response.rs      # API response wrappers
├── output/
│   ├── mod.rs           # Output dispatcher
│   ├── markdown.rs      # Markdown table formatting (default)
│   ├── table.rs         # ASCII table formatting
│   ├── json.rs          # JSON output
│   └── csv.rs           # CSV output
├── cache.rs             # Caching layer
├── config.rs            # Profile/configuration management
└── error.rs             # Error types
```

## API Endpoints Mapping

| CLI Command        | API Endpoint                             |
|--------------------|------------------------------------------|
| `aa llms`          | GET /api/v2/data/llms/models             |
| `aa text-to-image` | GET /api/v2/data/media/text-to-image     |
| `aa image-editing` | GET /api/v2/data/media/image-editing     |
| `aa text-to-speech`| GET /api/v2/data/media/text-to-speech    |
| `aa text-to-video` | GET /api/v2/data/media/text-to-video     |
| `aa image-to-video`| GET /api/v2/data/media/image-to-video    |
| `aa quota`         | (uses cached headers, no API call)       |

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| API changes breaking CLI | Version API client, handle unknown fields gracefully with `#[serde(flatten)]` |
| Rate limit exhaustion | Aggressive caching, quota warnings, clear rate limit messaging |
| Large response sizes | Stream parsing if needed, pagination support |
| Cross-platform path issues | Use `dirs` crate consistently |
| Quota tracking accuracy | Clearly indicate "last updated" time, offer `--refresh` |

## Migration Plan

N/A - This is a new project with no existing users.

## Open Questions

1. **Binary name**: `aa` is short but may conflict. Consider `aanalysis` or `artificial-analysis` as fallback.
2. **Completion scripts**: Ship pre-generated or generate on install?
3. **Update notifications**: Should CLI check for newer versions? (Privacy consideration)
