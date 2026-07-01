# Roadmap

This roadmap is intentionally conservative. The project should first keep chart facts stable and fixture-backed, then add localized renderers and application surfaces, and only then expand interpretation depth.

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
- [x] Document static-chart-first GUI direction.
- [x] Document TUI, MCP, and 3D as downstream consumers of typed facts/projections.
- [x] Document core chart-generation architecture, chart planes, diagnostics, and invariants.

## Phase 1: Rust workspace scaffolding

- [x] Create Rust workspace.
- [x] Add workspace crates:
  - [x] `iztro` core/library crate;
  - [x] `iztro-cli`;
  - [x] `iztro-i18n`;
  - [x] `iztro-gui` local desktop prototype.
- [x] Organize the `iztro` library into internal domain modules (not separate
  crates): `core`, `features`, `rules`, `reading`, and `render`.
- [x] Add basic CI for formatting, clippy, and tests.
- [x] Add serialization and fixture-based test infrastructure.

`core` organizes its source tree into domain modules: `model` (value objects, star facts, immutable chart facts, and renderer-neutral snapshots), `placement` (deterministic 安星 placement and overlay activation builders), and `facade` (public iztro-compatible chart-generation entry points). GUI-facing read models live outside `core` in the top-level `projection` module (the static chart projections), with the top-level `facade` module orchestrating them; the dependency direction is `core <- {analysis, projection} <- facade`. Rendering, localization, and application frontends live outside placement logic.

## Phase 2: Core chart models

- [x] Define heavenly stems, earthly branches, palaces, stars, mutagens, scopes, gender, and calendar options.
- [x] Define chart, palace, and star placement models.
- [x] Define decadal and horoscope overlay models.
- [x] Ensure implemented models are strongly typed and serializable.
- [x] Inventory upstream `iztro@2.5.8` runtime star names separately from represented chart facts.
- [x] Use the canonical low-level stem/branch and sexagenary-cycle value objects from `lunar-lite` directly.
- [x] Retain factual natal four-pillar facts through the `lunar-lite` `FourPillars` value object re-exported by `iztro-rs`.
- [x] Isolate Zi Wei-specific NaYin and five-element bureau logic in `core`.
- [x] Retain birth-year `StemBranch` as a reusable natal `Chart` fact.
- [x] Add `ChartProfile` metadata so generated charts carry method profile and chart-plane facts.
- [x] Add typed palace lookup helpers and required lookup variants.
- [x] Add renderer-neutral `ChartStackSnapshot` read model.
- [x] Add compact `ChartDiagnosticSnapshot` diagnostics for structural debugging.

Decadal, age, and horoscope models are defined as typed facts and overlays. `build_decadal_frame` derives the 12-period 大限 frame from natal chart facts, `build_age_period` derives a fixture-backed 小限 period from nominal age, and `HoroscopeChart` wraps an immutable natal `Chart` with temporal layers plus optional retained target context.

## Phase 3: Chart generation compatibility

