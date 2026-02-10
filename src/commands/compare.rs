//! Compare command.
//!
//! Side-by-side comparison of multiple LLM models.

use crate::error::{AppError, Result};
use crate::models::LlmModel;
use crate::output::OutputFormat;
use crate::utils::find_models_by_names;
use serde::Serialize;

/// Comparison field with value and formatting info.
#[derive(Debug, Clone, Serialize)]
struct CompareField {
    name: String,
    values: Vec<String>,
    winners: Vec<bool>,
}

/// Comparison result for JSON output.
#[derive(Debug, Clone, Serialize)]
struct CompareResult {
    models: Vec<String>,
    fields: Vec<CompareField>,
}

/// Field type for determining winner logic.
#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldType {
    /// Higher is better (intelligence, tps, coding)
    HigherBetter,
    /// Lower is better (price, latency)
    LowerBetter,
    /// No winner comparison (creator, etc.)
    NoWinner,
}

/// Type alias for field extractor functions.
type FieldExtractor = Box<dyn Fn(&LlmModel) -> Option<FieldValue>>;

/// Field definition for comparison.
struct FieldDef {
    name: &'static str,
    field_type: FieldType,
    extractor: FieldExtractor,
}

/// Field value union type.
#[derive(Debug, Clone)]
enum FieldValue {
    Float(f64),
    String(String),
}

impl FieldValue {
    fn to_display(&self) -> String {
        match self {
            FieldValue::Float(v) => format!("{:.1}", v),
            FieldValue::String(s) => s.clone(),
        }
    }
}

/// Format price for display.
fn format_price(value: f64) -> String {
    format!("${:.2}", value)
}

/// Get all field definitions.
fn get_field_defs(verbose: bool) -> Vec<FieldDef> {
    let mut fields: Vec<FieldDef> = vec![
        // Core identity
        FieldDef {
            name: "Creator",
            field_type: FieldType::NoWinner,
            extractor: Box::new(|m| Some(FieldValue::String(m.creator.clone()))),
        },
        // Benchmarks
        FieldDef {
            name: "Intelligence",
            field_type: FieldType::HigherBetter,
            extractor: Box::new(|m| m.intelligence.map(FieldValue::Float)),
        },
        FieldDef {
            name: "Coding",
            field_type: FieldType::HigherBetter,
            extractor: Box::new(|m| m.coding.map(FieldValue::Float)),
        },
        // Pricing
        FieldDef {
            name: "Input $/M",
            field_type: FieldType::LowerBetter,
            extractor: Box::new(|m| m.input_price.map(FieldValue::Float)),
        },
        FieldDef {
            name: "Output $/M",
            field_type: FieldType::LowerBetter,
            extractor: Box::new(|m| m.output_price.map(FieldValue::Float)),
        },
        FieldDef {
            name: "Blended $/M",
            field_type: FieldType::LowerBetter,
            extractor: Box::new(|m| m.price.map(FieldValue::Float)),
        },
        // Performance
        FieldDef {
            name: "TPS",
            field_type: FieldType::HigherBetter,
            extractor: Box::new(|m| m.tps.map(FieldValue::Float)),
        },
        FieldDef {
            name: "Latency (s)",
            field_type: FieldType::LowerBetter,
            extractor: Box::new(|m| m.latency.map(FieldValue::Float)),
        },
    ];

    // Verbose-only fields (additional AA benchmarks)
    if verbose {
        fields.extend(vec![
            FieldDef {
                name: "Math",
                field_type: FieldType::HigherBetter,
                extractor: Box::new(|m| m.math.map(FieldValue::Float)),
            },
            FieldDef {
                name: "MMLU-Pro",
                field_type: FieldType::HigherBetter,
                extractor: Box::new(|m| m.mmlu_pro.map(FieldValue::Float)),
            },
            FieldDef {
                name: "GPQA",
                field_type: FieldType::HigherBetter,
                extractor: Box::new(|m| m.gpqa.map(FieldValue::Float)),
            },
            FieldDef {
                name: "HLE",
                field_type: FieldType::HigherBetter,
                extractor: Box::new(|m| m.hle.map(FieldValue::Float)),
            },
            FieldDef {
                name: "LiveCodeBench",
                field_type: FieldType::HigherBetter,
                extractor: Box::new(|m| m.livecodebench.map(FieldValue::Float)),
            },
            FieldDef {
                name: "SciCode",
                field_type: FieldType::HigherBetter,
                extractor: Box::new(|m| m.scicode.map(FieldValue::Float)),
            },
            FieldDef {
                name: "Math 500",
                field_type: FieldType::HigherBetter,
                extractor: Box::new(|m| m.math_500.map(FieldValue::Float)),
            },
            FieldDef {
                name: "AIME",
                field_type: FieldType::HigherBetter,
                extractor: Box::new(|m| m.aime.map(FieldValue::Float)),
            },
        ]);
    }

    fields
}

