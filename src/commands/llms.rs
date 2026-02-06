//! LLM models command.

use crate::client::{Client, HostedDataClient};
use crate::error::Result;
use crate::models::LlmModel;
use crate::output::{format_output, Formattable, OutputFormat};
use serde::Serialize;
use tabled::Tabled;

/// Capability filter options.
#[derive(Debug, Default)]
pub struct CapabilityFilters {
    pub reasoning: bool,
    pub tool_call: bool,
    pub structured_output: bool,
    pub attachment: bool,
    pub min_context: Option<u64>,
    pub modality: Option<String>,
}

/// LLM model row for output.
#[derive(Debug, Clone, Serialize, Tabled)]
pub struct LlmRow {
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Creator")]
    pub creator: String,
    #[tabled(rename = "Intelligence")]
    pub intelligence: String,
    #[tabled(rename = "Input $/M")]
    pub input_price: String,
    #[tabled(rename = "Output $/M")]
    pub output_price: String,
    #[tabled(rename = "TPS")]
    pub tps: String,
    #[tabled(rename = "R")]
    pub reasoning: String,
    #[tabled(rename = "T")]
    pub tool_call: String,
    #[tabled(rename = "Context")]
    pub context_window: String,
}

impl Formattable for LlmRow {
    fn headers() -> &'static [&'static str] {
        &[
            "Name",
            "Creator",
            "Intelligence",
            "Input $/M",
            "Output $/M",
            "TPS",
            "R",
            "T",
            "Context",
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.creator.clone(),
            self.intelligence.clone(),
            self.input_price.clone(),
            self.output_price.clone(),
            self.tps.clone(),
            self.reasoning.clone(),
            self.tool_call.clone(),
            self.context_window.clone(),
        ]
    }
}

/// Format boolean capability for display.
fn format_bool(value: Option<bool>) -> String {
    match value {
        Some(true) => "+".into(),
        Some(false) => "-".into(),
        None => "?".into(),
    }
}

/// Format context window for display.
fn format_context(value: Option<u64>) -> String {
    match value {
        Some(v) if v >= 1_000_000 => format!("{}M", v / 1_000_000),
        Some(v) if v >= 1_000 => format!("{}K", v / 1_000),
        Some(v) => v.to_string(),
        None => "-".into(),
    }
}

impl From<&LlmModel> for LlmRow {
    fn from(model: &LlmModel) -> Self {
        Self {
            name: model.display_name().to_string(),
            creator: model.creator_name().to_string(),
            intelligence: model
                .intelligence()
                .map(|v| format!("{:.1}", v))
                .unwrap_or_else(|| "-".into()),
            input_price: model
                .input_price()
                .map(|v| format!("{:.2}", v))
                .unwrap_or_else(|| "-".into()),
            output_price: model
                .output_price()
                .map(|v| format!("{:.2}", v))
                .unwrap_or_else(|| "-".into()),
            tps: model
                .tps()
                .map(|v| format!("{:.1}", v))
                .unwrap_or_else(|| "-".into()),
            reasoning: format_bool(model.reasoning),
            tool_call: format_bool(model.tool_call),
            context_window: format_context(model.context_window),
        }
    }
}

