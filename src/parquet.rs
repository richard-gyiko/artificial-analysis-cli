//! Parquet serialization for cached data.
//!
//! Flattens nested model structures into flat rows suitable for SQL queries.

use crate::error::{AppError, Result};
use crate::models::{LlmModel, MediaModel};
use crate::schema;
use duckdb::{params, Connection};
use std::path::Path;

/// Flattened LLM model row for Parquet storage.
#[derive(Debug)]
pub struct LlmRow {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub creator: String,
    pub creator_slug: Option<String>,
    pub release_date: Option<String>,
    pub intelligence: Option<f64>,
    pub coding: Option<f64>,
    pub math: Option<f64>,
    pub mmlu_pro: Option<f64>,
    pub gpqa: Option<f64>,
    pub input_price: Option<f64>,
    pub output_price: Option<f64>,
    pub price: Option<f64>,
    pub tps: Option<f64>,
    pub latency: Option<f64>,
}

impl From<&LlmModel> for LlmRow {
    fn from(model: &LlmModel) -> Self {
        let evaluations = model.evaluations.as_ref();
        let pricing = model.pricing.as_ref();

        Self {
            id: model.id.clone(),
            name: model.name.clone(),
            slug: model.slug.clone(),
            creator: model.model_creator.name.clone(),
            creator_slug: model.model_creator.slug.clone(),
            release_date: model.release_date.clone(),
            intelligence: evaluations.and_then(|e| e.artificial_analysis_intelligence_index),
            coding: evaluations.and_then(|e| e.artificial_analysis_coding_index),
            math: evaluations.and_then(|e| e.artificial_analysis_math_index),
            mmlu_pro: evaluations.and_then(|e| e.mmlu_pro),
            gpqa: evaluations.and_then(|e| e.gpqa),
            input_price: pricing.and_then(|p| p.price_1m_input_tokens),
            output_price: pricing.and_then(|p| p.price_1m_output_tokens),
            price: pricing.and_then(|p| p.price_1m_blended_3_to_1),
            tps: model.median_output_tokens_per_second,
            latency: model.median_time_to_first_token_seconds,
        }
    }
}

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

/// Write LLM models to a Parquet file.
pub fn write_llms_parquet(models: &[LlmModel], path: &Path) -> Result<()> {
    let conn = Connection::open_in_memory()
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    // Create table using centralized schema
    conn.execute(&schema::LLMS.create_table_sql(), [])
        .map_err(|e| AppError::Cache(format!("DuckDB error: {}", e)))?;

    // Use Appender for bulk insert performance
    {
        let mut appender = conn
            .appender(schema::LLMS.name)
            .map_err(|e| AppError::Cache(format!("DuckDB appender error: {}", e)))?;

        for model in models {
            let row = LlmRow::from(model);
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
                    row.input_price,
                    row.output_price,
                    row.price,
                    row.tps,
                    row.latency,
                ])
                .map_err(|e| AppError::Cache(format!("DuckDB append error: {}", e)))?;
        }
        // Appender is flushed on drop
    }

    // Export to Parquet
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

    // Create table with flattened schema (using "media" as temp table name)
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

    // Use Appender for bulk insert performance
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
        // Appender is flushed on drop
    }

    // Export to Parquet
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
    use crate::models::{Evaluations, ModelCreator, Pricing};
    use tempfile::TempDir;

    fn make_llm_model(name: &str, creator: &str, intelligence: Option<f64>) -> LlmModel {
        LlmModel {
            id: format!("id-{}", name),
            name: name.to_string(),
            slug: name.to_lowercase().replace(' ', "-"),
            release_date: Some("2025-01".to_string()),
            model_creator: ModelCreator {
                id: format!("creator-{}", creator),
                name: creator.to_string(),
                slug: Some(creator.to_lowercase()),
                extra: serde_json::Value::Null,
            },
            evaluations: intelligence.map(|i| Evaluations {
                artificial_analysis_intelligence_index: Some(i),
                artificial_analysis_coding_index: Some(i * 0.9),
                artificial_analysis_math_index: Some(i * 0.8),
                mmlu_pro: Some(0.75),
                gpqa: Some(0.70),
                hle: None,
                livecodebench: None,
                scicode: None,
                math_500: None,
                aime: None,
                extra: serde_json::Value::Null,
            }),
            pricing: Some(Pricing {
                price_1m_blended_3_to_1: Some(2.0),
                price_1m_input_tokens: Some(1.0),
                price_1m_output_tokens: Some(5.0),
                extra: serde_json::Value::Null,
            }),
            median_output_tokens_per_second: Some(150.0),
            median_time_to_first_token_seconds: Some(0.5),
            median_time_to_first_answer_token: None,
            extra: serde_json::Value::Null,
        }
    }

    #[test]
    fn test_llm_row_from_model() {
        let model = make_llm_model("GPT-4o", "OpenAI", Some(55.0));
        let row = LlmRow::from(&model);

        assert_eq!(row.name, "GPT-4o");
        assert_eq!(row.creator, "OpenAI");
        assert_eq!(row.intelligence, Some(55.0));
        assert_eq!(row.input_price, Some(1.0));
        assert_eq!(row.tps, Some(150.0));
    }

    #[test]
    fn test_write_llms_parquet() {
        let temp_dir = TempDir::new().unwrap();
        let parquet_path = temp_dir.path().join("llms.parquet");

        let models = vec![
            make_llm_model("GPT-4o", "OpenAI", Some(55.0)),
            make_llm_model("Claude 3.5", "Anthropic", Some(52.0)),
        ];

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
        assert_eq!(count, 2);
    }
}
