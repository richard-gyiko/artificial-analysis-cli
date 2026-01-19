//! CLI integration tests.

use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("aa").unwrap()
}

/// Set up temp environment for config isolation (cross-platform)
fn cmd_with_temp_config(temp: &tempfile::TempDir) -> Command {
    let mut cmd = cmd();
    let config_dir = temp.path().join("config").join("aa");
    let cache_dir = temp.path().join("cache").join("aa");

    // Use AA_CONFIG_DIR and AA_CACHE_DIR for reliable isolation
    cmd.env("AA_CONFIG_DIR", &config_dir);
    cmd.env("AA_CACHE_DIR", &cache_dir);

    // Remove any existing API key
    cmd.env_remove("AA_API_KEY");
    cmd
}

#[test]
fn test_help() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLI for querying AI model benchmarks",
        ));
}

#[test]
fn test_version() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_profile_list_empty() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No profiles configured"));
}

#[test]
fn test_cache_status() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .arg("cache")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cache Status"));
}

#[test]
fn test_llms_requires_api_key() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .arg("llms")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No API key configured"));
}

#[test]
fn test_quota_no_data() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .env("AA_API_KEY", "test_key")
        .arg("quota")
        .assert()
        .success()
        .stdout(predicate::str::contains("No quota data available"));
}

#[test]
fn test_output_format_flags() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .arg("--json")
        .arg("--csv")
        .arg("llms")
        .assert()
        .failure();
}
