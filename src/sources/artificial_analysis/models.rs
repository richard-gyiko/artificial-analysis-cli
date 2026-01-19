//! Raw data models for Artificial Analysis API responses.
//!
//! These are the unprocessed structures directly from the AA API.

use serde::{Deserialize, Serialize};

/// Raw LLM Model from the AA API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AaLlmModel {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub release_date: Option<String>,
    pub model_creator: AaModelCreator,
    #[serde(default)]
    pub evaluations: Option<AaEvaluations>,
    #[serde(default)]
    pub pricing: Option<AaPricing>,
    /// Output generation speed (tokens per second)
    #[serde(default)]
    pub median_output_tokens_per_second: Option<f64>,
    /// Time to first token (seconds)
    #[serde(default)]
    pub median_time_to_first_token_seconds: Option<f64>,
    /// Time to first answer token (seconds)
    #[serde(default)]
    pub median_time_to_first_answer_token: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Model creator information from AA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AaModelCreator {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Model evaluations/benchmarks from AA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AaEvaluations {
    #[serde(default)]
    pub artificial_analysis_intelligence_index: Option<f64>,
    #[serde(default)]
    pub artificial_analysis_coding_index: Option<f64>,
    #[serde(default)]
    pub artificial_analysis_math_index: Option<f64>,
    #[serde(default)]
    pub mmlu_pro: Option<f64>,
    #[serde(default)]
    pub gpqa: Option<f64>,
    #[serde(default)]
    pub hle: Option<f64>,
    #[serde(default)]
    pub livecodebench: Option<f64>,
    #[serde(default)]
    pub scicode: Option<f64>,
    #[serde(default)]
    pub math_500: Option<f64>,
    #[serde(default)]
    pub aime: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Pricing information from AA (per million tokens, USD).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AaPricing {
    /// Blended price (3:1 input:output ratio)
    #[serde(default)]
    pub price_1m_blended_3_to_1: Option<f64>,
    /// Input token price per million
    #[serde(default)]
    pub price_1m_input_tokens: Option<f64>,
    /// Output token price per million
    #[serde(default)]
    pub price_1m_output_tokens: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Flattened AA LLM row for Parquet storage.
#[derive(Debug, Clone)]
pub struct AaLlmRow {
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
    pub hle: Option<f64>,
    pub livecodebench: Option<f64>,
    pub scicode: Option<f64>,
    pub math_500: Option<f64>,
    pub aime: Option<f64>,
    pub input_price: Option<f64>,
    pub output_price: Option<f64>,
    pub price: Option<f64>,
    pub tps: Option<f64>,
    pub latency: Option<f64>,
}

impl From<&AaLlmModel> for AaLlmRow {
    fn from(model: &AaLlmModel) -> Self {
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
            hle: evaluations.and_then(|e| e.hle),
            livecodebench: evaluations.and_then(|e| e.livecodebench),
            scicode: evaluations.and_then(|e| e.scicode),
            math_500: evaluations.and_then(|e| e.math_500),
            aime: evaluations.and_then(|e| e.aime),
            input_price: pricing.and_then(|p| p.price_1m_input_tokens),
            output_price: pricing.and_then(|p| p.price_1m_output_tokens),
            price: pricing.and_then(|p| p.price_1m_blended_3_to_1),
            tps: model.median_output_tokens_per_second,
            latency: model.median_time_to_first_token_seconds,
        }
    }
}

impl AaLlmModel {
    /// Get intelligence index.
    pub fn intelligence(&self) -> Option<f64> {
        self.evaluations
            .as_ref()?
            .artificial_analysis_intelligence_index
    }

    /// Get input token price (per million).
    pub fn input_price(&self) -> Option<f64> {
        self.pricing.as_ref()?.price_1m_input_tokens
    }

    /// Get output token price (per million).
    pub fn output_price(&self) -> Option<f64> {
        self.pricing.as_ref()?.price_1m_output_tokens
    }

    /// Get blended token price (per million).
    pub fn blended_price(&self) -> Option<f64> {
        self.pricing.as_ref()?.price_1m_blended_3_to_1
    }

    /// Get tokens per second.
    pub fn tps(&self) -> Option<f64> {
        self.median_output_tokens_per_second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aa_llm_model_deserialization() {
        let json = r#"{
            "id": "2dad8957-4c16-4e74-bf2d-8b21514e0ae9",
            "name": "o3-mini",
            "slug": "o3-mini",
            "model_creator": {
                "id": "e67e56e3-15cd-43db-b679-da4660a69f41",
                "name": "OpenAI",
                "slug": "openai"
            },
            "evaluations": {
                "artificial_analysis_intelligence_index": 62.9,
                "artificial_analysis_coding_index": 55.8,
                "artificial_analysis_math_index": 87.2,
                "mmlu_pro": 0.791,
                "gpqa": 0.748
            },
            "pricing": {
                "price_1m_blended_3_to_1": 1.925,
                "price_1m_input_tokens": 1.1,
                "price_1m_output_tokens": 4.4
            },
            "median_output_tokens_per_second": 153.831,
            "median_time_to_first_token_seconds": 14.939
        }"#;

        let model: AaLlmModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, "2dad8957-4c16-4e74-bf2d-8b21514e0ae9");
        assert_eq!(model.name, "o3-mini");
        assert_eq!(model.slug, "o3-mini");
        assert_eq!(model.model_creator.name, "OpenAI");
        assert_eq!(model.intelligence(), Some(62.9));
        assert_eq!(model.input_price(), Some(1.1));
        assert_eq!(model.output_price(), Some(4.4));
        assert_eq!(model.tps(), Some(153.831));
    }

    #[test]
    fn test_aa_llm_row_from_model() {
        let json = r#"{
            "id": "test-uuid",
            "name": "Test Model",
            "slug": "test-model",
            "model_creator": {
                "id": "creator-uuid",
                "name": "Test Corp",
                "slug": "testcorp"
            },
            "evaluations": {
                "artificial_analysis_intelligence_index": 50.0
            },
            "pricing": {
                "price_1m_input_tokens": 1.0,
                "price_1m_output_tokens": 2.0
            }
        }"#;

        let model: AaLlmModel = serde_json::from_str(json).unwrap();
        let row = AaLlmRow::from(&model);

        assert_eq!(row.id, "test-uuid");
        assert_eq!(row.name, "Test Model");
        assert_eq!(row.creator, "Test Corp");
        assert_eq!(row.creator_slug, Some("testcorp".to_string()));
        assert_eq!(row.intelligence, Some(50.0));
        assert_eq!(row.input_price, Some(1.0));
    }
}
