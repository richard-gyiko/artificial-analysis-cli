//! Unified LLM model that combines data from AA and models.dev.

use serde::{Deserialize, Serialize};

/// Unified LLM Model combining AA benchmarks with models.dev capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmModel {
    // === Core Identity (from AA) ===
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub release_date: Option<String>,
    pub creator: String,
    #[serde(default)]
    pub creator_slug: Option<String>,

    // === Benchmarks (from AA) ===
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

    // === Pricing (from AA, per million tokens, USD) ===
    #[serde(default)]
    pub input_price: Option<f64>,
    #[serde(default)]
    pub output_price: Option<f64>,
    #[serde(default)]
    pub price: Option<f64>,

    // === Performance (from AA) ===
    #[serde(default)]
    pub tps: Option<f64>,
    #[serde(default)]
    pub latency: Option<f64>,

    // === Capabilities (from models.dev) ===
    #[serde(default)]
    pub reasoning: Option<bool>,
    #[serde(default)]
    pub tool_call: Option<bool>,
    #[serde(default)]
    pub structured_output: Option<bool>,
    #[serde(default)]
    pub attachment: Option<bool>,
    #[serde(default)]
    pub temperature: Option<bool>,

    // === Limits (from models.dev) ===
    #[serde(default)]
    pub context_window: Option<u64>,
    #[serde(default)]
    pub max_input_tokens: Option<u64>,
    #[serde(default)]
    pub max_output_tokens: Option<u64>,

    // === Modalities (from models.dev) ===
    #[serde(default)]
    pub input_modalities: Option<Vec<String>>,
    #[serde(default)]
    pub output_modalities: Option<Vec<String>>,

    // === Additional Metadata (from models.dev) ===
    #[serde(default)]
    pub knowledge_cutoff: Option<String>,
    #[serde(default)]
    pub open_weights: Option<bool>,
    #[serde(default)]
    pub last_updated: Option<String>,

    // === Source Tracking ===
    /// Whether this model was matched to a models.dev entry
    #[serde(default)]
    pub models_dev_matched: bool,
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

    /// Check if model supports reasoning.
    pub fn supports_reasoning(&self) -> Option<bool> {
        self.reasoning
    }

    /// Check if model supports tool calling.
    pub fn supports_tool_call(&self) -> Option<bool> {
        self.tool_call
    }

    /// Check if model supports structured output.
    pub fn supports_structured_output(&self) -> Option<bool> {
        self.structured_output
    }

    /// Check if model supports attachments.
    pub fn supports_attachment(&self) -> Option<bool> {
        self.attachment
    }

    /// Get context window size.
    pub fn get_context_window(&self) -> Option<u64> {
        self.context_window
    }

    /// Get input modalities as a comma-separated string.
    pub fn input_modalities_str(&self) -> Option<String> {
        self.input_modalities.as_ref().map(|v| v.join(","))
    }

    /// Get output modalities as a comma-separated string.
    pub fn output_modalities_str(&self) -> Option<String> {
        self.output_modalities.as_ref().map(|v| v.join(","))
    }

    /// Check if model has a specific input modality.
    pub fn has_input_modality(&self, modality: &str) -> bool {
        self.input_modalities
            .as_ref()
            .map(|v| v.iter().any(|m| m.eq_ignore_ascii_case(modality)))
            .unwrap_or(false)
    }

    /// Check if model has a specific output modality.
    pub fn has_output_modality(&self, modality: &str) -> bool {
        self.output_modalities
            .as_ref()
            .map(|v| v.iter().any(|m| m.eq_ignore_ascii_case(modality)))
            .unwrap_or(false)
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
    fn test_basic_accessors() {
        let model = make_test_model();
        assert_eq!(model.display_name(), "GPT-4o");
        assert_eq!(model.creator_name(), "OpenAI");
        assert_eq!(model.intelligence(), Some(55.0));
        assert_eq!(model.input_price(), Some(2.5));
        assert_eq!(model.tps(), Some(150.0));
    }

    #[test]
    fn test_capability_accessors() {
        let model = make_test_model();
        assert_eq!(model.supports_reasoning(), Some(false));
        assert_eq!(model.supports_tool_call(), Some(true));
        assert_eq!(model.supports_structured_output(), Some(true));
        assert_eq!(model.get_context_window(), Some(128000));
    }

    #[test]
    fn test_modality_checks() {
        let model = make_test_model();
        assert!(model.has_input_modality("text"));
        assert!(model.has_input_modality("image"));
        assert!(model.has_input_modality("IMAGE")); // case insensitive
        assert!(!model.has_input_modality("audio"));
        assert!(model.has_output_modality("text"));
        assert!(!model.has_output_modality("image"));
    }

    #[test]
    fn test_modality_strings() {
        let model = make_test_model();
        assert_eq!(model.input_modalities_str(), Some("text,image".to_string()));
        assert_eq!(model.output_modalities_str(), Some("text".to_string()));
    }

    #[test]
    fn test_serialization() {
        let model = make_test_model();
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: LlmModel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, model.name);
        assert_eq!(deserialized.intelligence, model.intelligence);
        assert_eq!(deserialized.tool_call, model.tool_call);
    }
}
