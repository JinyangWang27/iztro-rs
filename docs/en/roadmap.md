# Roadmap

This roadmap is intentionally conservative. The project should first establish stable architecture and compatibility tests before expanding interpretation depth.

## Phase 0: Documentation and architecture

- Project specification.
- Bilingual README.
- Architecture document.
- Compatibility policy.
- Rule engine design.
- Terminology glossary.
- ADRs for key decisions.

## Phase 1: Rust workspace scaffolding

- Create Rust workspace.
- Add core crates:
  - `iztro-core`;
  - `iztro-features`;
  - `iztro-rules`;
  - `iztro-reading`;
  - `iztro-cli`.
- Add basic CI for formatting, clippy, and tests.
- Add serialization and snapshot-test infrastructure.

## Phase 2: Core chart models

- Define heavenly stems, earthly branches, palaces, stars, mutagens, scopes, gender, and calendar options.
- Define chart, palace, star placement, decadal, and horoscope models.
- Ensure models are strongly typed and serializable.

## Phase 3: Chart generation compatibility

- Implement minimal `by_solar` and `by_lunar` entry points.
- Port or reimplement chart-generation logic in small deterministic modules.
- Add golden tests against selected `iztro` outputs.
- Document known differences.

Current core slice: `by_lunar` accepts explicit lunar inputs plus explicit
birth-year stem and branch, builds deterministic natal chart facts, and validates
minimal chart fields, fourteen major stars, fourteen supported minor stars, and
twelve supported adjective stars (红鸾/天喜/天姚/天刑/台辅/封诰/三台/八座/
龙池/凤阁/天哭/天虚) against selected `iztro` 2.5.8 fixtures. The remaining
adjective stars, solar-to-lunar conversion, leap-month behavior, rat-hour
variants, temporal star scopes, and bindings remain deferred.

## Phase 4: Feature extraction

- Extract palace features.
- Extract star features.
- Extract mutagen flows.
- Extract palace relations, triads, and oppositions.
- Add strength-score placeholders.
- Add temporal activation interfaces.

First slice implemented: `BasicFeatureExtractor` (`iztro-features`) converts
deterministic chart facts into structured palace features, star features, natal
mutagen flows, and cyclic palace relations. Star features preserve all placed
star facts; the palace/domain mapping is optional metadata and is currently
limited to five direct palace-domain mappings (Life, Career, Wealth, Spouse,
Health), so stars elsewhere carry no domain. This is feature extraction only —
no rule matching, no claims, no interpretation, and no narrative. Strength
scoring and temporal activation interfaces remain deferred.

## Phase 5: Rule engine skeleton

- Define rule schema.
- Load rules from TOML.
- Match rules against extracted features.
- Emit structured claims with evidence and source metadata.
- Add deterministic unit tests for rule matching.

## Phase 6: Basic deterministic reading

- Add a small seed rule set.
- Generate domain-level claims for personality, career, wealth, and relationship.
- Render deterministic reports from structured claims.
- Keep narrative simple and evidence-based.

## Phase 7: Multi-method expansion

- Add method profiles.
- Support multiple chart-generation or feature-extraction strategies.
- Add optional rule sets for different schools or interpretation styles.
- Keep profile combinations explicit and testable.

## Phase 8: Bindings and applications

- CLI.
- Python bindings.
- WebAssembly bindings.
- TUI frontend, deferred until core models and report structures stabilize.
- GUI frontend, deferred until core models and report structures stabilize.
- Optional LLM-assisted narrative polishing.

Application frontends are intentionally deferred. Core crates should remain UI-agnostic, deterministic, and serializable so future CLI, TUI, GUI, WASM, and Python frontends can consume chart, feature, claim, evidence, and report structures without parsing narrative text.

## Release policy

Before `0.1.0`, APIs may change freely. After `0.1.0`, breaking changes should be documented in `CHANGELOG.md` and, where appropriate, ADRs.
