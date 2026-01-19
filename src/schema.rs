//! Centralized schema definitions for all tables.
//!
//! Single source of truth for table schemas, used by both Parquet serialization
//! and SQL query display.

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

// LLM table schema
pub const LLMS: TableDef = TableDef {
    name: "llms",
    command: "aa llms",
    parquet_file: "llms.parquet",
    columns: &[
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
    command: "aa text-to-image",
    parquet_file: "text_to_image.parquet",
    columns: MEDIA_COLUMNS,
};

pub const IMAGE_EDITING: TableDef = TableDef {
    name: "image_editing",
    command: "aa image-editing",
    parquet_file: "image_editing.parquet",
    columns: MEDIA_COLUMNS,
};

pub const TEXT_TO_SPEECH: TableDef = TableDef {
    name: "text_to_speech",
    command: "aa text-to-speech",
    parquet_file: "text_to_speech.parquet",
    columns: MEDIA_COLUMNS,
};

pub const TEXT_TO_VIDEO: TableDef = TableDef {
    name: "text_to_video",
    command: "aa text-to-video",
    parquet_file: "text_to_video.parquet",
    columns: MEDIA_COLUMNS,
};

pub const IMAGE_TO_VIDEO: TableDef = TableDef {
    name: "image_to_video",
    command: "aa image-to-video",
    parquet_file: "image_to_video.parquet",
    columns: MEDIA_COLUMNS,
};

/// All available tables.
pub const ALL_TABLES: &[&TableDef] = &[
    &LLMS,
    &TEXT_TO_IMAGE,
    &IMAGE_EDITING,
    &TEXT_TO_SPEECH,
    &TEXT_TO_VIDEO,
    &IMAGE_TO_VIDEO,
];

/// Get table definition by name.
pub fn get_table_def(name: &str) -> Option<&'static TableDef> {
    match name {
        "llms" => Some(&LLMS),
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
        let sql = LLMS.create_table_sql();
        assert!(sql.contains("CREATE TABLE llms"));
        assert!(sql.contains("id VARCHAR NOT NULL"));
        assert!(sql.contains("intelligence DOUBLE"));
    }

    #[test]
    fn test_media_tables_share_schema() {
        assert_eq!(TEXT_TO_IMAGE.columns.len(), IMAGE_EDITING.columns.len());
        assert_eq!(TEXT_TO_IMAGE.columns[0].name, IMAGE_EDITING.columns[0].name);
    }

    #[test]
    fn test_get_table_def() {
        assert!(get_table_def("llms").is_some());
        assert!(get_table_def("text_to_image").is_some());
        assert!(get_table_def("unknown").is_none());
    }

    #[test]
    fn test_all_tables_count() {
        assert_eq!(ALL_TABLES.len(), 6);
    }
}
