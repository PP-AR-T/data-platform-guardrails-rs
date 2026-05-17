#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataModel {
    pub version: String,
    pub dataset: Dataset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dataset {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

impl MetadataModel {
    #[must_use]
    pub fn new(version: impl Into<String>, dataset: Dataset) -> Self {
        Self {
            version: version.into(),
            dataset,
        }
    }
}

impl Dataset {
    #[must_use]
    pub fn new(name: impl Into<String>, fields: Vec<Field>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

impl Field {
    #[must_use]
    pub fn new(name: impl Into<String>, data_type: impl Into<String>, nullable: bool) -> Self {
        Self {
            name: name.into(),
            data_type: data_type.into(),
            nullable,
        }
    }
}
