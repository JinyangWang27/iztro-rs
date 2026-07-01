# ADR 0008: Classical Rule Engine

Status: Accepted

## Context

`iztro-rs` needs a rule engine that can encode Chinese classical rules without
turning source text into opaque prose, and without coupling the core crate to
runtime localization or report writing.

The first concrete source family is 《紫微斗数全书》. Early rule work also revealed
that some classical clauses are not immediately executable because their
conditions require a school-specific policy, a missing chart fact, or a more
precise relation model.

A purely data-driven DSL would be premature: the rule corpus is still being
segmented and normalized, while the Rust chart facts and query helpers are still
evolving. But storing all rule knowledge directly in Rust would make source
coverage and auditability poor.

## Decision

Introduce a **classical rule engine** under `crates/iztro/src/rules/classical/`
with a hybrid design:

- **Metadata is data-driven** from a Chinese-first corpus TOML
  (`rule-corpus/quan-shu/rules.toml`), embedded via `include_str!`.
- **Predicates are hand-coded** in Rust, reusing shared read-only query helpers
  from `rules::query`. No generic rule DSL is built yet.
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
- Unsupported rules are explicitly visible instead of silently absent.
- The old placeholder `Claim` scaffold has been removed; classical `Claim` is the
  only claim type.
- A generic condition DSL is deferred until rule volume proves the need.
