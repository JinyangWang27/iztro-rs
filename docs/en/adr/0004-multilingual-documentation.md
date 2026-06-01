# ADR 0004: Multilingual Documentation

## Status

Accepted.

## Context

The project requires English and Chinese support. English is better for open-source engineering collaboration, while Chinese is more precise for Zi Wei Dou Shu terminology.

## Decision

The project will maintain English and Chinese documentation from the beginning.

- English is canonical for engineering specifications.
- Chinese is canonical for Zi Wei Dou Shu terminology.
- Major documents should be mirrored under `docs/en` and `docs/zh-CN`.

## Consequences

- Documentation PRs should update both languages where applicable.
- Internal code should use stable keys or enums rather than localized strings.
- Localization should happen at output boundaries.
- The bilingual glossary becomes part of the project specification.
