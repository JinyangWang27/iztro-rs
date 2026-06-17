# Current Project Status

This document summarizes the current implemented surface after the recent `lunar-lite`, snapshot, renderer, decadal-frame, age-period, and demo work.

## Compatibility target

The current chart-generation compatibility target is `iztro@2.5.8`.

Compatibility is fixture-driven and scoped to the supported fact surface. The project does not yet claim full upstream API parity, full upstream horoscope facade payload parity, full serialization parity, or interpretation parity.

## Implemented chart-generation surface

The supported natal chart fact surface currently includes:

- typed request facades: `by_lunar` and `by_solar`;
- `lunar-lite` 1.0.0-backed solar-to-lunar conversion and normal-boundary four-pillar birth-year derivation for `by_solar`;
- leap-month and `fix_leap` handling for the supported slice;
- `BirthTime` / upstream `timeIndex` `0..=12`, including early Zi and late Zi;
- retained `Chart::birth_year()` stem-branch fact;
- twelve palace layout;
- Life Palace and Body Palace branches;
- palace heavenly stems;
- five-element bureau;
- represented typed natal stars;
- supported brightness and birth-year mutagens;
- untyped decorative runtime star families in `Palace::decorative_stars()`;
- branch-tagged typed temporal flow-star placements from explicit temporal contexts;
- decadal and yearly mutagen activation layers from explicit contexts;
- typed `DecadalFrame` derivation with 12 ten-year periods, direction, age ranges, and natal palace stem-branch facts;
- decadal temporal palace-name layout (`TemporalPalaceLayout`) attached to the selected decadal layer, keyed by `EarthlyBranch` and validated against the upstream horoscope fixture.
- typed `AgePeriod` / 小限 derivation for nominal age `1..=120`, with age context, branch/stem-branch, palace-name layout, and mutagen activations validated against the upstream horoscope fixture.
- typed `MonthlyPeriod` / 流月 derivation with independent month pillar and monthly Life palace branch facts, plus composed monthly flow-star, mutagen, and palace-name layer assembly validated against the upstream horoscope fixture.
- typed `DailyPeriod` / 流日 derivation with independent day pillar and daily Life palace branch facts, plus composed daily flow-star, mutagen, and palace-name layer assembly validated against the upstream horoscope fixture.
- typed `HourlyPeriod` / 流时 derivation with independent hour pillar and hourly Life palace branch facts, plus composed hourly flow-star, mutagen, and palace-name layer assembly validated against the upstream horoscope fixture.
- full horoscope stack assembly (`build_full_horoscope_chart` / `HoroscopeStackInput`): composes the decadal, age, yearly, monthly, daily, and hourly layers into one `HoroscopeChart` in a deterministic order, selecting the decadal period by the derived nominal age. This is supported model-level stack assembly for the implemented fields only — it is **not** identical to the upstream `FunctionalAstrolabe#horoscope` payload shape.
- yearly `yearlyDecStar` (岁前/将前十二神) as yearly-scope temporal decorative facts on the yearly layer, read through `TemporalLayer::temporal_decorative_stars()`. These are untyped: they do **not** appear in `Chart::stars()` or natal `Palace::decorative_stars()`.
- normalized `HoroscopeSupportedFieldsSnapshot` export from `HoroscopeChart`, fixture-backed against `crates/iztro/fixtures/iztro/horoscope.json` for the implemented decadal, age, yearly, monthly, daily, and hourly supported fields.
- typed `HoroscopeRuntime` projection and query helpers, fixture-backed against `crates/iztro/fixtures/iztro/horoscope_runtime.json`: `age_palace`, `palace`, `surround_palaces`, `has_horoscope_stars`, `not_have_horoscope_stars`, `has_one_of_horoscope_stars`, and `has_horoscope_mutagen`.
- serializable `HoroscopeFacadeSnapshot` export (`HoroscopeFacadeSnapshot::from_horoscope_chart`), fixture-backed against `crates/iztro/fixtures/iztro/horoscope_facade.json`: an upstream-like horoscope payload built from `HoroscopeChart`, `HoroscopeSupportedFieldsSnapshot`, `NatalFacadeSnapshot`, and `HoroscopeRuntime`. It reuses the supported-field scope blocks, embeds a minimal model-derived natal `astrolabe`, adds retained numeric target context (`solar_date`, `lunar_date` with `is_leap_month`, and `time_index`) when the chart was built by `build_full_horoscope_chart`, and exposes the Life-palace `age_palace` / `palace_projections` / `surround_palaces` projections. It is closer to the upstream `FunctionalAstrolabe#horoscope` payload shape but is **not** full package parity — complete upstream astrolabe helpers/localized labels, localized `lunarDate`/`solarDate` strings, BaZi strings, and the runtime query helpers remain deferred and are explicitly omitted.

