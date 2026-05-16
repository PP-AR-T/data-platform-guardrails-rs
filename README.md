# data-platform-guardrails-rs

`data-platform-guardrails-rs` is a small Rust CLI for validating synthetic metadata-driven lakehouse ingestion configuration files.

It is designed as a generic enterprise data-platform guardrail and is safe for public examples.

## Why metadata validation matters

Metadata validation helps teams catch configuration errors before ingestion runs:
- missing required fields
- invalid naming conventions
- incremental load misconfiguration
- invalid raw-layer transformation flags
- broken inter-entity dependencies

## Quick start

```bash
cargo run -- validate examples/valid/customer_ingestion.yml
cargo run -- validate examples/invalid/broken_customer_ingestion.yml
cargo run -- validate examples/valid/customer_ingestion.yml --format json
cargo run -- validate examples/valid/customer_ingestion.yml --format markdown
```

## Example default output

```text
Validation report for examples/invalid/broken_customer_ingestion.yml

[FAIL] required_fields (entity: customer) source.object is required
[FAIL] naming_convention (entity: Customer-Entity) entity_name must match ^[a-z][a-z0-9_]*$
[FAIL] incremental_watermark (entity: customer) watermark_column is required for incremental loads

Summary: 0 pass, 0 warn, 3 fail
```

## Public/IP-safe scope

This repository intentionally uses synthetic entities and source/target names.
No client, employer, or production-specific details are included.

## v0.1 limitations

- Focused on a small YAML schema
- Rule set is intentionally minimal
- No external service integrations

## Short roadmap

- Additional validation rules with clear rule IDs
- Optional schema versioning support
- Optional SARIF/CI-friendly output format
