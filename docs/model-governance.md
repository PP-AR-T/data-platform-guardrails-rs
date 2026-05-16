# Metadata model governance

## Versioning policy

- Schemas are stored under `schemas/<version>/`.
- Breaking changes require a new major model directory (for example, `v2`).
- Non-breaking additive updates are documented in version notes.

## Backward compatibility

- Existing model versions remain immutable after release.
- Rule logic can support multiple versions concurrently in `dpg-rules`.

## Change control

- Every model change must include:
  - updated schema artifact(s),
  - migration notes,
  - examples covering the change,
  - tests proving expected validation behavior.

## Deprecation and migration

- Deprecations are announced one model version before removal.
- Migration notes are tracked in `docs/migrations.md`.
