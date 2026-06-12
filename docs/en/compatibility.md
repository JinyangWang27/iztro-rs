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

Future compatibility fixtures may update this target only when the version change and expected output differences are documented.

For local upstream inspection, use the pinned npm reference workspace under `tools/iztro-reference`:

```bash
npm ci --prefix tools/iztro-reference
```

The committed fixture JSON files remain the compatibility source of truth.

## Current supported surface summary

The current fixture-backed chart-generation surface includes:

- typed `by_lunar` and `by_solar` request facades;
- `lunar-lite`-backed solar-to-lunar conversion for `by_solar`;
- leap-month / `fix_leap` behavior for the supported slice;
- upstream `timeIndex` `0..=12` rat-hour modeling through `BirthTime`;
- twelve palace layout, Life Palace, Body Palace, palace stems, and five-element bureau;
- represented typed natal stars, supported brightness, and birth-year mutagens;
- untyped decorative runtime star families;
- branch-tagged typed temporal flow-star placements from explicit temporal contexts;
- decadal and yearly temporal mutagen activation layers from explicit contexts;
- renderer-neutral `ChartStackSnapshot` and a deterministic plain text renderer demo.

The project still does not claim full upstream facade serialization parity, full horoscope assembly, full BaZi output, temporal decorative arrays, or interpretation/narrative parity.

## Star-name inventory

`iztro-core` keeps two separate star metadata surfaces:

- `represented_star_metadata_table().len() == 70` remains strict: it covers only the stars currently represented by chart facts, placed by Rust code, and validated by fixtures. Four represented adjective stars are algorithm-gated and appear only under `ChartAlgorithmKind::Zhongzhou`.
- `known_star_metadata_table().len() == 170` inventories the broader upstream `iztro@2.5.8` runtime star-name universe spanning the represented stars, decorative runtime arrays (`changsheng12`, `boshi12`, `suiqian12`, `jiangqian12`), and horoscope flow-star names for decadal, yearly, monthly, daily, and hourly scopes.

`represented_star_metadata_table()` stays natal-only. Decorative families are untyped `DecorativeStarPlacement`s and never appear in `Chart::stars()`. Horoscope flow stars are typed, branch-tagged `ScopedStarPlacement`s inside `TemporalLayer`s, not natal represented metadata.

The upstream locale key `xunzhong` / `旬中` is intentionally excluded because no built-in upstream `FunctionalStar` construction or `StarType` assignment was found for it in `iztro@2.5.8`. 四化 remain `Mutagen` / `MutagenActivation` facts, not `StarName` variants.

## Public facade compatibility

`by_lunar` and `by_solar` are the iztro-compatible facade entry points in `iztro-rs`. They mirror iztro's `astro.byLunar(...)` and `astro.bySolar(...)` conceptually, but use typed `LunarChartRequest` and `SolarChartRequest` request objects instead of JavaScript-style positional arguments.

`by_lunar` records the provided lunar date as chart input facts and delegates to the supported-star natal chart builder. It carries explicit leap-month semantics through `is_leap_month` and `fix_leap`; invalid leap requests are normalized against the real calendar rather than blindly echoed. The birth-year stem and branch remain explicit `by_lunar` inputs.

Birth time is represented by `BirthTime`, matching upstream `iztro` `timeIndex` values `0..=12`. `EarlyZi` (`0`) and `LateZi` (`12`) both project to `EarthlyBranch::Zi`, while branch-based request setters continue to map `Zi` to early Zi for backward compatibility.

`by_solar` validates the Gregorian/solar date, converts it to Chinese-lunisolar facts through the internal `lunar-lite` adapter, derives the birth-year `StemBranch` from the converted lunar year, sets `is_leap_month` and `fix_leap`, then delegates to `by_lunar`. It performs no chart construction of its own.

`lunar-lite` owns the canonical low-level stem/branch and sexagenary-cycle primitives (`HeavenlyStem`, `EarthlyBranch`, `StemBranch`) that `iztro-core` re-exports. `iztro-core` owns Zi Wei-specific NaYin and five-element bureau logic.

