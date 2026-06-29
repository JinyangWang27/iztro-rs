# Rust Scaffolding Plan

> Historical note: this document records the original implementation brief for the first Rust workspace scaffolding PR. The workspace has since moved beyond initial scaffolding. For the current implemented surface, see [`current-status.md`](current-status.md), [`roadmap.md`](roadmap.md), and [`architecture.md`](architecture.md).

This document is retained as historical context because it captures the original crate-boundary intent: deterministic chart facts in core, feature extraction separate from rule matching, structured claims separate from narrative, and no premature GUI/LLM dependencies.

## Original goal

Create the initial Rust workspace for `iztro-rs` without implementing full Zi Wei Dou Shu chart-generation algorithms or interpretation rules.

The first scaffolding PR was expected to establish:

- workspace structure;
- crate boundaries;
- minimal strongly typed domain models;
- placeholder traits and modules;
- basic CI;
- minimal tests proving that the workspace compiles and layer boundaries are usable.

## Current status since scaffolding

The current workspace now includes:

```text
crates/
  iztro/            # single public library crate
    src/
      core/         # deterministic chart facts and facade entry points
      features/     # feature extraction contracts
      rules/        # rule and claim contracts
      reading/      # deterministic report structures
      render/       # deterministic snapshot renderers
  iztro-cli/        # private (publish = false) command-line entry point
```

The domain boundaries that originally lived in separate crates are now modules inside the single `iztro` crate. The project has implemented chart-generation slices, fixture-backed compatibility with `iztro@2.5.8`, `lunar-lite`-backed solar-to-lunar conversion (behind the internal `core/calendar` adapter), renderer-neutral `ChartStackSnapshot`, and a deterministic plain text renderer demo.

The original non-goals below remain useful guardrails for incomplete areas: do not present deferred functionality as stable behavior.

## Original non-goals

The scaffolding phase did not aim to implement:

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

Several of these items have since been partially implemented in scoped, fixture-backed form. Full horoscope assembly, bindings, GUI/WASM/TUI frontends, and narrative remain deferred.

## Original Rust settings

Initial recommendation:

- Rust edition: `2024`.
- License: MIT.
- MSRV: unspecified until the first release target is clearer.

Initial dependencies were expected to stay minimal:

- `serde` for serializable models;
- `serde_json` for fixture and snapshot-friendly output;
- `thiserror` for library errors;
- `anyhow` only in CLI code;
- `toml` or `toml_edit` only when rule loading is introduced.

The current workspace still follows the same low-dependency direction.

## Original crate responsibilities

### `core`

Contains deterministic chart facts and core domain models.

This crate must not contain interpretation prose, rule matching, report rendering, CLI formatting, or UI assumptions. It now also exposes renderer-neutral snapshots, but actual rendering lives outside core.

### `features`

Contains feature extraction types and traits.

This crate consumes `core` and emits structured features. It must not render reports.

### `rules`

Contains rule and claim types.

Rules should emit structured claims, not final prose.

### `reading`

Contains report structures and deterministic reading interfaces.

This crate consumes structured claims. It should not recalculate chart facts.

### `render`

Contains renderer utilities for snapshot/read-model data.

This crate consumes `ChartStackSnapshot`. It should not generate chart facts, derive temporal periods, evaluate rules, or produce interpretation.

### `iztro-cli`

Contains command-line entry points.

The CLI should expose only behavior that is explicit about its current support boundary.

## CI expectations

All PRs should continue to satisfy:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Renderer/demo PRs may additionally run:

```text
cargo run -p iztro --example plain_text
```

## Acceptance criteria that still apply

New PRs are acceptable only if:

- layer boundaries are respected;
- deterministic chart facts stay separate from interpretation;
- renderers consume snapshots/read models instead of mutating chart facts;
- incomplete behavior is clearly marked as deferred;
- no TUI, GUI, WASM, binding, rule-data, or LLM dependency is introduced without a concrete scope and design note.
