---
description: "Use when making model changes, adding metadata fields, versioning schemas, or reviewing breaking changes to config types. Covers backward compatibility, change-control checklist, and deprecation policy."
applyTo: "examples/**"
---

# Schema and model governance

Read `docs/model-governance.md` before any structural change to config types or examples.

- Breaking changes to the metadata model require a new major schema version.
- Every model change must include:
  - updated example(s) in `examples/`
  - migration notes in `docs/migrations.md`
  - tests proving the new validation behavior
- Non-breaking additive changes must keep existing examples passing.
- Deprecations are announced one model version before removal.
- Architecture decisions go in `docs/decisions/` as numbered ADR files.
