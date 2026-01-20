//! Field merge logic for combining AA and models.dev data.

use super::matcher::find_match;
use crate::models::LlmModel;
use crate::sources::artificial_analysis::models::AaLlmModel;
use crate::sources::models_dev::models::{ModelsDevProvider, ModelsDevRow};
use std::collections::HashMap;

/// Merge AA models with models.dev data to create unified LlmModels.
pub fn merge_models(
    aa_models: &[AaLlmModel],
    models_dev_providers: &HashMap<String, ModelsDevProvider>,
) -> Vec<LlmModel> {
    aa_models
        .iter()
        .map(|aa| merge_single_model(aa, models_dev_providers))
        .collect()
}

/// Merge a single AA model with its models.dev counterpart (if found).
fn merge_single_model(aa: &AaLlmModel, providers: &HashMap<String, ModelsDevProvider>) -> LlmModel {
    let md_match = find_match(aa.model_creator.slug.as_deref(), &aa.slug, providers);

    // Get models.dev data if matched
    let md_row = md_match.as_ref().map(|m| {
        // Find the provider to create a proper row
        if let Some(provider) = providers.get(m.provider_id) {
            ModelsDevRow::from_provider_model(provider, m.model)
        } else {
            // This shouldn't happen - log a warning for debugging
            eprintln!(
                "Warning: Provider '{}' not found in map while processing matched model '{}'. \
                 This indicates a bug in the matching logic.",
                m.provider_id, m.model.id
            );
            // Fallback - create minimal provider to extract model data
            ModelsDevRow::from_provider_model(
                &ModelsDevProvider {
                    id: m.provider_id.to_string(),
                    name: m.provider_id.to_string(),
                    env: vec![],
                    npm: None,
                    doc: None,
                    api: None,
                    models: HashMap::new(),
                },
                m.model,
            )
        }
    });

    let evaluations = aa.evaluations.as_ref();
    let pricing = aa.pricing.as_ref();

    LlmModel {
        // Core identity (from AA)
        id: aa.id.clone(),
        name: aa.name.clone(),
        slug: aa.slug.clone(),
        release_date: aa.release_date.clone(),
        creator: aa.model_creator.name.clone(),
        creator_slug: aa.model_creator.slug.clone(),

        // Benchmarks (from AA)
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

        // Pricing (from AA)
        input_price: pricing.and_then(|p| p.price_1m_input_tokens),
        output_price: pricing.and_then(|p| p.price_1m_output_tokens),
        price: pricing.and_then(|p| p.price_1m_blended_3_to_1),

        // Performance (from AA)
        tps: aa.median_output_tokens_per_second,
        latency: aa.median_time_to_first_token_seconds,

        // Capabilities (from models.dev)
        reasoning: md_row.as_ref().and_then(|r| r.reasoning),
        tool_call: md_row.as_ref().and_then(|r| r.tool_call),
        structured_output: md_row.as_ref().and_then(|r| r.structured_output),
        attachment: md_row.as_ref().and_then(|r| r.attachment),
        temperature: md_row.as_ref().and_then(|r| r.temperature),

        // Limits (from models.dev)
        context_window: md_row.as_ref().and_then(|r| r.context_window),
        max_input_tokens: md_row.as_ref().and_then(|r| r.max_input_tokens),
        max_output_tokens: md_row.as_ref().and_then(|r| r.max_output_tokens),

        // Modalities (from models.dev, stored as comma-separated)
        input_modalities: md_row
            .as_ref()
            .and_then(|r| r.input_modalities.as_ref())
            .map(|s| s.split(',').map(String::from).collect()),
        output_modalities: md_row
            .as_ref()
            .and_then(|r| r.output_modalities.as_ref())
            .map(|s| s.split(',').map(String::from).collect()),

        // Additional metadata (from models.dev)
        knowledge_cutoff: md_row.as_ref().and_then(|r| r.knowledge.clone()),
        open_weights: md_row.as_ref().and_then(|r| r.open_weights),
        last_updated: md_row.as_ref().and_then(|r| r.last_updated.clone()),

        // Source tracking
        models_dev_matched: md_match.is_some(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sources::artificial_analysis::models::{AaEvaluations, AaModelCreator, AaPricing};
    use crate::sources::models_dev::models::{
        ModelsDevLimits, ModelsDevModalities, ModelsDevModel,
    };

    fn make_aa_model(name: &str, slug: &str, creator_slug: &str) -> AaLlmModel {
        AaLlmModel {
            id: format!("aa-{}", slug),
            name: name.to_string(),
            slug: slug.to_string(),
            release_date: Some("2024-01-01".to_string()),
            model_creator: AaModelCreator {
                id: "creator-id".to_string(),
                name: "OpenAI".to_string(),
                slug: Some(creator_slug.to_string()),
                extra: serde_json::Value::Null,
            },
            evaluations: Some(AaEvaluations {
                artificial_analysis_intelligence_index: Some(55.0),
                artificial_analysis_coding_index: Some(50.0),
                artificial_analysis_math_index: Some(60.0),
                mmlu_pro: None,
                gpqa: None,
                hle: None,
                livecodebench: None,
                scicode: None,
                math_500: None,
                aime: None,
                extra: serde_json::Value::Null,
            }),
            pricing: Some(AaPricing {
                price_1m_blended_3_to_1: Some(5.0),
                price_1m_input_tokens: Some(2.5),
                price_1m_output_tokens: Some(10.0),
                extra: serde_json::Value::Null,
            }),
            median_output_tokens_per_second: Some(150.0),
            median_time_to_first_token_seconds: Some(0.5),
            median_time_to_first_answer_token: None,
            extra: serde_json::Value::Null,
        }
    }

    fn make_md_provider_with_model(provider_id: &str, model_id: &str) -> ModelsDevProvider {
        let mut models = HashMap::new();
        models.insert(
            model_id.to_string(),
            ModelsDevModel {
                id: model_id.to_string(),
                name: model_id.to_string(),
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
                limit: Some(ModelsDevLimits {
                    context: Some(128000),
                    input: None,
                    output: Some(16384),
                }),
                cost: None,
                modalities: Some(ModelsDevModalities {
                    input: vec!["text".to_string(), "image".to_string()],
                    output: vec!["text".to_string()],
                }),
            },
        );
        ModelsDevProvider {
            id: provider_id.to_string(),
            name: provider_id.to_string(),
            env: vec![],
            npm: None,
            doc: None,
            api: None,
            models,
        }
    }

    #[test]
    fn test_merge_with_match() {
        let aa_models = vec![make_aa_model("GPT-4o", "gpt-4o", "openai")];
        let mut providers = HashMap::new();
        providers.insert(
            "openai".to_string(),
            make_md_provider_with_model("openai", "gpt-4o"),
        );

        let merged = merge_models(&aa_models, &providers);
        assert_eq!(merged.len(), 1);

        let model = &merged[0];
        // AA fields
        assert_eq!(model.name, "GPT-4o");
        assert_eq!(model.intelligence, Some(55.0));
        assert_eq!(model.input_price, Some(2.5));
        assert_eq!(model.tps, Some(150.0));

        // models.dev fields
        assert_eq!(model.tool_call, Some(true));
        assert_eq!(model.reasoning, Some(false));
        assert_eq!(model.context_window, Some(128000));
        assert_eq!(
            model.input_modalities,
            Some(vec!["text".to_string(), "image".to_string()])
        );
        assert!(model.models_dev_matched);
    }

    #[test]
    fn test_merge_without_match() {
        let aa_models = vec![make_aa_model("Unknown Model", "unknown-model", "unknown")];
        let providers = HashMap::new();

        let merged = merge_models(&aa_models, &providers);
        assert_eq!(merged.len(), 1);

        let model = &merged[0];
        // AA fields present
        assert_eq!(model.name, "Unknown Model");
        assert_eq!(model.intelligence, Some(55.0));

        // models.dev fields are None
        assert_eq!(model.tool_call, None);
        assert_eq!(model.context_window, None);
        assert!(!model.models_dev_matched);
    }
}
