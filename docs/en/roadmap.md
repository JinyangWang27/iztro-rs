# Roadmap

This roadmap is intentionally conservative. The project should first establish stable architecture and compatibility tests before expanding interpretation depth.

## Phase 0: Documentation and architecture

- [x] Project specification.
- [x] Bilingual README.
- [x] Architecture document.
- [x] Compatibility policy.
- [x] Rule engine design.
- [x] Terminology glossary.
- [x] ADRs for key decisions.
- [x] Current-status document.
- [x] Runnable plain text chart demo.

## Phase 1: Rust workspace scaffolding

- [x] Create Rust workspace.
- [x] Add core crates:
  - [x] `core`;
  - [x] `features`;
  - [x] `rules`;
  - [x] `reading`;
  - [x] `iztro-cli`;
  - [x] `render`.
- [x] Add basic CI for formatting, clippy, and tests.
- [x] Add serialization and fixture-based test infrastructure.

`core` organizes its source tree into domain modules: `model` (value objects, star facts, immutable chart facts, and renderer-neutral snapshots), `placement` (deterministic 安星 placement and overlay activation builders), and `facade` (public iztro-compatible entry points). Rendering lives outside core in `render`.

## Phase 2: Core chart models

- [x] Define heavenly stems, earthly branches, palaces, stars, mutagens, scopes, gender, and calendar options.
- [x] Define chart, palace, and star placement models.
- [x] Define decadal and horoscope overlay models.
- [x] Ensure implemented models are strongly typed and serializable.
- [x] Inventory upstream `iztro@2.5.8` runtime star names separately from represented chart facts.
- [x] Reuse `lunar-lite` for canonical low-level stem/branch and sexagenary-cycle primitives.
- [x] Isolate Zi Wei-specific NaYin and five-element bureau logic in `core`.
- [x] Retain birth-year `StemBranch` as a reusable natal `Chart` fact.
- [x] Add renderer-neutral `ChartStackSnapshot` read model.

Decadal, age, and horoscope models are defined as typed facts and overlays. `build_decadal_frame` derives the 12-period 大限 frame from natal chart facts, `build_age_period` derives a single fixture-backed 小限 period from nominal age, and `HoroscopeChart` wraps an immutable natal `Chart` with zero or more `TemporalLayer`s plus optional retained target context. Each layer has a non-natal `Scope`, a typed `TemporalContext`, scoped `StarPlacement`s, `MutagenActivation`s, and optional branch-keyed `ScopedDecorativeStarPlacement`s. `build_full_horoscope_chart` composes the decadal, age, yearly, monthly, daily, and hourly layers into one stack for the supported fact surface, retains numeric target solar/lunar/time context, and the yearly layer carries `yearlyDecStar` as yearly-scope temporal decorative facts. `HoroscopeSupportedFieldsSnapshot` exports that implemented fact surface in the fixture-normalized supported-field shape, `HoroscopeRuntime` exposes typed runtime helper projections and queries, and `HoroscopeFacadeSnapshot` combines both, the retained numeric target context, and a minimal `NatalFacadeSnapshot` as `astrolabe` into one upstream-like, serializable horoscope payload. The natal facade `astrolabe` snapshots additionally expose conventional Chinese (zh-CN) labels as additive `*_zh` fields via the deterministic `core::labels::zh_cn` lookups, while internal models stay language-neutral. Full upstream package parity — complete astrolabe helper/query methods, full multilingual/i18n infrastructure and complete upstream localized-string parity, localized date strings, and BaZi — remains deferred.

The current temporal algorithms include decadal-frame derivation, age-period derivation, monthly-period derivation, daily-period derivation, hourly-period derivation, yearly/decadal mutagen layers, scope-generic flow-star placement for the flow-star scopes, decadal temporal palace-name layout, age temporal palace-name layout, and composed monthly, daily, and hourly layer assembly. The selected decadal, age, monthly, daily, and hourly layers carry `TemporalPalaceLayout` values of 12 branch-keyed temporal palace names. Age is explicit non-natal temporal scope/context/layer-kind support only: it does not add scoped flow stars. The mutagen and flow-star builders are overlays only: no natal mutation and no interpretation. `build_full_horoscope_chart` assembles these overlays into one full stack without mutating natal facts or duplicating natal stars. 四化 stay modeled as `MutagenActivation` facts, not independent stars.

Star metadata is intentionally split. `represented_star_metadata_table().len() == 70` covers placed and fixture-covered natal stars, including algorithm-gated Zhongzhou-only 杂曜. `known_star_metadata_table().len() == 170` records upstream `iztro@2.5.8` runtime star-name entries, including represented natal stars, decorative runtime arrays, and horoscope flow-star names. Represented metadata stays natal-only; decorative runtime entries are known untyped runtime facts, while flow stars are known typed temporal facts placed through `TemporalLayer`.

