//! Data models for models.dev API responses.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level response from models.dev API.
/// The API returns a HashMap where keys are provider IDs.
pub type ModelsDevResponse = HashMap<String, ModelsDevProvider>;

/// Provider information from models.dev.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsDevProvider {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub npm: Option<String>,
    #[serde(default)]
    pub doc: Option<String>,
    #[serde(default)]
    pub api: Option<String>,
    #[serde(default)]
    pub models: HashMap<String, ModelsDevModel>,
}

/// Model information from models.dev.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsDevModel {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub family: Option<String>,
    // Capabilities
    #[serde(default)]
    pub attachment: Option<bool>,
    #[serde(default)]
    pub reasoning: Option<bool>,
    #[serde(default)]
    pub tool_call: Option<bool>,
    #[serde(default)]
    pub structured_output: Option<bool>,
    #[serde(default)]
    pub temperature: Option<bool>,
    // Metadata
    #[serde(default)]
    pub knowledge: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub last_updated: Option<String>,
    #[serde(default)]
    pub open_weights: Option<bool>,
    #[serde(default)]
    pub status: Option<String>,
    // Limits
    #[serde(default)]
    pub limit: Option<ModelsDevLimits>,
    // Cost
    #[serde(default)]
    pub cost: Option<ModelsDevCost>,
    // Modalities
    #[serde(default)]
    pub modalities: Option<ModelsDevModalities>,
}

/// Token limits from models.dev.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsDevLimits {
    #[serde(default)]
    pub context: Option<u64>,
    #[serde(default)]
    pub input: Option<u64>,
    #[serde(default)]
    pub output: Option<u64>,
}

/// Cost information from models.dev (per million tokens).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsDevCost {
    #[serde(default)]
    pub input: Option<f64>,
    #[serde(default)]
    pub output: Option<f64>,
}

/// Modality information from models.dev.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsDevModalities {
    #[serde(default)]
    pub input: Vec<String>,
    #[serde(default)]
    pub output: Vec<String>,
}

/// Flattened model row for Parquet storage.
/// Contains provider info combined with model info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsDevRow {
    pub provider_id: String,
    pub provider_name: String,
    pub model_id: String,
    pub model_name: String,
    pub family: Option<String>,
    // Capabilities
    pub attachment: Option<bool>,
    pub reasoning: Option<bool>,
    pub tool_call: Option<bool>,
    pub structured_output: Option<bool>,
    pub temperature: Option<bool>,
    // Metadata
    pub knowledge: Option<String>,
    pub release_date: Option<String>,
    pub last_updated: Option<String>,
    pub open_weights: Option<bool>,
    pub status: Option<String>,
    // Limits
    pub context_window: Option<u64>,
    pub max_input_tokens: Option<u64>,
    pub max_output_tokens: Option<u64>,
    // Cost
    pub cost_input: Option<f64>,
    pub cost_output: Option<f64>,
    // Modalities (stored as comma-separated strings for Parquet)
    pub input_modalities: Option<String>,
    pub output_modalities: Option<String>,
}

impl ModelsDevRow {
    /// Create a flattened row from provider and model.
    pub fn from_provider_model(provider: &ModelsDevProvider, model: &ModelsDevModel) -> Self {
        Self {
            provider_id: provider.id.clone(),
            provider_name: provider.name.clone(),
            model_id: model.id.clone(),
            model_name: model.name.clone(),
            family: model.family.clone(),
            attachment: model.attachment,
            reasoning: model.reasoning,
            tool_call: model.tool_call,
            structured_output: model.structured_output,
            temperature: model.temperature,
            knowledge: model.knowledge.clone(),
            release_date: model.release_date.clone(),
            last_updated: model.last_updated.clone(),
            open_weights: model.open_weights,
            status: model.status.clone(),
            context_window: model.limit.as_ref().and_then(|l| l.context),
            max_input_tokens: model.limit.as_ref().and_then(|l| l.input),
            max_output_tokens: model.limit.as_ref().and_then(|l| l.output),
            cost_input: model.cost.as_ref().and_then(|c| c.input),
            cost_output: model.cost.as_ref().and_then(|c| c.output),
            input_modalities: model.modalities.as_ref().map(|m| m.input.join(",")),
            output_modalities: model.modalities.as_ref().map(|m| m.output.join(",")),
        }
    }
}

