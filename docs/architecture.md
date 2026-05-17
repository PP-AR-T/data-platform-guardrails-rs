# Architecture

The repository is a single Rust workspace with clearly separated responsibilities:

- `crates/dpg-model`: canonical metadata entities and model version ownership.
- `crates/dpg-rules`: guardrail evaluation pipeline and rule implementations.
- `crates/dpg-adapter`: source interfaces plus concrete adapters. SQL Server support is optional.
- `crates/dpg-cli`: command interface for loading metadata and running validations.

Supporting directories:

- `schemas/`: versioned model artifacts and compatibility references.
- `examples/`: sample metadata and expected behavior.
- `tests/`: integration and end-to-end scenario organization.
- `docs/`: governance, decisions, and onboarding references.
`data-platform-guardrails-rs` is a single-binary CLI.

Flow:
1. Parse command-line arguments.
2. Read YAML metadata config.
3. Run deterministic validator functions.
4. Emit report as human-readable text, JSON, or Markdown.
5. Return exit code `1` if any failures exist; otherwise `0`.