## Phase 3: Chart generation compatibility

- [x] Implement minimal `by_lunar` entry point.
- [x] Implement minimal `by_solar` entry point.
- [x] Port or reimplement the current chart-generation slice in small deterministic modules.
- [x] Add golden tests against selected `iztro` outputs for the implemented slice.
- [x] Document known differences for the implemented slice.
- [x] Add the default-algorithm natal adjective stars.
- [x] Add Zhongzhou-only natal adjective stars.
- [x] Place decorative runtime star families as untyped `DecorativeStarPlacement`s.
- [x] Place scoped flow stars as branch-tagged `ScopedStarPlacement`s.
- [x] Add solar-to-lunar conversion and leap-month behavior through the internal `lunar-lite` adapter.
- [x] Add rat-hour variants for upstream `timeIndex` `0..=12`.
- [x] Derive the birth-year stem-branch through `lunar-lite` 1.0.0 four-pillar APIs and retain it on `Chart`.
- [x] Add typed decadal-frame derivation from natal chart facts.
- [x] Add decadal temporal palace-name layout on the selected decadal layer.
- [x] Add fixture-backed 小限 / age period context, palace-name layout, and mutagen overlay.
- [x] Add fixture-backed 流月 / monthly period context, palace-name layout, mutagen overlay, and flow-star layer assembly.
- [x] Add fixture-backed 流日 / daily period context, palace-name layout, mutagen overlay, and flow-star layer assembly.
- [x] Add fixture-backed 流时 / hourly period context, palace-name layout, mutagen overlay, and flow-star layer assembly.
- [x] Add full horoscope stack assembly: compose the 大限 / 小限 / 流年 / 流月 / 流日 / 流时 layers into one `HoroscopeChart` (`build_full_horoscope_chart`), selecting the decadal period by derived nominal age. Supported model-level assembly only; not upstream `FunctionalAstrolabe#horoscope` payload parity.
- [x] Add yearly `yearlyDecStar` (岁前/将前十二神) as yearly-scope temporal decorative facts on the yearly layer (`build_yearly_decorative_star_placements`). Untyped: not in `Chart::stars()` or natal `Palace::decorative_stars()`.
- [x] Add normalized `HoroscopeSupportedFieldsSnapshot` export for the implemented full horoscope supported-field fact surface. Fixture-backed against `horoscope.json`; not the raw upstream `FunctionalAstrolabe#horoscope` payload.
- [ ] Add full BaZi output.
- [ ] Add temporal decorative arrays beyond yearly `yearlyDecStar`.
- [x] Add typed upstream runtime query helpers and runtime palace projections. `HoroscopeRuntime` covers `agePalace`, `palace`, `surroundPalaces`, `hasHoroscopeStars`, `notHaveHoroscopeStars`, `hasOneOfHoroscopeStars`, and `hasHoroscopeMutagen` as typed Rust APIs backed by `horoscope_runtime.json`.
- [x] Add an upstream-like horoscope facade payload snapshot. `HoroscopeFacadeSnapshot` combines the `HoroscopeSupportedFieldsSnapshot` blocks, retained numeric target context, a minimal natal `astrolabe`, and the `HoroscopeRuntime` Life-palace projections into one serializable payload backed by `horoscope_facade.json`. The context includes numeric solar date, numeric lunar date with leap-month flag, and target time index when built through `build_full_horoscope_chart`. Closer to the upstream `FunctionalAstrolabe#horoscope` shape, but not full package parity.
- [x] Add a minimal natal astrolabe facade snapshot. `NatalFacadeSnapshot` is embedded as `astrolabe` in `HoroscopeFacadeSnapshot`, derives only from `HoroscopeChart::natal()` / `Chart`, and exposes modeled natal facts without temporal overlays or new placement logic.
- [ ] Add full facade serialization parity. Add complete upstream astrolabe helper/query methods, localized natal labels, localized `lunarDate`/`solarDate` strings, BaZi strings, decadal ranges, age arrays, and the runtime query helpers — all currently deferred from `HoroscopeFacadeSnapshot`.