- [x] Implement minimal `by_lunar` entry point.
- [x] Implement minimal `by_solar` entry point.
- [x] Port or reimplement the current chart-generation slice in small deterministic modules.
- [x] Add golden tests against selected `iztro` outputs for the implemented slice.
- [x] Document known differences for the implemented slice.
- [x] Add default-algorithm natal adjective stars.
- [x] Add Zhongzhou-only natal adjective stars.
- [x] Add Zhongzhou Heaven/Earth/Human chart-plane support as Rust extension behaviour.
- [x] Extract natal chart-plane anchor resolution into a dedicated placement resolver.
- [x] Add invariant coverage for supported natal algorithm/plane combinations.
- [x] Place decorative runtime star families as untyped `DecorativeStarPlacement`s.
- [x] Place scoped flow stars as branch-tagged `ScopedStarPlacement`s.
- [x] Add solar-to-lunar conversion and leap-month behavior through the internal `lunar-lite` calendar adapter (`core/calendar`).
- [x] Add boundary calculation policies for solar time, year boundary, leap-month boundary, and nominal-age boundary.
- [x] Add rat-hour variants for upstream `timeIndex` `0..=12`.
- [x] Derive the birth-year stem-branch through the internal `lunar-lite` calendar adapter and retain it on `Chart`.
- [x] Retain full factual natal four pillars on `by_solar` charts as optional `Chart::four_pillars()`, with `by_lunar` left explicit and unsupported for full pillars.
- [x] Add typed decadal-frame derivation from natal chart facts.
- [x] Add decadal temporal palace-name layout on the selected decadal layer.
- [x] Add fixture-backed 小限 / age period context, palace-name layout, and mutagen overlay.
- [x] Add fixture-backed 流月 / monthly period context, palace-name layout, mutagen overlay, and flow-star layer assembly.
- [x] Add fixture-backed 流日 / daily period context, palace-name layout, mutagen overlay, and flow-star layer assembly.
- [x] Add fixture-backed 流时 / hourly period context, palace-name layout, mutagen overlay, and flow-star layer assembly.
- [x] Add full horoscope stack assembly: compose the 大限 / 小限 / 流年 / 流月 / 流日 / 流时 layers into one `HoroscopeChart` (`build_full_horoscope_chart`), selecting the decadal period by derived nominal age.
- [x] Add yearly `yearlyDecStar` (岁前/将前十二神) as yearly-scope temporal decorative facts on the yearly layer.
- [x] Add normalized `HoroscopeSupportedFieldsSnapshot` export for the implemented full horoscope supported-field fact surface.
- [x] Add typed upstream runtime query helpers and runtime palace projections.
- [x] Add an upstream-like horoscope facade payload snapshot.
- [x] Add a minimal natal astrolabe facade snapshot.
- [x] Expose factual natal four pillars in facade snapshots.
- [ ] Add temporal decorative arrays beyond yearly `yearlyDecStar`.
- [ ] Add full facade serialization parity.
- [ ] Add full BaZi interpretation/output beyond factual `by_solar` natal four pillars.

Current supported chart-generation slice: `by_lunar` accepts explicit lunar inputs plus explicit birth-year stem and branch, validates them into a retained `Chart::birth_year()` fact, builds deterministic natal chart facts, and validates supported fields against selected `iztro@2.5.8` fixtures where upstream exposes a comparable surface. `by_solar` adds `lunar-lite`-backed solar-to-lunar conversion behind the internal `core/calendar` adapter, derives the birth-year stem-branch through boundary-configurable policy (datetime-level `YearBoundary::LiChun`), retains the factual `FourPillars`, and delegates placement to `by_lunar`. `ChartCalculationConfig` is a separate calculation-policy axis from `ChartAlgorithmKind` and `ChartPlane`; it covers solar time, year boundary, leap-month boundary, and runtime nominal-age boundary. Zhongzhou Earth/Human chart planes are Rust-only extensions because upstream `iztro@2.5.8` does not expose those planes.

Calculation diagnostics are exposed as generation reports and preview/resolution snapshots. They inspect resolved input facts and runtime nominal-age facts without making `Chart` store calculation config or changing chart serialization.

## Phase 4: Snapshot, rendering, and static GUI

- [x] Add `ChartStackSnapshot` as a renderer-neutral stacked read model.
- [x] Preserve conventional 12-palace grid positions in snapshot cells.
- [x] Preserve natal and temporal fact surfaces as separate layer/cell sections.
- [x] Add `render` crate.
- [x] Add deterministic plain text chart-stack renderer.
- [x] Add runnable plain text demo from real `by_solar` input.
- [x] Add GUI-ready static chart projection for one selected natal/temporal projection (`StaticChartProjection` in `projection`).
- [x] Add renderer-neutral highlight annotation DTOs, initially empty/reserved until feature/rule layers can populate them (`HighlightProjection`).
- [x] Add local Iced static chart GUI prototype consuming `StaticChartProjection`.
- [x] Add saved-chart startup flow for the GUI.
- [x] Add GUI temporal controls backed by `static_temporal_chart_view`.
- [x] Add renderer-side 三方四正 hover/click highlighting from prepared palace relationship fields.
- [x] Add renderer-side mutagen badges from prepared mutagen facts.
- [ ] Finish first-pass GUI polish: temporal control layout, cross-period navigation edge cases, palace label alignment, saved-chart edit/delete naming flow.
- [ ] Add richer non-GUI 2D palace-grid renderer if it remains useful after GUI work.
- [ ] Add timeline frame builder that treats static chart projections as reusable time slices.
- [ ] Add optional 3D stacked temporal view.

