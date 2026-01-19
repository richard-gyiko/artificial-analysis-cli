//! Parquet serialization for cached data.
//!
//! Writes data to Parquet files using DuckDB.

use crate::error::{AppError, Result};
use crate::models::{LlmModel, MediaModel};
use crate::schema;
use crate::sources::artificial_analysis::models::AaLlmRow;
use crate::sources::artificial_analysis::schema::AA_LLMS;
use crate::sources::models_dev::models::ModelsDevRow;
use crate::sources::models_dev::schema::MODELS_DEV;
use duckdb::{params, Connection};
use std::path::Path;

/// Flattened media model row for Parquet storage.
#[derive(Debug)]
pub struct MediaRow {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub creator: String,
    pub elo: Option<f64>,
    pub rank: Option<i32>,
    pub release_date: Option<String>,
}

impl From<&MediaModel> for MediaRow {
    fn from(model: &MediaModel) -> Self {
        Self {
            id: model.id.clone(),
            name: model.name.clone(),
            slug: model.slug.clone(),
            creator: model.model_creator.name.clone(),
            elo: model.elo,
            rank: model.rank.map(|r| r as i32),
            release_date: model.release_date.clone(),
        }
    }
}

/// Write raw AA LLM data to Parquet.
pub fn write_aa_llms_parquet(rows: &[AaLlmRow], path: &Path) -> Result<()> {
    let conn = Connection::open_in_memory()
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    conn.execute(&AA_LLMS.create_table_sql(), [])
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    {
        let mut appender = conn
            .appender(AA_LLMS.name)
            .map_err(|e| AppError::Cache(format!("DuckDB appender error: {}", e)))?;

        for row in rows {
            appender
                .append_row(params![
                    row.id,
                    row.name,
                    row.slug,
                    row.creator,
                    row.creator_slug,
                    row.release_date,
                    row.intelligence,
                    row.coding,
                    row.math,
                    row.mmlu_pro,
                    row.gpqa,
                    row.hle,
                    row.livecodebench,
                    row.scicode,
                    row.math_500,
                    row.aime,
                    row.input_price,
                    row.output_price,
                    row.price,
                    row.tps,
                    row.latency,
                ])
                .map_err(|e| AppError::Cache(format!("DuckDB append error: {}", e)))?;
        }
    }

    let path_str = path.to_string_lossy();
    conn.execute(
        &format!("COPY {} TO '{}' (FORMAT PARQUET)", AA_LLMS.name, path_str),
        [],
    )
    .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    Ok(())
}

/// Write raw models.dev data to Parquet.
pub fn write_models_dev_parquet(rows: &[ModelsDevRow], path: &Path) -> Result<()> {
    let conn = Connection::open_in_memory()
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    conn.execute(&MODELS_DEV.create_table_sql(), [])
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    {
        let mut appender = conn
            .appender(MODELS_DEV.name)
            .map_err(|e| AppError::Cache(format!("DuckDB appender error: {}", e)))?;

        for row in rows {
            appender
                .append_row(params![
                    row.provider_id,
                    row.provider_name,
                    row.model_id,
                    row.model_name,
                    row.family,
                    row.attachment,
                    row.reasoning,
                    row.tool_call,
                    row.structured_output,
                    row.temperature,
                    row.knowledge,
                    row.release_date,
                    row.last_updated,
                    row.open_weights,
                    row.status,
                    row.context_window.map(|v| v as i64),
                    row.max_input_tokens.map(|v| v as i64),
                    row.max_output_tokens.map(|v| v as i64),
                    row.cost_input,
                    row.cost_output,
                    row.input_modalities,
                    row.output_modalities,
                ])
                .map_err(|e| AppError::Cache(format!("DuckDB append error: {}", e)))?;
        }
    }

    let path_str = path.to_string_lossy();
    conn.execute(
        &format!(
            "COPY {} TO '{}' (FORMAT PARQUET)",
            MODELS_DEV.name, path_str
        ),
        [],
    )
    .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    Ok(())
}

/// Write merged LLM data to Parquet.
pub fn write_llms_parquet(models: &[LlmModel], path: &Path) -> Result<()> {
    let conn = Connection::open_in_memory()
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    conn.execute(&schema::LLMS.create_table_sql(), [])
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    {
        let mut appender = conn
            .appender(schema::LLMS.name)
            .map_err(|e| AppError::Cache(format!("DuckDB appender error: {}", e)))?;

        for model in models {
            appender
                .append_row(params![
                    model.id,
                    model.name,
                    model.slug,
                    model.creator,
                    model.creator_slug,
                    model.release_date,
                    model.intelligence,
                    model.coding,
                    model.math,
                    model.mmlu_pro,
                    model.gpqa,
                    model.hle,
                    model.livecodebench,
                    model.scicode,
                    model.math_500,
                    model.aime,
                    model.input_price,
                    model.output_price,
                    model.price,
                    model.tps,
                    model.latency,
                    model.reasoning,
                    model.tool_call,
                    model.structured_output,
                    model.attachment,
                    model.temperature,
                    model.context_window.map(|v| v as i64),
                    model.max_input_tokens.map(|v| v as i64),
                    model.max_output_tokens.map(|v| v as i64),
                    model.input_modalities.as_ref().map(|v| v.join(",")),
                    model.output_modalities.as_ref().map(|v| v.join(",")),
                    model.knowledge_cutoff,
                    model.open_weights,
                    model.last_updated,
                    model.models_dev_matched,
                ])
                .map_err(|e| AppError::Cache(format!("DuckDB append error: {}", e)))?;
        }
    }

    let path_str = path.to_string_lossy();
    conn.execute(
        &format!(
            "COPY {} TO '{}' (FORMAT PARQUET)",
            schema::LLMS.name,
            path_str
        ),
        [],
    )
    .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    Ok(())
}