/// Find winners for a set of values.
fn find_winners(values: &[Option<FieldValue>], field_type: FieldType) -> Vec<bool> {
    match field_type {
        FieldType::NoWinner => vec![false; values.len()],
        FieldType::HigherBetter => {
            let floats: Vec<Option<f64>> = values
                .iter()
                .map(|v| match v {
                    Some(FieldValue::Float(f)) => Some(*f),
                    _ => None,
                })
                .collect();

            let max = floats
                .iter()
                .filter_map(|f| *f)
                .fold(f64::NEG_INFINITY, f64::max);

            if max == f64::NEG_INFINITY {
                vec![false; values.len()]
            } else {
                floats
                    .iter()
                    .map(|f| f.map(|v| (v - max).abs() < 0.001).unwrap_or(false))
                    .collect()
            }
        }
        FieldType::LowerBetter => {
            let floats: Vec<Option<f64>> = values
                .iter()
                .map(|v| match v {
                    Some(FieldValue::Float(f)) => Some(*f),
                    _ => None,
                })
                .collect();

            let min = floats
                .iter()
                .filter_map(|f| *f)
                .fold(f64::INFINITY, f64::min);

            if min == f64::INFINITY {
                vec![false; values.len()]
            } else {
                floats
                    .iter()
                    .map(|f| f.map(|v| (v - min).abs() < 0.001).unwrap_or(false))
                    .collect()
            }
        }
    }
}

/// Run the compare command.
pub fn run(
    models: &[LlmModel],
    model_searches: &[String],
    verbose: bool,
    format: OutputFormat,
) -> Result<()> {
    // Find matching models
    let matched_models = find_models_by_names(models, model_searches);

    if matched_models.is_empty() {
        return Err(AppError::NotFound(format!(
            "No models found matching: {}",
            model_searches.join(", ")
        )));
    }

    if matched_models.len() < 2 {
        return Err(AppError::Config(
            "Need at least 2 models to compare. Try broader search terms.".into(),
        ));
    }

    // Get field definitions
    let field_defs = get_field_defs(verbose);

    // Build comparison data
    let model_names: Vec<String> = matched_models
        .iter()
        .map(|m| m.display_name().to_string())
        .collect();

    let compare_fields: Vec<CompareField> = field_defs
        .iter()
        .map(|def| {
            let values: Vec<Option<FieldValue>> =
                matched_models.iter().map(|m| (def.extractor)(m)).collect();

            let winners = find_winners(&values, def.field_type);

            let display_values: Vec<String> = values
                .iter()
                .zip(winners.iter())
                .map(|(v, &is_winner)| {
                    let base = match v {
                        Some(fv) => {
                            // Special formatting for price fields
                            if def.name.contains("$/M") {
                                if let FieldValue::Float(f) = fv {
                                    format_price(*f)
                                } else {
                                    fv.to_display()
                                }
                            } else {
                                fv.to_display()
                            }
                        }
                        None => "-".to_string(),
                    };
                    if is_winner {
                        format!("{} *", base)
                    } else {
                        base
                    }
                })
                .collect();

            CompareField {
                name: def.name.to_string(),
                values: display_values,
                winners,
            }
        })
        .collect();

    let result = CompareResult {
        models: model_names.clone(),
        fields: compare_fields,
    };

    // Output based on format
    match format {
        OutputFormat::Json => {
            let json =
                serde_json::to_string_pretty(&result).map_err(crate::error::AppError::from)?;
            println!("{}", json);
        }
        OutputFormat::Csv => {
            // CSV: header row is "Field", then model names; each row is a field and its values
            fn escape_csv_cell(value: &str) -> String {
                let needs_quotes =
                    value.contains(',') || value.contains('"') || value.contains('\n');
                if !needs_quotes {
                    return value.to_string();
                }
                let mut escaped = String::with_capacity(value.len() + 2);
                escaped.push('"');
                for ch in value.chars() {
                    if ch == '"' {
                        escaped.push('"');
                    }
                    escaped.push(ch);
                }
                escaped.push('"');
                escaped
            }

            // Header
            let mut header_cells = Vec::with_capacity(model_names.len() + 1);
            header_cells.push(escape_csv_cell("Field"));
            for name in &model_names {
                header_cells.push(escape_csv_cell(name));
            }
            println!("{}", header_cells.join(","));

            // Rows
            for field in &result.fields {
                let mut row_cells = Vec::with_capacity(model_names.len() + 1);
                row_cells.push(escape_csv_cell(&field.name));
                for value in &field.values {
                    row_cells.push(escape_csv_cell(value));
                }
                println!("{}", row_cells.join(","));
            }
        }
        OutputFormat::Plain => {
            // Plain: tab-separated values
            // Header
            print!("Field");
            for name in &model_names {
                print!("\t{}", name);
            }
            println!();

            // Rows
            for field in &result.fields {
                print!("{}", field.name);
                for value in &field.values {
                    print!("\t{}", value);
                }
                println!();
            }
            println!();
            println!("* = best in category");
        }
        OutputFormat::Markdown | OutputFormat::Table => {
            // Markdown table format (default)
            // Calculate column widths
            let field_name_width = result
                .fields
                .iter()
                .map(|f| f.name.len())
                .max()
                .unwrap_or(15);
            let model_widths: Vec<usize> = model_names
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    result
                        .fields
                        .iter()
                        .map(|f| f.values.get(i).map(|v| v.len()).unwrap_or(0))
                        .max()
                        .unwrap_or(0)
                        .max(name.len())
                })
                .collect();

            // Print header
            print!("| {:<width$} |", "Field", width = field_name_width);
            for (i, name) in model_names.iter().enumerate() {
                print!(" {:<width$} |", name, width = model_widths[i]);
            }
            println!();

            // Print separator
            print!("|{:-<width$}|", "", width = field_name_width + 2);
            for width in &model_widths {
                print!("{:-<width$}|", "", width = width + 2);
            }
            println!();

            // Print data rows
            for field in &result.fields {
                print!("| {:<width$} |", field.name, width = field_name_width);
                for (i, value) in field.values.iter().enumerate() {
                    print!(" {:<width$} |", value, width = model_widths[i]);
                }
                println!();
            }

            println!();
            println!("* = best in category");
        }
    }

    Ok(())
}
