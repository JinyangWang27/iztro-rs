# Classical rule authoring guardrails

This guide covers the invariants that must hold when adding or modifying
classical rules. The guardrail tests in
`crates/iztro/tests/classical_rule_guardrails.rs` enforce these at CI time.

## Runtime flow

```
source inventory (TOML, test-only)
  -> runtime rule metadata (classical_rules(), classical_rule_metadata())
  -> Rust predicate/evaluator (evaluator.rs)
  -> ClassicalSourceHit
  -> optional Claim
  -> RuleDiagnostic for unsupported rules
```

The source inventory (`crates/iztro/rule-corpus/quan-shu/source/volume-01.toml`) is
corpus-management data validated by tests only. Nothing in `src/` parses it.
The runtime rule corpus (`crates/iztro/rule-corpus/quan-shu/rules.toml`,
`crates/iztro/rule-corpus/patterns/rules.toml`) is embedded at compile time and parsed once
per process via `OnceLock`.

## Source text purity

For QuanShu rules, `source_text_zh_hans` must quote the cited source clause
**verbatim**. Do not store:

- modern interpretation
- paraphrase
- another tradition's phrase
- combined commentary

in `source_text_zh_hans`.

Put interpretation in:

- `normalized_note_zh_hans` (editor note in the corpus TOML)
- `ClaimSpec` (structured claim shape)
- i18n claim text (via `claim_key` resolved by `iztro-i18n`)
- future commentary metadata

The guardrail test `executable_quan_shu_rules_have_valid_source_metadata`
asserts this field is non-empty and has no leading/trailing whitespace. The
source-inventory test `source_item_text_matches_rule_source_text` asserts the
verbatim text matches the linked source item.

## Compact hit references

`ClassicalRuleHitRef` is the renderer-facing hit type returned by
`detect_analysis_layer`. It carries only:

- `rule_id` — stable rule identifier
- `scope` — the matched scope (natal, decadal, …)
- `claim_key` — optional i18n key for claim text
- `evidence` — structured evidence items for GUI highlighting

It **does not** carry `source_text_zh_hans`. The absence is enforced at
compile time by the struct definition. Renderers resolve verbatim source text
once per rule id via `classical_rule_metadata(rule_id)`.

## User-facing analysis is QuanShu-only

`AnalysisLayerRequest::user_facing()` restricts `classical.works` to
`[ClassicalWork::ZiWeiDouShuQuanShu]`. The future GUI shows 全书规则 and 格局
in **separate** tabs:

- The classical rule stream (this path) surfaces QuanShu source rules.
- The pattern stream surfaces `ClassicalWork::IztroPatternCatalog` rules.

Do not widen `user_facing()` to include `IztroPatternCatalog`. The guardrail
test `user_facing_analysis_request_is_quan_shu_only` pins this.

## Checklist for adding one executable QuanShu rule

- [ ] Confirm the source inventory entry exists in
  `crates/iztro/rule-corpus/quan-shu/source/volume-01.toml` with
  `status = "rule_linked"` and the rule id in `linked_rule_ids`.
- [ ] Quote source text verbatim in `source_text_zh_hans` (no interpretation).
- [ ] Add the rule entry to `crates/iztro/rule-corpus/quan-shu/rules.toml` with
  `status = "executable"`.
- [ ] Add a predicate branch in `crates/iztro/src/rules/classical/evaluator.rs`.
- [ ] List the rule id in `WIRED_EXECUTABLE` in
  `crates/iztro/tests/classical_source_inventory.rs`.
- [ ] Add at least one structured `Evidence` item in the evaluator.
- [ ] Add unit and/or integration tests (see
  `crates/iztro/tests/classical_rules.rs`).
- [ ] Add a `ClaimSpec` only when the interpretation is clear and agreed upon.
- [ ] Confirm `cargo test --workspace` passes, including the guardrail tests.
- [ ] Confirm user-facing analysis remains QuanShu-only (no change to
  `AnalysisLayerRequest::user_facing`).

## Checklist for unsupported rules

- [ ] Keep the source inventory entry.
- [ ] Add or keep the rule entry in `crates/iztro/rule-corpus/quan-shu/rules.toml`
  with a non-executable status (`normalized`, `ambiguous`, or `raw`).
- [ ] Do not add a predicate that guesses at unmolded facts. Return
  `RuleOutcome::Unsupported(UnsupportedReason::...)` if the rule id is wired
  but the condition is not yet modeled.
- [ ] Add a typed `UnsupportedReason` variant if the reason is novel.

## Guardrail test summary

| Test | What it checks |
|------|----------------|
| `no_duplicate_runtime_rule_ids` | All rule ids in `classical_rules()` are unique |
| `every_runtime_rule_has_metadata` | `classical_rule_metadata(id)` returns `Some` for every id |
| `executable_quan_shu_rules_have_valid_source_metadata` | Non-empty source fields, no whitespace padding |
| `quan_shu_runtime_metadata_has_no_duplicate_source_pairs` | No duplicate `(source_id, source_clause_id)` in QuanShu metadata |
| `user_facing_analysis_request_is_quan_shu_only` | `user_facing().classical.works == [ZiWeiDouShuQuanShu]` |
| `user_facing_rule_hits_exclude_pattern_catalog` | A chart firing 昌曲夹命 produces no IztroPatternCatalog hit via `user_facing()` |
| `quan_shu_source_hits_carry_evidence` | Every matched QuanShu source hit has non-empty evidence |
| `classical_rule_hit_ref_does_not_duplicate_source_text` | `ClassicalRuleHitRef` carries only the four compact fields; source text is resolved via metadata |
