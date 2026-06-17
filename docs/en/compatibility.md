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
- `lunar-lite` 0.3.1-backed solar-to-lunar conversion and normal-boundary four-pillar birth-year derivation for `by_solar`;
- leap-month / `fix_leap` behavior for the supported slice;
- upstream `timeIndex` `0..=12` rat-hour modeling through `BirthTime`;
- retained birth-year `StemBranch`, twelve palace layout, Life Palace, Body Palace, palace stems, and five-element bureau;
- represented typed natal stars, supported brightness, and birth-year mutagens;
- untyped decorative runtime star families;
- branch-tagged typed temporal flow-star placements from explicit temporal contexts;
- decadal and yearly temporal mutagen activation layers from explicit contexts;
- typed `DecadalFrame` derivation with 12 ten-year periods, direction, age ranges, and natal palace stem-branch facts;
- typed `MonthlyPeriod` derivation and composed monthly layer assembly, including independent month pillar and monthly temporal Life palace layout facts;
- typed `DailyPeriod` derivation and composed daily layer assembly, including independent day pillar and daily temporal Life palace layout facts;
- typed `HourlyPeriod` derivation and composed hourly layer assembly, including independent hour pillar and hourly temporal Life palace layout facts;
- full horoscope stack assembly (`build_full_horoscope_chart`) composing the decadal, age, yearly, monthly, daily, and hourly layers into one `HoroscopeChart` for the supported fact surface;
- yearly `yearlyDecStar` (岁前/将前十二神) modeled as yearly-scope temporal decorative facts on the yearly layer;
- normalized `HoroscopeSupportedFieldsSnapshot` export for the implemented full horoscope supported-field fact surface, fixture-backed against `crates/iztro/fixtures/iztro/horoscope.json`;
- model-level `HoroscopeRuntime` helpers for typed palace projections (`age_palace`, `palace`, `surround_palaces`) and typed runtime queries (`has_horoscope_stars`, `not_have_horoscope_stars`, `has_one_of_horoscope_stars`, `has_horoscope_mutagen`), fixture-backed against `crates/iztro/fixtures/iztro/horoscope_runtime.json`;
- serializable `HoroscopeFacadeSnapshot` export that combines the modeled full horoscope surface — the `HoroscopeSupportedFieldsSnapshot` blocks, a minimal model-derived natal `astrolabe` snapshot, the `HoroscopeRuntime` Life-palace projections, and the retained numeric target context (solar date, lunar date with leap-month flag, and target time index) — into one upstream-like payload, fixture-backed against `crates/iztro/fixtures/iztro/horoscope_facade.json`;
- renderer-neutral `ChartStackSnapshot` and a deterministic plain text renderer demo.

The project now provides an upstream-like horoscope facade snapshot for the modeled full horoscope surface, built from `HoroscopeChart`, `HoroscopeSupportedFieldsSnapshot`, `NatalFacadeSnapshot`, and `HoroscopeRuntime`. It is closer to the TS `FunctionalAstrolabe#horoscope` payload shape but is still **not** full package parity: the embedded `astrolabe` is intentionally minimal and contains only modeled natal facts; complete upstream astrolabe helper/query methods, localized labels, BaZi strings, decadal ranges, age arrays, bindings, renderers, rules, and narrative remain deferred.

## Star-name inventory

`core` keeps two separate star metadata surfaces:

- `represented_star_metadata_table().len() == 70` remains strict: it covers only the stars currently represented by chart facts, placed by Rust code, and validated by fixtures. Four represented adjective stars are algorithm-gated and appear only under `ChartAlgorithmKind::Zhongzhou`.
- `known_star_metadata_table().len() == 170` inventories the broader upstream `iztro@2.5.8` runtime star-name universe spanning the represented stars, decorative runtime arrays (`changsheng12`, `boshi12`, `suiqian12`, `jiangqian12`), and horoscope flow-star names for decadal, yearly, monthly, daily, and hourly scopes.

`represented_star_metadata_table()` stays natal-only. Decorative families are untyped `DecorativeStarPlacement`s and never appear in `Chart::stars()`. Horoscope flow stars are typed, branch-tagged `ScopedStarPlacement`s inside `TemporalLayer`s, not natal represented metadata.

The upstream locale key `xunzhong` / `旬中` is intentionally excluded because no built-in upstream `FunctionalStar` construction or `StarType` assignment was found for it in `iztro@2.5.8`. 四化 remain `Mutagen` / `MutagenActivation` facts, not `StarName` variants.

## Public facade compatibility

