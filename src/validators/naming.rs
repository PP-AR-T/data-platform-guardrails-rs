use crate::{
    config::Config,
    report::ValidationItem,
    validation::{fail, pass},
};

const RULE_ID: &str = "naming_convention";

pub fn validate(config: &Config) -> Vec<ValidationItem> {
    let mut out = Vec::new();

    for entity in config.entities.as_deref().unwrap_or_default() {
        let name = entity
            .entity_name
            .clone()
            .unwrap_or_else(|| "<unknown>".to_string());

        match entity.entity_name.as_deref() {
            Some(value) if is_valid_name(value) => {
                out.push(pass(RULE_ID, Some(name), "entity_name matches convention"));
            }
            Some(_) => {
                out.push(fail(
                    RULE_ID,
                    Some(name),
                    "entity_name must match ^[a-z][a-z0-9_]*$",
                ));
            }
            None => {}
        }
    }

    if out.is_empty() {
        out.push(pass(
            RULE_ID,
            None,
            "no entities available for naming check",
        ));
    }

    out
}

fn is_valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn naming_passes_for_snake_case() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer_orders
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(result.iter().any(|i| i.severity.to_string() == "pass"));
    }

    #[test]
    fn naming_fails_for_invalid_name() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: Customer-Orders
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message.contains("must match ^[a-z][a-z0-9_]*$"))
        );
    }
}
