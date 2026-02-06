//! Cost calculator command.
//!
//! Calculate token costs for one or more LLM models.

use crate::error::{AppError, Result};
use crate::models::LlmModel;
use crate::output::{format_output, Formattable, OutputFormat};
use crate::utils::{find_models_by_names, parse_tokens};
use serde::Serialize;
use tabled::Tabled;

/// Time period for cost projection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Period {
    Once,
    Daily,
    Monthly,
}

impl Period {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "once" => Ok(Period::Once),
            "daily" | "day" => Ok(Period::Daily),
            "monthly" | "month" => Ok(Period::Monthly),
            _ => Err(AppError::Config(format!(
                "Invalid period '{}'. Use 'once', 'daily', or 'monthly'.",
                s
            ))),
        }
    }
}

/// Cost calculation result for a single model.
#[derive(Debug, Clone, Serialize)]
pub struct CostResult {
    pub name: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub input_cost: Option<f64>,
    pub output_cost: Option<f64>,
    pub total_cost: Option<f64>,
    pub requests: u64,
    pub period: String,
    pub period_cost: Option<f64>,
    pub monthly_cost: Option<f64>,
}

/// Cost row for table output.
#[derive(Debug, Clone, Serialize, Tabled)]
pub struct CostRow {
    #[tabled(rename = "Model")]
    pub name: String,
    #[tabled(rename = "Input Cost")]
    pub input_cost: String,
    #[tabled(rename = "Output Cost")]
    pub output_cost: String,
    #[tabled(rename = "Total")]
    pub total_cost: String,
}

impl Formattable for CostRow {
    fn headers() -> &'static [&'static str] {
        &["Model", "Input Cost", "Output Cost", "Total"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.input_cost.clone(),
            self.output_cost.clone(),
            self.total_cost.clone(),
        ]
    }
}

/// Format a cost value for display.
fn format_cost(cost: Option<f64>) -> String {
    match cost {
        Some(c) if c < 0.01 => format!("${:.4}", c),
        Some(c) if c < 1.0 => format!("${:.3}", c),
        Some(c) => format!("${:.2}", c),
        None => "N/A".to_string(),
    }
}

/// Format a cost with asterisk for winner.
fn format_cost_with_winner(cost: Option<f64>, is_winner: bool) -> String {
    let formatted = format_cost(cost);
    if is_winner && cost.is_some() {
        format!("{} *", formatted)
    } else {
        formatted
    }
}

/// Format token count for display.
fn format_token_count(tokens: u64) -> String {
    if tokens >= 1_000_000 {
        format!("{:.1}M", tokens as f64 / 1_000_000.0)
    } else if tokens >= 1_000 {
        if tokens % 1_000 == 0 {
            format!("{}K", tokens / 1_000)
        } else {
            format!("{:.1}K", tokens as f64 / 1_000.0)
        }
    } else {
        tokens.to_string()
    }
}

/// Calculate cost for a model.
fn calculate_cost(
    model: &LlmModel,
    input_tokens: u64,
    output_tokens: u64,
    requests: u64,
    period: Period,
) -> CostResult {
    // Prices are per million tokens
    let input_cost = model
        .input_price
        .map(|p| (input_tokens as f64 / 1_000_000.0) * p);
    let output_cost = model
        .output_price
        .map(|p| (output_tokens as f64 / 1_000_000.0) * p);

    let total_cost = match (input_cost, output_cost) {
        (Some(i), Some(o)) => Some(i + o),
        (Some(i), None) => Some(i),
        (None, Some(o)) => Some(o),
        (None, None) => None,
    };

    // Apply requests multiplier
    let request_cost = total_cost.map(|c| c * requests as f64);

    // Calculate period costs
    let (period_str, period_cost, monthly_cost) = match period {
        Period::Once => ("once".to_string(), request_cost, None),
        Period::Daily => {
            let daily = request_cost;
            let monthly = daily.map(|d| d * 30.0);
            ("daily".to_string(), daily, monthly)
        }
        Period::Monthly => {
            let monthly = request_cost.map(|c| c * 30.0);
            ("monthly".to_string(), monthly, monthly)
        }
    };

    CostResult {
        name: model.display_name().to_string(),
        input_tokens,
        output_tokens,
        input_cost,
        output_cost,
        total_cost,
        requests,
        period: period_str,
        period_cost,
        monthly_cost,
    }
}

