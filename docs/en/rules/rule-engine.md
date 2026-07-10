# Classical Rule Engine

This document describes the **classical rule engine** under
`crates/iztro/src/rules/classical/`. It is the active rule engine: it encodes
rules from classical sources such as 《紫微斗数全书》 and turns chart facts into
structured, evidence-backed **claims**.

`rules::classical` is the canonical namespace for the rule engine. The
`crates/iztro/src/rules/` module exposes it directly (`pub mod classical`) and
re-exports the classical types/functions, so `rules::Claim` and friends point at
the classical claim model.

## Pipeline

```
Chart facts
  -> feature/query predicates        (reuse shared rules::query helpers)
  -> classical rule evaluation       (corpus metadata + hand-coded predicates)
  -> ClassicalSourceHit[]            (matched source/provenance)
  -> Claim[]                         (only when rule.claim exists)
  -> RuleDiagnostic[]                (typed, visible unsupported conditions)
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
2. **Rule predicates are hand-coded** in `predicates.rs`, reusing read-only
   chart query helpers from `rules::query` (clamp matching, brightness
   classification, star lookup). Pattern-specific query wrappers remain under
   `rules::pattern::query`.
3. The evaluator module pairs each rule’s metadata with its predicate and
   build a `ClassicalSourceHit`; they build a `Claim` only when `rule.claim`
   exists.

Note a deliberate divergence from the layered architecture's long-term shape:
rule predicates currently read `Chart` facts directly rather than consuming the
feature-extraction layer's `ChartFeatures`. Predicates need chart-level detail
(clamp geometry, brightness, void policy) that `ChartFeatures` does not carry
yet. Chaining features → rules is future work once feature structures
stabilize; until then the two layers are parallel by design.

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

## Renderer-neutral rule panel

`evaluate_classical` remains the low-level evaluation API. For GUI/renderer
consumers, `classical_rule_panel_view(chart, &ClassicalRulePanelRequest)` is the
renderer-facing grouping API. It runs one `evaluate_classical` pass and bundles
the result with optional corpus rule metadata into a single
`ClassicalRulePanelView`.

The panel **preserves** the existing split rather than collapsing it: `claims`,
`source_hits`, `diagnostics`, and `corpus_rules` stay as separate vectors. Claims
and source hits are never merged into one card model, so a rule that matches but
has no claim metadata still appears through `source_hits`, and corpus metadata is
for display/filtering only, not evaluation output.

`ClassicalRulePanelRequest::user_facing()` hides unsupported diagnostics
(`DiagnosticMode::None`); `developer()` surfaces them (`DiagnosticMode::AllUnsupported`).
Corpus rules can be filtered by status (`with_corpus_statuses`) and omitted
entirely (`without_corpus`); they are sorted deterministically by
`(work, source_id, source_clause_id, rule_id)`.

As elsewhere, `iztro` emits no localized prose here. The panel carries
`claim_key`, typed enums, and Chinese source text; localized rendering stays in
`iztro-i18n`.

## Context-oriented evaluation

Alongside the natal-only `evaluate_classical(chart, &request)`, the engine
exposes a context-oriented entry point:

```rust
evaluate_classical_in_context(&ClassicalRuleContext, &request) -> ClaimEvaluation
```

`ClassicalRuleContext` and `rules::pattern::PatternContext` are both thin
wrappers over `RuleEvaluationContext`, the shared selected-state context. It
carries the natal chart, an optional `&HoroscopeChart`, active scopes, and the
selected `EffectiveChartState` when construction succeeds.

Use `ClassicalRuleContext::natal(chart)` for natal-only evaluation.
`ClassicalRuleContext::horoscope(chart, active_scopes)` is a
compatibility/fail-closed constructor. Production selected-view analysis should
use `ClassicalRuleContext::horoscope_with_frame(chart, palace_frame_scope,
active_scopes)` so the selected palace frame is explicit.
`evaluate_classical(chart, &request)` is a thin natal-only wrapper over the
context API, and `classical_rule_panel_view` likewise wraps
`classical_rule_panel_view_in_context`, so existing call sites are unchanged.

Most current executable rules remain natal-only. 昌曲夹命 is the first
selected-state vertical slice: when evaluated through
`ClassicalRuleContext::horoscope_with_frame`, it may emit a source hit and claim
in the selected frame scope. QuanShu rules remain conservative and are not
promoted to temporal scopes automatically.

Selected-state queries can see active overlays projected into the selected palace
frame, but **exact star matching stays exact**. The selected frame scope controls
which palace labels and facts are visible; it never overloads star identity
equality. A base-star condition such as 文曲 (`WenQu`) matches only the exact
runtime 文曲 — a flow star like 流曲 (`LiuQu`) that becomes visible under a yearly
frame is an independent `StarName` and does not satisfy it. So 昌曲夹命 requires
exact 文昌 + 文曲 even in a temporal frame; natal 文昌/文曲 projected into the
selected frame still qualify, but 流昌/流曲 do not. Family-level matching is opt-in
and explicit (`StarSelector::Family` over `StarFamily`); rules that intend a
specific temporal blade resolve the exact flow member for that layer explicitly
via `StarFamily::exact_member_for_scope` (as 羊陀夹忌 does for 流羊/流陀) rather than
relying on any hidden base↔flow aliasing.

## Layer-level analysis (`analysis`)

The `analysis` module is a lightweight coordinator that composes the pattern and
classical engines for **cacheable, per-layer** detection. It lives outside
`core` (which must not depend on `rules`) and exists to back a future GUI's two
sidebar tabs — 全书规则 (classical rules) and 格局 (patterns) — without eagerly
computing every overlay or shipping a heavy grouped-text payload.

Key types:

- `AnalysisLayerKey` — identifies one cacheable layer (`Natal`, `Decadal`,
  `Age`, `Yearly`, `Monthly`, `Daily`, `Hourly`) with the temporal indexes that
  address it. `scope()`, `claim_scope()`, and `pattern_scope()` map it to the
  existing `Scope` / `ClaimScope` / `PatternScope` types.
- `analysis_layers_for_selection(selection)` — expands a
  `StaticTemporalNavigationSelection` into the ancestor chain of layers it makes
  visible. A year selection includes **both** `Age` (小限) and `Yearly` (流年),
  which are distinct scopes.
- `detect_analysis_layer(&ctx, key, &request) -> AnalysisLayerResult` — analyzes
  exactly one layer over a `TemporalAnalysisContext { natal, horoscope }`. It
  overrides only the **scope** of the underlying classical/pattern requests to
  `key` (every other filter — notably `works` — is preserved from the caller's
  request) and returns compact `rule_hits: Vec<ClassicalRuleHitRef>` plus
  `pattern_hits: Vec<PatternDetection>`. Pattern hits are produced by
  `rules::pattern` and may come from supported temporal overlay facts; classical
  rule hits remain conservative and current executable QuanShu rules are still
  natal-only. The `TemporalAnalysisContext` must correspond to `key`: the key
  drives cache identity and scope assignment and is **not** currently validated
  against the horoscope's selected overlays, so keeping context and key in sync
  is the caller's responsibility.
- `AnalysisLayerRequest::user_facing()` restricts the classical rule stream to
  `ClassicalWork::ZiWeiDouShuQuanShu`. Because the GUI shows 全书规则 and 格局 in
  **separate** tabs, the analysis rule-hit stream must not include project
  pattern-catalog rules (`ClassicalWork::IztroPatternCatalog`), which surface
  through the pattern (格局) stream instead. The future 全书规则 tab should
  therefore consume these QuanShu-filtered rule hits; `classical_rule_metadata`
  stays work-agnostic and resolves metadata for any rule id, including
  pattern-catalog entries.
- `ClassicalRuleHitRef` — a compact hit (`rule_id`, `scope`, `claim_key`,
  `evidence`). It deliberately **omits** `source_text_zh_hans`; a renderer
  resolves verbatim source text once per rule via
  `classical_rule_metadata(rule_id) -> Option<&'static ClassicalRuleMetadata>`.
  `ClassicalRuleMetadata::source_text_zh_hans` is the verbatim source clause and
  never carries an interpretation or claim text. Most current executable rules
  carry `applicable_scopes = &[ClaimScope::Natal]`. Overlay-aware rules,
  currently 昌曲夹命, explicitly advertise wider applicable scopes; QuanShu /
  太微赋 rules are not promoted to temporal scopes automatically.

