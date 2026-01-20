//! Model matching algorithm for correlating AA and models.dev data.

use crate::sources::models_dev::models::{ModelsDevModel, ModelsDevProvider};
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;

/// Pre-compiled regex patterns for version suffix stripping.
static RE_DATE: Lazy<Regex> = Lazy::new(|| Regex::new(r"-\d{8}$").unwrap());
static RE_DATE_DASHED: Lazy<Regex> = Lazy::new(|| Regex::new(r"-\d{4}-\d{2}-\d{2}$").unwrap());
static RE_VERSION: Lazy<Regex> = Lazy::new(|| Regex::new(r"-v\d+(\.\d+)*$").unwrap());

/// Pre-compiled regex pattern for version separator normalization (dots to dashes between digits).
static RE_VERSION_SEPARATOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d)\.(\d)").unwrap());

/// Pre-compiled regex pattern for compressed version expansion (e.g., -35- → -3-5-).
/// Matches a hyphen followed by two consecutive digits, followed by either another hyphen or end of string.
/// We use two patterns: one for mid-string (-35-) and one for end of string (-35$).
static RE_COMPRESSED_VERSION_MID: Lazy<Regex> = Lazy::new(|| Regex::new(r"-(\d)(\d)-").unwrap());
static RE_COMPRESSED_VERSION_END: Lazy<Regex> = Lazy::new(|| Regex::new(r"-(\d)(\d)$").unwrap());

/// Provider name mapping from AA to models.dev (pre-computed).
/// AA uses different provider slugs than models.dev in some cases.
static PROVIDER_MAPPING: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("meta", "llama");
    map.insert("meta-llama", "llama");
    map.insert("x-ai", "xai");
    map.insert("x.ai", "xai");
    map
});

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
    /// Match after normalizing version separators (dots to dashes)
    NormalizedVersionSeparator,
    /// Match after stripping provider prefix from models.dev ID
    StrippedProviderPrefix,
    /// Match after stripping -reasoning or -non-reasoning suffix
    ReasoningVariant,
    /// Match after expanding compressed version numbers (e.g., 35 → 3-5)
    ExpandedVersion,
    /// Match after adding -it suffix for Gemma models
    GemmaItSuffix,
    /// Match after stripping effort level suffix (-low, -medium, -high, -minimal)
    EffortLevel,
}

/// Normalize a provider slug to match models.dev conventions.
pub fn normalize_provider(slug: &str) -> String {
    let lower = slug.to_lowercase();
    PROVIDER_MAPPING
        .get(lower.as_str())
        .unwrap_or(&lower.as_str())
        .to_string()
}

/// Normalize version separators by converting dots to dashes between digits.
/// Uses Cow to avoid allocation when no transformation is needed.
/// Examples:
/// - "gemini-2.5-flash" -> "gemini-2-5-flash"
/// - "gpt-4.1-mini" -> "gpt-4-1-mini"
/// - "claude-3.5-sonnet" -> "claude-3-5-sonnet"
pub fn normalize_version_separators(slug: &str) -> Cow<'_, str> {
    RE_VERSION_SEPARATOR.replace_all(slug, "$1-$2")
}

/// Strip provider prefix from model slug (e.g., "mistral/mistral-large-3" -> "mistral-large-3").
/// Uses Cow to avoid allocation when no transformation is needed.
/// models.dev IDs sometimes have provider prefixes like "provider/model".
pub fn strip_provider_prefix(slug: &str) -> Cow<'_, str> {
    if let Some(pos) = slug.find('/') {
        Cow::Owned(slug[pos + 1..].to_string())
    } else {
        Cow::Borrowed(slug)
    }
}

/// Strip -reasoning or -non-reasoning suffix from model slugs.
/// These suffixes indicate benchmark configuration, not different models.
/// Examples:
/// - "gemini-2-5-flash-reasoning" -> Some("gemini-2-5-flash")
/// - "deepseek-v3-2-non-reasoning" -> Some("deepseek-v3-2")
/// - "gpt-4o" -> None
pub fn strip_reasoning_suffix(slug: &str) -> Option<Cow<'_, str>> {
    // Check -non-reasoning first (longer suffix)
    if let Some(base) = slug.strip_suffix("-non-reasoning") {
        Some(Cow::Borrowed(base))
    } else if let Some(base) = slug.strip_suffix("-reasoning") {
        Some(Cow::Borrowed(base))
    } else {
        None
    }
}

