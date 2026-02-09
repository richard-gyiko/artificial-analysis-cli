//! Model matching utilities.
//!
//! Substring matching for finding models by name or slug.

use crate::models::LlmModel;

/// Filter models by a search term using substring matching.
///
/// Matches against:
/// - Model slug (case-sensitive substring match)
/// - Model name (case-insensitive substring match)
///
/// Returns all models that match the search term.
pub fn filter_models_by_name<'a>(models: &'a [LlmModel], search: &str) -> Vec<&'a LlmModel> {
    let search_lower = search.to_lowercase();
    models
        .iter()
        .filter(|m| m.slug.contains(search) || m.name.to_lowercase().contains(&search_lower))
        .collect()
}

/// Filter models by creator using substring matching.
///
/// Matches against:
/// - Creator slug (case-sensitive substring match)
/// - Creator name (case-insensitive substring match)
pub fn filter_models_by_creator<'a>(models: &'a [LlmModel], creator: &str) -> Vec<&'a LlmModel> {
    let creator_lower = creator.to_lowercase();
    models
        .iter()
        .filter(|m| {
            m.creator_slug.as_deref().unwrap_or("").contains(creator)
                || m.creator.to_lowercase().contains(&creator_lower)
        })
        .collect()
}

/// Find all models matching any of the given search terms.
///
/// Returns a deduplicated list of models that match at least one search term.
pub fn find_models_by_names<'a>(models: &'a [LlmModel], searches: &[String]) -> Vec<&'a LlmModel> {
    let mut result: Vec<&'a LlmModel> = Vec::new();
    let mut seen_ids: std::collections::HashSet<&str> = std::collections::HashSet::new();

    for search in searches {
        let matches = filter_models_by_name(models, search);
        for model in matches {
            if seen_ids.insert(&model.id) {
                result.push(model);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_model(id: &str, name: &str, slug: &str, creator: &str) -> LlmModel {
        LlmModel {
            id: id.to_string(),
            name: name.to_string(),
            slug: slug.to_string(),
            creator: creator.to_string(),
            creator_slug: Some(creator.to_lowercase()),
            ..Default::default()
        }
    }

    #[test]
    fn test_filter_by_name() {
        let models = vec![
            make_model("1", "GPT-5 (high)", "gpt-5-high", "OpenAI"),
            make_model("2", "GPT-5 (low)", "gpt-5-low", "OpenAI"),
            make_model("3", "Claude 4.5 Sonnet", "claude-4.5-sonnet", "Anthropic"),
        ];

        let matches = filter_models_by_name(&models, "gpt-5");
        assert_eq!(matches.len(), 2);

        let matches = filter_models_by_name(&models, "Claude");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].name, "Claude 4.5 Sonnet");
    }

    #[test]
    fn test_filter_by_creator() {
        let models = vec![
            make_model("1", "GPT-5", "gpt-5", "OpenAI"),
            make_model("2", "Claude 4.5", "claude-4.5", "Anthropic"),
            make_model("3", "Gemini 2.5", "gemini-2.5", "Google"),
        ];

        let matches = filter_models_by_creator(&models, "openai");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].creator, "OpenAI");
    }

    #[test]
    fn test_find_by_multiple_names() {
        let models = vec![
            make_model("1", "GPT-5", "gpt-5", "OpenAI"),
            make_model("2", "Claude 4.5", "claude-4.5", "Anthropic"),
            make_model("3", "Gemini 2.5", "gemini-2.5", "Google"),
        ];

        let searches = vec!["gpt".to_string(), "claude".to_string()];
        let matches = find_models_by_names(&models, &searches);
        assert_eq!(matches.len(), 2);
    }
}
