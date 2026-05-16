use crate::report::ValidationReport;

pub fn render(report: &ValidationReport) -> String {
    serde_json::to_string_pretty(report)
        .unwrap_or_else(|err| format!("{{\"error\":\"failed to render JSON: {err}\"}}"))
}
