# ADR 0002: iztro Compatibility

## Status

Accepted.

## Context

`iztro-rs` is inspired by `iztro`. The project should benefit from `iztro` as a compatibility reference while still using Rust-native models and architecture.

## Decision

`iztro-rs` will validate selected chart-generation behavior against `iztro` where applicable. Compatibility is a testing target, not a requirement to copy internal architecture or public APIs exactly.

## Consequences

- Golden fixtures should identify the exact `iztro` version or commit used.
- Internal Rust APIs may differ for stronger typing and extensibility.
- Differences from `iztro` should be documented rather than hidden.
- Directly adapted logic should preserve appropriate MIT attribution.
