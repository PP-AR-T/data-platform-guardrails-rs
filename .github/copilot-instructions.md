# data-platform-guardrails-rs

Rust CLI that validates metadata-driven lakehouse ingestion configs.

## Structure

| Path | Role |
|---|---|
| `src/validators/` | Individual validator functions |
| `src/validation.rs` | Evaluation pipeline |
| `src/report/` | Output formatters (text, JSON, Markdown) |
| `src/cli.rs` | CLI argument parsing |
| `src/config.rs` | Config types and YAML deserialization |
| `examples/` | Sample metadata (valid + invalid) |

## Commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -- validate examples/valid/customer_ingestion.yml
cargo run -- validate examples/valid/customer_ingestion.yml --format json
```

## Docs-first rule

**Before proposing any change, read the relevant doc.** Do not embed doc content in responses — link to the file instead.

| Concern | Doc |
|---|---|
| Crate boundaries and data flow | `docs/architecture.md` |
| Metadata fields and ingestion model | `docs/metadata-driven-ingestion.md` |
| CI/CD pipeline behavior | `docs/ci-cd-guardrails.md` |
| Schema versioning, breaking changes | `docs/model-governance.md` |
| Release and contribution flow | `docs/operating-model.md` |
| Migration notes | `docs/migrations.md` |
| Architecture decisions (ADRs) | `docs/decisions/` |