/// Expand compressed version numbers in model names.
/// Uses Cow to avoid allocation when no transformation is needed.
/// Examples:
/// - "gpt-35-turbo" -> "gpt-3-5-turbo"
/// - "claude-35-sonnet" -> "claude-3-5-sonnet"
/// - "claude-21" -> "claude-2-1"
///
/// Pattern: When a model name contains two consecutive digits after a hyphen
/// that look like a version (e.g., `-35-`, `-21`), expand them to `-3-5-`, `-2-1`.
pub fn expand_compressed_version(slug: &str) -> Cow<'_, str> {
    // First handle mid-string patterns (-35- → -3-5-)
    let expanded = RE_COMPRESSED_VERSION_MID.replace_all(slug, "-$1-$2-");
    // Then handle end-of-string patterns (-35$ → -3-5)
    match expanded {
        Cow::Borrowed(s) => RE_COMPRESSED_VERSION_END.replace_all(s, "-$1-$2"),
        Cow::Owned(s) => {
            let result = RE_COMPRESSED_VERSION_END.replace_all(&s, "-$1-$2");
            match result {
                Cow::Borrowed(_) => Cow::Owned(s),
                Cow::Owned(new_s) => Cow::Owned(new_s),
            }
        }
    }
}

/// Try to add -it suffix for Gemma models.
/// Gemma models in AA are named `gemma-3-12b` but models.dev uses `gemma-3-12b-it` (instruction-tuned).
///
/// Returns `Some(slug + "-it")` if slug starts with "gemma-" and doesn't end with "-it",
/// otherwise returns `None`.
pub fn try_add_it_suffix(slug: &str) -> Option<String> {
    if slug.starts_with("gemma-") && !slug.ends_with("-it") {
        Some(format!("{}-it", slug))
    } else {
        None
    }
}

/// Strip effort level suffixes for specific providers.
/// For Google, OpenAI, and Anthropic ONLY, `-low`, `-medium`, `-high`, `-minimal`
/// are always effort levels, never part of model names.
///
/// These providers use different naming for model sizes (mini, nano, flash, lite).
///
/// Returns `Some(base)` if a suffix was stripped, `None` otherwise.
pub fn strip_effort_suffix_for_provider<'a>(
    slug: &'a str,
    creator: Option<&str>,
) -> Option<Cow<'a, str>> {
    // Only strip for specific providers
    let creator_lower = creator?.to_lowercase();
    if !matches!(creator_lower.as_str(), "google" | "openai" | "anthropic") {
        return None;
    }

    // Try stripping effort level suffixes
    for suffix in &["-low", "-medium", "-high", "-minimal"] {
        if let Some(base) = slug.strip_suffix(suffix) {
            return Some(Cow::Borrowed(base));
        }
    }

    None
}

/// Strip version suffixes from model slugs.
/// Uses Cow to avoid allocation when no transformation is needed.
/// Examples:
/// - "claude-3-5-sonnet-20241022" -> "claude-3-5-sonnet"
/// - "gpt-4o-2024-08-06" -> "gpt-4o"
pub fn strip_version_suffix(slug: &str) -> Cow<'_, str> {
    // Pattern 1: -YYYYMMDD suffix
    let stripped = RE_DATE.replace(slug, "");

    // Pattern 2: -YYYY-MM-DD suffix
    let stripped = match stripped {
        Cow::Borrowed(s) => RE_DATE_DASHED.replace(s, ""),
        Cow::Owned(s) => {
            let result = RE_DATE_DASHED.replace(&s, "");
            match result {
                Cow::Borrowed(_) => Cow::Owned(s),
                Cow::Owned(new_s) => Cow::Owned(new_s),
            }
        }
    };

    // Pattern 3: -vX.Y.Z version suffix
    match stripped {
        Cow::Borrowed(s) => RE_VERSION.replace(s, ""),
        Cow::Owned(s) => {
            let result = RE_VERSION.replace(&s, "");
            match result {
                Cow::Borrowed(_) => Cow::Owned(s),
                Cow::Owned(new_s) => Cow::Owned(new_s),
            }
        }
    }
}

