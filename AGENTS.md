# AGENTS.md

This file gives instructions to coding agents working on `iztro-rs`.

## Project status

`iztro-rs` is in early design and scaffolding. The repository currently prioritizes architecture, documentation, deterministic domain modeling, and testable interfaces over feature completeness.

Do not assume the project already has a stable public API.

## Required reading

Before making substantial changes, read:

- `README.md`
- `docs/en/project-spec.md`
- `docs/en/architecture.md`
- `docs/en/engineering-principles.md`
- `docs/en/rule-engine.md`
- `docs/en/compatibility.md`
- `docs/en/terminology.md`

For Chinese terminology, also check:

- `docs/zh-CN/terminology.md`

## Architecture boundaries

Preserve the four-layer architecture:

1. **Core Chart Layer**: deterministic chart facts and domain models.
2. **Feature Extraction Layer**: semantic features derived from chart facts.
3. **Rule Engine Layer**: structured claims with evidence.
4. **Narrative Layer**: human-readable report rendering.

Do not mix responsibilities across layers.

Examples:

- Core chart models must not contain interpretation prose.
- Feature extraction must not perform final rule evaluation.
- Rules must emit structured claims, not long narrative paragraphs.
- Narrative rendering must not recalculate chart-generation facts.

## Engineering principles

Follow Rust-oriented modular design inspired by TDD and SOLID:

- prefer strong types over raw strings;
- prefer small modules and small traits over monolithic engines;
- use composition over inheritance-style thinking;
- use enums for closed sets and traits for extensible strategies;
- keep public contracts explicit;
- make deterministic behavior testable;
- avoid large abstractions without tests or documented rationale.

## Testing expectations

For deterministic logic, prefer test-driven development:

1. write or update a failing test;
2. implement the smallest deterministic change;
3. refactor while keeping tests green.

Expected test categories:

- unit tests for pure helpers and small deterministic functions;
- golden tests against selected `iztro` fixtures;
- snapshot tests for serialized chart outputs and structured claims;
- rule tests for feature input to claim output;
- integration tests for chart to features to claims to deterministic report.

If tests cannot be added yet because the workspace is still being scaffolded, document the intended test case in the PR body or relevant docs.

## Compatibility expectations

`iztro-rs` is inspired by `iztro` and should validate selected chart-generation behavior against `iztro` where applicable.

Compatibility does not require copying `iztro` internals or public APIs exactly. Prefer Rust-native models and explicit compatibility fixtures.

When adding golden tests, record the exact `iztro` version, tag, or commit used.

## Rule-engine expectations

Rules should produce structured claims, not final prose.

A claim should preserve:

- domain;
- themes;
- polarity;
- strength;
- evidence;
- counter-evidence where applicable;
- source metadata.

Exploratory rule data may start as draft data, but accepted rules should eventually have rule-matching tests.

## Multilingual expectations

English documentation is canonical for engineering specifications. Chinese documentation is canonical for Zi Wei Dou Shu terminology.

When adding or changing major docs, update both English and Chinese versions where applicable. If only one language is updated, explain why.

Internal code should use stable keys, enums, or typed identifiers. Do not use localized Chinese or English display strings as internal logic keys.

## What not to do

Do not:

- add an LLM-first interpretation path that bypasses structured claims;
- put narrative text directly into chart-generation models;
- create a single all-purpose astrology engine trait;
- introduce multi-school support as scattered `if` or `match` logic across unrelated modules;
- silently diverge from documented compatibility behavior;
- add large rule sets without source metadata;
- add generated code or broad scaffolding without clear module boundaries.

## Preferred workflow

For non-trivial changes:

1. identify the affected layer;
2. check whether an ADR or existing doc already constrains the design;
3. add or update tests if deterministic behavior changes;
4. keep the PR narrow;
5. update bilingual docs when public concepts or architecture change;
6. record new architecture decisions as ADRs.

## PR checklist for agents

Before opening or updating a PR, verify:

- [ ] The change respects the documented layer boundaries.
- [ ] Deterministic behavior has tests or a documented testing plan.
- [ ] New abstractions are small and justified.
- [ ] Public terminology matches the glossary.
- [ ] English and Chinese docs are updated where applicable.
- [ ] Compatibility implications are documented.
- [ ] No narrative interpretation bypasses structured claims.