`by_lunar` and `by_solar` are the iztro-compatible facade entry points in `iztro-rs`. They mirror iztro's `astro.byLunar(...)` and `astro.bySolar(...)` conceptually, but use typed `LunarChartRequest` and `SolarChartRequest` request objects instead of JavaScript-style positional arguments.

`by_lunar` records the provided lunar date as chart input facts and delegates to the supported-star natal chart builder. It carries explicit leap-month semantics through `is_leap_month` and `fix_leap`; invalid leap requests are normalized against the real calendar rather than blindly echoed. The birth-year stem and branch remain explicit `by_lunar` inputs and are validated into the chart's retained birth-year `StemBranch`.

Birth time is represented by `BirthTime`, matching upstream `iztro` `timeIndex` values `0..=12`. `EarlyZi` (`0`) and `LateZi` (`12`) both project to `EarthlyBranch::Zi`, while branch-based request setters continue to map `Zi` to early Zi for backward compatibility.

`by_solar` validates the Gregorian/solar date, converts it to Chinese-lunisolar facts through the internal `lunar-lite` adapter, derives the birth-year `StemBranch` through `lunar-lite` 0.3.1's `four_pillars_from_solar_date_with_options` with `YearDivide::Normal` and `MonthDivide::Normal`, sets `is_leap_month` and `fix_leap`, then delegates to `by_lunar`. It performs no chart construction of its own.

`lunar-lite` owns the canonical low-level stem/branch and sexagenary-cycle primitives (`HeavenlyStem`, `EarthlyBranch`, `StemBranch`) that `core` re-exports. `core` owns Zi Wei-specific NaYin and five-element bureau logic.

## Horoscope layer models

`core` defines model-only horoscope overlays: `HoroscopeChart` wraps an immutable natal `Chart` and holds zero or more `TemporalLayer`s, each with a non-natal `Scope`, a typed `TemporalContext`, scoped `StarPlacement`s, and `MutagenActivation`s. These models carry only temporal facts supplied explicitly by the caller, and a layer never duplicates natal placements.

A yearly mutagen overlay builder and a decadal mutagen overlay builder are available as model-level temporal activation builders. Given explicit temporal stem-branch/context facts, they produce `TemporalLayer`s whose `MutagenActivation`s apply the relevant Heavenly Stem to represented stars actually present in the natal chart. They derive no calendar facts, place no flow stars, do not mutate natal placements, and do not perform interpretation.

A scoped flow-star builder (`build_flow_star_layer`) places the horoscope flow stars (流曜) for one explicit `TemporalContext`. The placement is branch-based and does not perform horoscope palace-name derivation.

`build_decadal_frame` derives the typed 12-period 大限 frame from a natal chart: it starts at the natal Life Palace, uses the five-element bureau number as the first start age, walks forward for Yang male or Yin female charts and reverse otherwise, and records each period's natal palace branch/name/stem plus stem-branch pair. It does not create a `TemporalLayer`, attach mutagens or flow stars, derive temporal palace names, or render prose.

`build_monthly_period` derives one 流月 period from natal facts plus an explicit target solar date and target `BirthTime`. It follows upstream `FunctionalAstrolabe#horoscope`: the monthly `StemBranch` comes from the target date's normal-boundary month pillar, while the monthly temporal Life palace index is derived separately from the target yearly branch, natal lunar month, natal birth-hour branch, and target lunar month. `build_monthly_horoscope_layer` composes that period into a `Scope::Monthly` layer with monthly flow stars, monthly mutagen activations, and the monthly `TemporalPalaceLayout`. It does not assemble daily/hourly layers or attach temporal decorative arrays.

`build_daily_period` derives one 流日 period from natal facts plus an explicit target solar date and target `BirthTime`. It follows upstream `FunctionalAstrolabe#horoscope`: the daily `StemBranch` comes from the target date's normal-boundary day pillar, while the daily temporal Life palace index is derived separately by counting on from the 流月 palace index by the target lunar day. The daily stem-branch branch is therefore independent from the daily Life palace branch. `build_daily_horoscope_layer` composes that period into a `Scope::Daily` layer with daily flow stars, daily mutagen activations, and the daily `TemporalPalaceLayout`. It does not assemble the full horoscope stack or attach temporal decorative arrays.

`build_hourly_period` derives one 流时 period from natal facts plus an explicit target solar date and target `BirthTime`. It follows upstream `FunctionalAstrolabe#horoscope`: the hourly `StemBranch` comes from the target date/time's normal-boundary hour pillar, while the hourly temporal Life palace index is derived separately by counting on from the 流日 palace index by the target double-hour. The hourly stem-branch branch is therefore independent from the hourly Life palace branch. `build_hourly_horoscope_layer` composes that period into a `Scope::Hourly` layer with hourly flow stars, hourly mutagen activations, and the hourly `TemporalPalaceLayout`. It does not attach temporal decorative arrays.

