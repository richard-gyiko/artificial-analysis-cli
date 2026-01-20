//! Parquet schema for models.dev raw data.

use crate::schema::{Column, TableDef};

/// Raw models.dev table schema (internal use only).
pub const MODELS_DEV: TableDef = TableDef {
    name: "models_dev",
    command: "", // Internal table, not user-facing
    parquet_file: "models_dev.parquet",
    columns: &[
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
        // Cost
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
