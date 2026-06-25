# Classical Rule Engine

This document describes the **classical rule engine** introduced under
`crates/iztro/src/rules/classical/`. It encodes rules from classical sources such
as 《紫微斗数全书》 and turns chart facts into structured, evidence-backed
**claims**.

It complements the higher-level [`rule-engine.md`](../rule-engine.md), which
describes the longer-term feature → claim engine. The classical engine is the
first concrete, data-driven slice of that vision.

> **Transitional status.** The placeholder scaffold in `crates/iztro/src/rules/`
> (the feature-oriented `Claim` / `RuleEngine` / `Evidence` stubs) is **not** the
> destination. It will be migrated into, or retired in favor of, the classical
> engine in a follow-up. The two coexist today only so existing scaffold tests
> keep passing.

## Pipeline

```
Chart facts
  -> feature/query predicates        (reuse core/pattern query helpers)
  -> classical rule evaluation       (corpus metadata + hand-coded predicates)
  -> ClassicalSourceHit[]            (matched source/provenance)
  -> structured Claim[]              (only when rule.claim exists)
  -> [optional] localized rendering   (iztro-i18n, via claim_key)
  -> JSON export                      (serde)
```

## Sources of truth

The engine deliberately keeps four representations separate:

| Concern | Canonical source | Notes |
| --- | --- | --- |
| Classical terminology | **Chinese source text** | `SourceRef::source_text_zh_hans`, authored in the corpus TOML. |
| Machine logic | **Rust enums / stable keys** | `ClassicalRuleId`, `ClaimDomain`, `ClaimTheme`, … Chinese strings are never logic keys. |
| Output rendering | **Fluent `.ftl` resources** | `iztro-i18n` renders labels and claim short text from stable keys. |
| Rule authoring | **Corpus TOML** | `crates/iztro/rule-corpus/quan-shu/rules.toml` and `crates/iztro/rule-corpus/patterns/rules.toml`. |
| Export | **JSON** | A serialization of claims; never the authoring source. |

`iztro` never depends on `iztro-i18n`: the core crate emits stable keys and
structured facts only; localized prose lives in Fluent resources.

## Hybrid design (metadata + predicates)

This is intentionally **not** a generic rule DSL yet:

1. **Rule source/predicate metadata is data-driven** from the corpus TOML (id,
   source, status, work, school).
   Optional `[rule.claim]` metadata holds interpretation fields (domain, themes,
   polarity, base strength, claim key).
2. **Rule predicates are hand-coded** in `predicates.rs`, reusing the read-only
   chart query helpers in `core/pattern/` (clamp matching, brightness
   classification, star lookup) — no second copy of that logic.
3. The `quan_shu.rs` evaluators pair each rule's metadata with its predicate and
   build a `ClassicalSourceHit`; they build a `Claim` only when `rule.claim`
   exists.

## Conservative emission and typed diagnostics

A source hit is emitted **only when an executable rule condition matches on
modeled chart facts**. A claim is emitted only for that same match when the rule
has a `ClaimSpec`.
Each evaluator returns a typed `RuleOutcome`:

- `Matched { source_hit, claim }` — facts modeled and the condition matched;
- `NotApplicable` — facts modeled, condition did not match (no claim);
- `Unsupported(UnsupportedReason)` — the rule is encoded but its condition is not
  yet backed by a modeled fact / defined policy.

The engine exposes two entry points:

- `evaluate_classical_claims(chart, &request) -> Vec<Claim>` — claims only;
- `evaluate_classical(chart, &request) -> ClaimEvaluation { claims, source_hits, diagnostics }`
  — also returns matched `ClassicalSourceHit`s and typed `RuleDiagnostic`s, so unsupported conditions are
  **visible**, not silently dropped.

`SourceRef` remains the claim-facing citation type. `ClassicalSourceHit` is the
evaluation-facing source/provenance record for a matched predicate.

## Request filtering and ordering

`ClaimEvaluationRequest` filters claims by `domains`, `themes`, `polarities`,
`works`, `rule_ids`, and `scopes`. Each field is an allow-list; an empty vec
imposes no constraint.

Source hits are filtered only by provenance dimensions: `works`, `rule_ids`, and
`scopes`. Domain/theme/polarity filters remain claim filters and do not suppress
matched source provenance.

Unsupported diagnostics default to `DiagnosticMode::AllUnsupported`: every
unsupported corpus rule remains visible even when claim filters are applied.
Callers that want a narrower UI/export surface may choose
`DiagnosticMode::MatchingRequest`, which applies the request filters to rule
metadata as far as possible. Domain/theme/polarity diagnostic filters only match
rules that have `rule.claim`; source-only rules do not pretend to have
interpretive metadata. `DiagnosticMode::None` suppresses diagnostics.

Returned claims are sorted deterministically by
`(scope, domain, rule_id, claim_key)`.
Returned source hits are sorted deterministically by
`(scope, work, source_id, source_clause_id, rule_id)`.

