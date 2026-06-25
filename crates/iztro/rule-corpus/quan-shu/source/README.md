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

## Files

- `volume-01.toml`: pilot source inventory for the five currently encoded classical pilot rules.

This is not a complete line-by-line inventory of Volume 1. It is a deliberately small first slice that establishes the inventory format and links the existing rule `source_id`s to reviewed source passages where possible.

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

- `source_text_zh_hans` preserves the source passage as found in the imported Markdown whenever possible.
- `normalized_clause_zh_hans` records the rule-facing clause shape currently used by the pilot rule.
- `notes_zh_hans` records source variants, unresolved location work, and normalization caveats.

The Markdown source text is treated as canonical for this source-import PR. If existing pilot rules use a different normalized wording, the inventory records that difference in `normalized_clause_zh_hans` and `notes_zh_hans` instead of rewriting the imported volumes.

## Relationship to the rule corpus and runtime

- The Markdown volumes under `docs/zh-CN/sources/quan_shu/` are the canonical, human-readable source text.
- This source inventory TOML is machine-checkable corpus tracking only. It is **not** part of the runtime chart-evaluation path: nothing in `crates/iztro/src/` parses it, `evaluate_classical` does not depend on it, and the Markdown volumes are never parsed at runtime.
- `crates/iztro/tests/classical_source_inventory.rs` validates the inventory and its links to `crates/iztro/rule-corpus/quan-shu/rules.toml` using private, test-only structs. It asserts that the inventory parses, has unique `source_id`s, has non-empty required fields, that every rule `source_id` exists in the inventory, that every `linked_rule_ids` entry exists in the rule corpus, and that linked source items and rules agree on `source_id` and `work`. It also locks the 天马空亡 source wording to `马遇空亡，终身奔走`.

## Known pilot limitations

These are intentionally **allowed** in this pilot slice and are not test failures yet:

- `anchor = "TODO"` for items not yet located in the Markdown volumes;
- `section = "待校"` for sections still pending source review;
- `normalized_clause_zh_hans` differing from `source_text_zh_hans` (the imported Markdown wording is preserved as canonical, while the rule-facing clause shape is recorded separately);
- the rule's own `source_text_zh_hans` may keep a variant (e.g. `马落空亡`) while the inventory records the imported canonical wording (`马遇空亡`); the divergence is recorded in `notes_zh_hans`;
- the inventory contains only the five pilot entries for the currently encoded rules;
- only `volume-01.toml` exists; Volume 2 and Volume 3 have no source inventory TOML yet.

Tightening these (resolving TODO anchors, 待校 sections, and reconciling variants) is deferred to follow-up source-review PRs.

## Notes

For this PR, the inventory is intentionally a pilot slice: it only records source entries for the five existing classical pilot rules. Full line-by-line inventory, linting, and coverage reporting should be added in follow-up PRs.
