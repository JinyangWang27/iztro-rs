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

The fixtures are:

- `fixtures/iztro/minimal_natal_1990_05_17_chen_female.json`
- `fixtures/iztro/major_stars_1990_05_17_chen_female.json`

The minimal-natal fixture compares only fields currently implemented by
`iztro-rs`:

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

### Fourteen major stars

The `major_stars_1990_05_17_chen_female.json` fixture compares represented
facts for the fourteen major stars (主星) against iztro's per-palace
`majorStars`:

- the major-star name in each palace;
- the palace branch each star occupies;
- each major star's brightness;
- supported birth-year mutagens for represented major stars.

Placement reproduces iztro 2.5.8 (`getStartIndex` and `getMajorStar`): 紫微 is
derived from the five-element bureau and the lunar day, 天府 is its reflection
across the 寅–申 axis, and the 紫微 and 天府 series fan out by fixed offsets.
Every placed star is category `major`, scope `natal` (iztro `origin`).
Brightness reproduces iztro 2.5.8 `STARS_INFO` for the fourteen represented
major stars, preserving `de` (`得`) as `advantage` and `li` (`利`) as
`favourable`. Birth-year mutagens reproduce iztro 2.5.8 Heavenly Stem mutagens
only where the target star is one of the represented fourteen major stars.

The lunar day is supplied explicitly (`input.lunar_day`) because full calendar
conversion is deferred. The public
`build_natal_chart_with_major_stars` builder path is what the compatibility
test exercises: it first builds the minimal natal chart, then uses the derived
five-element bureau, explicit lunar day, and explicit birth year stem to place
the fourteen major stars and attach supported factual star state. This fixture
still does **not** compare feature extraction, rule-engine output, narrative
output, calendar conversion, minor stars, adjective stars, non-major stars,
non-major mutagens, decadal scopes, yearly scopes, or other temporal scopes.

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
