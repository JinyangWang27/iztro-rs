# ADR 0001: Layered Architecture

## Status

Accepted.

## Context

`iztro-rs` needs to support deterministic chart generation, structured feature extraction, rule-based interpretation, and human-readable reports. Mixing these concerns would make the project difficult to test and extend.

## Decision

The project will use four conceptual layers:

1. Core Chart Layer.
2. Feature Extraction Layer.
3. Rule Engine Layer.
4. Narrative Layer.

Each layer should consume the layer below it and avoid leaking concerns downward.

## Consequences

- Chart generation can be tested without interpretation logic.
- Rule matching can be tested without natural-language rendering.
- Narrative output can be changed without changing chart algorithms.
- Coding agents can scaffold each layer independently.
