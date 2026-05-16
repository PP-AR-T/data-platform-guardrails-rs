# Architecture

`data-platform-guardrails-rs` is a single-binary CLI.

Flow:
1. Parse command-line arguments.
2. Read YAML metadata config.
3. Run deterministic validator functions.
4. Emit report as human-readable text, JSON, or Markdown.
5. Return exit code `1` if any failures exist; otherwise `0`.