/// All normalized variants of an AA slug we should try to match.
/// Pre-computed to enable single-pass matching.
struct NormalizedSlugs<'a> {
    /// Slug with version suffix stripped (if different)
    stripped_version: Option<Cow<'a, str>>,
    /// Slug with version separators normalized (dots to dashes)
    normalized_separators: Option<Cow<'a, str>>,
    /// Slug with reasoning suffix stripped
    stripped_reasoning: Option<Cow<'a, str>>,
    /// Reasoning slug with normalized separators
    reasoning_normalized: Option<Cow<'a, str>>,
    /// Slug with expanded compressed version (e.g., 35 -> 3-5)
    expanded_version: Option<Cow<'a, str>>,
    /// Expanded slug with normalized separators
    expanded_normalized: Option<Cow<'a, str>>,
    /// Slug with -it suffix added (for Gemma models)
    with_it_suffix: Option<String>,
    /// Slug with effort suffix stripped
    stripped_effort: Option<Cow<'a, str>>,
    /// Effort-stripped slug with normalized separators
    effort_normalized: Option<Cow<'a, str>>,
}

impl<'a> NormalizedSlugs<'a> {
    fn new(aa_slug: &'a str, aa_creator_slug: Option<&str>) -> Self {
        // Pre-compute all variants
        let stripped_version = {
            let v = strip_version_suffix(aa_slug);
            if v.as_ref() != aa_slug {
                Some(v)
            } else {
                None
            }
        };

        let normalized_separators = {
            let v = normalize_version_separators(aa_slug);
            if v.as_ref() != aa_slug {
                Some(v)
            } else {
                None
            }
        };

        let stripped_reasoning = strip_reasoning_suffix(aa_slug);

        let reasoning_normalized = stripped_reasoning.as_ref().and_then(|base| {
            let v = normalize_version_separators(base);
            if v.as_ref() != base.as_ref() {
                Some(v.into_owned().into())
            } else {
                None
            }
        });

        let expanded_version = {
            let v = expand_compressed_version(aa_slug);
            if v.as_ref() != aa_slug {
                Some(v)
            } else {
                None
            }
        };

        let expanded_normalized = expanded_version.as_ref().and_then(|exp| {
            let v = normalize_version_separators(exp);
            if v.as_ref() != exp.as_ref() {
                Some(v.into_owned().into())
            } else {
                None
            }
        });

        let with_it_suffix = try_add_it_suffix(aa_slug);

        let stripped_effort = strip_effort_suffix_for_provider(aa_slug, aa_creator_slug);

        let effort_normalized = stripped_effort.as_ref().and_then(|base| {
            let v = normalize_version_separators(base);
            if v.as_ref() != base.as_ref() {
                Some(v.into_owned().into())
            } else {
                None
            }
        });

        Self {
            stripped_version,
            normalized_separators,
            stripped_reasoning,
            reasoning_normalized,
            expanded_version,
            expanded_normalized,
            with_it_suffix,
            stripped_effort,
            effort_normalized,
        }
    }
}

/// All normalized variants of a models.dev model ID we should consider.
/// Pre-computed to enable efficient matching.
struct ModelIdVariants<'a> {
    /// Original model ID (lowercased for comparison)
    original_lower: String,
    /// Model ID with version suffix stripped
    stripped_version: Cow<'a, str>,
    /// Model ID with version separators normalized
    normalized_separators: Cow<'a, str>,
    /// Model ID with provider prefix stripped
    stripped_prefix: Cow<'a, str>,
}

impl<'a> ModelIdVariants<'a> {
    fn new(model_id: &'a str) -> Self {
        Self {
            original_lower: model_id.to_lowercase(),
            stripped_version: strip_version_suffix(model_id),
            normalized_separators: normalize_version_separators(model_id),
            stripped_prefix: strip_provider_prefix(model_id),
        }
    }
}

