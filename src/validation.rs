use crate::{
    config::Config,
    report::{Severity, ValidationItem, ValidationReport},
    validators,
};

pub fn validate_config(config: &Config) -> ValidationReport {
    let mut items = Vec::new();
    items.extend(validators::required_fields::validate(config));
    items.extend(validators::naming::validate(config));
    items.extend(validators::incremental::validate(config));
    items.extend(validators::raw_layer::validate(config));
    items.extend(validators::dependencies::validate(config));
    ValidationReport { items }
}

pub fn pass(
    rule_id: &'static str,
    entity_name: Option<String>,
    message: impl Into<String>,
) -> ValidationItem {
    ValidationItem {
        severity: Severity::Pass,
        rule_id: rule_id.to_string(),
        entity_name,
        message: message.into(),
    }
}

pub fn fail(
    rule_id: &'static str,
    entity_name: Option<String>,
    message: impl Into<String>,
) -> ValidationItem {
    ValidationItem {
        severity: Severity::Fail,
        rule_id: rule_id.to_string(),
        entity_name,
        message: message.into(),
    }
}

pub fn warn(
    rule_id: &'static str,
    entity_name: Option<String>,
    message: impl Into<String>,
) -> ValidationItem {
    ValidationItem {
        severity: Severity::Warn,
        rule_id: rule_id.to_string(),
        entity_name,
        message: message.into(),
    }
}
