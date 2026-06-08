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

## Phase 1: Rust workspace scaffolding

- [x] Create Rust workspace.
- [x] Add core crates:
  - [x] `iztro-core`;
  - [x] `iztro-features`;
  - [x] `iztro-rules`;
  - [x] `iztro-reading`;
  - [x] `iztro-cli`.
- [x] Add basic CI for formatting, clippy, and tests.
- [x] Add serialization and fixture-based test infrastructure.

## Phase 2: Core chart models

- [x] Define heavenly stems, earthly branches, palaces, stars, mutagens, scopes, gender, and calendar options.
- [x] Define chart, palace, and star placement models.
- [x] Define decadal and horoscope models.
- [x] Ensure implemented models are strongly typed and serializable.

Decadal and horoscope models are defined as overlays: `HoroscopeChart` wraps an
immutable natal `Chart` and holds zero or more `TemporalLayer`s, each with a
non-natal `Scope`, a typed `TemporalContext`, scoped `StarPlacement`s, and
`MutagenActivation`s. These are model-only facts supplied explicitly by the
caller; temporal star placement and calendar derivation remain deferred to
Phase 3. The first two temporal algorithms on top of these models are now
available: `build_yearly_mutagen_layer` produces a yearly `TemporalLayer` of
mutagen activations from an explicit yearly stem-branch, and
`build_decadal_mutagen_layer` produces the decadal (大限) analogue from an
explicit decadal stem-branch plus starting age. Both reuse the shared Heavenly
Stem mutagen table over the stars present in the natal chart. They are overlays
only — no flow stars, no natal mutation, no calendar/age-range/大限命宫/decadal
palace derivation — and 四化 stay modeled as `MutagenActivation` facts, not
independent stars.

## Phase 3: Chart generation compatibility

- [x] Implement minimal `by_lunar` entry point.
- [ ] Implement minimal `by_solar` entry point.
- [x] Port or reimplement the current chart-generation slice in small deterministic modules.
- [x] Add golden tests against selected `iztro` outputs for the implemented slice.
- [x] Document known differences for the implemented slice.
- [x] Add the default-algorithm natal adjective stars. All 38 of iztro 2.5.8's default-algorithm 杂曜 are placed; see the compatibility "Default-algorithm natal adjective-star set" for the per-star placement basis. The four Zhongzhou-only 杂曜 (龙德/截空/劫煞/大耗) and Zhongzhou algorithm selection remain deferred.
- [ ] Add solar-to-lunar conversion, leap-month behavior, rat-hour variants, temporal star scopes, and bindings.

Current core slice: `by_lunar` accepts explicit lunar inputs plus explicit birth-year stem and branch, builds deterministic natal chart facts, and validates minimal chart fields, fourteen major stars, fourteen supported minor stars, and the complete default-algorithm set of 38 natal adjective/helper stars against selected `iztro` 2.5.8 fixtures — 14 major + 14 minor + 38 adjective/helper = 66 represented natal stars. The Zhongzhou-only 杂曜 and Zhongzhou algorithm selection, 神煞 beyond the default `getAdjectiveStar` slice, 流曜, solar-to-lunar conversion, leap-month behavior, rat-hour variants, temporal/horoscope star scopes, and bindings remain deferred. 四化 remain `Mutagen` facts on placements, not independent stars.

## Phase 4: Feature extraction

- [x] Extract palace features.
- [x] Extract star features.
- [x] Extract natal mutagen flows.
- [x] Extract palace relations, triads, and oppositions.
- [ ] Add strength-score placeholders.
- [ ] Add temporal activation interfaces.

First slice implemented: `BasicFeatureExtractor` (`iztro-features`) converts deterministic chart facts into structured palace features, star features, natal mutagen flows, and cyclic palace relations. Star features preserve all placed star facts; the palace/domain mapping is optional metadata and is currently limited to five direct palace-domain mappings (Life, Career, Wealth, Spouse, Health), so stars elsewhere carry no domain. This is feature extraction only — no rule matching, no claims, no interpretation, and no narrative. Strength scoring and temporal activation interfaces remain deferred.

## Phase 5: Rule engine skeleton

- [ ] Define rule schema.
- [ ] Load rules from TOML.
- [ ] Match rules against extracted features.
- [ ] Emit structured claims with evidence and source metadata.
- [ ] Add deterministic unit tests for rule matching.

## Phase 6: Basic deterministic reading

- [ ] Add a small seed rule set.
- [ ] Generate domain-level claims for personality, career, wealth, and relationship.
- [ ] Render deterministic reports from structured claims.
- [ ] Keep narrative simple and evidence-based.

## Phase 7: Multi-method expansion

- [ ] Add method profiles.
- [ ] Support multiple chart-generation or feature-extraction strategies.
- [ ] Add optional rule sets for different schools or interpretation styles.
- [ ] Keep profile combinations explicit and testable.

## Phase 8: Bindings and applications

- [ ] CLI.
- [ ] Python bindings.
- [ ] WebAssembly bindings.
- [ ] TUI frontend, deferred until core models and report structures stabilize.
- [ ] GUI frontend, deferred until core models and report structures stabilize.
- [ ] Optional LLM-assisted narrative polishing.

Application frontends are intentionally deferred. Core crates should remain UI-agnostic, deterministic, and serializable so future CLI, TUI, GUI, WASM, and Python frontends can consume chart, feature, claim, evidence, and report structures without parsing narrative text.

## Release policy

Before `0.1.0`, APIs may change freely. After `0.1.0`, breaking changes should be documented in `CHANGELOG.md` and, where appropriate, ADRs.
