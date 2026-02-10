//! LLM model from Artificial Analysis.
//!
//! This struct contains only AA data (benchmarks, performance, pricing).
//! For capability data (tool_call, reasoning, context_window, etc.),
//! query the `providers` table from models.dev.

use serde::{Deserialize, Serialize};

/// LLM Model from Artificial Analysis.
///
/// Contains benchmark scores, performance metrics, and canonical pricing.
/// For capability data, query the `providers` table.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LlmModel {
    // === Core Identity ===
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub release_date: Option<String>,
    pub creator: String,
    #[serde(default)]
    pub creator_slug: Option<String>,

    // === Benchmarks ===
    #[serde(default)]
    pub intelligence: Option<f64>,
    #[serde(default)]
    pub coding: Option<f64>,
    #[serde(default)]
    pub math: Option<f64>,
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

    // === Pricing (per million tokens, USD) ===
    #[serde(default)]
    pub input_price: Option<f64>,
    #[serde(default)]
    pub output_price: Option<f64>,
    #[serde(default)]
    pub price: Option<f64>,

    // === Performance ===
    #[serde(default)]
    pub tps: Option<f64>,
    #[serde(default)]
    pub latency: Option<f64>,
}

impl LlmModel {
    /// Get the display name.
    pub fn display_name(&self) -> &str {
        &self.name
    }

    /// Get creator name.
    pub fn creator_name(&self) -> &str {
        &self.creator
    }

    /// Get intelligence index.
    pub fn get_intelligence(&self) -> Option<f64> {
        self.intelligence
    }

    /// Get input token price (per million).
    pub fn get_input_price(&self) -> Option<f64> {
        self.input_price
    }

    /// Get output token price (per million).
    pub fn get_output_price(&self) -> Option<f64> {
        self.output_price
    }

    /// Get blended token price (per million).
    pub fn get_blended_price(&self) -> Option<f64> {
        self.price
    }

    /// Get tokens per second.
    pub fn get_tps(&self) -> Option<f64> {
        self.tps
    }
}

// Backward compatibility: implement the old accessor methods
impl LlmModel {
    /// Get intelligence index (backward compat).
    pub fn intelligence(&self) -> Option<f64> {
        self.intelligence
    }

    /// Get input token price (backward compat).
    pub fn input_price(&self) -> Option<f64> {
        self.input_price
    }

    /// Get output token price (backward compat).
    pub fn output_price(&self) -> Option<f64> {
        self.output_price
    }

    /// Get blended token price (backward compat).
    pub fn blended_price(&self) -> Option<f64> {
        self.price
    }

    /// Get tokens per second (backward compat).
    pub fn tps(&self) -> Option<f64> {
        self.tps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_model() -> LlmModel {
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
        }
    }

    #[test]
    fn test_basic_accessors() {
        let model = make_test_model();
        assert_eq!(model.display_name(), "GPT-4o");
        assert_eq!(model.creator_name(), "OpenAI");
        assert_eq!(model.intelligence(), Some(55.0));
        assert_eq!(model.input_price(), Some(2.5));
        assert_eq!(model.tps(), Some(150.0));
    }

    #[test]
    fn test_serialization() {
        let model = make_test_model();
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: LlmModel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, model.name);
        assert_eq!(deserialized.intelligence, model.intelligence);
    }
}
