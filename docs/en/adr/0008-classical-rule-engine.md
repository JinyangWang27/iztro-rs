# ADR 0008: Classical Rule Engine (Chinese-first, hybrid)

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
- **Predicates are hand-coded** in Rust, reusing shared read-only query helpers
  from `rules::pattern::query`. No generic rule DSL is built yet.
- Rules emit typed `Claim`s (domain, themes, polarity, strength, evidence,
  counter-evidence, source refs, stable `claim_key`). Chinese strings are never
  used as logic keys.
- **Conservative emission:** a claim is produced only when its condition matches
  on modeled facts. Unsupported conditions return a typed
  `RuleOutcome::Unsupported`, surfaced as a visible `RuleDiagnostic`.
- `iztro` stays independent of `iztro-i18n`. Localized labels and claim text live
  in Fluent resources, keyed off stable enum identity.
- The classical engine is the active source/claim rule engine. (Update: the
  placeholder `rules/` scaffold it originally coexisted with has since been
  removed. Pattern detection is also rule-engine code and now lives under
  `rules::pattern`; `rules::classical` remains the classical claim/source-hit
  engine.)

Corpus enum values use the crate-wide `snake_case` serde convention (consistent
with every other enum in `iztro`), so authored TOML and exported JSON share one
casing.

## Consequences

- Classical rules can be authored as Chinese-first data, separate from matching
  code.
- Claims are testable, filterable, serializable, and localizable.
- Unsupported rules are explicit and visible rather than silently non-firing.
- The classical `Claim` is now the sole claim type; the earlier placeholder
  scaffold `Claim` has been removed.
- A generic condition DSL is deferred until enough rules justify it.