## Rule statuses

`RuleStatus` records a rule's encoding maturity:

| Status | Meaning |
| --- | --- |
| `Raw` | Unsegmented source line. |
| `Segmented` | Split into discrete statements. |
| `Normalized` | Normalized into a structured intent. |
| `Executable` | Backed by a working predicate over modeled facts. |
| `Tested` | Executable with positive/negative fixtures over realistic generated charts or reviewed source-grounded fixtures, suitable for stable public consumption. Synthetic pilot tests alone do not imply this status. |
| `Ambiguous` | Meaning or condition is ambiguous. |
| `Rejected` | Not used. |

Not every 全书 sentence is immediately executable; statuses make that explicit.

## PatternDetection vs Claim

`iztro` already has `core::pattern` **pattern detection** (格局). The two are
distinct:

- A **`PatternDetection`** is a structured statement that a known 格局 shape is
  present (status, family, involved stars/palaces, evidence). It is a *chart
  fact about arrangement*.
- A **`Claim`** is a rule's structured *judgement* (domain, themes, polarity,
  strength, evidence, counter-evidence, source) intended for downstream
  interpretation, filtering, and localized rendering.

A classical rule may match the same structural shape as a known pattern (the
昌曲夹命 claim records `EvidenceKind::PatternShapeMatched {
pattern: ChangQuJiaMing }`), but this does not claim that
`core::pattern::detect_patterns` was run. The claim still carries domain /
theme / polarity / source semantics a pattern detection does not.

## Worked example: 马遇空亡，终身奔走

1. **Source.** The corpus line:

   ```toml
   id = "migration.tian_ma_void.restless_movement"
   source_id = "quan_shu.v01.tai_wei_fu.001"
   source_clause_id = "ma_yu_kong_wang"
   source_text_zh_hans = "马遇空亡，终身奔走"
   status = "executable"

   [rule.claim]
   domain = "migration"
   themes = ["restless_movement", "instability"]
   polarity = "mixed_negative"
   claim_key = "claim.migration.tian-ma-void.restless-movement"
   ```

2. **What is 空亡?** Not every star whose name contains 空. `VoidKind` enumerates
   only the modeled 空亡 family (旬空 `XunKong`, 空亡 `KongWang`, 截路 `JieLu`,
   截空 `JieKong`) and **excludes** 天空/地空/地劫. A `VoidPolicy` names the set a
   rule consults; `VoidPolicy::DEFAULT` includes every modeled kind, while
   `VoidPolicy::XUN_KONG_ONLY` and `VoidPolicy::new(...)` support narrower
   policies for future rules or schools.

3. **Predicate.** `tian_ma_affected_by_void` finds 天马's palace and checks
   whether a void star counted by the policy shares it.

4. **Source hit and claim.** On a match the evaluator emits a
   `ClassicalSourceHit` carrying the QuanShu work, passage id, clause id,
   Chinese source text, rule status, scope, and evidence. Because this rule has
   `[rule.claim]`, it also emits a claim carrying
   `EvidenceKind::StarAffectedByVoid { star: TianMa, void_kind, branch }`, the
   corpus domain/themes/polarity/strength, the `SourceRef` (Chinese text), and
   `claim_key`.

5. **Rendering (optional).** `iztro-i18n`'s `claim_text(&claim)` resolves the
   `claim_key` (dots mapped to hyphens) to localized text: *“天马受空亡影响，主奔
   波迁动之象。”* / *“Tian Ma is affected by a void condition, indicating restless
   movement …”*.

6. **Export.** `serde_json::to_string(&claim)` yields deterministic JSON with the
   rule id, claim key, Chinese source text, domain, themes, polarity, strength,
   evidence, and counter-evidence.

See [`quan-shu-corpus.md`](../../zh-CN/rules/quan-shu-corpus.md) (Chinese) for the
corpus authoring format.

## Source inventory (passage + clauses)

A rule's `source_id` identifies a **source passage** in the QuanShu source
inventory (`crates/iztro/rule-corpus/quan-shu/source/`); `source_clause_id`
identifies an individual candidate phrase (a *clause*) within that passage. One
passage can hold several clauses, each linking zero or more rules — this is what
lets the inventory scale beyond one item per rule.

For QuanShu rules, `ClassicalSourceHit` cites the classical passage/clause. For
pattern-catalog rules, it cites the project-owned `pattern.*` metadata entry
instead; those pattern entries are not tracked by the QuanShu source inventory.

The source inventory is **corpus-governance data, not runtime data**: nothing in
`src/` parses it, `evaluate_classical` never depends on it, and the Markdown
volumes are never parsed at runtime. Its consistency (passage/clause structure,
clause ↔ rule links, text containment) is enforced only by
`crates/iztro/tests/classical_source_inventory.rs`. Pending passages may use
`section = "待校"` and `anchor = "TODO"` until located.
