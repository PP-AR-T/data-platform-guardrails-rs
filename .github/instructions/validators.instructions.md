---
description: "Use when writing, editing, reviewing, or testing validator functions, validation rules, or the evaluation pipeline. Covers rule structure, naming, error reporting, and new-rule checklist."
applyTo: "src/validators/**"
---

# Validator conventions

Read `docs/architecture.md` and `docs/metadata-driven-ingestion.md` before adding or changing a rule.

- Each validator is a pure function — no I/O, no side effects.
- Validators receive the parsed config and return `Vec<ValidationResult>`.
- Naming: `validate_<concern>` (e.g. `validate_naming`, `validate_incremental`).
- Every new rule requires:
  - implementation in `src/validators/`
  - registration in `src/validators/mod.rs`
  - a valid example in `examples/valid/`
  - an invalid example in `examples/invalid/`
  - at least one unit test
  - docs update if behavior is externally visible