`build_full_horoscope_chart` composes the decadal, age, yearly, monthly, daily, and hourly layers into one `HoroscopeChart` for a target solar date/time (`HoroscopeStackInput`). It derives the target lunar date, derives the nominal age (虚岁 = target lunar year − natal birth lunar year + 1), selects the covering decadal period by that nominal age (never a hard-coded index), pushes the six layers in the deterministic order decadal → age → yearly → monthly → daily → hourly, and retains the target context on the chart. The retained context records the numeric target solar date, numeric target lunar date, the target lunar leap-month flag reported by `lunar-lite`, and the upstream target `timeIndex`. The yearly layer additionally carries `yearlyDecStar` as yearly-scope temporal decorative facts. This is supported model-level stack assembly for the implemented fields only: it does not reproduce the upstream `FunctionalAstrolabe#horoscope` payload shape.

`HoroscopeSupportedFieldsSnapshot` exports the implemented full horoscope fact surface from a `HoroscopeChart` in the normalized snake_case supported-field shape used by `horoscope.json`: decadal, age, yearly, monthly, daily, and hourly blocks with period index, stem/branch, Yin-first palace names, four-transform targets, flow stars where implemented, nominal age, yearly 年解 branch, and yearly `yearlyDecStar`. It is intended for deterministic compatibility validation. It omits raw Chinese labels, upstream runtime query helpers, runtime palace projections, embedded natal astrolabe payloads, and full upstream facade JSON parity.

`HoroscopeRuntime` provides typed Rust equivalents for the upstream runtime helper slice. `age_palace`, `palace`, and `surround_palaces` project by branch: natal palace name/stem/star facts remain available, and temporal palace labels are additive instead of overwriting natal identity. `has_horoscope_stars`, `not_have_horoscope_stars`, `has_one_of_horoscope_stars`, and `has_horoscope_mutagen` are fixture-backed against upstream `iztro@2.5.8`. They query existing model facts only; they do not duplicate natal stars into temporal layers, attach new placements, mutate natal chart facts, or change placement semantics. Full facade payload parity remains deferred.

