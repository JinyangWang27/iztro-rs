# Current Project Status

This document summarizes the current implemented surface after the `lunar-lite`, fixture, facade snapshot, full horoscope stack, static-chart view model, chart-plane, diagnostic, i18n, pattern, and local GUI prototype work.

## Compatibility target

The current chart-generation compatibility target is `iztro@2.5.8`.

Compatibility is fixture-driven and scoped to the supported fact surface. The project does not yet claim full upstream API parity, full upstream horoscope facade payload parity, full serialization parity, or interpretation parity.

## Implemented chart-generation surface

The supported natal chart fact surface currently includes:

- typed request facades: `by_lunar` and `by_solar`;
- `lunar-lite` 1.0.0-backed solar-to-lunar conversion and boundary-configurable birth-year/four-pillar derivation for `by_solar`;
- leap-month and `fix_leap` handling for the supported slice, now exposed through `LeapMonthBoundary`;
- `BirthTime` / upstream `timeIndex` `0..=12`, including early Zi and late Zi;
- retained `Chart::birth_year()` stem-branch fact;
- retained optional `Chart::four_pillars()` natal fact for `by_solar` charts, using `lunar_lite::FourPillars` directly, also exposed through facade snapshots as `NatalFacadeSnapshot::four_pillars()`;
- explicit `ChartProfile` metadata (`MethodProfile` + `ChartPlane`) retained on generated `Chart` values;
- typed palace lookup helpers by palace name and branch, plus required lookup variants for invariant-sensitive code;
- `Chart::diagnostic_snapshot()` for compact structural diagnostics and invariant debugging;
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
- decadal temporal palace-name layout (`TemporalPalaceLayout`) attached to the selected decadal layer, keyed by `EarthlyBranch` and validated against the upstream horoscope fixture;
- typed `AgePeriod` / 小限 derivation for nominal age `1..=120`, with age context, branch/stem-branch, palace-name layout, and mutagen activations validated against the upstream horoscope fixture;
- typed `MonthlyPeriod` / 流月 derivation with independent month pillar and monthly Life palace branch facts, plus composed monthly flow-star, mutagen, and palace-name layer assembly validated against the upstream horoscope fixture;
- typed `DailyPeriod` / 流日 derivation with independent day pillar and daily Life palace branch facts, plus composed daily flow-star, mutagen, and palace-name layer assembly validated against the upstream horoscope fixture;
- typed `HourlyPeriod` / 流时 derivation with independent hour pillar and hourly Life palace branch facts, plus composed hourly flow-star, mutagen, and palace-name layer assembly validated against the upstream horoscope fixture;
- full horoscope stack assembly (`build_full_horoscope_chart` / `HoroscopeStackInput`): composes the decadal, age, yearly, monthly, daily, and hourly layers into one `HoroscopeChart` in a deterministic order, selecting the decadal period by the derived nominal age. This is supported model-level stack assembly for the implemented fields only — it is **not** identical to the upstream `FunctionalAstrolabe#horoscope` payload shape;
- yearly `yearlyDecStar` (岁前/将前十二神) as yearly-scope temporal decorative facts on the yearly layer, read through `TemporalLayer::temporal_decorative_stars()`. These are untyped: they do **not** appear in `Chart::stars()` or natal `Palace::decorative_stars()`;
- normalized `HoroscopeSupportedFieldsSnapshot` export from `HoroscopeChart`, fixture-backed against `crates/iztro/fixtures/iztro/horoscope.json` for the implemented decadal, age, yearly, monthly, daily, and hourly supported fields;
- typed `HoroscopeRuntime` projection and query helpers, fixture-backed against `crates/iztro/fixtures/iztro/horoscope_runtime.json`: `age_palace`, `palace`, `surround_palaces`, `has_horoscope_stars`, `not_have_horoscope_stars`, `has_one_of_horoscope_stars`, and `has_horoscope_mutagen`;
- serializable `HoroscopeFacadeSnapshot` export (`HoroscopeFacadeSnapshot::from_horoscope_chart`), fixture-backed against `crates/iztro/fixtures/iztro/horoscope_facade.json`: an upstream-like horoscope payload built from `HoroscopeChart`, `HoroscopeSupportedFieldsSnapshot`, `NatalFacadeSnapshot`, and `HoroscopeRuntime`.

`by_solar` now attaches factual natal four pillars to `Chart` through `Chart::four_pillars()`, derived by `lunar-lite` with the configured `YearBoundary` and normal month-boundary semantics. `by_lunar` remains conservative: it only receives an explicit birth-year stem/branch today, so `Chart::four_pillars()` is `None` for `by_lunar` charts until a later PR decides whether to accept explicit `FourPillars` or derive them from a normalized solar date. Full BaZi interpretation remains deferred; this implemented surface is only 年柱/月柱/日柱/时柱 fact retention.

