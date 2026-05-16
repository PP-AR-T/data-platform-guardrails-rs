use dpg_model::{Dataset, Field, MetadataModel};
use std::fs;
use std::path::Path;

pub trait MetadataSource {
    fn load(&self) -> Result<MetadataModel, String>;
}

pub struct FileMetadataSource {
    path: String,
}

impl FileMetadataSource {
    #[must_use]
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl MetadataSource for FileMetadataSource {
    fn load(&self) -> Result<MetadataModel, String> {
        let content = fs::read_to_string(Path::new(&self.path))
            .map_err(|err| format!("Failed to read metadata file: {err}"))?;
        parse_metadata_text(&content)
    }
}

#[cfg(feature = "sqlserver")]
pub mod sqlserver {
    use super::MetadataSource;
    use dpg_model::MetadataModel;

    pub struct SqlServerMetadataSource {
        pub connection_string: String,
        pub dataset: String,
    }

    impl SqlServerMetadataSource {
        #[must_use]
        pub fn new(connection_string: impl Into<String>, dataset: impl Into<String>) -> Self {
            Self {
                connection_string: connection_string.into(),
                dataset: dataset.into(),
            }
        }
    }

    impl MetadataSource for SqlServerMetadataSource {
        fn load(&self) -> Result<MetadataModel, String> {
            Err(
                "SQL Server integration is optional and not enabled for runtime by default. Use dedicated integration tests when wiring a real instance."
                    .to_string(),
            )
        }
    }
}

fn parse_metadata_text(content: &str) -> Result<MetadataModel, String> {
    let mut version = None;
    let mut dataset_name = String::new();
    let mut fields = Vec::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(value) = line.strip_prefix("version=") {
            let parsed = value.trim();
            if parsed.is_empty() {
                return Err("Metadata version must not be empty.".to_string());
            }
            version = Some(parsed.to_string());
            continue;
        }

        if let Some(value) = line.strip_prefix("dataset=") {
            dataset_name = value.trim().to_string();
            continue;
        }

        if let Some(value) = line.strip_prefix("field=") {
            let segments: Vec<&str> = value.split(':').map(str::trim).collect();
            if segments.len() < 2 || segments.len() > 3 {
                return Err(format!(
                    "Invalid field entry '{value}'. Use field=name:type[:nullable]"
                ));
            }

            let nullable = match segments.get(2).copied().unwrap_or("false") {
                "true" => true,
                "false" => false,
                other => {
                    return Err(format!(
                        "Invalid nullable flag '{other}' in field entry '{value}'"
                    ));
                }
            };

            fields.push(Field::new(segments[0], segments[1], nullable));
            continue;
        }

        return Err(format!("Unknown metadata directive: {line}"));
    }

    let version = version.ok_or_else(|| {
        "Missing required metadata directive: version=<model-version>".to_string()
    })?;

    Ok(MetadataModel::new(
        version,
        Dataset::new(dataset_name, fields),
    ))
}

#[cfg(test)]
mod tests {
    use super::parse_metadata_text;

    #[test]
    fn parses_valid_text_model() {
        let text =
            "version=v1\ndataset=sales_orders\nfield=order_id:string\nfield=amount:decimal:true\n";

        let parsed = parse_metadata_text(text).expect("expected valid model");
        assert_eq!(parsed.dataset.name, "sales_orders");
        assert_eq!(parsed.dataset.fields.len(), 2);
    }

    #[test]
    fn rejects_unknown_directives() {
        let err = parse_metadata_text("oops=bad").expect_err("expected parse error");
        assert!(err.contains("Unknown metadata directive"));
    }

    #[test]
    fn rejects_missing_version() {
        let err = parse_metadata_text("dataset=sales_orders\nfield=order_id:string")
            .expect_err("expected parse error");
        assert!(err.contains("Missing required metadata directive"));
    }
}
