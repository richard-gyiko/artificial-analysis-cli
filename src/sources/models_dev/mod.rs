//! models.dev data source.
//!
//! Contains the models.dev API client, raw data models, and Parquet schema.

mod client;
pub mod models;
pub mod schema;

pub use client::ModelsDevClient;
pub use schema::MODELS;