/// Find the minimum cost among results (for winner highlighting).
fn find_min_cost(results: &[CostResult]) -> Option<f64> {
    results
        .iter()
        .filter_map(|r| r.total_cost)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
}

/// Run the cost command.
pub fn run(
    models: &[LlmModel],
    model_searches: &[String],
    input_tokens_str: &str,
    output_tokens_str: &str,
    requests: u64,
    period_str: &str,
    format: OutputFormat,
) -> Result<()> {
    // Parse token counts
    let input_tokens = parse_tokens(input_tokens_str)?;
    let output_tokens = parse_tokens(output_tokens_str)?;
    let period = Period::from_str(period_str)?;

    // Find matching models
    let matched_models = find_models_by_names(models, model_searches);

    if matched_models.is_empty() {
        return Err(AppError::NotFound(format!(
            "No models found matching: {}",
            model_searches.join(", ")
        )));
    }

    // Calculate costs
    let results: Vec<CostResult> = matched_models
        .iter()
        .map(|m| calculate_cost(m, input_tokens, output_tokens, requests, period))
        .collect();

    // Find winner (lowest cost)
    let min_cost = find_min_cost(&results);

    // Output based on format
    if format == OutputFormat::Json {
        println!("{}", crate::output::json::format_json(&results));
        return Ok(());
    }

    // Create rows for all output formats (even single model uses table for CSV/Plain)
    let rows: Vec<CostRow> = results
        .iter()
        .map(|r| {
            let is_winner = results.len() > 1 && min_cost.is_some() && r.total_cost == min_cost;
            CostRow {
                name: r.name.clone(),
                input_cost: format_cost(r.input_cost),
                output_cost: format_cost(r.output_cost),
                total_cost: format_cost_with_winner(r.total_cost, is_winner),
            }
        })
        .collect();

    match format {
        OutputFormat::Json => unreachable!(), // Handled above
        OutputFormat::Csv | OutputFormat::Plain => {
            // For CSV and Plain, just output the table data without any preamble
            println!("{}", format_output(&rows, format));
        }
        OutputFormat::Markdown | OutputFormat::Table => {
            // For Markdown/Table, show full human-readable output
            if results.len() == 1 {
                // Single model: detailed output
                let result = &results[0];
                println!("Model: {}", result.name);
                println!();
                println!("Cost per request:");
                println!(
                    "  Input ({} tokens):    {}",
                    format_token_count(result.input_tokens),
                    format_cost(result.input_cost)
                );
                println!(
                    "  Output ({} tokens):   {}",
                    format_token_count(result.output_tokens),
                    format_cost(result.output_cost)
                );
                println!(
                    "  Total:                  {}",
                    format_cost(result.total_cost)
                );

                if requests > 1 || period != Period::Once {
                    println!();
                    if requests > 1 {
                        println!(
                            "Cost for {} requests:    {}",
                            requests,
                            format_cost(result.period_cost)
                        );
                    }
                    if period == Period::Daily {
                        println!(
                            "Daily ({} requests):     {}",
                            requests,
                            format_cost(result.period_cost)
                        );
                        println!(
                            "Monthly (30 days):       {}",
                            format_cost(result.monthly_cost)
                        );
                    } else if period == Period::Monthly {
                        println!(
                            "Monthly:                 {}",
                            format_cost(result.monthly_cost)
                        );
                    }
                }
            } else {
                // Multiple models: comparison table with preamble
                println!(
                    "Cost Comparison ({} input / {} output tokens per request)",
                    format_token_count(input_tokens),
                    format_token_count(output_tokens)
                );

                if requests > 1 {
                    println!("Requests: {}", requests);
                }

                println!();
                println!("{}", format_output(&rows, format));

                if min_cost.is_some() {
                    println!();
                    println!("* = lowest cost");
                }

                // Show projections if requested
                if period != Period::Once {
                    println!();
                    if period == Period::Daily {
                        println!("Daily costs ({} requests):", requests);
                        for r in &results {
                            println!("  {}: {}", r.name, format_cost(r.period_cost));
                        }
                        println!();
                        println!("Monthly estimates (30 days):");
                        for r in &results {
                            println!("  {}: {}", r.name, format_cost(r.monthly_cost));
                        }
                    } else {
                        println!("Monthly costs ({} requests/day):", requests);
                        for r in &results {
                            println!("  {}: {}", r.name, format_cost(r.monthly_cost));
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
