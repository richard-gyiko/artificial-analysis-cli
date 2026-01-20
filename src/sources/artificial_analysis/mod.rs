//! Artificial Analysis data source.
//!
//! Contains the AA API client, raw data models, and Parquet schema for AA data.

mod client;
pub mod models;
pub mod schema;

pub use client::AaClient;
