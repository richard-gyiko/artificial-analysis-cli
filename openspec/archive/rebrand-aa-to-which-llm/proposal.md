# Change: Rebrand CLI from `aa` to `which-llm`

## Why

The current CLI name `aa` (Artificial Analysis) is cryptic and doesn't convey the tool's purpose. Rebranding to `which-llm` aligns with the existing skill name and clearly communicates the CLI's value proposition: helping users decide which LLM to use for their task.

## What Changes

- **BREAKING**: Binary name changes from `aa` to `which-llm`
- **BREAKING**: Environment variable `AA_API_KEY` → `ARTIFICIAL_ANALYSIS_API_KEY`
- **BREAKING**: Environment variable `AA_CONFIG_DIR` → `WHICH_LLM_CONFIG_DIR`
- **BREAKING**: Config directory `~/.config/aa/` → `~/.config/which-llm/`
- **BREAKING**: Cache directory `~/.cache/aa/` → `~/.cache/which-llm/`
- Package name in Cargo.toml: `aa` → `which-llm`
- Update all documentation, error messages, and examples
- Update Homebrew/Scoop installation instructions

## Impact

- Affected specs: `cli` (all commands reference `aa`)
- Affected code:
  - `Cargo.toml` - package name
  - `src/config.rs` - config paths, env vars
  - `src/cache.rs` - cache paths
  - `src/error.rs` - error messages
  - `tests/cli.rs` - test env vars
  - `README.md` - all documentation

## Migration

Users with existing `aa` configuration will need to:
1. Rename `~/.config/aa/` to `~/.config/which-llm/`
2. Rename `~/.cache/aa/` to `~/.cache/which-llm/`
3. Update any scripts using `AA_API_KEY` to `ARTIFICIAL_ANALYSIS_API_KEY`
4. Update Homebrew/Scoop: `brew uninstall aa && brew install which-llm`
