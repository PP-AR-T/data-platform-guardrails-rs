use crate::report::ValidationReport;

pub fn render(report: &ValidationReport, config_path: &str) -> String {
    let mut lines = vec![
        format!("# Validation report: `{config_path}`"),
        String::new(),
        "| severity | rule_id | entity_name | message |".to_string(),
        "|---|---|---|---|".to_string(),
    ];

    for item in &report.items {
        lines.push(format!(
            "| {} | {} | {} | {} |",
            item.severity,
            item.rule_id,
            item.entity_name.clone().unwrap_or_else(|| "-".to_string()),
            item.message.replace('|', "\\|")
        ));
    }

    let (pass, warn, fail) = report.summary_counts();
    lines.push(String::new());
    lines.push(format!(
        "**Summary:** {pass} pass, {warn} warn, {fail} fail"
    ));

    lines.join("\n")
}
