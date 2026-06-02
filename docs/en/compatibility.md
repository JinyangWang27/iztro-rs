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

The current compatibility target is:

- `iztro` npm package version `2.5.8`.

Future compatibility fixtures may update this target only when the version
change and expected output differences are documented.

## Current fixtures

The first fixture is:

- `fixtures/iztro/minimal_natal_1990_05_17_chen_female.json`

This fixture compares only fields currently implemented by `iztro-rs`:

- birth time;
- gender;
- life palace branch;
- body palace branch;
- palace branches;
- palace names;
- palace heavenly stems;
- five-element bureau (五行局).

Palace heavenly stems are generated from the birth year stem via the classical
起五行寅例 and compared against iztro's per-palace `heavenlyStem`. The
five-element bureau is compared against iztro's `fiveElementsClass` (`火六局`
maps to the `fire6` bureau).

The birth year stem is currently provided explicitly in the fixture input
(`birth_year_stem`) because Gregorian-to-ganzhi year conversion is deferred.

It intentionally does not compare stars, brightness, mutagens, decadal scopes,
yearly scopes, or narrative output.

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
