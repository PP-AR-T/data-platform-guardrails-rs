pub mod json;
pub mod markdown;

use serde::Serialize;

use crate::cli::OutputFormat;

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Pass,
    Warn,
    Fail,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Pass => write!(f, "pass"),
            Severity::Warn => write!(f, "warn"),
            Severity::Fail => write!(f, "fail"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationItem {
    pub severity: Severity,
    pub rule_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub items: Vec<ValidationItem>,
}

impl ValidationReport {
    pub fn has_failures(&self) -> bool {
        self.items.iter().any(|i| i.severity == Severity::Fail)
    }

    pub fn summary_counts(&self) -> (usize, usize, usize) {
        self.items
            .iter()
            .fold((0, 0, 0), |(pass, warn, fail), item| match item.severity {
                Severity::Pass => (pass + 1, warn, fail),
                Severity::Warn => (pass, warn + 1, fail),
                Severity::Fail => (pass, warn, fail + 1),
            })
    }
}

pub fn render_output(
    report: &ValidationReport,
    format: &OutputFormat,
    config_path: &str,
) -> String {
    match format {
        OutputFormat::Human => render_human(report, config_path),
        OutputFormat::Json => json::render(report),
        OutputFormat::Markdown => markdown::render(report, config_path),
    }
}

fn render_human(report: &ValidationReport, config_path: &str) -> String {
    let mut lines = vec![
        format!("Validation report for {config_path}"),
        String::new(),
    ];

    for item in &report.items {
        let entity_suffix = item
            .entity_name
            .as_ref()
            .map(|name| format!(" (entity: {name})"))
            .unwrap_or_default();
        lines.push(format!(
            "[{}] {}{} {}",
            item.severity.to_string().to_uppercase(),
            item.rule_id,
            entity_suffix,
            item.message
        ));
    }

    let (pass, warn, fail) = report.summary_counts();
    lines.push(String::new());
    lines.push(format!("Summary: {pass} pass, {warn} warn, {fail} fail"));

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_counts_work() {
        let report = ValidationReport {
            items: vec![
                ValidationItem {
                    severity: Severity::Pass,
                    rule_id: "a".to_string(),
                    entity_name: None,
                    message: "ok".to_string(),
                },
                ValidationItem {
                    severity: Severity::Fail,
                    rule_id: "b".to_string(),
                    entity_name: None,
                    message: "bad".to_string(),
                },
            ],
        };

        assert_eq!(report.summary_counts(), (1, 0, 1));
        assert!(report.has_failures());
    }
}