Current supported chart-generation slice: `by_lunar` accepts explicit lunar inputs plus explicit birth-year stem and branch, validates them into a retained `Chart::birth_year()` fact, builds deterministic natal chart facts, and validates supported fields against selected `iztro@2.5.8` fixtures. `by_solar` adds `lunar-lite` 1.0.0-backed solar-to-lunar conversion, derives the birth-year stem-branch through the normal-boundary four-pillar API, and delegates to `by_lunar`. Default/non-Zhongzhou output remains 66 typed natal stars; Zhongzhou output remains 68 typed natal stars. Decorative runtime families, decadal frames, 小限 / age periods, 流月 / monthly periods, 流日 / daily periods, 流时 / hourly periods, scoped flow stars, yearly `yearlyDecStar` temporal decorative facts, and the normalized full-horoscope supported-fields snapshot are separate fact/export surfaces, so metadata counts and natal star counts stay stable.

## Phase 4: Snapshot and rendering

- [x] Add `ChartStackSnapshot` as a renderer-neutral stacked read model.
- [x] Preserve conventional 12-palace grid positions in snapshot cells.
- [x] Preserve natal and temporal fact surfaces as separate layer/cell sections.
- [x] Add `render` crate.
- [x] Add deterministic plain text chart-stack renderer.
- [x] Add runnable plain text demo from real `by_solar` input.
- [x] Add GUI-ready static chart view model for one selected natal/temporal projection (`StaticChartViewSnapshot` in `core::view`).
- [x] Add renderer-neutral highlight annotation DTOs, initially empty/reserved until feature/rule layers can populate them (`HighlightView`).
- [ ] Add richer 2D palace-grid renderer.
- [ ] Add 文墨天机-style static chart GUI/WASM prototype consuming the static chart view model.
- [ ] Add CLI integration for rendering.
- [ ] Add timeline frame builder that treats static chart view models as reusable time slices.
- [ ] Add optional 3D stacked temporal view.

The render layer consumes snapshots and view models; it must not generate chart facts, derive temporal periods, evaluate rules, or produce interpretation. A future 文墨天机-style GUI should first reproduce the static 12-palace chart from a renderer-neutral static chart view model. That same static view model should later serve as one frame in a temporal sequence, allowing a 3D frontend to place frames along a time axis without changing core chart-generation logic.

## Phase 5: Feature extraction

- [x] Extract palace features.
- [x] Extract star features.
- [x] Extract natal mutagen flows.
- [x] Extract palace relations, triads, and oppositions.
- [ ] Add strength-score placeholders.
- [ ] Add temporal activation interfaces.
- [ ] Add pattern-hit interfaces suitable for later 成格 and highlight annotations.

First slice implemented: `BasicFeatureExtractor` (`features`) converts deterministic chart facts into structured palace features, star features, natal mutagen flows, and cyclic palace relations. Star features preserve all placed star facts; the palace/domain mapping is optional metadata and is currently limited to five direct palace-domain mappings (Life, Career, Wealth, Spouse, Health), so stars elsewhere carry no domain. This is feature extraction only — no rule matching, no claims, no interpretation, and no narrative. Strength scoring, temporal activation interfaces, and pattern-hit interfaces remain deferred.

## Phase 6: Rule engine skeleton

- [ ] Define rule schema.
- [ ] Load rules from TOML.
- [ ] Match rules against extracted features.
- [ ] Emit structured claims with evidence and source metadata.
- [ ] Emit structured pattern/highlight annotations for 成格, limit-triggered, and flow-triggered configurations.
- [ ] Add deterministic unit tests for rule matching.

Pattern and 成格 highlighting should flow from features and rules into structured annotations. Renderers may highlight the involved palaces, stars, mutagens, or temporal scopes, but they should not contain astrology-specific rule logic.

## Phase 7: Basic deterministic reading

- [ ] Add a small seed rule set.
- [ ] Generate domain-level claims for personality, career, wealth, and relationship.
- [ ] Render deterministic reports from structured claims.
- [ ] Keep narrative simple and evidence-based.

## Phase 8: Multi-method expansion

- [ ] Add richer method profile configuration.
- [ ] Support multiple chart-generation or feature-extraction strategies.
- [ ] Add optional rule sets for different schools or interpretation styles.
- [ ] Keep profile combinations explicit and testable.

## Phase 9: Bindings and applications

- [ ] CLI.
- [ ] Python bindings.
- [ ] WebAssembly bindings.
- [ ] TUI frontend.
- [ ] GUI frontend.
- [ ] Optional LLM-assisted narrative polishing.

Application frontends remain consumers of typed facts, snapshots, view models, features, claims, evidence, annotations, and reports. They should not parse narrative text to recover domain facts or embed chart-generation/rule logic in UI code.

## Release policy

Before `0.1.0`, APIs may change freely. After `0.1.0`, breaking changes should be documented in `CHANGELOG.md` and, where appropriate, ADRs.
