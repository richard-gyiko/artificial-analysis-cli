//! Data source abstraction layer.
//!
//! This module provides the structure for multiple data sources:
//! - `artificial_analysis`: Primary source for benchmarks and performance metrics
//! - `models_dev`: Secondary source for capability metadata

pub mod artificial_analysis;
pub mod models_dev;

pub use artificial_analysis::AaClient;
pub use models_dev::ModelsDevClient;
