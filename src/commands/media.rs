//! Media model commands (text-to-image, video, speech, etc.).

use crate::client::Client;
use crate::error::Result;
use crate::models::MediaModel;
use crate::output::{format_output, Formattable, OutputFormat};
use serde::Serialize;
use tabled::Tabled;

/// Media model row for output.
#[derive(Debug, Clone, Serialize, Tabled)]
pub struct MediaRow {
    #[tabled(rename = "Rank")]
    pub rank: String,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Creator")]
    pub creator: String,
    #[tabled(rename = "ELO")]
    pub elo: String,
}

impl Formattable for MediaRow {
    fn headers() -> &'static [&'static str] {
        &["Rank", "Name", "Creator", "ELO"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.rank.clone(),
            self.name.clone(),
            self.creator.clone(),
            self.elo.clone(),
        ]
    }
}

impl From<&MediaModel> for MediaRow {
    fn from(model: &MediaModel) -> Self {
        Self {
            rank: model
                .rank
                .map(|r| r.to_string())
                .unwrap_or_else(|| "-".into()),
            name: model.name.clone(),
            creator: model.creator_name().to_string(),
            elo: model
                .elo
                .map(|e| format!("{:.0}", e))
                .unwrap_or_else(|| "-".into()),
        }
    }
}

/// Media model row with category breakdown.
#[derive(Debug, Clone, Serialize, Tabled)]
pub struct MediaRowWithCategories {
    #[tabled(rename = "Rank")]
    pub rank: String,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Creator")]
    pub creator: String,
    #[tabled(rename = "ELO")]
    pub elo: String,
    #[tabled(rename = "Categories")]
    pub categories: String,
}

impl Formattable for MediaRowWithCategories {
    fn headers() -> &'static [&'static str] {
        &["Rank", "Name", "Creator", "ELO", "Categories"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.rank.clone(),
            self.name.clone(),
            self.creator.clone(),
            self.elo.clone(),
            self.categories.clone(),
        ]
    }
}

impl From<&MediaModel> for MediaRowWithCategories {
    fn from(model: &MediaModel) -> Self {
        let categories = model
            .categories
            .as_ref()
            .map(|cats| {
                cats.iter()
                    .filter_map(|cat| {
                        let name = cat
                            .style_category
                            .as_deref()
                            .or(cat.subject_matter_category.as_deref())
                            .unwrap_or("Unknown");
                        cat.elo.map(|e| format!("{}:{:.0}", name, e))
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();

        Self {
            rank: model
                .rank
                .map(|r| r.to_string())
                .unwrap_or_else(|| "-".into()),
            name: model.name.clone(),
            creator: model.creator_name().to_string(),
            elo: model
                .elo
                .map(|e| format!("{:.0}", e))
                .unwrap_or_else(|| "-".into()),
            categories,
        }
    }
}

/// Format and print media models.
fn print_media_output(models: &[MediaModel], format: OutputFormat, categories: bool) {
    if format == OutputFormat::Json {
        println!("{}", crate::output::json::format_json(models));
    } else if categories {
        let rows: Vec<MediaRowWithCategories> =
            models.iter().map(MediaRowWithCategories::from).collect();
        println!("{}", format_output(&rows, format));
    } else {
        let rows: Vec<MediaRow> = models.iter().map(MediaRow::from).collect();
        println!("{}", format_output(&rows, format));
    }
}

/// Run text-to-image command.
pub async fn run_text_to_image(
    client: &Client,
    refresh: bool,
    format: OutputFormat,
    categories: bool,
) -> Result<()> {
    let models = client.get_text_to_image(refresh).await?;
    print_media_output(&models, format, categories);
    Ok(())
}

/// Run image-editing command.
pub async fn run_image_editing(client: &Client, refresh: bool, format: OutputFormat) -> Result<()> {
    let models = client.get_image_editing(refresh).await?;
    print_media_output(&models, format, false);
    Ok(())
}

/// Run text-to-speech command.
pub async fn run_text_to_speech(
    client: &Client,
    refresh: bool,
    format: OutputFormat,
) -> Result<()> {
    let models = client.get_text_to_speech(refresh).await?;
    print_media_output(&models, format, false);
    Ok(())
}

/// Run text-to-video command.
pub async fn run_text_to_video(
    client: &Client,
    refresh: bool,
    format: OutputFormat,
    categories: bool,
) -> Result<()> {
    let models = client.get_text_to_video(refresh).await?;
    print_media_output(&models, format, categories);
    Ok(())
}

/// Run image-to-video command.
pub async fn run_image_to_video(
    client: &Client,
    refresh: bool,
    format: OutputFormat,
    categories: bool,
) -> Result<()> {
    let models = client.get_image_to_video(refresh).await?;
    print_media_output(&models, format, categories);
    Ok(())
}