/// Write media models to a Parquet file.
pub fn write_media_parquet(models: &[MediaModel], path: &Path) -> Result<()> {
    let conn = Connection::open_in_memory()
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    conn.execute(
        "CREATE TABLE media (
            id VARCHAR NOT NULL,
            name VARCHAR NOT NULL,
            slug VARCHAR NOT NULL,
            creator VARCHAR NOT NULL,
            elo DOUBLE,
            rank INTEGER,
            release_date VARCHAR
        )",
        [],
    )
    .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    {
        let mut appender = conn
            .appender("media")
            .map_err(|e| AppError::Cache(format!("DuckDB appender error: {}", e)))?;

        for model in models {
            let row = MediaRow::from(model);
            appender
                .append_row(params![
                    row.id,
                    row.name,
                    row.slug,
                    row.creator,
                    row.elo,
                    row.rank,
                    row.release_date,
                ])
                .map_err(|e| AppError::Cache(format!("DuckDB append error: {}", e)))?;
        }
    }

    let path_str = path.to_string_lossy();
    conn.execute(
        &format!("COPY media TO '{}' (FORMAT PARQUET)", path_str),
        [],
    )
    .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_test_llm_model() -> LlmModel {
        LlmModel {
            id: "test-id".to_string(),
            name: "GPT-4o".to_string(),
            slug: "gpt-4o".to_string(),
            release_date: Some("2024-05-13".to_string()),
            creator: "OpenAI".to_string(),
            creator_slug: Some("openai".to_string()),
            intelligence: Some(55.0),
            coding: Some(50.0),
            math: Some(60.0),
            mmlu_pro: Some(0.75),
            gpqa: Some(0.70),
            hle: None,
            livecodebench: None,
            scicode: None,
            math_500: None,
            aime: None,
            input_price: Some(2.5),
            output_price: Some(10.0),
            price: Some(5.0),
            tps: Some(150.0),
            latency: Some(0.5),
            reasoning: Some(false),
            tool_call: Some(true),
            structured_output: Some(true),
            attachment: Some(true),
            temperature: Some(true),
            context_window: Some(128000),
            max_input_tokens: None,
            max_output_tokens: Some(16384),
            input_modalities: Some(vec!["text".to_string(), "image".to_string()]),
            output_modalities: Some(vec!["text".to_string()]),
            knowledge_cutoff: Some("2024-04".to_string()),
            open_weights: Some(false),
            last_updated: Some("2024-11-20".to_string()),
            models_dev_matched: true,
        }
    }

    #[test]
    fn test_write_llms_parquet() {
        let temp_dir = TempDir::new().unwrap();
        let parquet_path = temp_dir.path().join("llms.parquet");

        let models = vec![make_test_llm_model()];
        write_llms_parquet(&models, &parquet_path).unwrap();

        assert!(parquet_path.exists());

        // Verify we can read it back
        let conn = Connection::open_in_memory().unwrap();
        let count: i64 = conn
            .query_row(
                &format!(
                    "SELECT COUNT(*) FROM read_parquet('{}')",
                    parquet_path.to_string_lossy()
                ),
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        // Verify capability columns exist
        let tool_call: Option<bool> = conn
            .query_row(
                &format!(
                    "SELECT tool_call FROM read_parquet('{}')",
                    parquet_path.to_string_lossy()
                ),
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(tool_call, Some(true));
    }

    #[test]
    fn test_write_models_dev_parquet() {
        let temp_dir = TempDir::new().unwrap();
        let parquet_path = temp_dir.path().join("models_dev.parquet");

        let rows = vec![ModelsDevRow {
            provider_id: "openai".to_string(),
            provider_name: "OpenAI".to_string(),
            model_id: "gpt-4o".to_string(),
            model_name: "GPT-4o".to_string(),
            family: Some("gpt-4".to_string()),
            attachment: Some(true),
            reasoning: Some(false),
            tool_call: Some(true),
            structured_output: Some(true),
            temperature: Some(true),
            knowledge: Some("2024-04".to_string()),
            release_date: Some("2024-05-13".to_string()),
            last_updated: Some("2024-11-20".to_string()),
            open_weights: Some(false),
            status: None,
            context_window: Some(128000),
            max_input_tokens: None,
            max_output_tokens: Some(16384),
            cost_input: Some(2.5),
            cost_output: Some(10.0),
            input_modalities: Some("text,image".to_string()),
            output_modalities: Some("text".to_string()),
        }];

        write_models_dev_parquet(&rows, &parquet_path).unwrap();
        assert!(parquet_path.exists());

        let conn = Connection::open_in_memory().unwrap();
        let count: i64 = conn
            .query_row(
                &format!(
                    "SELECT COUNT(*) FROM read_parquet('{}')",
                    parquet_path.to_string_lossy()
                ),
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }
}
