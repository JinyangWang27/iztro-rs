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

Decadal and horoscope models are defined as typed facts and overlays. `build_decadal_frame` derives the 12-period 大限 frame from natal chart facts, while `HoroscopeChart` wraps an immutable natal `Chart` and holds zero or more `TemporalLayer`s, each with a non-natal `Scope`, a typed `TemporalContext`, scoped `StarPlacement`s, and `MutagenActivation`s. Temporal overlays are still model-only facts supplied explicitly by the caller; full temporal layer assembly remains deferred.

The current temporal algorithms include decadal-frame derivation, yearly/decadal mutagen layers, scope-generic flow-star placement, and decadal temporal palace-name layout. The selected decadal layer now carries a `TemporalPalaceLayout` of 12 branch-keyed temporal palace names; yearly/monthly/daily/hourly/age palace-name derivation remains deferred. The mutagen and flow-star builders are overlays only: no natal mutation, no temporal layer attachment from derived frames, and no interpretation. 四化 stay modeled as `MutagenActivation` facts, not independent stars.

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
- [x] Derive the birth-year stem-branch through `lunar-lite` 0.3.1 four-pillar APIs and retain it on `Chart`.
- [x] Add typed decadal-frame derivation from natal chart facts.
- [x] Add decadal temporal palace-name layout on the selected decadal layer.
- [ ] Add full BaZi output.
- [ ] Add full horoscope assembly: attach 大限 frames, derive 流年 / 流月 / 流日 / 流时 / 小限 periods, and add their palace-name layout.
- [ ] Add temporal decorative arrays such as upstream `yearlyDecStar`.
- [ ] Add full facade serialization parity.

Current supported chart-generation slice: `by_lunar` accepts explicit lunar inputs plus explicit birth-year stem and branch, validates them into a retained `Chart::birth_year()` fact, builds deterministic natal chart facts, and validates supported fields against selected `iztro@2.5.8` fixtures. `by_solar` adds `lunar-lite` 0.3.1-backed solar-to-lunar conversion, derives the birth-year stem-branch through the normal-boundary four-pillar API, and delegates to `by_lunar`. Default/non-Zhongzhou output remains 66 typed natal stars; Zhongzhou output remains 68 typed natal stars. Decorative runtime families, decadal frames, and scoped flow stars are separate fact surfaces, so metadata counts and natal star counts stay stable.

## Phase 4: Snapshot and rendering

- [x] Add `ChartStackSnapshot` as a renderer-neutral stacked read model.
- [x] Preserve conventional 12-palace grid positions in snapshot cells.
- [x] Preserve natal and temporal fact surfaces as separate layer/cell sections.
- [x] Add `render` crate.
- [x] Add deterministic plain text chart-stack renderer.
- [x] Add runnable plain text demo from real `by_solar` input.
- [ ] Add richer 2D palace-grid renderer.
- [ ] Add CLI integration for rendering.
- [ ] Add GUI/WASM/TUI frontends.
- [ ] Add optional 3D stacked temporal view.

The render layer consumes `ChartStackSnapshot`; it must not generate chart facts, derive temporal periods, evaluate rules, or produce interpretation. A future 文墨天机-style GUI should select temporal contexts and render a stack/projection, not mutate the natal chart.

## Phase 5: Feature extraction

- [x] Extract palace features.
- [x] Extract star features.
- [x] Extract natal mutagen flows.
- [x] Extract palace relations, triads, and oppositions.
- [ ] Add strength-score placeholders.
- [ ] Add temporal activation interfaces.

First slice implemented: `BasicFeatureExtractor` (`features`) converts deterministic chart facts into structured palace features, star features, natal mutagen flows, and cyclic palace relations. Star features preserve all placed star facts; the palace/domain mapping is optional metadata and is currently limited to five direct palace-domain mappings (Life, Career, Wealth, Spouse, Health), so stars elsewhere carry no domain. This is feature extraction only — no rule matching, no claims, no interpretation, and no narrative. Strength scoring and temporal activation interfaces remain deferred.

## Phase 6: Rule engine skeleton

- [ ] Define rule schema.
- [ ] Load rules from TOML.
- [ ] Match rules against extracted features.
- [ ] Emit structured claims with evidence and source metadata.
- [ ] Add deterministic unit tests for rule matching.

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

Application frontends remain consumers of typed facts, snapshots, features, claims, evidence, and reports. They should not parse narrative text to recover domain facts.

## Release policy

Before `0.1.0`, APIs may change freely. After `0.1.0`, breaking changes should be documented in `CHANGELOG.md` and, where appropriate, ADRs.