/// Flatten the entire API response into rows for storage.
pub fn flatten_response(response: &ModelsDevResponse) -> Vec<ModelsDevRow> {
    let mut rows = Vec::new();
    for provider in response.values() {
        for model in provider.models.values() {
            rows.push(ModelsDevRow::from_provider_model(provider, model));
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_model() {
        let json = r#"{
            "id": "gpt-4o",
            "name": "GPT-4o",
            "family": "gpt-4",
            "attachment": true,
            "reasoning": false,
            "tool_call": true,
            "structured_output": true,
            "temperature": true,
            "knowledge": "2024-04",
            "release_date": "2024-05-13",
            "last_updated": "2024-11-20",
            "open_weights": false,
            "modalities": {
                "input": ["text", "image"],
                "output": ["text"]
            },
            "limit": {
                "context": 128000,
                "output": 16384
            },
            "cost": {
                "input": 2.5,
                "output": 10.0
            }
        }"#;

        let model: ModelsDevModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, "gpt-4o");
        assert_eq!(model.name, "GPT-4o");
        assert_eq!(model.attachment, Some(true));
        assert_eq!(model.reasoning, Some(false));
        assert_eq!(model.tool_call, Some(true));
        assert_eq!(model.structured_output, Some(true));
        assert_eq!(model.limit.as_ref().unwrap().context, Some(128000));
        assert_eq!(
            model.modalities.as_ref().unwrap().input,
            vec!["text", "image"]
        );
    }

    #[test]
    fn test_deserialize_provider() {
        let json = r#"{
            "id": "openai",
            "name": "OpenAI",
            "env": ["OPENAI_API_KEY"],
            "npm": "@ai-sdk/openai",
            "doc": "https://platform.openai.com/docs",
            "models": {
                "gpt-4o": {
                    "id": "gpt-4o",
                    "name": "GPT-4o",
                    "tool_call": true
                }
            }
        }"#;

        let provider: ModelsDevProvider = serde_json::from_str(json).unwrap();
        assert_eq!(provider.id, "openai");
        assert_eq!(provider.name, "OpenAI");
        assert_eq!(provider.models.len(), 1);
        assert!(provider.models.contains_key("gpt-4o"));
    }

    #[test]
    fn test_flatten_row() {
        let provider = ModelsDevProvider {
            id: "openai".into(),
            name: "OpenAI".into(),
            env: vec!["OPENAI_API_KEY".into()],
            npm: Some("@ai-sdk/openai".into()),
            doc: None,
            api: None,
            models: HashMap::new(),
        };

        let model = ModelsDevModel {
            id: "gpt-4o".into(),
            name: "GPT-4o".into(),
            family: Some("gpt-4".into()),
            attachment: Some(true),
            reasoning: Some(false),
            tool_call: Some(true),
            structured_output: Some(true),
            temperature: Some(true),
            knowledge: Some("2024-04".into()),
            release_date: Some("2024-05-13".into()),
            last_updated: Some("2024-11-20".into()),
            open_weights: Some(false),
            status: None,
            limit: Some(ModelsDevLimits {
                context: Some(128000),
                input: None,
                output: Some(16384),
            }),
            cost: Some(ModelsDevCost {
                input: Some(2.5),
                output: Some(10.0),
            }),
            modalities: Some(ModelsDevModalities {
                input: vec!["text".into(), "image".into()],
                output: vec!["text".into()],
            }),
        };

        let row = ModelsDevRow::from_provider_model(&provider, &model);
        assert_eq!(row.provider_id, "openai");
        assert_eq!(row.model_id, "gpt-4o");
        assert_eq!(row.context_window, Some(128000));
        assert_eq!(row.input_modalities, Some("text,image".into()));
        assert_eq!(row.output_modalities, Some("text".into()));
    }
}
