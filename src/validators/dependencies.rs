use std::collections::HashSet;

use crate::{
    config::Config,
    report::ValidationItem,
    validation::{fail, pass, warn},
};

const RULE_ID: &str = "dependency_integrity";

pub fn validate(config: &Config) -> Vec<ValidationItem> {
    let entities = config.entities.as_deref().unwrap_or_default();
    if entities.is_empty() {
        return vec![warn(
            RULE_ID,
            None,
            "no entities available for dependency check",
        )];
    }

    let entity_names: HashSet<String> = entities
        .iter()
        .filter_map(|e| e.entity_name.clone())
        .collect();

    let mut out = Vec::new();

    for entity in entities {
        let name = entity
            .entity_name
            .clone()
            .unwrap_or_else(|| "<unknown>".to_string());

        let mut local_fail = false;
        for dep in &entity.dependencies {
            if !entity_names.contains(dep) {
                local_fail = true;
                out.push(fail(
                    RULE_ID,
                    Some(name.clone()),
                    format!("dependency '{dep}' does not refer to a valid entity"),
                ));
            }
        }

        if !local_fail {
            out.push(pass(
                RULE_ID,
                Some(name),
                "all dependencies refer to valid entities",
            ));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn dependencies_fail_for_unknown_entity() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    dependencies: [orders]
  - entity_name: product
    dependencies: []
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message.contains("does not refer to a valid entity"))
        );
    }

    #[test]
    fn dependencies_pass_for_known_entities() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    dependencies: [orders]
  - entity_name: orders
    dependencies: []
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message == "all dependencies refer to valid entities")
        );
    }

    #[test]
    fn dependencies_fail_for_single_entity_with_unknown_dependency() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    dependencies: [orders]
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message.contains("does not refer to a valid entity"))
        );
    }

    #[test]
    fn dependencies_pass_for_single_entity_without_dependencies() {
        let cfg: Config = serde_yaml::from_str(
            r#"
platform: generic_lakehouse
entities:
  - entity_name: customer
    dependencies: []
"#,
        )
        .expect("yaml should parse");

        let result = validate(&cfg);
        assert!(
            result
                .iter()
                .any(|i| i.message == "all dependencies refer to valid entities")
        );
    }
}