**Layer assignment and caching.** Detection of a layer may *inspect* ancestor
overlays, but the returned hits always belong to the requested layer.
`detect_analysis_layer` never computes ancestor layers; the caller requests
missing ancestors separately and caches each result by `AnalysisLayerKey`.
Future cross-layer rules (e.g. 流年化忌冲照本命命宫, not implemented here) must
assign their hit to the **deepest** triggering layer:

| Interaction | Assigned layer |
| --- | --- |
| 本命 + 流年 | Yearly (流年) |
| 大限 + 流年 | Yearly (流年) |
| 流年 + 流月 | Monthly (流月) |
| 流月 + 流日 | Daily (流日) |

This keeps caching natural: changing month/day/hour within the same year never
invalidates the cached yearly result, and changing day/hour within the same
month never invalidates the cached monthly result. The GUI groups cached results
by `AnalysisLayerKey::scope()` and hides empty groups; no rendering lives in
`iztro`.

The selected-view batch facade,
`detect_static_temporal_analysis_layers_from_chart`, builds the selected
horoscope context once, validates requested keys against the exact visible
`AnalysisLayerKey`s, and detects each key with its own truncated active-scope
chain. Consumers should cache results by the full `AnalysisLayerKey`, not by
scope alone.

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

The Volume 1 太微赋 source units are now **fully linked** into runtime rule
metadata: every atomic rule-candidate aphorism (one `source_item`) links to one
or more rules, and the section's closing remark links to a `Rejected` rule that
documents the exclusion (see `docs/zh-CN/rules/quan-shu-coverage.md`). Many of
these rules are `Normalized` or `Ambiguous` rather than `Executable`, and they
carry no `[rule.claim]` until a predicate is implemented — **executable coverage
is intentionally conservative**. Non-executable rules emit neither a claim nor a
source hit at runtime (the evaluator returns `NotApplicable`); their value is an
auditable, status-tagged record of each cited source unit. Each non-executable
rule must carry a `normalized_note_zh_hans`, enforced by
`crates/iztro/tests/classical_source_inventory.rs`.

