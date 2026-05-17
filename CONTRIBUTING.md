# Contributing

## Local checks
Run these commands before opening a pull request:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
Thanks for contributing to `data-platform-guardrails-rs`.

## Development

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Guidelines

- Keep changes small and focused.
- Use synthetic examples only.
- Avoid adding confidential data or environment details.
