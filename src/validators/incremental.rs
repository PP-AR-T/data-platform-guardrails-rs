use crate::{
    config::Config,
    report::ValidationItem,
    validation::{fail, pass},
};

const RULE_ID: &str = "incremental_watermark";

pub fn validate(config: &Config) -> Vec<ValidationItem> {
    let mut out = Vec::new();

    for entity in config.entities.as_deref().unwrap_or_default() {
        let name = entity
            .entity_name
            .clone()
            .unwrap_or_else(|| "<unknown>".to_string());

        let load_type = entity
            .load
            .as_ref()
            .and_then(|l| l.load_type.as_deref())
            .map(str::trim)
            .unwrap_or("");

        if load_type == "incremental" {
            let watermark = entity
                .load
                .as_ref()
                .and_then(|l| l.watermark_column.as_deref())
                .map(str::trim)
                .unwrap_or("");

            if watermark.is_empty() {
                out.push(fail(
                    RULE_ID,
                    Some(name),
                    "watermark_column is required for incremental loads",
                ));
            } else {
                out.push(pass(
                    RULE_ID,
                    Some(name),
                    "incremental load has watermark_column",
                ));
            }
        }
    }

    if out.is_empty() {
        out.push(pass(RULE_ID, None, "no incremental entities to validate"));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn incremental_requires_watermark() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    load:
      load_type: incremental
      primary_key: customer_id
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message == "watermark_column is required for incremental loads")
        );
    }

    #[test]
    fn incremental_passes_with_watermark() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    load:
      load_type: incremental
      primary_key: customer_id
      watermark_column: updated_at
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message == "incremental load has watermark_column")
        );
    }
}
