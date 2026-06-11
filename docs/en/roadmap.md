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

`iztro-core` organizes its source tree into domain modules: `model` (value
objects, star facts, and immutable chart facts), `placement` (deterministic 安星
placement and overlay activation builders), `facade` (public iztro-compatible
entry points), and `feature` (a boundary reserved for future derived-fact
extraction). The crate error type stays at the crate root. This is an internal
reorganization only; the public API and chart behavior are unchanged.

## Phase 2: Core chart models

- [x] Define heavenly stems, earthly branches, palaces, stars, mutagens, scopes, gender, and calendar options.
- [x] Define chart, palace, and star placement models.
- [x] Define decadal and horoscope models.
- [x] Ensure implemented models are strongly typed and serializable.
- [x] Inventory upstream `iztro@2.5.8` runtime star names separately from represented chart facts.

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

Star metadata is intentionally split. `represented_star_metadata_table().len() ==
70` covers placed and fixture-covered natal stars, including algorithm-gated
Zhongzhou-only 杂曜. `known_star_metadata_table().len() == 170` records upstream
`iztro@2.5.8` runtime star-name entries, including represented natal stars,
decorative runtime arrays, and horoscope flow-star names. Represented metadata
stays natal-only; decorative runtime entries are known untyped runtime facts,
while flow stars are known typed temporal facts placed through `TemporalLayer`.
`xunzhong` / `旬中` is excluded as locale-only, and 神煞 placement beyond the
supported natal `getAdjectiveStar` slice, yearly-scope decorative arrays, full
horoscope palace derivation, brightness expansion, and mutagen-as-star modeling
remain deferred.

## Phase 3: Chart generation compatibility

- [x] Implement minimal `by_lunar` entry point.
- [x] Implement minimal `by_solar` entry point.
- [x] Port or reimplement the current chart-generation slice in small deterministic modules.
- [x] Add golden tests against selected `iztro` outputs for the implemented slice.
- [x] Document known differences for the implemented slice.
- [x] Add the default-algorithm natal adjective stars. All 38 of iztro 2.5.8's default-algorithm 杂曜 are placed; see the compatibility "Default-algorithm natal adjective-star set" for the per-star placement basis.
- [x] Add Zhongzhou-only natal adjective stars. `ChartAlgorithmKind::Zhongzhou` places 龙德/截空/劫煞/大耗 from upstream iztro 2.5.8 fixtures, omits default 截路/空亡, and preserves the Zhongzhou 天伤/天使 swap.
- [x] Place decorative runtime star families. `by_lunar` places 长生/博士/岁前/将前十二神 as untyped `DecorativeStarPlacement`s on each palace, separate from `Chart::stars()`. 岁破 is known and can replace the seventh 岁前 slot under Zhongzhou, but it is not a supplemental thirteenth 岁前 placement.
- [x] Place scoped flow stars. `build_flow_star_layer` places the decadal/yearly/monthly/daily/hourly 流曜 (and yearly 年解) as branch-tagged `ScopedStarPlacement`s through normalized `FlowStarScope` + `FlowStarBase` identity.
- [x] Add solar-to-lunar conversion and leap-month behavior. `by_solar` converts Gregorian dates to Chinese-lunisolar facts through an internal `lunar-lite` adapter and delegates to `by_lunar`; `by_lunar` carries explicit `is_leap_month`/`fix_leap` semantics for the supported slice. Both are fixture-backed against `iztro@2.5.8`. Calendar-backend types are not exposed in the public API.
- [x] Add rat-hour variants. `BirthTime` models upstream `iztro` `timeIndex` `0..=12`, preserving early Zi (`0`) and late Zi (`12`) while keeping branch-based request APIs backward compatible.
- [ ] Add full BaZi output, full horoscope assembly, bindings, feature extraction, rules, and narrative.

Current core slice: `by_lunar` accepts explicit lunar inputs plus explicit birth-year stem and branch, builds deterministic natal chart facts, and validates minimal chart fields, fourteen major stars, fourteen supported minor stars, the complete default-algorithm set of 38 natal adjective/helper stars, and the Zhongzhou 40-star natal adjective/helper output against selected `iztro` 2.5.8 fixtures. Default/non-Zhongzhou output remains 14 major + 14 minor + 38 adjective/helper = 66 natal stars; Zhongzhou output is 14 major + 14 minor + 40 adjective/helper = 68 natal stars. The represented metadata table has 70 stars because default-only and Zhongzhou-only natal adjective stars are both represented. The decorative runtime families (长生/博士/岁前/将前十二神) and scoped 流曜 are now placed as separate facts (see below). `by_solar` adds minimal `lunar-lite`-backed solar-to-lunar conversion and delegates to `by_lunar`, which now models fixture-backed leap-month behavior (`is_leap_month`/`fix_leap`) and rat-hour variants (`BirthTime` / `timeIndex` `0..=12`) for the supported slice. Full BaZi output, full horoscope assembly (period derivation and palace-name layout), the upstream yearly decorative arrays (`yearlyDecStar`), bindings, feature extraction, rules, and narrative remain deferred. 四化 remain `Mutagen` facts on placements, not independent stars.

The four decorative runtime families are placed by `by_lunar` as untyped
`DecorativeStarPlacement`s in a separate `Palace::decorative_stars()` collection,
so `Chart::stars()` stays typed-only and its 66/68 counts are unchanged. They
resolve only through `try_known_star_metadata` (no `StarKind`). Upstream emits
exactly 12 岁前 entries: 岁破 is known and can replace 大耗 under Zhongzhou, but it
is not placed as an additional 岁前 entry.

Flow-star runtime identity is normalized by `FlowStarScope` + `FlowStarBase`:
scope-specific upstream names such as `YunKui`, `LiuKui`, `YueKui`, `RiKui`, and
`ShiKui` remain distinct `StarName` variants, and `flow_star_name(scope, base)`
exposes their shared identity. `build_flow_star_layer` now uses this to place the
ten matrix 流曜 (plus yearly 年解) as typed, branch-tagged `ScopedStarPlacement`s
inside a `TemporalLayer`. This places flow stars without changing metadata table
counts, altering natal `by_lunar` output, or modeling temporal mutagens as
stars; `represented_star_metadata_table()` stays natal-only (70).

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