The render layer consumes snapshots and projections; it must not generate chart facts, derive temporal periods, evaluate rules, or produce interpretation. The static GUI is the primary near-term frontend because it validates the chart projection visually. Timeline and 3D views should be later consumers of the same frame model, not new chart engines.

## Phase 5: Runtime i18n

- [x] Add `crates/iztro-i18n`.
- [x] Use Fluent resources bundled at compile time.
- [x] Support `en-US` and `zh-Hans` initially.
- [x] Make `en-US` the default runtime/GUI locale.
- [x] Preserve Simplified Chinese terminology as a first-class locale.
- [x] Add typed helpers for stars, palaces, mutagens, temporal labels, brightness labels, and shared UI strings.
- [x] Migrate existing `iztro-gui` user-facing strings so the current UI is usable in either English or Simplified Chinese.
- [x] Keep core facts language-neutral and keep localization at presentation/export boundaries.
- [ ] Add additional locales only after the English/Simplified Chinese surface remains stable.
- [ ] Audit future GUI/TUI/MCP surfaces for hardcoded user-facing strings.

The i18n crate is separate from chart generation. Facade snapshots may keep additive conventional zh-CN labels for compatibility/readability, but GUI runtime localization comes through `iztro-i18n`.

## Phase 6: TUI and MCP tooling

- [ ] Add CLI integration for selected render/view outputs.
- [ ] Add a TUI frontend over `ChartStackSnapshot` / `StaticChartProjection`.
- [ ] Define stable machine-readable query outputs for coding agents.
- [ ] Add MCP server/tooling only after the typed facade/query surface is stable.
- [ ] Expose chart facts, view snapshots, pattern hits, claims, and evidence as structured outputs.
- [ ] Avoid exposing only prose when a typed fact surface exists.

The TUI and MCP layers are tooling/application consumers. They should not duplicate placement logic, parse rendered text, or become an alternative interpretation engine.

## Phase 7: Feature extraction and patterns

- [x] Extract palace features.
- [x] Extract star features.
- [x] Extract natal mutagen flows.
- [x] Extract palace relations, triads, and oppositions.
- [x] Add first read-only `rules::pattern` slice for explainable classical pattern detection over chart facts.
- [ ] Add strength-score placeholders.
- [ ] Add temporal activation interfaces.
- [ ] Add pattern-hit interfaces suitable for later 成格 and highlight annotations.

First feature slice: `BasicFeatureExtractor` (`features`) converts deterministic chart facts into structured palace features, star features, natal mutagen flows, and cyclic palace relations. The pattern slice recognizes chart facts as structured results; it does not emit prose and does not mutate chart state.

## Phase 8: Rule engine skeleton

- [ ] Define rule schema.
- [ ] Load rules from TOML.
- [ ] Match rules against extracted features.
- [ ] Emit structured claims with evidence and source metadata.
- [ ] Emit structured pattern/highlight annotations for 成格, limit-triggered, and flow-triggered configurations.
- [ ] Add deterministic unit tests for rule matching.

Pattern and 成格 highlighting should flow from features and rules into structured annotations. Renderers may highlight the involved palaces, stars, mutagens, or temporal scopes, but they should not contain astrology-specific rule logic.

## Phase 9: Basic deterministic reading

- [ ] Add a small seed rule set.
- [ ] Generate domain-level claims for personality, career, wealth, and relationship.
- [ ] Render deterministic reports from structured claims.
- [ ] Keep narrative simple and evidence-based.

## Phase 10: Multi-method expansion

- [ ] Add richer method profile configuration.
- [ ] Support multiple chart-generation or feature-extraction strategies.
- [ ] Add optional rule sets for different schools or interpretation styles.
- [ ] Keep profile combinations explicit and testable.

## Phase 11: Bindings and applications

- [ ] Python bindings.
- [ ] WebAssembly bindings.
- [ ] GUI/WASM application.
- [ ] Optional LLM-assisted narrative polishing.

Application frontends remain consumers of typed facts, snapshots, projections, features, claims, evidence, annotations, and reports. They should not parse narrative text to recover domain facts or embed chart-generation/rule logic in UI code.

## Release policy

Before `0.1.0`, APIs may change freely. After `0.1.0`, breaking changes should be documented in `CHANGELOG.md` and, where appropriate, ADRs.
