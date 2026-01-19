//! Data models for media model responses.

use serde::{Deserialize, Serialize};

/// Media model from the API (text-to-image, video, speech, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaModel {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub model_creator: MediaCreator,
    #[serde(default)]
    pub elo: Option<f64>,
    #[serde(default)]
    pub rank: Option<u32>,
    #[serde(default)]
    pub ci95: Option<String>,
    #[serde(default)]
    pub appearances: Option<u64>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub categories: Option<Vec<CategoryScore>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Media model creator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCreator {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Score for a category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScore {
    #[serde(default)]
    pub style_category: Option<String>,
    #[serde(default)]
    pub subject_matter_category: Option<String>,
    #[serde(default)]
    pub format_category: Option<String>,
    #[serde(default)]
    pub elo: Option<f64>,
    #[serde(default)]
    pub ci95: Option<String>,
    #[serde(default)]
    pub appearances: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl MediaModel {
    /// Get the ELO score or default.
    pub fn elo_score(&self) -> f64 {
        self.elo.unwrap_or(0.0)
    }

    /// Get the rank or default.
    pub fn ranking(&self) -> u32 {
        self.rank.unwrap_or(0)
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
    fn test_media_model_deserialization() {
        let json = r#"{
            "id": "a1ee4d6f-d136-434b-bb1d-066fe5f9bf6f",
            "name": "GPT Image 1.5 (high)",
            "slug": "openai-gpt_image-1-5",
            "model_creator": {
                "id": "e67e56e3-15cd-43db-b679-da4660a69f41",
                "name": "OpenAI"
            },
            "elo": 1246,
            "rank": 1,
            "ci95": "-11/11",
            "appearances": 5980,
            "release_date": "2025-12"
        }"#;

        let model: MediaModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, "a1ee4d6f-d136-434b-bb1d-066fe5f9bf6f");
        assert_eq!(model.name, "GPT Image 1.5 (high)");
        assert_eq!(model.elo_score(), 1246.0);
        assert_eq!(model.ranking(), 1);
        assert_eq!(model.creator_name(), "OpenAI");
    }

    #[test]
    fn test_media_model_minimal() {
        let json = r#"{
            "id": "uuid-123",
            "name": "Midjourney",
            "slug": "midjourney",
            "model_creator": { "id": "mj-id", "name": "Midjourney" }
        }"#;

        let model: MediaModel = serde_json::from_str(json).unwrap();
        assert_eq!(model.id, "uuid-123");
        assert_eq!(model.elo_score(), 0.0);
        assert_eq!(model.ranking(), 0);
    }
}