`HoroscopeFacadeSnapshot` is the serializable facade/export layer, not a new engine layer: `HoroscopeFacadeSnapshot::from_horoscope_chart` wraps the already-modeled facts into one deterministic payload that moves toward the upstream `FunctionalAstrolabe#horoscope` shape. It reuses the `HoroscopeSupportedFieldsSnapshot` decadal/age/yearly/monthly/daily/hourly blocks verbatim (flattened to the top level), embeds `NatalFacadeSnapshot` as `astrolabe` from `HoroscopeChart::natal()`, adds a numeric `context` from `HoroscopeChart::target_context()` when available, and reuses `HoroscopeRuntime` for the `age_palace`, `palace_projections`, and `surround_palaces` Life-palace projections — each preserving the natal-versus-temporal split (natal palace name/stem/stars stay separate from the period's temporal palace name, temporal stars, and temporal mutagen activations). The minimal `astrolabe` contains gender, birth-year stem/branch, five-element bureau, Life/Body Palace branches, twelve natal palaces, palace branch/name/stem/roles, typed natal stars, and natal decorative stars. Its context contains `solar_date`, `lunar_date` (including `is_leap_month`), and `time_index` for charts built by `build_full_horoscope_chart`. Manually assembled charts without retained target context keep the older lunar-only fallback from temporal layer contexts and omit solar/time fields. The snapshot is fixture-backed against `horoscope_facade.json`. It adds no placement logic, and it stays explicit about deferred fields: localized upstream `lunarDate` and `solarDate` strings, complete upstream astrolabe helper/query methods, localized natal labels, BaZi strings, decadal ranges, age arrays, the runtime query helpers, and full upstream package parity remain deferred.

### Facade/export star ordering

Core engine placement facts are **order-independent**. A palace is the *set* of stars placed in it, so the core placement compatibility tests compare star sets, not array order — Rust and upstream TS `iztro` do not necessarily emit a palace's stars in the same `Vec` order, and that incidental order carries no semantic meaning.

The facade/export layer must not depend on that accidental order. `NatalFacadePalaceSnapshot` therefore imposes one stable, deterministic Rust-side ordering on each exported palace's star arrays:

- typed natal stars are ordered by `(kind, name, brightness, mutagen)`;
- decorative natal stars are ordered by `(family, name)`.

The keys use the canonical declaration-order `Ord` of `StarKind`, `StarName`, `Brightness`, `Mutagen`, and `DecorativeStarFamily`; those derives are a sort key only and carry no astrological ranking. Repeated construction of the same facade snapshot is byte-identical, and the policy is pinned by `facade_star_ordering.rs`.

This is a Rust-side canonical order, **not** a claim of upstream TS `iztro` palace-star array-order parity, which remains deferred.

## Runtime star-family placement

Typed stars and decorative runtime entries are separate fact surfaces, and `Chart::stars()` returns typed `StarPlacement`s only.

The four decorative families (长生/博士/岁前/将前十二神) are modeled as untyped `DecorativeStarPlacement`s (`name` + `DecorativeStarFamily` + `Scope`) rather than fake-typed `StarPlacement`s. They live in `Palace::decorative_stars()` and are read through `Chart::decorative_stars()` / `Chart::decorative_star()`.

Because decorative entries are separate facts, default/non-Zhongzhou `Chart::stars()` remains 66 typed natal `StarPlacement`s and Zhongzhou `Chart::stars()` remains 68 typed natal `StarPlacement`s.

Flow-star placement is implemented through normalized `FlowStarScope` + `FlowStarBase` identity. Scope-specific upstream names such as `YunKui`, `LiuKui`, `YueKui`, `RiKui`, and `ShiKui` remain distinct `StarName` variants. `build_flow_star_layer` places the ten matrix 流曜 plus yearly 年解 as typed, branch-tagged `ScopedStarPlacement`s inside a `TemporalLayer`.

Yearly `yearlyDecStar` (岁前/将前十二神) is modeled as yearly-scope temporal decorative facts. `build_yearly_decorative_star_placements` reuses the natal 岁前/将前 rule anchored on the flowing-year branch and emits branch-keyed `ScopedDecorativeStarPlacement`s scoped to `Scope::Yearly`. These are untyped decorative facts: they are **not** typed stars, never appear in `Chart::stars()`, and are kept separate from the natal `Palace::decorative_stars()`. `build_yearly_horoscope_layer` (and therefore `build_full_horoscope_chart`) attaches them to the yearly `TemporalLayer`, read through `TemporalLayer::temporal_decorative_stars()`. Snapshots expose them on the yearly layer through `PalaceLayerCellSnapshot::temporal_decorative_stars()`, distinct from natal decorative facts.

## Snapshot and render compatibility

`ChartStackSnapshot` is a renderer-neutral read model. It is not an upstream `iztro` facade payload and is not intended to match upstream JSON shape.

It preserves:

- chart identity fields such as birth context and method profile;
- birth-year stem-branch;
- natal Life/Body Palace branches and five-element bureau;
- conventional 12-palace visual grid positions;
- a stacked layer model: natal layer first, then temporal layers;
- separate cell sections for typed natal stars, decorative stars, scoped temporal stars, and mutagen activations.

`render`'s plain text renderer consumes `ChartStackSnapshot` for demos and debugging. Renderer output is deterministic but not part of chart-generation compatibility with upstream `iztro`.

`HoroscopeSupportedFieldsSnapshot` is a separate compatibility/export DTO, not a renderer model. Use it when comparing the implemented horoscope supported fields against fixtures; use `ChartStackSnapshot` when rendering natal and temporal layers.

## Current fixtures

The fixtures are intentionally supported-field-only. They cover the current natal, decorative, flow-star, solar/lunar conversion, leap-month, and rat-hour slices against `iztro@2.5.8` where applicable.

Key fixture groups include:

- minimal natal chart facts;
- major stars;
- minor stars;
- default and Zhongzhou adjective/helper stars;
- runtime decorative families;
- flow stars;
- horoscope monthly period/layer cases;
- horoscope daily period/layer cases;
- horoscope hourly period/layer cases;
- full horoscope stack assembly cases;
- normalized full-horoscope supported-fields snapshot cases;
- horoscope facade payload snapshot cases;
- e2e supported `by_lunar` cases;
- e2e supported `by_solar` cases;
- leap-month behavior;
- rat-hour behavior.

The exact fixture files live under `crates/iztro/fixtures/iztro/`. Regeneration scripts live under `tools/iztro-reference`.

## Deferred compatibility work

Deferred surfaces include:

- full upstream facade serialization parity;
- full BaZi output;
- bindings;
- feature extraction for temporal activation;
- rules;
- narrative and interpretation output.
