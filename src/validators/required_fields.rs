use crate::{
    config::{Config, EntityConfig},
    report::ValidationItem,
    validation::{fail, pass},
};

const RULE_ID: &str = "required_fields";

pub fn validate(config: &Config) -> Vec<ValidationItem> {
    let mut out = Vec::new();

    if config
        .platform
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
    {
        out.push(fail(RULE_ID, None, "platform is required"));
    }

    let entities = config.entities.as_deref().unwrap_or_default();
    if entities.is_empty() {
        out.push(fail(RULE_ID, None, "at least one entity is required"));
        return out;
    }

    for entity in entities {
        validate_entity_required_fields(entity, &mut out);
    }

    if out.is_empty() {
        out.push(pass(RULE_ID, None, "all required fields are present"));
    }

    out
}

fn validate_entity_required_fields(entity: &EntityConfig, out: &mut Vec<ValidationItem>) {
    let name = entity
        .entity_name
        .clone()
        .unwrap_or_else(|| "<unknown>".to_string());

    if entity
        .entity_name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
    {
        out.push(fail(RULE_ID, Some(name.clone()), "entity_name is required"));
    }

    let source = entity.source.as_ref();
    if source
        .and_then(|s| s.system.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
    {
        out.push(fail(
            RULE_ID,
            Some(name.clone()),
            "source.system is required",
        ));
    }

    if source
        .and_then(|s| s.object.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
    {
        out.push(fail(
            RULE_ID,
            Some(name.clone()),
            "source.object is required",
        ));
    }

    let target = entity.target.as_ref();
    if target
        .and_then(|t| t.layer.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
    {
        out.push(fail(
            RULE_ID,
            Some(name.clone()),
            "target.layer is required",
        ));
    }

    if target
        .and_then(|t| t.object.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
    {
        out.push(fail(
            RULE_ID,
            Some(name.clone()),
            "target.object is required",
        ));
    }

    let load = entity.load.as_ref();
    if load
        .and_then(|l| l.load_type.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
    {
        out.push(fail(
            RULE_ID,
            Some(name.clone()),
            "load.load_type is required",
        ));
    }

    if load
        .and_then(|l| l.primary_key.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none()
    {
        out.push(fail(RULE_ID, Some(name), "load.primary_key is required"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn required_fields_pass_for_valid_input() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    source: { system: source_crm, object: customers }
    target: { layer: raw, object: raw_customer }
    load: { load_type: full, primary_key: customer_id }
    rules: { allow_business_transforms_in_raw: false }
    dependencies: []
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].rule_id, RULE_ID);
    }

    #[test]
    fn required_fields_fail_when_missing() {
        let cfg: Config = serde_yaml::from_str(
            r#"
entities:
  - entity_name: customer
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(result.iter().any(|i| i.message == "platform is required"));
        assert!(
            result
                .iter()
                .any(|i| i.message == "source.system is required")
        );
    }
}
