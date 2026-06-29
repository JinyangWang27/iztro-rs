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
  -> QuanShu rule metadata TOML OR pattern-derived runtime rule metadata TOML
  -> Rust predicates / pattern detector / evaluator
  -> structured Claim[] OR PatternDetection[]
```

## Structure: atomic source items

A single physical Markdown line often holds several independent aphorisms, e.g.

```text
禄逢冲破，吉处藏凶。马遇空亡，终身奔走。
```

These are **two** source items, not one passage with two clauses. The inventory
models two levels:

```text
source item = one atomic cited QuanShu source unit / rule-candidate aphorism,
              identified by a stable mnemonic `source_id`
rule        = an executable / normalized / ambiguous / rejected interpretation
              linked from a source item via `linked_rule_ids`
```

Source-item boundaries are **semantic, not typographical**:

- `。` is the default top-level breaker;
- a single `。` sentence holding clearly parallel independent aphorisms is split
  further (e.g. `紫微天府全依辅弼之功，七杀破军专依羊铃之虐`);
- the condition/result commas of one aphorism are **not** split (e.g.
  `魁钺同行，位居台辅` stays one source item).

`source_id` identifies the **cited source unit**, not a physical line/passage,
and is a stable mnemonic (e.g. `ma_yu_kong_wang`). `source_order` preserves
source order separately from stable identity, so inserting an earlier aphorism
only requires reviewing `source_order`, never rewriting stable `source_id`
references. A rule links to a source item through its own `source_id`; the item
mirrors that link through `linked_rule_ids`. QuanShu rules no longer carry
`source_clause_id`.

### Canonical encoding: grouped/defaulted TOML

To avoid repeating the shared section metadata on every item, the TOML is
**grouped**: a `source_group` carries the shared defaults (`source_id_prefix`,
`work`, `volume`, `section`, `category`, `status`, `doc_path`, `anchor`) and each
`source_group.item` is one atomic source unit. The full id of an item is
`source_id = source_id_prefix + item.key`.

```toml
[[source_group]]
source_id_prefix = "quan_shu.v01.tai_wei_fu."
work = "zi_wei_dou_shu_quan_shu"
volume = 1
section = "太微赋"
category = "aphorism_rule"
status = "rule_linked"
doc_path = "docs/zh-CN/sources/quan_shu/volume-01.md"
anchor = "太微赋"

