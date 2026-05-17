use crate::report::ValidationReport;

pub fn render(report: &ValidationReport) -> String {
    serde_json::to_string_pretty(report)
        .unwrap_or_else(|err| format!("{{\"error\":\"failed to render JSON: {err}\"}}"))
    serde_json::to_string_pretty(report).unwrap_or_else(|err| {
        serde_json::to_string(&serde_json::json!({
            "error": format!("failed to render JSON: {err}"),
        }))
        .unwrap_or_else(|_| "{\"error\":\"failed to render JSON\"}".to_string())
    })
}
