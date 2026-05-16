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
