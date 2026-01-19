//! Merge orchestration for combining data from multiple sources.
//!
//! This module handles:
//! - Matching models between AA and models.dev
//! - Combining fields from both sources
//! - Building the merged view

mod combiner;
mod matcher;

pub use combiner::merge_models;
pub use matcher::{find_match, normalize_provider, strip_version_suffix, MatchResult};