Default/non-Zhongzhou natal output remains 66 typed natal stars. Zhongzhou natal output remains 68 typed natal stars. `represented_star_metadata_table().len() == 70` stays natal-only, while `known_star_metadata_table().len() == 170` inventories the broader upstream runtime star-name universe.

## Domain boundary decisions

The following boundaries are deliberate:

- `lunar-lite` owns canonical low-level `HeavenlyStem`, `EarthlyBranch`, and `StemBranch` primitives.
- `core` owns Zi Wei-specific NaYin and five-element bureau logic.
- `Chart` retains birth-year `StemBranch` as a natal identity fact.
- `Chart::stars()` returns typed natal `StarPlacement`s only.
- `Palace::decorative_stars()` contains untyped natal decorative runtime facts.
- `TemporalLayer::placements()` contains branch-tagged typed temporal placements.
- `MutagenActivation` records 四化 activation facts and is not modeled as a fake star.
- `HoroscopeChart` wraps an immutable natal `Chart` and additive temporal layers.

## Snapshot and rendering surface

`ChartStackSnapshot` is the renderer-neutral read model for demos and future frontends.

It preserves:

- chart identity fields such as birth context and method profile;
- birth-year stem-branch;
- natal Life/Body Palace branches and five-element bureau;
- conventional 12-palace visual grid positions;
- one natal layer plus zero or more temporal layers;
- separate cell sections for natal typed stars, natal decorative stars, scoped temporal stars, temporal decorative stars, and mutagen activations;
- per-cell temporal palace names for implemented temporal layers, kept separate from the natal palace name so temporal labels never overwrite natal spatial facts.

`render` currently provides a deterministic plain text renderer over `ChartStackSnapshot`. The top-level README and `docs/en/demo.md` show the current end-to-end flow:

```text
solar input -> by_solar -> ChartStackSnapshot -> render module plain text output
```

`HoroscopeSupportedFieldsSnapshot` is separate from `ChartStackSnapshot`: it is a compatibility/export DTO for normalized supported-field validation, not a renderer model and not the raw upstream `FunctionalAstrolabe#horoscope` JSON payload.

## Deferred work

The following remain intentionally out of scope for the current supported surface:

- full BaZi output;
- temporal decorative arrays beyond yearly `yearlyDecStar` (e.g. decadal/monthly/daily/hourly decorative arrays);
- full upstream facade serialization parity (the upstream `FunctionalAstrolabe#horoscope` payload shape);
- bindings;
- richer renderers and GUI/WASM/TUI frontends;
- feature extraction for temporal activation;
- rule evaluation;
- deterministic readings;
- narrative or LLM-assisted prose.

## Near-term direction

The next implementation work should stay incremental:

1. Continue keeping compatibility fixture-backed.
2. Build richer renderers or CLI demos on top of `ChartStackSnapshot`, not directly on `Chart` internals.
3. Full horoscope stack assembly now composes decadal, age, yearly, monthly, daily, and hourly into one stack, retains the numeric target context used for assembly, the yearly layer carries `yearlyDecStar` as temporal decorative facts, `HoroscopeSupportedFieldsSnapshot` exports the implemented supported-field surface, `HoroscopeRuntime` exposes typed runtime helper projections/queries, and `HoroscopeFacadeSnapshot` combines them with a minimal natal `astrolabe` into one upstream-like, serializable horoscope payload. Remaining horoscope work — complete upstream astrolabe helpers/localized labels, localized date strings, BaZi strings, and full upstream package parity — stays incremental and fixture-backed.
4. Only after the fact surface is stable, expand feature extraction, rules, and narrative.