/// Run the LLM models command.
pub async fn run(
    client: &Client,
    refresh: bool,
    format: OutputFormat,
    model_filter: Option<&str>,
    creator_filter: Option<&str>,
    sort_by: Option<&str>,
    capability_filters: CapabilityFilters,
) -> Result<()> {
    let mut models = client.get_llm_models(refresh).await?;

    // Apply name/slug filters
    if let Some(slug) = model_filter {
        models.retain(|m| {
            m.slug.contains(slug) || m.name.to_lowercase().contains(&slug.to_lowercase())
        });
    }

    if let Some(creator) = creator_filter {
        models.retain(|m| {
            m.creator_slug.as_deref().unwrap_or("").contains(creator)
                || m.creator.to_lowercase().contains(&creator.to_lowercase())
        });
    }

    // Apply capability filters
    if capability_filters.reasoning {
        models.retain(|m| m.reasoning == Some(true));
    }

    if capability_filters.tool_call {
        models.retain(|m| m.tool_call == Some(true));
    }

    if capability_filters.structured_output {
        models.retain(|m| m.structured_output == Some(true));
    }

    if capability_filters.attachment {
        models.retain(|m| m.attachment == Some(true));
    }

    if let Some(min_ctx) = capability_filters.min_context {
        models.retain(|m| m.context_window.map(|c| c >= min_ctx).unwrap_or(false));
    }

    if let Some(ref modality_spec) = capability_filters.modality {
        // Format: "input:image" or "output:text"
        let parts: Vec<&str> = modality_spec.split(':').collect();
        if parts.len() == 2 && !parts[1].is_empty() {
            let direction = parts[0];
            let modality = parts[1];
            match direction {
                "input" => {
                    models.retain(|m| m.has_input_modality(modality));
                }
                "output" => {
                    models.retain(|m| m.has_output_modality(modality));
                }
                _ => {
                    eprintln!(
                        "Unknown modality direction: {}. Use 'input' or 'output'.",
                        direction
                    );
                }
            }
        } else {
            eprintln!("Invalid modality format. Use 'input:<type>' or 'output:<type>'.");
        }
    }

    // Apply sorting
    if let Some(field) = sort_by {
        match field.to_lowercase().as_str() {
            "intelligence" | "intel" => {
                models.sort_by(|a, b| {
                    b.intelligence
                        .partial_cmp(&a.intelligence)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            "price" | "input" => {
                models.sort_by(|a, b| {
                    a.input_price
                        .partial_cmp(&b.input_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            "speed" | "tps" => {
                models.sort_by(|a, b| {
                    b.tps
                        .partial_cmp(&a.tps)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            "context" => {
                models.sort_by(|a, b| b.context_window.cmp(&a.context_window));
            }
            _ => {
                eprintln!("Unknown sort field: {}. Using default order.", field);
            }
        }
    }

    // Format output
    if format == OutputFormat::Json {
        // For JSON, output the raw models
        println!("{}", crate::output::json::format_json(&models));
    } else {
        let rows: Vec<LlmRow> = models.iter().map(LlmRow::from).collect();
        println!("{}", format_output(&rows, format));
    }

    Ok(())
}

/// Run the LLM models command using hosted data.
pub async fn run_hosted(
    client: &HostedDataClient,
    refresh: bool,
    format: OutputFormat,
    model_filter: Option<&str>,
    creator_filter: Option<&str>,
    sort_by: Option<&str>,
    capability_filters: CapabilityFilters,
) -> Result<()> {
    let mut models = client.get_llm_models(refresh).await?;

    // Apply name/slug filters
    if let Some(slug) = model_filter {
        models.retain(|m| {
            m.slug.contains(slug) || m.name.to_lowercase().contains(&slug.to_lowercase())
        });
    }

    if let Some(creator) = creator_filter {
        models.retain(|m| {
            m.creator_slug.as_deref().unwrap_or("").contains(creator)
                || m.creator.to_lowercase().contains(&creator.to_lowercase())
        });
    }

    // Apply capability filters
    if capability_filters.reasoning {
        models.retain(|m| m.reasoning == Some(true));
    }

    if capability_filters.tool_call {
        models.retain(|m| m.tool_call == Some(true));
    }

    if capability_filters.structured_output {
        models.retain(|m| m.structured_output == Some(true));
    }

    if capability_filters.attachment {
        models.retain(|m| m.attachment == Some(true));
    }

    if let Some(min_ctx) = capability_filters.min_context {
        models.retain(|m| m.context_window.map(|c| c >= min_ctx).unwrap_or(false));
    }

    if let Some(ref modality_spec) = capability_filters.modality {
        // Format: "input:image" or "output:text"
        let parts: Vec<&str> = modality_spec.split(':').collect();
        if parts.len() == 2 && !parts[1].is_empty() {
            let direction = parts[0];
            let modality = parts[1];
            match direction {
                "input" => {
                    models.retain(|m| m.has_input_modality(modality));
                }
                "output" => {
                    models.retain(|m| m.has_output_modality(modality));
                }
                _ => {
                    eprintln!(
                        "Unknown modality direction: {}. Use 'input' or 'output'.",
                        direction
                    );
                }
            }
        } else {
            eprintln!("Invalid modality format. Use 'input:<type>' or 'output:<type>'.");
        }
    }

    // Apply sorting
    if let Some(field) = sort_by {
        match field.to_lowercase().as_str() {
            "intelligence" | "intel" => {
                models.sort_by(|a, b| {
                    b.intelligence
                        .partial_cmp(&a.intelligence)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            "price" | "input" => {
                models.sort_by(|a, b| {
                    a.input_price
                        .partial_cmp(&b.input_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            "speed" | "tps" => {
                models.sort_by(|a, b| {
                    b.tps
                        .partial_cmp(&a.tps)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            "context" => {
                models.sort_by(|a, b| b.context_window.cmp(&a.context_window));
            }
            _ => {
                eprintln!("Unknown sort field: {}. Using default order.", field);
            }
        }
    }

    // Format output
    if format == OutputFormat::Json {
        // For JSON, output the raw models
        println!("{}", crate::output::json::format_json(&models));
    } else {
        let rows: Vec<LlmRow> = models.iter().map(LlmRow::from).collect();
        println!("{}", format_output(&rows, format));
    }

    Ok(())
}
