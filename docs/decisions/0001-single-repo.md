# ADR 0001: Keep model, guardrails, and adapters in one repository

## Status
Accepted

## Context

The platform requires tightly coordinated evolution of metadata schemas, guardrail rules, and source adapters. Splitting these into separate repositories increases version coordination overhead and complicates validation workflows.

## Decision

Maintain model artifacts, validation engine, adapters, CLI, examples, and governance documentation in a single repository with a clear internal structure.

## Consequences

- Faster end-to-end iteration and review.
- Single CI path for cross-component compatibility checks.
- Clear crate-level boundaries prevent accidental coupling while retaining one source of truth.