## Horoscope layer models

`iztro-core` defines model-only horoscope overlays: `HoroscopeChart` wraps an immutable natal `Chart` and holds zero or more `TemporalLayer`s, each with a non-natal `Scope`, a typed `TemporalContext`, scoped `StarPlacement`s, and `MutagenActivation`s. These models carry only temporal facts supplied explicitly by the caller, and a layer never duplicates natal placements.

A yearly mutagen overlay builder and a decadal mutagen overlay builder are available as model-level temporal activation builders. Given explicit temporal stem-branch/context facts, they produce `TemporalLayer`s whose `MutagenActivation`s apply the relevant Heavenly Stem to represented stars actually present in the natal chart. They derive no calendar facts, place no flow stars, do not mutate natal placements, and do not perform interpretation.

A scoped flow-star builder (`build_flow_star_layer`) places the horoscope flow stars (流曜) for one explicit `TemporalContext`. The placement is branch-based and does not perform horoscope palace-name derivation.

Full 大限/流年/流月/流日/流时 assembly, age ranges, period derivation, temporal palace names, and temporal decorative arrays remain deferred.

## Runtime star-family placement

Typed stars and decorative runtime entries are separate fact surfaces, and `Chart::stars()` returns typed `StarPlacement`s only.

The four decorative families (长生/博士/岁前/将前十二神) are modeled as untyped `DecorativeStarPlacement`s (`name` + `DecorativeStarFamily` + `Scope`) rather than fake-typed `StarPlacement`s. They live in `Palace::decorative_stars()` and are read through `Chart::decorative_stars()` / `Chart::decorative_star()`.

Because decorative entries are separate facts, default/non-Zhongzhou `Chart::stars()` remains 66 typed natal `StarPlacement`s and Zhongzhou `Chart::stars()` remains 68 typed natal `StarPlacement`s.

Flow-star placement is implemented through normalized `FlowStarScope` + `FlowStarBase` identity. Scope-specific upstream names such as `YunKui`, `LiuKui`, `YueKui`, `RiKui`, and `ShiKui` remain distinct `StarName` variants. `build_flow_star_layer` places the ten matrix 流曜 plus yearly 年解 as typed, branch-tagged `ScopedStarPlacement`s inside a `TemporalLayer`.

## Snapshot and render compatibility

`ChartStackSnapshot` is a renderer-neutral read model. It is not an upstream `iztro` facade payload and is not intended to match upstream JSON shape.

It preserves:

- chart identity fields such as birth context and method profile;
- natal Life/Body Palace branches and five-element bureau;
- conventional 12-palace visual grid positions;
- a stacked layer model: natal layer first, then temporal layers;
- separate cell sections for typed natal stars, decorative stars, scoped temporal stars, and mutagen activations.

`iztro-render`'s plain text renderer consumes `ChartStackSnapshot` for demos and debugging. Renderer output is deterministic but not part of chart-generation compatibility with upstream `iztro`.

## Current fixtures

The fixtures are intentionally supported-field-only. They cover the current natal, decorative, flow-star, solar/lunar conversion, leap-month, and rat-hour slices against `iztro@2.5.8` where applicable.

Key fixture groups include:

- minimal natal chart facts;
- major stars;
- minor stars;
- default and Zhongzhou adjective/helper stars;
- runtime decorative families;
- flow stars;
- e2e supported `by_lunar` cases;
- e2e supported `by_solar` cases;
- leap-month behavior;
- rat-hour behavior.

The exact fixture files live under `fixtures/iztro/`. Regeneration scripts live under `tools/iztro-reference`.

## Deferred compatibility work

Deferred surfaces include:

- full upstream facade serialization parity;
- full BaZi output;
- full horoscope assembly and temporal palace-name derivation;
- upstream yearly decorative arrays such as `yearlyDecStar`;
- bindings;
- feature extraction for temporal activation;
- rules;
- narrative and interpretation output.