Those factual natal four pillars are also exported through facade snapshots. `NatalFacadeSnapshot` carries an optional `NatalFacadeFourPillarsSnapshot` (`four_pillars`) reusing `lunar_lite::FourPillars` as the underlying fact: each pillar stays a machine-readable `StemBranch` with an additive conventional zh-CN `*_zh` label. The field is `Some(..)` for `by_solar`-derived charts and omitted/`None` for `by_lunar`-derived charts. This is a factual export only: 十神, 藏干, 五行 scoring, 喜用神, 成格, readings, and the rest of full BaZi interpretation remain deferred and are intentionally absent.

Default/non-Zhongzhou natal output remains 66 typed natal stars. Zhongzhou natal output remains 68 typed natal stars. `represented_star_metadata_table().len() == 70` stays natal-only, while `known_star_metadata_table().len() == 170` inventories the broader upstream runtime star-name universe.

## Chart planes (天地人三盘)

`ChartPlane` is a separate axis from `ChartAlgorithmKind`. It defaults to `Heaven` (天盘), which reproduces existing chart generation byte-for-byte.

The Zhongzhou (中州) family also supports the Earth (地盘) and Human (人盘) planes. They are generated by an anchor-aware minimal chart rebuild, not by mutating a completed chart:

- `Zhongzhou + Earth` re-anchors the Life Palace (命宫) to the Heaven chart's Body Palace (身宫) branch;
- `Zhongzhou + Human` re-anchors the Life Palace to the Heaven chart's Fortune Palace (福德宫, `PalaceName::Spirit`) branch.

After re-anchoring, palace names, palace stems, and the five-element bureau are recomputed from the new Life Palace, while the Body Palace branch keeps its original calculated value. The existing deterministic placement strategy then runs unchanged, so star placers never branch on `ChartPlane`. Plane dispatch is delegated from the `by_lunar` facade to the dedicated natal plane resolver (`core::placement::natal::plane::resolve_natal_chart_anchor`). The 中州天盘 is **not** treated as the 全书 (QuanShu) algorithm; it is the Heaven plane of the Zhongzhou family.

Requesting `Earth` or `Human` for any non-Zhongzhou family (QuanShu / Placeholder) returns `ChartError::UnsupportedChartPlane`.

`Zhongzhou + Earth` and `Zhongzhou + Human` are Rust extension behaviour rather than upstream `iztro@2.5.8` parity targets, because upstream TS `iztro` does not expose those chart planes. They are covered by structural invariants, anchor resolver tests, diagnostics, and architecture documentation instead of TS fixtures.

## Input calculation policy

`ChartCalculationConfig` is a third axis, separate from `ChartAlgorithmKind` and `ChartPlane`. It controls how a birth clock time becomes a 时辰 *before* chart generation. The clock-time entry points `by_solar_with_options` / `by_lunar_with_options` resolve the input through `core::calculation::resolve_birth_datetime` and then delegate to the existing `by_solar` / `by_lunar` paths, so `Chart` serialization is unchanged.

The calculation policy now includes `SolarTimePolicy`, `YearBoundary`, `LeapMonthBoundary`, and `NominalAgeBoundary`. Defaults preserve existing behavior: clock time, lunar-new-year cyclic-year boundary (`ChineseNewYearEve`: previous year lasts through 除夕, new year begins at 正月初一), mid-month leap-month split, and natural-year nominal age. `YearBoundary` and `LeapMonthBoundary` affect natal input normalization. `NominalAgeBoundary` affects only runtime/full-horoscope nominal-age resolution.

The clock-time facades now also expose report APIs: `by_solar_with_options_report`, `by_lunar_with_options_report`, `resolve_solar_birth_input`, `resolve_lunar_birth_input`, and `build_full_horoscope_chart_report`. These return calculation diagnostic snapshots alongside the generated chart or horoscope. They report resolved clock time, apparent-solar-time longitude/equation corrections, effective birth year, leap-month `fix_leap` mapping, and resolved nominal age while leaving normal `Chart` serialization unchanged.

The default policy (`SolarTimePolicy::ClockTime`) derives the 时辰 directly from the clock time. `SolarTimePolicy::ApparentSolarTime` applies an exact longitude correction (`4 * (longitude − timezone_meridian)` minutes, with the longitude difference normalised across the antimeridian) and may move the resolved solar date across midnight. `EquationOfTimePolicy::Approximate` is not implemented yet and returns `ChartError::UnsupportedEquationOfTimePolicy`. Apparent solar time is rejected for lunar-date input (`ChartError::ApparentSolarTimeRequiresSolarDate`).

Note: `ChartError` now derives `PartialEq` but no longer derives `Eq`, because calculation-policy validation errors can carry floating-point longitude values.

## Domain boundary decisions

The following boundaries are deliberate:

- `lunar-lite` owns canonical low-level `HeavenlyStem`, `EarthlyBranch`, `StemBranch`, and `FourPillars` primitives.
- `core` owns Zi Wei-specific NaYin and five-element bureau logic.
- `Chart` retains birth-year `StemBranch` as a natal identity fact.
- `Chart::stars()` returns typed natal `StarPlacement`s only.
- `Palace::decorative_stars()` contains untyped natal decorative runtime facts.
- `TemporalLayer::placements()` contains branch-tagged typed temporal placements.
- `TemporalLayer::temporal_decorative_stars()` contains scoped temporal decorative facts.
- `MutagenActivation` records 四化 activation facts and is not modeled as a fake star.
- `HoroscopeChart` wraps an immutable natal `Chart` and additive temporal layers.
- Facade snapshots may expose additive conventional zh-CN labels for compatibility/export convenience, but internal chart facts remain enum/value-object driven.