/// Helper function to find a model matching a predicate across all providers.
/// Returns the first match found.
fn find_model_by<F>(
    providers: &HashMap<String, ModelsDevProvider>,
    mut predicate: F,
) -> Option<(&str, &ModelsDevModel)>
where
    F: FnMut(&str, &ModelIdVariants<'_>) -> bool,
{
    for provider in providers.values() {
        for (model_id, model) in &provider.models {
            let variants = ModelIdVariants::new(model_id);
            if predicate(&provider.id, &variants) {
                return Some((&provider.id, model));
            }
        }
    }
    None
}

/// Find a matching models.dev model for an AA model.
pub fn find_match<'a>(
    aa_creator_slug: Option<&str>,
    aa_model_slug: &str,
    providers: &'a HashMap<String, ModelsDevProvider>,
) -> Option<MatchResult<'a>> {
    let aa_provider = aa_creator_slug.map(normalize_provider).unwrap_or_default();
    let aa_slug = aa_model_slug.to_lowercase();
    let provider_was_normalized = aa_creator_slug
        .map(|s| s.to_lowercase() != aa_provider)
        .unwrap_or(false);

    // Pre-compute all AA slug variants
    let slugs = NormalizedSlugs::new(&aa_slug, aa_creator_slug);

    // Strategy 1: Try exact match with normalized provider (HashMap lookup - O(1))
    if let Some(provider) = providers.get(&aa_provider) {
        // Try direct lookup with lowercased comparison
        for (model_id, model) in &provider.models {
            if model_id.to_lowercase() == aa_slug {
                return Some(MatchResult {
                    provider_id: &provider.id,
                    model,
                    match_type: if provider_was_normalized {
                        MatchType::NormalizedProvider
                    } else {
                        MatchType::Exact
                    },
                });
            }
        }
    }

    // Strategy 2: Try exact match across all providers (linear scan)
    if let Some((provider_id, model)) =
        find_model_by(providers, |_, variants| variants.original_lower == aa_slug)
    {
        return Some(MatchResult {
            provider_id,
            model,
            match_type: MatchType::Exact,
        });
    }

    // Strategy 3: Fuzzy matching (version suffix stripped)
    // 3a: AA slug has version, try matching stripped AA slug to stripped models.dev slugs
    if let Some(ref base_slug) = slugs.stripped_version {
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.stripped_version.to_lowercase() == base_slug.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::Fuzzy,
            });
        }
    }

    // 3b: models.dev has version, AA doesn't - strip models.dev version to match AA
    if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
        variants.stripped_version.to_lowercase() == aa_slug
    }) {
        return Some(MatchResult {
            provider_id,
            model,
            match_type: MatchType::Fuzzy,
        });
    }

    // Strategy 4: Normalized version separators (dots <-> dashes)
    // 4a: Normalize AA slug and try direct match
    if let Some(ref normalized) = slugs.normalized_separators {
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.original_lower == normalized.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::NormalizedVersionSeparator,
            });
        }
    }

    // 4b: Normalize models.dev slug to match AA slug
    if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
        variants.normalized_separators.to_lowercase() == aa_slug
    }) {
        return Some(MatchResult {
            provider_id,
            model,
            match_type: MatchType::NormalizedVersionSeparator,
        });
    }

    // Strategy 5: Stripped provider prefix (e.g., "mistral/mistral-large-3" -> "mistral-large-3")
    if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
        variants.stripped_prefix.to_lowercase() == aa_slug
    }) {
        return Some(MatchResult {
            provider_id,
            model,
            match_type: MatchType::StrippedProviderPrefix,
        });
    }

    // Strategy 6: Reasoning variants (-reasoning, -non-reasoning suffix)
    if let Some(ref base_slug) = slugs.stripped_reasoning {
        // 6a: Direct match with base slug
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.original_lower == base_slug.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::ReasoningVariant,
            });
        }

        // 6b: Normalized version separators on reasoning-stripped slug
        if let Some(ref normalized) = slugs.reasoning_normalized {
            if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
                variants.original_lower == normalized.as_ref()
            }) {
                return Some(MatchResult {
                    provider_id,
                    model,
                    match_type: MatchType::ReasoningVariant,
                });
            }
        }

        // 6c: Normalize models.dev slugs to match reasoning-stripped slug
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.normalized_separators.to_lowercase() == base_slug.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::ReasoningVariant,
            });
        }

        // 6d: Strip provider prefix to match reasoning-stripped slug
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.stripped_prefix.to_lowercase() == base_slug.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::ReasoningVariant,
            });
        }
    }

    // Strategy 7: Expanded compressed version (e.g., 35 -> 3-5)
    if let Some(ref expanded) = slugs.expanded_version {
        // 7a: Direct match with expanded slug
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.original_lower == expanded.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::ExpandedVersion,
            });
        }

        // 7b: Normalized version separators on expanded slug
        if let Some(ref normalized) = slugs.expanded_normalized {
            if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
                variants.original_lower == normalized.as_ref()
            }) {
                return Some(MatchResult {
                    provider_id,
                    model,
                    match_type: MatchType::ExpandedVersion,
                });
            }
        }

        // 7c: Normalize models.dev slugs to match expanded slug
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.normalized_separators.to_lowercase() == expanded.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::ExpandedVersion,
            });
        }
    }

    // Strategy 8: Gemma -it suffix
    if let Some(ref it_slug) = slugs.with_it_suffix {
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.original_lower == it_slug.as_str()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::GemmaItSuffix,
            });
        }
    }

    // Strategy 9: Effort level suffix for Google/OpenAI/Anthropic
    if let Some(ref base_slug) = slugs.stripped_effort {
        // 9a: Direct match with effort-stripped slug
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.original_lower == base_slug.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::EffortLevel,
            });
        }

        // 9b: Normalized version separators on effort-stripped slug
        if let Some(ref normalized) = slugs.effort_normalized {
            if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
                variants.original_lower == normalized.as_ref()
            }) {
                return Some(MatchResult {
                    provider_id,
                    model,
                    match_type: MatchType::EffortLevel,
                });
            }
        }

        // 9c: Normalize models.dev slugs to match effort-stripped slug
        if let Some((provider_id, model)) = find_model_by(providers, |_, variants| {
            variants.normalized_separators.to_lowercase() == base_slug.as_ref()
        }) {
            return Some(MatchResult {
                provider_id,
                model,
                match_type: MatchType::EffortLevel,
            });
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
            strip_version_suffix("claude-3-5-sonnet-20241022").as_ref(),
            "claude-3-5-sonnet"
        );
        assert_eq!(strip_version_suffix("gpt-4o-2024-08-06").as_ref(), "gpt-4o");
        assert_eq!(strip_version_suffix("model-v1.2.3").as_ref(), "model");
        assert_eq!(strip_version_suffix("gpt-4o").as_ref(), "gpt-4o");
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

    #[test]
    fn test_normalize_version_separators() {
        // Dots to dashes between digits
        assert_eq!(
            normalize_version_separators("gemini-2.5-flash").as_ref(),
            "gemini-2-5-flash"
        );
        assert_eq!(
            normalize_version_separators("gpt-4.1-mini").as_ref(),
            "gpt-4-1-mini"
        );
        assert_eq!(
            normalize_version_separators("claude-3.5-sonnet").as_ref(),
            "claude-3-5-sonnet"
        );
        // Multiple separate version numbers (non-consecutive)
        assert_eq!(
            normalize_version_separators("model-1.2-foo-3.4").as_ref(),
            "model-1-2-foo-3-4"
        );
        // No change needed
        assert_eq!(
            normalize_version_separators("gemini-2-5-flash").as_ref(),
            "gemini-2-5-flash"
        );
        // No digits with dots
        assert_eq!(normalize_version_separators("gpt-4o").as_ref(), "gpt-4o");
    }

    #[test]
    fn test_strip_provider_prefix() {
        // With provider prefix
        assert_eq!(
            strip_provider_prefix("mistral/mistral-large-3").as_ref(),
            "mistral-large-3"
        );
        assert_eq!(
            strip_provider_prefix("qwen/qwen3-vl-8b-instruct").as_ref(),
            "qwen3-vl-8b-instruct"
        );
        // No prefix - returns unchanged
        assert_eq!(strip_provider_prefix("gpt-4o").as_ref(), "gpt-4o");
        assert_eq!(
            strip_provider_prefix("claude-3-5-sonnet").as_ref(),
            "claude-3-5-sonnet"
        );
    }

    #[test]
    fn test_normalized_version_separator_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "google".to_string(),
            make_provider(
                "google",
                vec![("gemini-2-5-flash", make_model("gemini-2-5-flash"))],
            ),
        );

        // AA uses dots, models.dev uses dashes
        let result = find_match(Some("google"), "gemini-2.5-flash", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "gemini-2-5-flash");
        assert_eq!(result.match_type, MatchType::NormalizedVersionSeparator);
    }

    #[test]
    fn test_normalized_version_separator_match_reverse() {
        let mut providers = HashMap::new();
        providers.insert(
            "openai".to_string(),
            make_provider("openai", vec![("gpt-4.1-mini", make_model("gpt-4.1-mini"))]),
        );

        // AA uses dashes, models.dev uses dots
        let result = find_match(Some("openai"), "gpt-4-1-mini", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "gpt-4.1-mini");
        assert_eq!(result.match_type, MatchType::NormalizedVersionSeparator);
    }

    #[test]
    fn test_stripped_provider_prefix_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "mistral".to_string(),
            make_provider(
                "mistral",
                vec![(
                    "mistral/mistral-large-3",
                    make_model("mistral/mistral-large-3"),
                )],
            ),
        );

        // AA has no prefix, models.dev has provider prefix
        let result = find_match(Some("mistral"), "mistral-large-3", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "mistral/mistral-large-3");
        assert_eq!(result.match_type, MatchType::StrippedProviderPrefix);
    }

    #[test]
    fn test_stripped_provider_prefix_match_qwen() {
        let mut providers = HashMap::new();
        providers.insert(
            "qwen".to_string(),
            make_provider(
                "qwen",
                vec![(
                    "qwen/qwen3-vl-8b-instruct",
                    make_model("qwen/qwen3-vl-8b-instruct"),
                )],
            ),
        );

        // AA has no prefix, models.dev has provider prefix
        let result = find_match(Some("qwen"), "qwen3-vl-8b-instruct", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "qwen/qwen3-vl-8b-instruct");
        assert_eq!(result.match_type, MatchType::StrippedProviderPrefix);
    }

    #[test]
    fn test_strip_reasoning_suffix() {
        // -reasoning suffix
        assert_eq!(
            strip_reasoning_suffix("gemini-2-5-flash-reasoning"),
            Some(Cow::Borrowed("gemini-2-5-flash"))
        );
        assert_eq!(
            strip_reasoning_suffix("claude-3-5-sonnet-reasoning"),
            Some(Cow::Borrowed("claude-3-5-sonnet"))
        );

        // -non-reasoning suffix
        assert_eq!(
            strip_reasoning_suffix("deepseek-v3-2-non-reasoning"),
            Some(Cow::Borrowed("deepseek-v3-2"))
        );
        assert_eq!(
            strip_reasoning_suffix("o1-non-reasoning"),
            Some(Cow::Borrowed("o1"))
        );

        // No suffix - returns None
        assert_eq!(strip_reasoning_suffix("gpt-4o"), None);
        assert_eq!(strip_reasoning_suffix("claude-3-5-sonnet"), None);
        // Should NOT strip -low/-medium/-high
        assert_eq!(strip_reasoning_suffix("gemini-2-5-flash-low"), None);
        assert_eq!(strip_reasoning_suffix("model-high"), None);
    }

    #[test]
    fn test_reasoning_variant_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "google".to_string(),
            make_provider(
                "google",
                vec![("gemini-2.5-flash", make_model("gemini-2.5-flash"))],
            ),
        );

        // AA's reasoning variant should match models.dev's base model (with dot normalization)
        let result = find_match(Some("google"), "gemini-2-5-flash-reasoning", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "gemini-2.5-flash");
        assert_eq!(result.match_type, MatchType::ReasoningVariant);
    }

    #[test]
    fn test_non_reasoning_variant_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "deepseek".to_string(),
            make_provider(
                "deepseek",
                vec![("deepseek-v3.2", make_model("deepseek-v3.2"))],
            ),
        );

        // AA's non-reasoning variant should match models.dev's base model (with dot normalization)
        let result = find_match(Some("deepseek"), "deepseek-v3-2-non-reasoning", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "deepseek-v3.2");
        assert_eq!(result.match_type, MatchType::ReasoningVariant);
    }

    #[test]
    fn test_expand_compressed_version() {
        // Compressed version numbers should be expanded
        assert_eq!(
            expand_compressed_version("claude-35-sonnet").as_ref(),
            "claude-3-5-sonnet"
        );
        assert_eq!(
            expand_compressed_version("gpt-35-turbo").as_ref(),
            "gpt-3-5-turbo"
        );
        assert_eq!(
            expand_compressed_version("claude-21").as_ref(),
            "claude-2-1"
        );

        // Multiple compressed versions
        assert_eq!(
            expand_compressed_version("model-35-foo-21").as_ref(),
            "model-3-5-foo-2-1"
        );

        // No change needed - already expanded
        assert_eq!(
            expand_compressed_version("claude-3-5-sonnet").as_ref(),
            "claude-3-5-sonnet"
        );
        assert_eq!(expand_compressed_version("gpt-4o").as_ref(), "gpt-4o");

        // Single digit after hyphen (not compressed)
        assert_eq!(
            expand_compressed_version("gpt-4-turbo").as_ref(),
            "gpt-4-turbo"
        );
    }

    #[test]
    fn test_expanded_version_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "anthropic".to_string(),
            make_provider(
                "anthropic",
                vec![("claude-3-5-sonnet", make_model("claude-3-5-sonnet"))],
            ),
        );

        // AA has compressed version, models.dev has expanded
        let result = find_match(Some("anthropic"), "claude-35-sonnet", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "claude-3-5-sonnet");
        assert_eq!(result.match_type, MatchType::ExpandedVersion);
    }

    #[test]
    fn test_try_add_it_suffix() {
        // Gemma models without -it suffix
        assert_eq!(
            try_add_it_suffix("gemma-3-12b"),
            Some("gemma-3-12b-it".to_string())
        );
        assert_eq!(
            try_add_it_suffix("gemma-2-9b"),
            Some("gemma-2-9b-it".to_string())
        );

        // Already has -it suffix
        assert_eq!(try_add_it_suffix("gemma-3-12b-it"), None);

        // Not a Gemma model
        assert_eq!(try_add_it_suffix("llama-3-70b"), None);
        assert_eq!(try_add_it_suffix("gpt-4o"), None);
    }

    #[test]
    fn test_gemma_it_suffix_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "google".to_string(),
            make_provider(
                "google",
                vec![("gemma-3-12b-it", make_model("gemma-3-12b-it"))],
            ),
        );

        // AA has no -it suffix, models.dev has -it suffix
        let result = find_match(Some("google"), "gemma-3-12b", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "gemma-3-12b-it");
        assert_eq!(result.match_type, MatchType::GemmaItSuffix);
    }

    #[test]
    fn test_strip_effort_suffix_for_provider() {
        // Google - should strip
        assert_eq!(
            strip_effort_suffix_for_provider("gemini-2-5-flash-low", Some("google")),
            Some(Cow::Borrowed("gemini-2-5-flash"))
        );
        assert_eq!(
            strip_effort_suffix_for_provider("gemini-2-5-flash-medium", Some("Google")),
            Some(Cow::Borrowed("gemini-2-5-flash"))
        );
        assert_eq!(
            strip_effort_suffix_for_provider("gemini-2-5-flash-high", Some("GOOGLE")),
            Some(Cow::Borrowed("gemini-2-5-flash"))
        );
        assert_eq!(
            strip_effort_suffix_for_provider("gemini-2-5-flash-minimal", Some("google")),
            Some(Cow::Borrowed("gemini-2-5-flash"))
        );

        // OpenAI - should strip
        assert_eq!(
            strip_effort_suffix_for_provider("gpt-5-low", Some("openai")),
            Some(Cow::Borrowed("gpt-5"))
        );
        assert_eq!(
            strip_effort_suffix_for_provider("gpt-5-medium", Some("OpenAI")),
            Some(Cow::Borrowed("gpt-5"))
        );

        // Anthropic - should strip
        assert_eq!(
            strip_effort_suffix_for_provider("claude-4-high", Some("anthropic")),
            Some(Cow::Borrowed("claude-4"))
        );

        // Mistral - should NOT strip (not in allowed providers)
        assert_eq!(
            strip_effort_suffix_for_provider("mistral-medium", Some("mistral")),
            None
        );
        assert_eq!(
            strip_effort_suffix_for_provider("mistral-large-low", Some("Mistral")),
            None
        );

        // No suffix to strip
        assert_eq!(
            strip_effort_suffix_for_provider("gpt-4o", Some("openai")),
            None
        );

        // No creator provided
        assert_eq!(strip_effort_suffix_for_provider("gpt-5-low", None), None);
    }

    #[test]
    fn test_effort_suffix_google_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "google".to_string(),
            make_provider("google", vec![("gemini-3-pro", make_model("gemini-3-pro"))]),
        );

        // AA has effort suffix, models.dev has base model
        let result = find_match(Some("google"), "gemini-3-pro-low", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "gemini-3-pro");
        assert_eq!(result.match_type, MatchType::EffortLevel);
    }

    #[test]
    fn test_effort_suffix_openai_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "openai".to_string(),
            make_provider("openai", vec![("gpt-5", make_model("gpt-5"))]),
        );

        // AA has effort suffix, models.dev has base model
        let result = find_match(Some("openai"), "gpt-5-medium", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "gpt-5");
        assert_eq!(result.match_type, MatchType::EffortLevel);
    }

    #[test]
    fn test_effort_suffix_not_mistral() {
        let mut providers = HashMap::new();
        providers.insert(
            "mistral".to_string(),
            make_provider("mistral", vec![("mistral", make_model("mistral"))]),
        );

        // "mistral-medium" for Mistral should NOT match "mistral" by stripping -medium
        // because Mistral is not in the allowed providers list
        let result = find_match(Some("mistral"), "mistral-medium", &providers);
        // Should not match via EffortLevel (might match via other methods or not at all)
        assert!(
            result.is_none()
                || result
                    .as_ref()
                    .map(|r| r.match_type != MatchType::EffortLevel)
                    .unwrap_or(true)
        );
    }

    // New tests for case sensitivity fix
    #[test]
    fn test_case_insensitive_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "openai".to_string(),
            make_provider("openai", vec![("GPT-4o", make_model("GPT-4o"))]),
        );

        // AA slug is lowercase but models.dev has mixed case
        let result = find_match(Some("openai"), "gpt-4o", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.model.id, "GPT-4o");
        assert_eq!(result.match_type, MatchType::Exact);
    }

    #[test]
    fn test_case_insensitive_fuzzy_match() {
        let mut providers = HashMap::new();
        providers.insert(
            "anthropic".to_string(),
            make_provider(
                "anthropic",
                vec![(
                    "Claude-3-5-Sonnet-20241022",
                    make_model("Claude-3-5-Sonnet-20241022"),
                )],
            ),
        );

        // AA has versioned slug (lowercase), models.dev has mixed case
        let result = find_match(Some("anthropic"), "claude-3-5-sonnet", &providers);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.match_type, MatchType::Fuzzy);
    }

    #[test]
    fn test_cow_no_allocation_when_unchanged() {
        // Test that Cow::Borrowed is returned when no transformation needed
        let result = normalize_version_separators("gpt-4o");
        assert!(matches!(result, Cow::Borrowed(_)));

        let result = strip_provider_prefix("gpt-4o");
        assert!(matches!(result, Cow::Borrowed(_)));

        let result = expand_compressed_version("claude-3-5-sonnet");
        assert!(matches!(result, Cow::Borrowed(_)));

        let result = strip_version_suffix("gpt-4o");
        assert!(matches!(result, Cow::Borrowed(_)));
    }
}
