# QuanShu source inventory

This directory contains structured source-inventory TOML for the Chinese source text in:

```text
docs/zh-CN/sources/quan_shu/
```

The Markdown files are the human-readable source layer. The TOML files are the machine-checkable inventory layer used to track which source passages have been reviewed, normalized, linked to rules, or deferred.

## Intended pipeline

```text
source Markdown
  -> source inventory TOML
  -> classical rule metadata TOML
  -> Rust predicates / evaluator
  -> structured Claim[]
```

## Structure: passage + clauses

A source passage often contains several candidate rule phrases, e.g.

```text
禄逢冲破，吉处藏凶。马遇空亡，终身奔走。
```

The inventory therefore models three levels:

```text
source item = a source passage / location, identified by `source_id`
clause      = an individual candidate rule phrase inside that passage,
              identified by `clause_id` (unique within the source item)
rule        = an executable/normalized interpretation linked from a clause
              via `linked_rule_ids`
```

`source_id` identifies the **passage**, not the semantic rule phrase (so it
scales when one passage yields multiple rules). A rule links to a clause by
carrying both `source_id` and `source_clause_id`; the clause mirrors that link
through its `linked_rule_ids`.

```toml
[[source_item]]
source_id = "quan_shu.v01.tai_wei_fu.001"
section = "太微赋"
source_text_zh_hans = "禄逢冲破，吉处藏凶。马遇空亡，终身奔走。"

[[source_item.clause]]
clause_id = "ma_yu_kong_wang"
text_zh_hans = "马遇空亡，终身奔走"
linked_rule_ids = ["migration.tian_ma_void.restless_movement"]
```

## Files

- `volume-01.toml`: source inventory for Volume 1. It links the five currently encoded classical pilot rules to reviewed 太微赋 passages, keeps two `待校`/`TODO` pending items not yet located, and segments a first batch of additional 太微赋「例曰」passages into clauses that are **not yet linked to rules** (`linked_rule_ids = []`).

This is not a complete line-by-line inventory of Volume 1. It is a deliberately small slice that establishes the inventory format, links existing rule `source_id`/`source_clause_id`s to reviewed source passages where possible, and exercises the coverage report. Both linked and unlinked clauses are useful: an unlinked clause records reviewed source text that has been segmented but not yet normalized or implemented as a rule.

## Coverage report

A committed-but-generated coverage report lives at:

```text
docs/zh-CN/rules/quan-shu-coverage.md
```

It summarizes the inventory (source items, clauses, linked/unlinked clauses, and linked rules by status) and is maintained by tests: `crates/iztro/tests/classical_source_coverage.rs` recomputes the metrics and asserts the committed file is current. Expanding this inventory must regenerate the report, or the test fails. A segmentation PR does not need to add executable rules; it can add unlinked clauses (`linked_rule_ids = []`) and update the report.

## Status values

The `status` field tracks source-processing maturity, not executable rule maturity:

- `raw`: source passage recorded only;
- `segmented`: passage split into candidate clauses;
- `normalized`: candidate rule intent identified;
- `rule_linked`: linked to one or more rule ids;
- `ambiguous`: retained but requires source/school review;
- `rejected`: deliberately not used as a rule source.

## Categories

The `category` field is deliberately broad. Common values include:

- `placement_formula`: 安星诀 / 起例诀 / 排盘诀; belongs to chart construction, not interpretive claims;
- `aphorism_rule`: classical 断语 that can become a claim rule;
- `pattern_rule`: 格局 / 成格 rules;
- `modifier_rule`: 加会、逢煞、逢忌、破格等修正规则;
- `temporal_rule`: 大限、流年、流月等限运规则;
- `commentary`: explanatory text retained for context.

## Text fields

- `source_text_zh_hans` (source item) preserves the whole source passage as found in the imported Markdown whenever possible.
- `text_zh_hans` (clause) records one candidate rule phrase carved out of that passage.
- `notes_zh_hans` (on the source item or on a clause) records source variants, unresolved location work, and normalization caveats.

The Markdown source text is treated as canonical for this source-import PR. If an existing pilot rule uses a different normalized wording than its clause, the inventory records that difference in the clause/source-item `notes_zh_hans` instead of rewriting the imported volumes.

## Relationship to the rule corpus and runtime

- The Markdown volumes under `docs/zh-CN/sources/quan_shu/` are the canonical, human-readable source text.
- This source inventory TOML is machine-checkable corpus tracking only. It is **not** part of the runtime chart-evaluation path: nothing in `crates/iztro/src/` parses it, `evaluate_classical` does not depend on it, and the Markdown volumes are never parsed at runtime.
- `crates/iztro/tests/classical_source_inventory.rs` validates the inventory and its links to `crates/iztro/rule-corpus/quan-shu/rules.toml` using private, test-only structs. It asserts that the inventory parses, has unique `source_id`s, has non-empty required fields, that `clause_id`s are unique within a source item and clause fields are non-empty, that every rule `source_id`/`source_clause_id` exists in the inventory, that every `linked_rule_ids` entry exists in the rule corpus, that linked clauses and rules agree on `source_id`, `source_clause_id` and `work`, that clause text matches/contains the linked rule's source text (or a `notes_zh_hans` documents the divergence), and that a located passage's text contains each of its clause texts. It also locks the 天马空亡 clause wording to `马遇空亡，终身奔走`.

## Known pilot limitations

These are intentionally **allowed** in this pilot slice and are not test failures yet:

- `anchor = "TODO"` for items not yet located in the Markdown volumes;
- `section = "待校"` for sections still pending source review;
- a clause text differing from its linked rule's `source_text_zh_hans` (the imported Markdown wording is preserved as canonical, while a normalized rule clause is documented via `notes_zh_hans`);
- the inventory covers the five encoded pilot rules plus a first batch of segmented-but-unlinked 太微赋 clauses; it is not a complete inventory of Volume 1;
- only `volume-01.toml` exists; Volume 2 and Volume 3 have no source inventory TOML yet.

Tightening these (resolving TODO anchors, 待校 sections, and reconciling variants) is deferred to follow-up source-review PRs.

## Notes

The inventory is still a partial slice: it records the five existing classical pilot rules and a first batch of segmented 太微赋 clauses, with a maintained coverage report (`docs/zh-CN/rules/quan-shu-coverage.md`). A complete line-by-line inventory and corpus linting should be added in follow-up PRs; normalizing and implementing the segmented clauses is handled separately from segmentation.
