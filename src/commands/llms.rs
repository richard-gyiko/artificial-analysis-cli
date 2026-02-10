//! LLM models command.

use crate::client::{Client, HostedDataClient};
use crate::error::Result;
use crate::models::LlmModel;
use crate::output::{format_output, Formattable, OutputFormat};
use serde::Serialize;
use tabled::Tabled;

/// LLM model row for output (AA data only).
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
        ]
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