[[source_group.item]]
key = "ma_yu_kong_wang"
source_order = 2
source_text_zh_hans = "马遇空亡，终身奔走"
linked_rule_ids = ["migration.tian_ma_void.restless_movement"]
```

The grouped TOML is the **single canonical source of truth**. Tests deserialize
it and expand each `source_group.item` (joining group defaults, computing
`source_id`) into a flat source-item view for validation and coverage.

`source_text_zh_hans` quotes the cited source unit verbatim and uses the
clause-style form **without** a sentence-final `。`. Interpretation belongs in
the linked rule's `normalized_note_zh_hans`, `ClaimSpec`, or i18n claim text —
never here.

## Files

- `volume-01.toml`: source inventory for Volume 1. Every 太微赋「例曰」aphorism is its own atomic source item linked to a runtime rule (`status = "rule_linked"`). The end-of-volume pattern catalogues `定富局`, `定贵局`, `定贫贱局`, and `定杂局` are segmented as `pattern_rule` source groups. The inventory itself remains source-governance data; a conservative executable subset is wired separately through `core::pattern` and `rule-corpus/patterns/rules.toml`.

> This inventory tracks **only** genuine 《紫微斗数全书》 passages. Rules derived from chart structures the project models directly rather than from a cited QuanShu passage — e.g. 羊陀夹命 and 昌曲夹命 (夹宫 shapes) — are **not** QuanShu source entries; they live in `crates/iztro/rule-corpus/patterns/` with `work = "iztro_pattern_catalog"` and `pattern.*` source ids, and are not tracked here. `pending` is reserved for items believed to be from QuanShu but not yet located in the Markdown volumes.

This is not a complete line-by-line inventory of Volume 1. It is a deliberately small slice that establishes the inventory format, links each reviewed 太微赋 source unit to a rule via `source_id`, and records the first source-backed pattern catalogues. Future expansion may add `raw` / `segmented` source items that are not yet linked (`linked_rule_ids = []`); for the current 太微赋 normalization map every `rule_linked` item carries at least one link. Segmented `pattern_rule` entries may stay unlinked in the inventory even when a conservative subset is referenced by pattern-derived runtime rules.

## Coverage report

A committed-but-generated coverage report lives at:

```text
docs/zh-CN/rules/quan-shu-coverage.md
```

It summarizes the inventory (source items, located/pending and linked/unlinked source items, and linked rules by status) and is maintained by tests: `crates/iztro/tests/classical_source_coverage.rs` recomputes the metrics and asserts the committed file is current. Expanding this inventory must regenerate the report, or the test fails. A segmentation PR does not need to add executable rules; it can add unlinked `raw` / `segmented` source items (`linked_rule_ids = []`) and update the report.

## Status values

The `status` field tracks source-processing maturity, not executable rule maturity:

- `raw`: source unit recorded only;
- `segmented`: source line split into atomic source items;
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

- `source_text_zh_hans` (source item) quotes the cited source unit verbatim, in clause-style form without a sentence-final `。`.
- `notes_zh_hans` (on the source item) records purely **explanatory** notes — unresolved location work, or a source-fidelity remark (e.g. a unit that is read together with the next line in the original). It does **not** authorize the source text to diverge from the linked rule.

The Markdown source text is canonical. A QuanShu rule's `source_text_zh_hans` must quote the same source unit verbatim, and the source item's text must **equal** it after light punctuation normalization (a `notes_zh_hans` does not bypass this). Interpretation/paraphrase belongs in `normalized_note_zh_hans`, `ClaimSpec`, or i18n claim text, not in the source inventory. A future genuine source variant should be opted in explicitly (e.g. a `source_text_variant_ok` field), not waved through with a note.

## Relationship to the rule corpus and runtime

- The Markdown volumes under `docs/zh-CN/sources/quan_shu/` are the canonical, human-readable source text.
- This source inventory TOML is machine-checkable corpus tracking only. It is **not** part of the runtime chart-evaluation path: nothing in `crates/iztro/src/` parses it, `evaluate_classical` does not depend on it, and the Markdown volumes are never parsed at runtime.
- Source-backed pattern detections carry small static metadata in `core::pattern::metadata` only after a pattern has an executable detector. A separate runtime rule entry in `crates/iztro/rule-corpus/patterns/rules.toml` is required before `evaluate_classical` can emit a source hit or claim for that pattern.
- `crates/iztro/tests/classical_source_inventory.rs` validates the inventory and its links to `crates/iztro/rule-corpus/quan-shu/rules.toml` and QuanShu-sourced entries in `crates/iztro/rule-corpus/patterns/rules.toml` using private, test-only structs. It asserts that the inventory parses, has unique `source_id`s, has non-empty required fields, that 太微赋 and the four source-backed pattern sections have continuous section-local `source_order`, that source ids are stable mnemonics (not purely numeric), that every `rule_linked` item is linked, that every QuanShu rule `source_id` exists in the inventory, that every `linked_rule_ids` entry exists in the QuanShu rule corpus, that linked items and rules agree on `source_id` and `work`, and that a source item's text **equals** the linked rule's `source_text_zh_hans` after light punctuation normalization (a `notes_zh_hans` does not bypass this). It also locks the 天马空亡 source unit to `马遇空亡，终身奔走`, the 禄马交驰 unit to `禄马最喜交驰`, and the 日月反背 unit to `日月最嫌反背`. QuanShu-sourced pattern runtime rules must cite existing `pattern_rule` inventory items and must match their source text exactly; project-owned pattern rules keep `work = "iztro_pattern_catalog"` and `pattern.*` source ids.

## Known pilot limitations

These are intentionally **allowed** in this pilot slice and are not test failures yet:

- `anchor = "TODO"` for items not yet located in the Markdown volumes;
- `section = "待校"` for sections still pending source review;
- the inventory covers the complete 太微赋 normalization map plus the four end-of-volume pattern catalogues; it is not a complete inventory of every section of Volume 1;
- only `volume-01.toml` exists; Volume 2 and Volume 3 have no source inventory TOML yet.

Tightening these (resolving TODO anchors, 待校 sections, and reconciling variants) is deferred to follow-up source-review PRs.

## Notes

The inventory is still a partial slice of the whole work: it records the complete 太微赋 normalization map as atomic source items, with a maintained coverage report (`docs/zh-CN/rules/quan-shu-coverage.md`). A complete line-by-line inventory of the remaining sections and corpus linting should be added in follow-up PRs; normalizing and implementing each linked source unit as an executable rule is handled separately from inventory segmentation.
