# data-platform-guardrails-rs

A Rust workspace for validating metadata-driven lakehouse ingestion configurations, schema contracts, naming standards, dependency rules, and CI/CD deployment readiness.

## Repository layout

- `crates/dpg-model`: metadata model entities and version anchors
- `crates/dpg-rules`: validation rules and evaluation pipeline
- `crates/dpg-adapter`: source abstractions with optional SQL Server integration
- `crates/dpg-cli`: command-line validation entrypoint
- `schemas/`: versioned schema artifacts
- `examples/`: sample metadata and expected outputs
- `tests/`: integration scenario structure
- `docs/`: architecture, governance, decisions, and operations

## Quick start

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p dpg-cli -- examples/metadata/v1/valid.metadata
```
