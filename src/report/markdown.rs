use std::path::Path;

use crate::report::ValidationReport;

pub fn render(report: &ValidationReport, config_path: &Path) -> String {
    let mut lines = vec![
        format!("# Validation report: `{}`", config_path.display()),
        String::new(),
        "| severity | rule_id | entity_name | message |".to_string(),
        "|---|---|---|---|".to_string(),
    ];

    for item in &report.items {
        lines.push(format!(
            "| {} | {} | {} | {} |",
            escape_cell(&item.severity.to_string()),
            escape_cell(&item.rule_id),
            escape_cell(item.entity_name.as_deref().unwrap_or("-")),
            escape_cell(&item.message)
        ));
    }

    let (pass, warn, fail) = report.summary_counts();
    lines.push(String::new());
    lines.push(format!(
        "**Summary:** {pass} pass, {warn} warn, {fail} fail"
    ));

    lines.join("\n")
}

fn escape_cell(value: &str) -> String {
    value
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\n', "<br>")
        .replace('|', "\\|")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::report::{Severity, ValidationItem, ValidationReport};

    use super::render;

    #[test]
    fn markdown_escapes_newlines_and_pipes_in_cells() {
        let report = ValidationReport {
            items: vec![ValidationItem {
                severity: Severity::Fail,
                rule_id: "rule|id".to_string(),
                entity_name: Some("customer\norders".to_string()),
                message: "line1\r\nline2|line3".to_string(),
            }],
        };

        let output = render(&report, Path::new("config.yml"));
        assert!(output.contains("rule\\|id"));
        assert!(output.contains("customer<br>orders"));
        assert!(output.contains("line1<br>line2\\|line3"));
    }
}
