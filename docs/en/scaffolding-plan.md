# Rust Scaffolding Plan

This document is the implementation brief for the first Rust code scaffolding PR. It is intentionally operational and should be read together with `AGENTS.md`, `docs/en/architecture.md`, and `docs/en/engineering-principles.md`.

## Goal

Create the initial Rust workspace for `iztro-rs` without implementing full Zi Wei Dou Shu chart-generation algorithms or interpretation rules.

The first scaffolding PR should establish:

- workspace structure;
- crate boundaries;
- minimal strongly typed domain models;
- placeholder traits and modules;
- basic CI;
- minimal tests proving that the workspace compiles and layer boundaries are usable.

## Non-goals

Do not implement:

- full calendar conversion;
- full `by_solar` or `by_lunar` algorithms;
- complete star placement;
- full mutagen calculation;
- interpretation rule knowledge bases;
- TUI or GUI frontends;
- Python bindings;
- WebAssembly bindings;
- LLM integration;
- large generated datasets.

## Rust settings

Initial recommendation:

- Rust edition: `2024`.
- License: MIT.
- MSRV: unspecified until the first release target is clearer.

Initial dependencies should stay minimal:

- `serde` for serializable models;
- `serde_json` for fixture and snapshot-friendly output;
- `thiserror` for library errors;
- `anyhow` only in CLI code;
- `toml` or `toml_edit` only when rule loading is introduced.

Avoid adding framework dependencies before there is a concrete need.

## Workspace layout

Create this structure:

```text
Cargo.toml
crates/
  iztro-core/
    Cargo.toml
    src/
      lib.rs
      calendar.rs
      ganzhi.rs
      palace.rs
      star.rs
      mutagen.rs
      chart.rs
      profile.rs
      error.rs
  iztro-features/
    Cargo.toml
    src/
      lib.rs
      extractor.rs
      palace_features.rs
      star_features.rs
      mutagen_flows.rs
      relations.rs
      domains.rs
  iztro-rules/
    Cargo.toml
    src/
      lib.rs
      rule.rs
      condition.rs
      effect.rs
      claim.rs
      engine.rs
      loader.rs
  iztro-reading/
    Cargo.toml
    src/
      lib.rs
      report.rs
      section.rs
      renderer.rs
  iztro-cli/
    Cargo.toml
    src/
      main.rs
.github/
  workflows/
    ci.yml
```

## Crate responsibilities

### `iztro-core`

Contains deterministic chart facts and core domain models.

Expected initial types include placeholders for:

- heavenly stems;
- earthly branches;
- gender;
- time index;
- palace names;
- star names;
- star categories;
- brightness;
- mutagens;
- scopes;
- chart;
- palace;
- star placement;
- method profile metadata;
- chart errors.

This crate must not contain interpretation prose, rule matching, report rendering, CLI formatting, or UI assumptions.

### `iztro-features`

Contains feature extraction types and traits.

Expected initial items:

- `FeatureExtractor` trait;
- `ChartFeatures` placeholder;
- palace feature structures;
- star feature structures;
- mutagen flow structures;
- palace relation structures;
- domain enum or identifiers.

This crate consumes `iztro-core` and emits structured features. It must not render reports.

### `iztro-rules`

Contains rule and claim types.

Expected initial items:

- rule metadata;
- condition placeholder;
- effect placeholder;
- claim structure;
- evidence structure;
- `RuleEvaluator` trait or equivalent;
- simple rule-engine skeleton;
- loader placeholder that does not require a full rule format yet.

Rules should emit structured claims, not final prose.

### `iztro-reading`

Contains report structures and deterministic rendering interfaces.

Expected initial items:

- `ReadingReport`;
- `ReadingSection`;
- `ReportRenderer` trait;
- deterministic placeholder renderer.

This crate consumes structured claims. It should not recalculate chart facts.

### `iztro-cli`

Contains a minimal CLI entry point.

The first CLI may only print a placeholder message or version information. It should not expose incomplete astrology behavior as if it were stable.

## Initial CI

Add GitHub Actions for:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

If `-D warnings` is too strict during initial scaffolding, document the reason in the PR and keep the relaxation temporary.

## Initial tests

The first scaffolding PR should include minimal tests that verify:

- core enums serialize or round-trip where implemented;
- placeholder chart structures can be constructed;
- a dummy feature extractor can produce a `ChartFeatures` value;
- a dummy rule evaluator can emit a structured claim;
- a dummy report renderer can render a report structure.

Do not add golden `iztro` fixtures yet unless the exact compatibility target has been selected.

## Acceptance criteria

The scaffolding PR is acceptable only if:

- `cargo fmt` passes;
- `cargo clippy` passes or the PR explicitly documents a temporary exception;
- `cargo test` passes;
- all crates compile;
- public structs, enums, and traits have basic documentation;
- layer boundaries are respected;
- no TUI or GUI dependencies are introduced;
- no LLM dependencies are introduced;
- no large rule data is introduced;
- placeholder APIs clearly indicate incomplete behavior.

## Suggested Codex task

Use this prompt for the first code-scaffolding task:

```text
Read AGENTS.md and docs/en/scaffolding-plan.md.

Create the initial Rust workspace for iztro-rs.

Do not implement full chart-generation algorithms. Only scaffold strongly typed domain models, traits, placeholder modules, and minimal tests so the workspace compiles.

Follow the documented four-layer architecture:
- iztro-core
- iztro-features
- iztro-rules
- iztro-reading
- iztro-cli

Add basic CI for cargo fmt, cargo clippy, and cargo test.

Keep the PR small and focused.
```
