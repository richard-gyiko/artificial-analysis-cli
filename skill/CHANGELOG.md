# Changelog

All notable changes to this skill will be documented in this file.

## [1.1.0] - 2026-01-19

### Changed
- Recalibrated skill thresholds for Intelligence Index v4.0
  - Transformational: 30 → 20
  - Analytical: 45 → 38
  - Tool-using: 40/40 → 35/35
  - Agentic: 55/45 → 48/42
- Updated score interpretation tiers in BENCHMARKS.md
- Added calibration notes explaining the v4.0 scale (SOTA ~50)

### Fixed
- Agentic threshold (55) was impossible to meet - no model qualified

## [1.0.0] - 2026-01-19

### Added
- Initial skill release
- Task classification into 4 skill types
- Benchmark-based model selection via `aa` CLI
- Reference documentation for benchmarks, queries, and multi-model patterns
