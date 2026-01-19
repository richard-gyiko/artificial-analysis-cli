//! Data models for LLM responses.

use serde::{Deserialize, Serialize};

/// LLM Model from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmModel {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub release_date: Option<String>,
    pub model_creator: ModelCreator,
    #[serde(default)]
    pub evaluations: Option<Evaluations>,
    #[serde(default)]
    pub pricing: Option<Pricing>,
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

/// Model creator information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCreator {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Model evaluations/benchmarks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evaluations {
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

/// Pricing information (per million tokens, USD).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
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

impl LlmModel {
    /// Get the display name.
    pub fn display_name(&self) -> &str {
        &self.name
    }

    /// Get intelligence index or default.
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

    /// Get creator name.
    pub fn creator_name(&self) -> &str {
        &self.model_creator.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_model_deserialization() {
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

        let model: LlmModel = serde_json::from_str(json).unwrap();
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
    fn test_llm_model_with_missing_fields() {
        let json = r#"{
            "id": "test-uuid",
            "name": "Test Model",
            "slug": "test-model",
            "model_creator": {
                "id": "creator-uuid",
                "name": "Test Corp"
            }
        }"#;

        let model: LlmModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, "test-uuid");
        assert!(model.evaluations.is_none());
        assert!(model.pricing.is_none());
        assert_eq!(model.intelligence(), None);
        assert_eq!(model.tps(), None);
    }

    #[test]
    fn test_display_name() {
        let json = r#"{
            "id": "uuid-1",
            "name": "GPT-4o",
            "slug": "gpt-4o",
            "model_creator": { "id": "openai-id", "name": "OpenAI", "slug": "openai" }
        }"#;

        let model: LlmModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.display_name(), "GPT-4o");
        assert_eq!(model.creator_name(), "OpenAI");
    }
}
