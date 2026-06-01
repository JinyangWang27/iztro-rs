# ADR 0005: Engineering Principles

## Status

Accepted.

## Context

`iztro-rs` will contain deterministic chart-generation logic, feature extraction, rule matching, claim aggregation, and report rendering. These areas are sensitive to boundary cases and can become difficult to maintain if responsibilities are mixed.

The project also needs to support coding-agent-assisted development. Clear engineering principles help keep generated code aligned with the architecture.

## Decision

The project will follow Rust-oriented modular design principles inspired by TDD and SOLID:

- deterministic domain logic should be tested as executable specification;
- TDD is strongly encouraged for chart generation, feature extraction, rule evaluation, and deterministic rendering;
- SOLID is interpreted through Rust idioms rather than class inheritance;
- small traits and modules are preferred over monolithic engines;
- composition, method profiles, traits, and enums should be used deliberately;
- high-level orchestration should depend on stable contracts rather than concrete implementations.

## Consequences

- Contributors should add tests for deterministic behavior.
- New abstractions should have a documented reason.
- Multi-school support should use explicit method profiles and small strategy traits where appropriate.
- Coding agents should preserve the layer boundaries and avoid creating large all-purpose traits.
- Exploratory rule data may begin without full tests, but accepted rules should eventually have rule-matching tests.
