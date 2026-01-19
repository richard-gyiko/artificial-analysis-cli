//! API endpoint definitions.

/// Base URL for the Artificial Analysis API.
pub const API_BASE: &str = "https://artificialanalysis.ai/api/v2";

/// Endpoint paths.
pub const LLM_MODELS: &str = "/data/llms/models";
pub const TEXT_TO_IMAGE: &str = "/data/media/text-to-image";
pub const IMAGE_EDITING: &str = "/data/media/image-editing";
pub const TEXT_TO_SPEECH: &str = "/data/media/text-to-speech";
pub const TEXT_TO_VIDEO: &str = "/data/media/text-to-video";
pub const IMAGE_TO_VIDEO: &str = "/data/media/image-to-video";
