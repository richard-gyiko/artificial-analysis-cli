//! Model matching algorithm for correlating AA and models.dev data.

use crate::sources::models_dev::models::{ModelsDevModel, ModelsDevProvider};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Pre-compiled regex patterns for version suffix stripping.
static RE_DATE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-\d{8}$").unwrap());
static RE_DATE_DASHED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"-\d{4}-\d{2}-\d{2}$").unwrap());
static RE_VERSION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-v\d+(\.\d+)*$").unwrap());

/// Result of a model match attempt.
#[derive(Debug, Clone)]
pub struct MatchResult<'a> {
    pub provider_id: &'a str,
    pub model: &'a ModelsDevModel,
    pub match_type: MatchType,
}

/// Type of match found.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchType {
    /// Exact composite key match (provider/model)
    Exact,
    /// Fuzzy match after stripping version suffix
    Fuzzy,
    /// Provider name was normalized before match
    NormalizedProvider,
}

/// Provider name mapping from AA to models.dev.
/// AA uses different provider slugs than models.dev in some cases.
fn provider_mapping() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("meta", "llama");
    map.insert("meta-llama", "llama");
    map.insert("x-ai", "xai");
    map.insert("x.ai", "xai");
    map
}

/// Normalize a provider slug to match models.dev conventions.
pub fn normalize_provider(slug: &str) -> String {
    let lower = slug.to_lowercase();
    let mapping = provider_mapping();
    mapping
        .get(lower.as_str())
        .unwrap_or(&lower.as_str())
        .to_string()
}

/// Strip version suffixes from model slugs.
/// Examples:
/// - "claude-3-5-sonnet-20241022" -> "claude-3-5-sonnet"
/// - "gpt-4o-2024-08-06" -> "gpt-4o"
pub fn strip_version_suffix(slug: &str) -> String {
    // Pattern 1: -YYYYMMDD suffix
    let stripped = RE_DATE.replace(slug, "");

    // Pattern 2: -YYYY-MM-DD suffix
    let stripped = RE_DATE_DASHED.replace(&stripped, "");

    // Pattern 3: -vX.Y.Z version suffix
    let stripped = RE_VERSION.replace(&stripped, "");

    stripped.to_string()
}

/// Find a matching models.dev model for an AA model.
pub fn find_match<'a>(
    aa_creator_slug: Option<&str>,
    aa_model_slug: &str,
    providers: &'a HashMap<String, ModelsDevProvider>,
) -> Option<MatchResult<'a>> {
    let aa_provider = aa_creator_slug.map(normalize_provider).unwrap_or_default();
    let aa_slug = aa_model_slug.to_lowercase();

    // 1. Try exact match with normalized provider
    if let Some(provider) = providers.get(&aa_provider) {
        if let Some(model) = provider.models.get(&aa_slug) {
            return Some(MatchResult {
                provider_id: &provider.id,
                model,
                match_type: if aa_creator_slug.map(|s| s.to_lowercase())
                    == Some(aa_provider.clone())
                {
                    MatchType::Exact
                } else {
                    MatchType::NormalizedProvider
                },
            });
        }
    }

    // 2. Try exact match across all providers (in case creator slug differs)
    for (provider_id, provider) in providers {
        if let Some(model) = provider.models.get(&aa_slug) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::Exact,
            });
        }
    }

    // 3. Try fuzzy matching (strip version suffixes)
    let base_slug = strip_version_suffix(&aa_slug);
    if base_slug != aa_slug {
        // First try with normalized provider
        if let Some(provider) = providers.get(&aa_provider) {
            for (model_id, model) in &provider.models {
                if strip_version_suffix(model_id) == base_slug {
                    return Some(MatchResult {
                        provider_id: &provider.id,
                        model,
                        match_type: MatchType::Fuzzy,
                    });
                }
            }
        }

        // Try across all providers
        for (provider_id, provider) in providers {
            for (model_id, model) in &provider.models {
                if strip_version_suffix(model_id) == base_slug {
                    return Some(MatchResult {
                        provider_id,
                        model,
                        match_type: MatchType::Fuzzy,
                    });
                }
            }
        }
    }

    // 4. Try matching model slugs that differ only in the models.dev side having a version
    for (provider_id, provider) in providers {
        for (model_id, model) in &provider.models {
            if strip_version_suffix(model_id) == aa_slug {
                return Some(MatchResult {
                    provider_id,
                    model,
                    match_type: MatchType::Fuzzy,
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sources::models_dev::models::ModelsDevModel;

    fn make_provider(id: &str, models: Vec<(&str, ModelsDevModel)>) -> ModelsDevProvider {
        let mut model_map = HashMap::new();
        for (model_id, model) in models {
            model_map.insert(model_id.to_string(), model);
        }
        ModelsDevProvider {
            id: id.to_string(),
            name: id.to_string(),
            env: vec![],
            npm: None,
            doc: None,
            api: None,
            models: model_map,
        }
    }

    fn make_model(id: &str) -> ModelsDevModel {
        ModelsDevModel {
            id: id.to_string(),
            name: id.to_string(),
            family: None,
            attachment: Some(true),
            reasoning: Some(false),
            tool_call: Some(true),
            structured_output: Some(true),
            temperature: Some(true),
            knowledge: None,
            release_date: None,
            last_updated: None,
            open_weights: None,
            status: None,
            limit: None,
            cost: None,
            modalities: None,
        }
    }

    #[test]
    fn test_normalize_provider() {
        assert_eq!(normalize_provider("meta"), "llama");
        assert_eq!(normalize_provider("Meta"), "llama");
        assert_eq!(normalize_provider("openai"), "openai");
        assert_eq!(normalize_provider("OpenAI"), "openai");
        assert_eq!(normalize_provider("anthropic"), "anthropic");
    }

    #[test]
    fn test_strip_version_suffix() {
        assert_eq!(
            strip_version_suffix("claude-3-5-sonnet-20241022"),
            "claude-3-5-sonnet"
        );
        assert_eq!(strip_version_suffix("gpt-4o-2024-08-06"), "gpt-4o");
        assert_eq!(strip_version_suffix("model-v1.2.3"), "model");
        assert_eq!(strip_version_suffix("gpt-4o"), "gpt-4o");
    }

    #[test]
    fn test_exact_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "openai".to_string(),
            make_provider("openai", vec![("gpt-4o", make_model("gpt-4o"))]),
        );

        let result = find_match(Some("openai"), "gpt-4o", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "gpt-4o");
        assert_eq!(result.match_type, MatchType::Exact);
    }

    #[test]
    fn test_normalized_provider_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "llama".to_string(),
            make_provider("llama", vec![("llama-3-70b", make_model("llama-3-70b"))]),
        );

        // AA uses "meta" but models.dev uses "llama"
        let result = find_match(Some("meta"), "llama-3-70b", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "llama-3-70b");
        assert_eq!(result.match_type, MatchType::NormalizedProvider);
    }

    #[test]
    fn test_fuzzy_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "anthropic".to_string(),
            make_provider(
                "anthropic",
                vec![(
                    "claude-3-5-sonnet-20241022",
                    make_model("claude-3-5-sonnet-20241022"),
                )],
            ),
        );

        // AA has versioned slug, models.dev might have base name
        let result = find_match(Some("anthropic"), "claude-3-5-sonnet", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.match_type, MatchType::Fuzzy);
    }

    #[test]
    fn test_no_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "openai".to_string(),
            make_provider("openai", vec![("gpt-4o", make_model("gpt-4o"))]),
        );

        let result = find_match(Some("unknown"), "unknown-model", &providers);
        assert!(result.is_none());
    }
}
