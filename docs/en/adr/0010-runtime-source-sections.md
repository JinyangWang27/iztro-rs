# ADR 0010: Runtime Source Section Table

Status: Accepted

## Context

Display layers (starting with the GUI inspector) need to cite where a classical
rule hit or a source-backed pattern detection comes from: work, 卷 (volume), and
节 (section heading), pointing into the canonical Markdown volumes under
`docs/zh-CN/sources/quan_shu/`.

That metadata lived only in the QuanShu source inventory
(`rule-corpus/quan-shu/source/volume-*.toml`), which is corpus-governance data
parsed exclusively by tests (ADR 0008 era boundary: "nothing in `src/` parses
it"). At runtime, rules carried only `source_id` and verbatim source text;
`PatternSourceMetadata` duplicated a `section` string by hand.

Embedding the full inventory at runtime would cost ~100KB of binary for data
that is mostly governance bookkeeping (item ordering, link status, categories).
Denormalizing volume/section into every `[[rule]]` entry would copy group-level
facts across 64 rules and still leave pattern detections uncovered.

## Decision

Add a small **runtime section table**, `rule-corpus/quan-shu/sections.toml`:
one entry per inventory `source_group`, carrying only `source_id_prefix`,
`work`, `volume`, and `section`. It is embedded via `include_str!` and exposed
as `rules::source::source_section(source_id) -> Option<&'static SourceSection>`.

- Lookup is an exact `HashMap` hit on the id truncated after its final `.`.
  This is valid because inventory item keys never contain dots — an invariant
  the sync test enforces implicitly by resolving every inventory item.
- `SourceSection` stores neither `anchor` (always equal to `section`) nor
  `doc_path` (always derivable as
  `docs/zh-CN/sources/quan_shu/volume-{volume:02}.md`). The sync test
  (`tests/source_sections.rs`) enforces both derivability invariants against
  the inventory, in both directions, exempting pending (`待校`/`TODO`) units.
- `SourceSection` lives in `rules::source` beside `SourceRef`/`ClassicalWork`:
  a citation is metadata about a source, not a separate concept.
- `PatternSourceMetadata.section` is removed; the section table is the single
  runtime owner of section metadata for both rule hits and pattern detections.
- The full inventory and all item-level/governance fields (`status`,
  `category`, `linked_rule_ids`, `source_order`) remain test-only. The
  runtime/governance boundary moves by exactly one small table, no further.

## Consequences

- GUI and other display layers can cite 《紫微斗数全书》卷/节 for any resolvable
  `source_id` without parsing governance data, at ~1–2KB binary cost.
- Section metadata is stored once; rules.toml, pattern metadata, and the
  inventory cannot drift apart (tests fail if they do).
- Adding a new inventory `source_group` requires a matching `sections.toml`
  entry; `tests/source_sections.rs` fails loudly until it exists.
- If a future source work breaks the anchor/doc_path derivability invariants,
  the table must grow those fields back; the tripwire test documents this.
