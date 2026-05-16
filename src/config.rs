use std::fs;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub platform: Option<String>,
    pub entities: Option<Vec<EntityConfig>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityConfig {
    pub entity_name: Option<String>,
    pub source: Option<SourceConfig>,
    pub target: Option<TargetConfig>,
    pub load: Option<LoadConfig>,
    pub rules: Option<RulesConfig>,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceConfig {
    pub system: Option<String>,
    pub object: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetConfig {
    pub layer: Option<String>,
    pub object: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoadConfig {
    pub load_type: Option<String>,
    pub primary_key: Option<String>,
    pub watermark_column: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RulesConfig {
    pub allow_business_transforms_in_raw: Option<bool>,
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file at '{path}'"))?;
    let parsed: Config =
        serde_yaml::from_str(&raw).with_context(|| format!("invalid YAML at '{path}'"))?;
    Ok(parsed)
}
