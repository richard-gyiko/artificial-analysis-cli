//! Centralized schema definitions for all tables.
//!
//! Single source of truth for table schemas, used by both Parquet serialization
//! and SQL query display.

use crate::sources::MODELS;

/// Column definition.
#[derive(Debug, Clone, Copy)]
pub struct Column {
    pub name: &'static str,
    pub sql_type: &'static str,
    pub nullable: bool,
}

/// Table definition.
#[derive(Debug, Clone, Copy)]
pub struct TableDef {
    pub name: &'static str,
    pub command: &'static str,
    pub parquet_file: &'static str,
    pub columns: &'static [Column],
}

impl TableDef {
    /// Generate CREATE TABLE SQL statement.
    pub fn create_table_sql(&self) -> String {
        let columns: Vec<String> = self
            .columns
            .iter()
            .map(|col| {
                let nullable = if col.nullable { "" } else { " NOT NULL" };
                format!("{} {}{}", col.name, col.sql_type, nullable)
            })
            .collect();
        format!(
            "CREATE TABLE {} (\n    {}\n)",
            self.name,
            columns.join(",\n    ")
        )
    }
}

/// Benchmarks table schema - contains Artificial Analysis data.
///
/// For capability data (tool_call, reasoning, context_window, etc.),
/// query the `models` table from models.dev.
pub const BENCHMARKS: TableDef = TableDef {
    name: "benchmarks",
    command: "which-llm refresh",
    parquet_file: "benchmarks.parquet",
    columns: &[
        // Core identity
        Column {
            name: "id",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "name",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "slug",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "creator",
            sql_type: "VARCHAR",
            nullable: false,
        },
        Column {
            name: "creator_slug",
            sql_type: "VARCHAR",
            nullable: true,
        },
        Column {
            name: "release_date",
            sql_type: "VARCHAR",
            nullable: true,
        },
        // Benchmarks
        Column {
            name: "intelligence",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "coding",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "math",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "mmlu_pro",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "gpqa",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "hle",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "livecodebench",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "scicode",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "math_500",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "aime",
            sql_type: "DOUBLE",
            nullable: true,
        },
        // Pricing
        Column {
            name: "input_price",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "output_price",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "price",
            sql_type: "DOUBLE",
            nullable: true,
        },
        // Performance
        Column {
            name: "tps",
            sql_type: "DOUBLE",
            nullable: true,
        },
        Column {
            name: "latency",
            sql_type: "DOUBLE",
            nullable: true,
        },
    ],
};

// Media columns (shared by all media tables)
const MEDIA_COLUMNS: &[Column] = &[
    Column {
        name: "id",
        sql_type: "VARCHAR",
        nullable: false,
    },
    Column {
        name: "name",
        sql_type: "VARCHAR",
        nullable: false,
    },
    Column {
        name: "slug",
        sql_type: "VARCHAR",
        nullable: false,
    },
    Column {
        name: "creator",
        sql_type: "VARCHAR",
        nullable: false,
    },
    Column {
        name: "elo",
        sql_type: "DOUBLE",
        nullable: true,
    },
    Column {
        name: "rank",
        sql_type: "INTEGER",
        nullable: true,
    },
    Column {
        name: "release_date",
        sql_type: "VARCHAR",
        nullable: true,
    },
];

// Individual media table definitions
pub const TEXT_TO_IMAGE: TableDef = TableDef {
    name: "text_to_image",
    command: "which-llm refresh",
    parquet_file: "text_to_image.parquet",
    columns: MEDIA_COLUMNS,
};

pub const IMAGE_EDITING: TableDef = TableDef {
    name: "image_editing",
    command: "which-llm refresh",
    parquet_file: "image_editing.parquet",
    columns: MEDIA_COLUMNS,
};

pub const TEXT_TO_SPEECH: TableDef = TableDef {
    name: "text_to_speech",
    command: "which-llm refresh",
    parquet_file: "text_to_speech.parquet",
    columns: MEDIA_COLUMNS,
};

pub const TEXT_TO_VIDEO: TableDef = TableDef {
    name: "text_to_video",
    command: "which-llm refresh",
    parquet_file: "text_to_video.parquet",
    columns: MEDIA_COLUMNS,
};

pub const IMAGE_TO_VIDEO: TableDef = TableDef {
    name: "image_to_video",
    command: "which-llm refresh",
    parquet_file: "image_to_video.parquet",
    columns: MEDIA_COLUMNS,
};

/// All available tables (user-facing).
pub const ALL_TABLES: &[&TableDef] = &[
    &BENCHMARKS,
    &MODELS,
    &TEXT_TO_IMAGE,
    &IMAGE_EDITING,
    &TEXT_TO_SPEECH,
    &TEXT_TO_VIDEO,
    &IMAGE_TO_VIDEO,
];

/// Get table definition by name.
pub fn get_table_def(name: &str) -> Option<&'static TableDef> {
    match name {
        "benchmarks" => Some(&BENCHMARKS),
        "models" => Some(&MODELS),
        "text_to_image" => Some(&TEXT_TO_IMAGE),
        "image_editing" => Some(&IMAGE_EDITING),
        "text_to_speech" => Some(&TEXT_TO_SPEECH),
        "text_to_video" => Some(&TEXT_TO_VIDEO),
        "image_to_video" => Some(&IMAGE_TO_VIDEO),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table_sql() {
        let sql = BENCHMARKS.create_table_sql();
        assert!(sql.contains("CREATE TABLE benchmarks"));
        assert!(sql.contains("id VARCHAR NOT NULL"));
        assert!(sql.contains("intelligence DOUBLE"));
        // Verify capability fields are NOT present
        assert!(!sql.contains("reasoning"));
        assert!(!sql.contains("tool_call"));
        assert!(!sql.contains("context_window"));
        assert!(!sql.contains("models_dev_matched"));
    }

    #[test]
    fn test_media_tables_share_schema() {
        assert_eq!(TEXT_TO_IMAGE.columns.len(), IMAGE_EDITING.columns.len());
        assert_eq!(TEXT_TO_IMAGE.columns[0].name, IMAGE_EDITING.columns[0].name);
    }

    #[test]
    fn test_get_table_def() {
        assert!(get_table_def("benchmarks").is_some());
        assert!(get_table_def("models").is_some());
        assert!(get_table_def("text_to_image").is_some());
        assert!(get_table_def("unknown").is_none());
    }

    #[test]
    fn test_all_tables_count() {
        assert_eq!(ALL_TABLES.len(), 7);
    }

    #[test]
    fn test_benchmarks_has_only_aa_columns() {
        let benchmarks = get_table_def("benchmarks").unwrap();
        let column_names: Vec<_> = benchmarks.columns.iter().map(|c| c.name).collect();

        // AA columns should be present
        assert!(column_names.contains(&"intelligence"));
        assert!(column_names.contains(&"coding"));
        assert!(column_names.contains(&"tps"));
        assert!(column_names.contains(&"input_price"));

        // models.dev columns should NOT be present
        assert!(!column_names.contains(&"reasoning"));
        assert!(!column_names.contains(&"tool_call"));
        assert!(!column_names.contains(&"structured_output"));
        assert!(!column_names.contains(&"context_window"));
        assert!(!column_names.contains(&"models_dev_matched"));
    }
}
