# Operating model

## Ownership

- Model definitions: `dpg-model` maintainers.
- Rule implementation: `dpg-rules` maintainers.
- Source integrations: `dpg-adapter` maintainers.
- User experience and automation: `dpg-cli` maintainers.

## Release flow

1. update model/rules/adapters as needed,
2. run local checks,
3. pass CI quality gates,
4. tag release for workspace crates.

## Contribution flow

1. pick a scope area,
2. include schema/examples/tests/docs updates for behavior changes,
3. run formatting, lint, and tests,
4. open a PR with rationale and impact notes.

## Getting started

- Validate repository: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
- Run CLI against fixture: `cargo run -p dpg-cli -- examples/metadata/v1/valid.metadata`
