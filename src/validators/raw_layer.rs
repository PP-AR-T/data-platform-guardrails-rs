use crate::{
    config::Config,
    report::ValidationItem,
    validation::{fail, pass},
};

const RULE_ID: &str = "raw_layer_guardrail";

pub fn validate(config: &Config) -> Vec<ValidationItem> {
    let mut out = Vec::new();

    for entity in config.entities.as_deref().unwrap_or_default() {
        let name = entity
            .entity_name
            .clone()
            .unwrap_or_else(|| "<unknown>".to_string());

        let is_raw_layer = entity
            .target
            .as_ref()
            .and_then(|t| t.layer.as_deref())
            .map(str::trim)
            == Some("raw");

        if !is_raw_layer {
            continue;
        }

        let allow_business = entity
            .rules
            .as_ref()
            .and_then(|r| r.allow_business_transforms_in_raw)
            .unwrap_or(false);

        if allow_business {
            out.push(fail(
                RULE_ID,
                Some(name),
                "raw layer must not allow business transforms",
            ));
        } else {
            out.push(pass(
                RULE_ID,
                Some(name),
                "raw layer business transform guardrail satisfied",
            ));
        }
    }

    if out.is_empty() {
        out.push(pass(RULE_ID, None, "no raw-layer entities to validate"));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn raw_layer_fails_when_business_transforms_enabled() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    target: { layer: raw, object: raw_customer }
    rules: { allow_business_transforms_in_raw: true }
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message == "raw layer must not allow business transforms")
        );
    }

    #[test]
    fn raw_layer_passes_when_business_transforms_disabled() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    target: { layer: raw, object: raw_customer }
    rules: { allow_business_transforms_in_raw: false }
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message == "raw layer business transform guardrail satisfied")
        );
    }
}