## Snapshot, rendering, and GUI surface

`ChartStackSnapshot` is the renderer-neutral read model for demos and future frontends.

It preserves:

- chart identity fields such as birth context and method profile;
- birth-year stem-branch and optional natal four pillars;
- natal Life/Body Palace branches and five-element bureau;
- conventional 12-palace visual grid positions;
- one natal layer plus zero or more temporal layers;
- separate cell sections for natal typed stars, natal decorative stars, scoped temporal stars, temporal decorative stars, and mutagen activations;
- per-cell temporal palace names for implemented temporal layers, kept separate from the natal palace name so temporal labels never overwrite natal spatial facts.

`render` currently provides a deterministic plain text renderer over `ChartStackSnapshot`. The end-to-end demo flow is:

```text
solar input -> by_solar -> ChartStackSnapshot -> render module plain text output
```

`StaticChartViewSnapshot` is the GUI-facing static-chart read model. It supports a 文墨天机-style 12-palace chart, selected natal/temporal overlays, prepared palace relationships for 三方四正 highlighting, mutagen display facts, and reserved highlight annotations. The local `iztro-gui` crate is an Iced desktop prototype that consumes this read model, persists saved chart inputs locally, regenerates charts deterministically through core facades, and drives temporal selection through `static_temporal_chart_view` rather than mutating the natal chart.

The GUI is currently a chart-fact viewer. It does not perform star placement, temporal derivation, mutagen calculation, 三方四正 branch arithmetic, 成格 detection, BaZi interpretation, rule matching, or narrative generation in UI code.

## Runtime localization direction

The current documentation and facade snapshots distinguish machine-readable facts from additive display labels. Runtime localization is implemented by the dedicated `crates/iztro-i18n` crate using Fluent resources.

Current implementation:

- default GUI/runtime locale: `en-US`;
- first secondary locale: `zh-Hans`;
- `iztro-i18n` owns locale parsing, Fluent bundles, fallback, and typed label helpers;
- `iztro-gui` consumes `iztro-i18n` and is usable in either English or Simplified Chinese;
- core chart models remain language-neutral;
- existing Chinese domain labels are preserved as localized output, not as internal identity.

Future i18n work should broaden coverage only at presentation/export boundaries: additional locales, more shared UI strings, and complete upstream localized-string parity remain deferred.

## Tooling and application direction

Application surfaces should be ordered by reuse value and architectural risk:

1. **Renderer-neutral facts and view models**: already the foundation for demos and GUI.
2. **Static GUI**: primary near-term frontend target, because it validates the static 12-palace view model and temporal controls visually.
3. **TUI**: useful as a lightweight terminal renderer/debugger over the same snapshots, without introducing GUI layout complexity.
4. **MCP / coding-agent tooling**: useful after the public query/export surface is stable, so agents can request chart facts, view snapshots, pattern hits, and evidence without scraping prose.
5. **Timeline and 3D views**: later consumers of reusable `StaticChartViewSnapshot` frames and structured highlights, not separate chart engines.

## Deferred work

The following remain intentionally out of scope for the current supported surface:

- full BaZi interpretation/output beyond factual `by_solar` natal four pillars;
- temporal decorative arrays beyond yearly `yearlyDecStar`;
- full upstream facade serialization parity;
- additional locales beyond English/Simplified Chinese and complete upstream localized-string parity; the desktop GUI already uses `iztro-i18n` for the current English/Simplified Chinese surface, while core models stay language-neutral and facade snapshots continue to expose additive zh-CN `*_zh` labels via `core::labels::zh_cn`;
- CLI integration beyond current examples;
- TUI frontend;
- MCP server/tooling interface;
- production-quality GUI/WASM frontend;
- timeline frame builder and 3D stacked temporal view;
- feature extraction for temporal activation;
- complete rule evaluation;
- deterministic readings;
- narrative or LLM-assisted prose.

## Near-term direction

The next implementation work should stay incremental:

1. Keep compatibility fixture-backed and avoid broad rewrites of chart placement logic.
2. Expand `crates/iztro-i18n` coverage with additional locales, more shared UI strings, and stricter UI string audits.
3. Continue improving the Iced static chart GUI on top of `StaticChartViewSnapshot`, especially saved charts, temporal navigation, layout consistency, and localized UI text.
4. Add a small TUI or CLI renderer only as a consumer of existing snapshots/view models.
5. Design MCP after the typed facade/query surface is stable enough to expose to coding agents.
6. Build timeline/3D experiments only after static chart frames and highlight annotations are stable.
7. Expand feature extraction, pattern/rule output, and narrative only after the fact and rendering surfaces remain stable.
