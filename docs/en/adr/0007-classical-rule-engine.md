# ADR 0007: Classical Rule Engine (Chinese-first, hybrid)

## Status

Accepted.

## Context

`iztro-rs` needs to encode rules from classical sources (e.g. 《紫微斗数全书》)
and turn chart facts into structured claims. A placeholder `rules/` scaffold
already exists, but its `Claim`/`Evidence` shapes are prose-oriented and do not
match the requirements for machine-readable, evidence-backed claims with stable
typed identity, Chinese source preservation, and localized rendering.

Two tensions had to be resolved:

1. How rules are authored vs. how they are matched.
2. How to introduce a richer claim model without breaking the existing scaffold
   and its tests.

## Decision

Introduce a **classical rule engine** under `crates/iztro/src/rules/classical/`
with a hybrid design:

- **Metadata is data-driven** from a Chinese-first corpus TOML
  (`rule-corpus/quan-shu/rules.toml`), embedded via `include_str!`.
- **Predicates are hand-coded** in Rust, reusing the existing `core/pattern`
  query helpers. No generic rule DSL is built yet.
- Rules emit typed `Claim`s (domain, themes, polarity, strength, evidence,
  counter-evidence, source refs, stable `claim_key`). Chinese strings are never
  used as logic keys.
- **Conservative emission:** a claim is produced only when its condition matches
  on modeled facts. Unsupported conditions return a typed
  `RuleOutcome::Unsupported`, surfaced as a visible `RuleDiagnostic`.
- `iztro` stays independent of `iztro-i18n`. Localized labels and claim text live
  in Fluent resources, keyed off stable enum identity.
- The classical engine is a **transitional slice**: the placeholder scaffold will
  be migrated into it (or retired) in a follow-up. They coexist for now only to
  keep existing tests passing.

Corpus enum values use the crate-wide `snake_case` serde convention (consistent
with every other enum in `iztro`), so authored TOML and exported JSON share one
casing.

## Consequences

- Classical rules can be authored as Chinese-first data, separate from matching
  code.
- Claims are testable, filterable, serializable, and localizable.
- Unsupported rules are explicit and visible rather than silently non-firing.
- A second `Claim` type temporarily coexists with the scaffold; convergence is
  tracked as follow-up work.
- A generic condition DSL is deferred until enough rules justify it.
