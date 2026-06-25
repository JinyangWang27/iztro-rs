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

## Notes

For this PR, the inventory is intentionally a pilot slice: it only records source entries for the five existing classical pilot rules. Full line-by-line inventory, linting, and coverage reporting should be added in follow-up PRs.
