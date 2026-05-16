# CI/CD guardrails

This CLI can run in pull-request workflows to fail fast on invalid ingestion metadata.

Recommended pattern:
1. Commit metadata changes.
2. Run `lakehouse-contracts-rs validate <file>` in CI.
3. Block merges when validation reports `fail` severity items.
