# Compatibility Policy

`iztro-rs` is inspired by `iztro` and should initially validate chart-generation behavior against `iztro` where applicable.

## What compatibility means

Compatibility means:

- selected chart-generation outputs should match `iztro` golden fixtures;
- differences should be documented;
- public Rust models should preserve the same conceptual chart facts where possible;
- tests should make compatibility explicit rather than implicit.

## What compatibility does not mean

Compatibility does not require:

- identical internal architecture;
- identical public API names;
- identical string-based data representation;
- identical narrative or interpretation output;
- support for every `iztro` feature in the first release.

## Compatibility target

The exact `iztro` compatibility target should be pinned before implementing golden tests. Candidate targets:

- the version used by `py-iztro`;
- the latest released `iztro` version;
- a specific Git commit or tag.

The selected target must be recorded in this document once decided.

## Golden tests

Golden tests should include:

- solar date chart generation;
- lunar date chart generation;
- leap-month behavior;
- early and late rat hour behavior;
- year boundary behavior;
- default algorithm behavior;
- Zhongzhou behavior if supported.

## Acknowledgement

`iztro` is licensed under the MIT License. Any directly adapted logic should keep proper attribution in source comments or documentation where appropriate.
