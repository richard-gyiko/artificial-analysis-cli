//! Parquet schema for models data (from models.dev).

use crate::schema::{Column, TableDef};

/// Models table schema - models with capabilities from models.dev.
/// Each row represents a model available from a specific provider with
/// capabilities, context limits, and provider-specific pricing.
pub const MODELS: TableDef = TableDef {
    name: "models",
    command: "which-llm refresh",
    parquet_file: "models.parquet",
    columns: &[
        // Provider identity
        Column {
            name: "provider_id",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "provider_name",
            sql_type: "VARCHAR",
            nullable: false,
        },
        // Provider metadata
        Column {
            name: "provider_env",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "provider_npm",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "provider_api",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "provider_doc",
            sql_type: "VARCHAR",
            nullable: true,
        },
        // Model identity
        Column {
            name: "model_id",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "model_name",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "family",
            sql_type: "VARCHAR",
            nullable: true,
        },
        // Capabilities
        Column {
            name: "attachment",
            sql_type: "BOOLEAN",
            nullable: true,
        },
        Column {
            name: "reasoning",
            sql_type: "BOOLEAN",
            nullable: true,
        },
        Column {
            name: "tool_call",
            sql_type: "BOOLEAN",
            nullable: true,
        },
        Column {
            name: "structured_output",
            sql_type: "BOOLEAN",
            nullable: true,
        },
        Column {
            name: "temperature",
            sql_type: "BOOLEAN",
            nullable: true,
        },
        // Metadata
        Column {
            name: "knowledge",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "release_date",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "last_updated",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "open_weights",
            sql_type: "BOOLEAN",
            nullable: true,
        },
        Column {
            name: "status",
            sql_type: "VARCHAR",
            nullable: true,
        },
        // Limits
        Column {
            name: "context_window",
            sql_type: "BIGINT",
            nullable: true,
        },
        Column {
            name: "max_input_tokens",
            sql_type: "BIGINT",
            nullable: true,
        },
        Column {
            name: "max_output_tokens",
            sql_type: "BIGINT",
            nullable: true,
        },
        // Cost (per million tokens)
        Column {
            name: "cost_input",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "cost_output",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "cost_cache_read",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "cost_cache_write",
            sql_type: "DOUBLE",
            nullable: true,
        },
        // Modalities (comma-separated)
        Column {
            name: "input_modalities",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "output_modalities",
            sql_type: "VARCHAR",
            nullable: true,
        },
    ],
};
