//! CLI integration tests.

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn cmd() -> Command {
    Command::cargo_bin("which-llm").unwrap()
}

/// Set up temp environment for config isolation (cross-platform)
fn cmd_with_temp_config(temp: &tempfile::TempDir) -> Command {
    let mut cmd = cmd();
    let config_dir = temp.path().join("config").join("which-llm");
    let cache_dir = temp.path().join("cache").join("which-llm");

    // Use WHICH_LLM_CONFIG_DIR and WHICH_LLM_CACHE_DIR for reliable isolation
    cmd.env("WHICH_LLM_CONFIG_DIR", &config_dir);
    cmd.env("WHICH_LLM_CACHE_DIR", &cache_dir);

    // Remove any existing API key
    cmd.env_remove("ARTIFICIAL_ANALYSIS_API_KEY");
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
fn test_refresh_api_mode_requires_api_key() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .arg("refresh")
        .arg("--use-api")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No API key configured"));
}

#[test]
fn test_query_help() {
    cmd()
        .arg("query")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("SQL"));
}

#[test]
fn test_tables_command() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .arg("tables")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available tables"))
        .stdout(predicate::str::contains("benchmarks"))
        .stdout(predicate::str::contains("text_to_image"))
        .stdout(predicate::str::contains("not cached"));
}

#[test]
fn test_query_no_sql() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .arg("query")
        .assert()
        .success()
        .stderr(predicate::str::contains("No SQL query provided"));
}

#[test]
fn test_query_missing_table() {
    let temp = tempfile::tempdir().unwrap();
    cmd_with_temp_config(&temp)
        .arg("query")
        .arg("SELECT * FROM benchmarks")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"))
        .stderr(predicate::str::contains("which-llm refresh"));
}
