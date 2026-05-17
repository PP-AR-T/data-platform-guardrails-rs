---
description: "Use when editing GitHub Actions workflows, CI pipeline steps, or CD deployment config. Covers job ordering, validation gate, and exit-code handling."
applyTo: ".github/workflows/**"
---

# CI/CD conventions

Read `docs/ci-cd-guardrails.md` before editing any workflow.

- Pipeline order: fmt-check → clippy → test → validate examples.
- Validation gate: run `cargo run -- validate <file>` on all files in `examples/valid/` and `examples/invalid/`.
  - Exit code `0` expected for valid files; exit code `1` expected for invalid files.
- Block merge when validation reports any `fail`-severity item.
- Do not add steps that bypass `-- -D warnings` on clippy.