### Promoting a normalized rule to executable

Executable coverage grows one rule at a time, never in bulk (1–2 rules per PR):

1. pick one `status = "normalized"` rule whose condition is expressible over
   modeled chart facts;
2. author its `[rule.claim]` block (domain, themes, polarity, base strength,
   `claim_key`) and the matching claim text in the i18n `claims.ftl` resources;
3. implement the predicate arm in `evaluator.rs`, reusing `rules::query`
   helpers — or return a typed `RuleOutcome::Unsupported` (like
   `LU_MA_JIAO_CHI`) when a needed chart fact is not modeled yet;
4. add positive and negative rule tests (`tests/classical_rules.rs`);
5. flip `status` to `executable` (`tested` once fixture coverage justifies it).

The evaluator's test guard forces every `executable`/`tested` corpus rule to
have a predicate arm, so a forgotten step fails loudly.

## PatternDetection vs Claim

`iztro` already has `rules::pattern` **pattern detection** (格局). The two are
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
`rules::pattern::detect_patterns` was run. The claim still carries domain /
theme / polarity / source semantics a pattern detection does not.

## Worked example: 马遇空亡，终身奔走

1. **Source.** The corpus line:

   ```toml
   id = "migration.tian_ma_void.restless_movement"
   source_id = "quan_shu.v01.tai_wei_fu.ma_yu_kong_wang"
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
   `ClassicalSourceHit` carrying the QuanShu work, atomic source id,
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

## Star tags (overlapping interpretive taxonomy)

`StarTag` is a reusable, **overlapping** taxonomy on `StarName` in the core star
model, layered above the mutually exclusive coarse grouping `StarCategory`
(`Major` / `Minor` / `Adjective`). A star may carry several tags; for example 地空
is both 空劫 (`KongJie`) and 空曜 (`VoidSymbol`). Two Tan Lang QuanShu rules are now
executable on this layer:

- **贪居亥子，名为犯水桃花** (`relationship.tan_ju_hai_zi.water_romance`):
  conservatively 贪狼 placed in the 亥 or 子 branch.
- **刑遇贪狼，号曰风流彩杖** (`relationship.xing_yu_tan_lang.romance_with_penalty`):
  conservatively 贪狼 sharing a palace with a 刑曜 (`StarTag::Punishment` = 擎羊、
  天刑).

`StarTag::VoidSymbol` (空曜) is **broad interpretive taxonomy** and is deliberately
kept distinct from `VoidKind`, which remains the **narrow, unchanged** 空亡-family
used by 马遇空亡 (旬空 / 空亡 / 截路 / 截空). The concepts are distinct even though some 
stars, such as 旬空 and 截空, may appear in both taxonomies. `VoidKind` answers the 
narrow 空亡-family question for rules like 马遇空亡; `StarTag::VoidSymbol` answers 
the broader interpretive 空曜 question. 天空、地空、地劫 are `VoidSymbol` members 
but never `VoidKind`.

## Source inventory (atomic source items)

A rule's `source_id` identifies an **atomic cited source unit** (one
rule-candidate aphorism) in the QuanShu source inventory
(`crates/iztro/rule-corpus/quan-shu/source/`). A single physical Markdown line
may contain several such units — boundaries are semantic, not typographical — so
each `source_item` is one aphorism, not a physical line/passage with nested
clauses. `source_id` is a stable mnemonic (e.g. `…tai_wei_fu.ma_yu_kong_wang`),
and `source_order` preserves source order separately from stable identity. A
source item links zero or more rules via `linked_rule_ids`. QuanShu rules no
longer carry `source_clause_id` (the field remains on `ClassicalRule` for the
pattern catalog and backwards compatibility).

The inventory TOML is stored in a compact **grouped** form: a `source_group`
carries the shared section defaults and each `source_group.item` is one atomic
unit, with `source_id = source_id_prefix + item.key`. This grouped TOML is the
single canonical source; the tests expand it into the flat per-item view.

For QuanShu rules, `ClassicalSourceHit` cites the classical source unit. For
pattern-catalog rules, it cites the project-owned `pattern.*` metadata entry
instead; those pattern entries are not tracked by the QuanShu source inventory.

The source inventory is **corpus-governance data, not runtime data**: nothing in
`src/` parses it, `evaluate_classical` never depends on it, and the Markdown
volumes are never parsed at runtime. Its consistency (unique stable ids,
continuous `source_order`, item ↔ rule links, verbatim source text) is enforced
only by `crates/iztro/tests/classical_source_inventory.rs`. Pending units may use
`section = "待校"` and `anchor = "TODO"` until located.

### Runtime section table (`sections.toml`)

The one piece of section metadata available at runtime is the small table
`rule-corpus/quan-shu/sections.toml`: one entry per inventory `source_group`
carrying only `source_id_prefix`, `work`, `volume`, and `section`. It is
embedded via `include_str!` and exposed as
`rules::source::source_section(source_id) -> Option<&SourceSection>`, which
truncates the id after its final `.` and performs an exact prefix lookup
(inventory item keys never contain dots). Display layers use it to cite 卷/节
for both classical rule hits and source-backed pattern detections
(`PatternSourceMetadata` deliberately stores no section of its own).

The table intentionally stores neither `anchor` (always equal to `section`)
nor `doc_path` (always derivable as
`docs/zh-CN/sources/quan_shu/volume-{volume:02}.md`).
`crates/iztro/tests/source_sections.rs` keeps the table in sync with the
inventory in both directions and enforces those two derivability invariants.
All item-level data and governance fields (`status`, `category`,
`linked_rule_ids`, `source_order`) remain test-only; see ADR 0010.
