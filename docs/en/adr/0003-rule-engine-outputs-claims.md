# ADR 0003: Rule Engine Outputs Structured Claims

## Status

Accepted.

## Context

A naive interpretation engine can turn each matched rule directly into prose. That approach is hard to test, hard to aggregate, and prone to contradictory reports.

## Decision

Rules will emit structured claims instead of final narrative text. A claim includes domain, themes, polarity, strength, evidence, counter-evidence, and source metadata.

## Consequences

- Claims can be unit tested.
- Multiple claims can be aggregated before rendering.
- Conflict resolution can happen before narrative generation.
- The same claims can be rendered in multiple languages or styles.
- LLM-assisted rendering, if added later, will consume claims rather than raw chart JSON.
