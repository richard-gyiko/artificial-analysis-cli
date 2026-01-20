//! Parquet schema for Artificial Analysis raw data.

use crate::schema::{Column, TableDef};

/// Raw AA LLMs table schema (internal use only).
pub const AA_LLMS: TableDef = TableDef {
    name: "aa_llms",
    command: "", // Internal table, not user-facing
    parquet_file: "aa_llms.parquet",
    columns: &[
        Column {
            name: "id",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "name",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "slug",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "creator",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "creator_slug",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "release_date",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "intelligence",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "coding",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "math",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "mmlu_pro",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "gpqa",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "hle",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "livecodebench",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "scicode",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "math_500",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "aime",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "input_price",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "output_price",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "price",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "tps",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "latency",
            sql_type: "DOUBLE",
            nullable: true,
        },
    ],
};
