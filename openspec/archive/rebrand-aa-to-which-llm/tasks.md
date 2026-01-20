# Tasks: Rebrand CLI from `aa` to `which-llm`

## 1. Core Configuration
- [x] 1.1 Update `Cargo.toml` package name from `aa` to `which-llm`
- [x] 1.2 Update `src/config.rs` - change `AA_CONFIG_DIR` to `WHICH_LLM_CONFIG_DIR`
- [x] 1.3 Update `src/config.rs` - change config path from `aa` to `which-llm`
- [x] 1.4 Update `src/config.rs` - change `AA_API_KEY` to `ARTIFICIAL_ANALYSIS_API_KEY`
- [x] 1.5 Update `src/cache.rs` - change cache directory from `aa` to `which-llm`

## 2. Error Messages & User-Facing Text
- [x] 2.1 Update `src/error.rs` - change error messages referencing `aa`
- [x] 2.2 Update CLI help text and user-agent strings
- [x] 2.3 Update any other user-facing strings referencing `aa`

## 3. Tests
- [x] 3.1 Update `tests/cli.rs` - change `AA_API_KEY` references
- [x] 3.2 Run full test suite to verify changes

## 4. Documentation
- [x] 4.1 Update `README.md` - all command examples from `aa` to `which-llm`
- [x] 4.2 Update `README.md` - installation instructions (Homebrew, Scoop, manual)
- [x] 4.3 Update `README.md` - environment variable documentation

## 5. Build & Release (Post-Merge)
- [x] 5.1 Update GitHub Actions workflows if they reference binary name
- [x] 5.2 Update Homebrew formula (external repo: richard-gyiko/homebrew-tap)
- [x] 5.3 Update Scoop manifest (external repo: richard-gyiko/scoop-bucket)
- [ ] 5.4 Verify release artifacts use new binary name

## 6. Verification
- [x] 6.1 Build and test binary locally
- [x] 6.2 Verify `which-llm --help` works
- [x] 6.3 Verify config file creation at `~/.config/which-llm/`
- [x] 6.4 Verify `ARTIFICIAL_ANALYSIS_API_KEY` env var works
