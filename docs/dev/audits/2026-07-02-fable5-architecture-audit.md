# iztro-rs Architecture Audit — 2026-07-02

- **Reviewer**: Claude Fable 5 (read-only architecture audit and remediation-planning pass)
- **Target**: `main` @ `3329241` (post PR #155, "refactor(pattern): move detectors to named-pattern layout")
- **Verification commands run**: `cargo fmt --all -- --check` ✅, `cargo clippy --workspace --all-targets -- -D warnings` ✅, `cargo test --workspace` ✅ (all suites green, incl. `patterns`, `classical_source_inventory`, `classical_rule_guardrails`)
- **Scope**: docs (`current-status`, `project-spec`, `architecture`, ADRs 0008/0009, `patterns`, `rules/rule-engine`), `crates/iztro/src/{core,rules,analysis,projection,facade,features,reading,render}`, `crates/iztro-gui`, `crates/iztro-i18n`, `crates/iztro-cli`, `rule-corpus/`, `crates/iztro/tests/`

## Triage status

- [ ] F1 — Ancestor analysis-layer cache identity instability
- [x] F2 — GUI derivation-guard test does not scan `static_chart_screen/`
  - Addressed by: PR #157
- [ ] F3 — Shared relation type/helpers live inside `rules::pattern`
- [ ] F4 — Overlay-aware rule wiring duplicated by string literal
- [ ] F5 — No detector-registration guard for `PatternId`
- [ ] F6 — `effective_stars_in_palace` duplicate/misleading helper
- [ ] F7 — `matches_rule_metadata` hard-codes `ClaimScope::Natal`
- [ ] F8 — Docs polarity drift for 马头带剑 / reserved id note
- [ ] F9 — i18n claim-key coverage skips patterns corpus
- [ ] F10 — Overlapping palace-relation vocabularies
- [ ] F11 — Panicking constructors on future binding/MCP surface
- [ ] F12 — Stringly `work` in `PatternSourceMetadata`

No files were modified during the audit itself; this document records the findings.

---

## 1. Executive summary

Architecture health: **strong**. The four-layer boundary (core facts → rules → analysis → projection/facade → GUI) holds in code, not just docs. Core has zero dependencies on `rules`/`analysis`/`projection` (verified by import scan). The GUI derives nothing (verified by import scan, manual read of `app.rs`/`analysis.rs`/inspector, and a self-guard test — though that guard has a hole, see F2). Temporal overlays are additive (`HoroscopeChart::push_layer` is the only mutator; `EffectiveChartState` is read-only, scope-validated, fails loudly on missing/duplicate layers). Source text discipline is enforced by real tests, not convention.

Highest-risk area: **analysis-layer cache identity** (F1). `AnalysisLayerKey::Monthly`/`Daily` results are not content-stable across selection depth, because ancestor layers are built from the deepest selection's concrete target date, and the monthly layer's stem-branch is a solar-term month pillar that varies by day within one lunar month. This is exactly the ancestor-cache-pollution class the design tried to prevent via scope truncation — scope truncation works; **target-coordinate truncation is missing**.

Readiness:

| Direction | Verdict |
|---|---|
| More pattern detector expansion | **Ready** (add F5 detector-registration guard first, cheap) |
| More classical rule expansion | **Ready** (guardrails are excellent; add F4 wiring test as corpus grows) |
| MCP design | **Ready now** — read-model surface is coherent and serializable |
| MCP implementation | **After F1 fix** — agents will cache by `AnalysisLayerKey`; key must be content-stable first |
| Python binding design | **Ready** at read-model level (snapshots/projections/analysis results all `Serialize`) |

---

## 2. Confirmed boundary violations

### F3. Shared relation type/helpers live inside `rules::pattern`; classical engine imports pattern internals

Type:
- Confirmed boundary violation (drift against ADR 0009 §8)

Severity:
- Medium

Confidence:
- High

Evidence:
- `crates/iztro/src/rules/classical/evidence.rs:13` — `use crate::rules::pattern::relation::PalaceRelation;` (also `evaluator.rs:31`).
- `crates/iztro/src/rules/pattern/relation.rs` — generic branch-level relation helpers (`opposite`, `trine_branches`, `san_fang_si_zheng`, `clamp_branches`) and the `PalaceRelation` enum.
- `crates/iztro/src/rules/query.rs:301` — a **second private copy** of `clamp_branches`.
- `EvidenceKind::PalaceRelation` (classical evidence model) embeds the pattern-namespace enum; it also appears in serialized claim JSON.

Why it matters:
- ADR 0009 §8: generic queries shared by both engines belong in `rules::query`; neither engine may depend on the other's internals. `PalaceRelation` + branch relations are generic chart geometry used by both engines' evidence, so `classical → pattern::relation` is the exact dependency shape the ADR forbids, and the duplicated `clamp_branches` is the exact duplication the ADR forbids. (`PatternId` in `EvidenceKind::PatternShapeMatched` is different — a deliberate, documented cross-reference to canonical pattern identity; not a violation.)

Minimal safe fix:
- Move `relation.rs` to `rules::relation` (or fold into `rules::query`); `pub use` from `rules::pattern` to keep the public path stable (serde output unchanged — enum is `snake_case`-tagged by value, not by module path). Delete the private `clamp_branches` in `rules/query.rs` in favor of the shared one.

Optional long-term cleanup:
- Also unify with `features::relations::PalaceRelation`/`PalaceRelationKind` naming (third relation vocabulary at `PalaceName` level; see F10).

Files likely touched:
- `rules/pattern/relation.rs` → `rules/relation.rs`, `rules/mod.rs`, `rules/query.rs`, `rules/classical/{evidence,evaluator}.rs`, `rules/pattern/{mod,query}.rs`, `lib.rs` re-export.

Tests to add/update:
- Existing serde snapshot tests must stay byte-identical (patterns/classical suites). No new tests needed; run `cargo test -p iztro`.

PR recommendation:
- Immediate / Small / normal review.

---

## 3. Semantic risks / near-misses

### F1. Ancestor analysis-layer results are not content-stable across selection depth (Monthly/Daily cache identity)

Type:
- Semantic risk

Severity:
- High

Confidence:
- High for the mechanism (verified in code); Medium-High that real charts diverge (solar-term boundary falls inside nearly every lunar month; not reproduced with a concrete fixture in this audit — do that first in the fix PR).

Evidence:
- `crates/iztro/src/analysis/selected.rs:80` — one shared horoscope stack built via `build_selected_temporal_chart(natal, selection)` for the **deepest** selection; every requested ancestor key is detected against it.
- `crates/iztro/src/core/placement/overlay/selected_temporal.rs:22,157-217` — a Monthly selection resolves its target as representative lunar day 15 + `BirthTime::EarlyZi`; a Daily selection uses the actual day + `EarlyZi`; an Hourly selection uses actual day + **actual hour**.
- `crates/iztro/src/core/placement/overlay/partial_horoscope.rs:126-159` — that single `target` feeds `build_monthly_period`, `build_daily_period`, `build_hourly_period`.
- `crates/iztro/src/core/model/chart/monthly.rs:75-104` — monthly `stem_branch` = `conversion.four_pillars().monthly`, i.e. the target **solar date's** month pillar (节气-based). Days within one lunar month straddle a 节气 boundary, so day 15 vs the selected day can carry different month pillars.
- `crates/iztro/src/core/placement/overlay/monthly_horoscope.rs:17-30` — monthly flow stars + mutagen activations derive from that stem-branch, so monthly-layer facts differ.
- Exposure path exists in the shipped GUI: `crates/iztro-gui/src/app.rs` `refresh_analysis` requests **all missing keys under the current selection** — e.g. `SelectToday` jumps straight to an Hourly selection, so `AnalysisLayerKey::Monthly{...}` gets detected and cached from the actual-day/actual-hour stack. Navigating up to the Monthly view then reuses that cached result, which a fresh Monthly-selection detection (day 15) could contradict. Same class for `Daily` under `Hourly` (hour index 12 = late Zi shifts the resolved lunar day in `solar_to_lunar`).
- Note: scope truncation (`analysis_scopes_for_layer_key`, `EffectiveChartState` active-scope validation) works correctly — descendant *scopes* never leak. The leak is descendant *target coordinates* baked into ancestor layer construction. `detect.rs:27-31` already documents "context must correspond to key … not currently validated" as a caller contract; the batch facade violates the spirit of it for ancestor keys.

Why it matters:
- `AnalysisLayerKey` is the cache identity for the GUI today and MCP/agents tomorrow. Same key → two possible fact sets depending on navigation path = nondeterministic, path-dependent analysis output, contradicting the project's core determinism promise and the docs' explicit "changing 流日/流时 never changes a cached 流月/流年 result" claim (`docs/en/rules/rule-engine.md`).

Minimal safe fix:
- In `detect_static_temporal_analysis_layers_from_chart`, build each requested key's context from the **key's own canonical selection** (Monthly key → `StaticTemporalNavigationSelection::Monthly{...}`, i.e. representative day 15/EarlyZi; Daily key → its Daily selection with EarlyZi), instead of one shared deepest-selection stack. Keys sharing a canonical selection can share a built stack (memoize per distinct selection inside the call). No change to chart placement, no change to the projection/display path (`static_temporal_chart_view` correctly shows date-true overlays for the selected view — leave it alone).

Optional long-term cleanup:
- Decide and document whether 流月 analysis identity *should* be representative-day-based (current key shape implies yes); if a school needs date-true monthly analysis, that is a new key shape, not a silent behavior.

Files likely touched:
- `crates/iztro/src/analysis/selected.rs` (context per key), possibly a small helper in `core/placement/overlay/selected_temporal.rs` (canonical selection for a key).

Tests to add/update:
- `tests/analysis_selected.rs`: `ancestor_layer_results_are_identical_across_selection_depths` — pick a fixture date where the lunar month straddles a 节气; assert `detect(...Monthly key...)` equal under Monthly, Daily, and Hourly selections (and Daily key equal under Daily vs Hourly with `hour_index = 12`).
- First write this test against current code to *prove* divergence (red), then fix (green).

PR recommendation:
- Immediate / Medium / **deeper review justified** (temporal semantics).

### F6. `rules::query::effective_stars_in_palace` is an unguarded duplicate of `selected_stars_in_palace`

Type:
- Semantic risk (API invites the PR #145 bug class)

Severity:
- Medium

Confidence:
- High

Evidence:
- `crates/iztro/src/rules/query.rs:74-91` — the two functions have byte-identical bodies. Every other `effective_*` helper takes `match_scope` and gates through `effective_state_for_match_scope`; `effective_stars_in_palace` takes no scope and reads the selected state directly, so it is a selected-state helper wearing the effective-family name.
- Wrapped verbatim by `rules/pattern/query.rs:277-293`.

Why it matters:
- The pattern module's own docs call the selected-vs-source distinction "load-bearing" (PR #145). A misnamed helper in the shared layer re-opens that trap for the next rule author.

Minimal safe fix:
- Delete `effective_stars_in_palace` (and its pattern wrapper) or give it the `match_scope` parameter + gate like its siblings. Grep callers first; tests stay green if none rely on the ungated form.

Files likely touched:
- `rules/query.rs`, `rules/pattern/query.rs`, any detector call sites.

Tests to add/update:
- None new; existing pattern suites cover behavior.

PR recommendation:
- Immediate / Small / normal review.

### F7. `matches_rule_metadata` hard-codes `ClaimScope::Natal` for scope filtering

Type:
- Semantic risk (stale assumption)

Severity:
- Low

Confidence:
- High

Evidence:
- `crates/iztro/src/rules/classical/engine.rs:88` — `let scope_ok = self.scopes.is_empty() || self.scopes.contains(&ClaimScope::Natal);` inside metadata-level filtering used by `DiagnosticMode::MatchingRequest`.

Why it matters:
- Assumes every corpus rule is natal-scoped. Already false for 昌曲夹命 (overlay-aware). Today harmless — only `Unsupported` rules produce diagnostics and all current unsupported rules are natal — but it will silently mis-filter the first overlay-aware rule that gains an `Unsupported` arm.

Minimal safe fix:
- Resolve scopes via `classical_rule_metadata(rule.id).applicable_scopes` (or leave scope unconstrained for diagnostics with a comment).

Files likely touched:
- `rules/classical/engine.rs`.

Tests to add/update:
- Unit test: `MatchingRequest` with `scopes=[Yearly]` keeps a diagnostic for a rule whose `applicable_scopes` includes Yearly.

PR recommendation:
- Immediate (piggyback on F4 PR) / Small.

### F11. Panicking `horoscope_with_frame` constructors are on the public API

Type:
- Semantic risk (future MCP/binding surface)

Severity:
- Low (today), Medium once MCP exists

Confidence:
- High

Evidence:
- `core/rule_context.rs:88-102` (`.expect(...)`), mirrored by `PatternContext::horoscope_with_frame`, `ClassicalRuleContext::horoscope_with_frame`; also `pattern_spec()` panics on unknown id (`registry.rs:438-440`). All re-exported at crate root.

Why it matters:
- Internal callers validate first (analysis facade validates keys, `EffectiveChartState` returns `ChartError`), so no current bug. But an MCP/PyO3 layer calling these with attacker-shaped input turns validation errors into process aborts.

Minimal safe fix:
- Nothing now; record the rule "MCP/bindings call only `Result`-returning facades" (already implicit in §11 below). Optionally add `try_horoscope_with_frame` later.

PR recommendation:
- Defer (document in MCP design doc) / Small.

---

## 4. Cleanup candidates

### F10. Three overlapping palace-relation vocabularies

Type:
- Cleanup candidate

Severity:
- Low

Confidence:
- High

Evidence:
- `features/relations.rs` (`PalaceRelation`, `PalaceRelationKind`, `PalaceRelations` at `PalaceName` level, placeholder-era), `rules/pattern/relation.rs` (`PalaceRelation` at branch level, live), and the crate root re-exports the pattern one while `features` exports its own under the same name.

Why it matters:
- Two public types named `PalaceRelation` with different semantics; `features` is scaffold-status but publicly exported.

Minimal safe fix:
- When doing F3's move, rename the features type to `PalaceNameRelation` (or mark the whole `features` module clearly scaffold in rustdoc). No behavior change.

PR recommendation:
- Defer (bundle with F3 or next features work) / Small.

### F12. Stringly `work` in `PatternSourceMetadata`

Type:
- Cleanup candidate

Severity:
- Low

Confidence:
- High

Evidence:
- `rules/pattern/registry.rs:32` — `QUAN_SHU_WORK: &str = "zi_wei_dou_shu_quan_shu"` while `rules::classical::ClassicalWork` is the typed enum with the same serde string.

Why it matters:
- Same identity in two representations; the inventory cross-check test papers over it, but a typo in a future pattern source entry compiles.

Minimal safe fix:
- Either reuse `ClassicalWork` (after F3, a shared `rules::source` module is the clean home), or add a const-eq test binding the string to `ClassicalWork::ZiWeiDouShuQuanShu`'s serde name.

PR recommendation:
- Defer / Small.

### Minor (no template needed)

- `evaluate_classical_in_context` re-runs every natal predicate per layer; with 7 layers × 66 rules this is trivially cheap now — note only, no action.
- `classical_rule_metadata` is a linear scan per lookup; fine at current corpus size, switch to a map if MCP makes it hot.
- `source_*` aliases in `rules/pattern/query.rs` duplicate `*_for_scope` names 1:1 — documented as intentional naming migration; finish the migration eventually (pick one name).
- `iztro-cli` is a 5-line stub; matches docs ("CLI beyond examples deferred"). No action.

---

## 5. Pattern/classical rule architecture audit

Clean overall:

- **Patterns are rules**: `rules::pattern` is the only pattern engine; `core::pattern` does not exist; `core::rule_context.rs` is the documented transitional home of `RuleEvaluationContext` (ADR 0009 explicitly sanctions it). No `is_pattern`/`is_classical` flags anywhere in the context (grep-verified).
- **Single canonical pattern identity** holds: QuanShu catalogue entries do not create classical runtime rules (`classical_source_inventory.rs::source_backed_pattern_catalogues_do_not_create_classical_rules` enforces it); `rule-corpus/patterns/rules.toml` holds only the two project-owned `pattern.*` rules; `evaluate_classical` never consumes pattern detections.
- **Separation of source hit / claim / diagnostic / corpus metadata** preserved end-to-end (`RuleOutcome`, `ClaimEvaluation`, panel view keeps four separate vectors).
- **Conservative emission** verified: brightness rules gate on `is_bright`/`is_dim` with `Unknown` never qualifying; no partial/near-pattern status; `Broken` reserved for formed-then-damaged.
- Gaps: F4 (overlay-wiring duplication, below), F5 (detector-registration guard, below), F3 (relation namespace).

### F4. Overlay-aware rule wiring is duplicated by string literal in two modules with no consistency test

Type:
- Missing test / drift risk

Severity:
- Medium

Confidence:
- High

Evidence:
- `rules/classical/evaluator.rs:98-109` — `evaluate_in_context` dispatches 昌曲夹命 to the selected-state arm via const `CHANG_QU_CLAMP_LIFE`.
- `rules/classical/metadata.rs:41-46` — `applicable_scopes_for_rule` matches the **same rule id as a separate string literal** to advertise `ALL_SELECTED_SCOPES`.
- No test ties the two: a rule could advertise temporal scopes without a context arm (GUI shows it as overlay-capable, hits never appear) or gain a context arm without advertising (hits in unadvertised scopes).

Why it matters:
- This is the "do not promote classical rules to temporal scopes implicitly" invariant. Right now it survives on one contributor remembering two files.

Minimal safe fix:
- Single source of truth: one `const OVERLAY_AWARE_RULES: &[(&str, &'static [ClaimScope])]` consumed by both `applicable_scopes_for_rule` and (as a membership check) `evaluate_in_context`; plus a test that evaluates each overlay-aware rule in a non-natal frame on a matching synthetic chart and asserts the emitted scope ∈ `applicable_scopes`, and that natal-only rules never emit non-natal scopes.

Files likely touched:
- `rules/classical/{evaluator,metadata}.rs`, `tests/classical_rules.rs` or `classical_rule_guardrails.rs`.

PR recommendation:
- Immediate / Small.

### F5. No detector-registration guard for `PatternId`

Type:
- Missing test

Severity:
- Medium

Confidence:
- High

Evidence:
- `PatternId::ALL` has 26 variants (`model.rs:82`); `patterns/mod.rs::detect_all` wires 25 detectors; `LingChangTuoWu` is intentionally detector-less (only signal: a Chinese condition note in `registry.rs:125` "当前保留 id，未注册检测器"). Registry coverage tests (`patterns.rs:1551`) cover *metadata*, not *detector wiring*. `docs/en/patterns.md` catalog also omits `LingChangTuoWu` entirely.

Why it matters:
- The "detector omission / duplicate registration" risk is real: add a `PatternId` + registry entry, forget the `detect_all` line, everything compiles and all existing tests pass; the pattern just silently never fires. Duplicate `detect(...)` lines would double-emit.

Minimal safe fix:
- Test with an explicit allowlist: `const DETECTORLESS: [PatternId; 1] = [LingChangTuoWu];` then assert each non-exempt pattern id appears exactly once in a machine-checkable detector list — e.g. change `detect_all` to iterate a `const DETECTORS: &[(PatternId, DetectFn)]` table and assert table ids == `ALL` minus allowlist, no duplicates. That refactor also kills the duplicate-registration risk structurally.

Files likely touched:
- `rules/pattern/patterns/mod.rs` (+ table), `tests/patterns.rs`.

Tests to add/update:
- `every_pattern_id_has_exactly_one_detector_or_is_allowlisted`.

PR recommendation:
- Immediate / Small (mechanical table refactor keeps detection order explicit).

---

## 6. Source/corpus/metadata audit

Checked: `rule-corpus/quan-shu/rules.toml` (64 rules; 5 executable, 1 normalized-but-handled `lu_ma_jiao_chi` mapped to typed `Unsupported`), `rule-corpus/patterns/rules.toml` (2 rules, both executable, `work = iztro_pattern_catalog`, `pattern.*` source ids), `source/volume-01.toml` grouped inventory, runtime metadata table, evaluator dispatch.

Consistent:

- `HANDLED_RULE_IDS` (8) = 7 executable + 1 normalized-with-typed-unsupported; three-way guardrails in `evaluator.rs` tests (`every_executable_corpus_rule_is_handled`, `handled_rule_ids_reference_real_corpus_rules`, `every_handled_id_reaches_a_live_dispatch_arm`) close the loop in both directions.
- All 9 pattern `PatternSourceMetadata.source_id`s resolve in `volume-01.toml` grouped form (prefix+key verified); `canonical_pattern_metadata_references_source_inventory` enforces it with category checks.
- Verbatim source text: inventory tests check item text ↔ rule text equality, whitespace, unique ids, continuous `source_order`, link symmetry. `ClassicalRuleHitRef` never carries source text (compile-time + test). `RiChuFuSang` id/display/provenance triple matches docs exactly.
- No duplicate rule ids; no duplicate `(source_id, source_clause_id)` pairs (guardrail tests).

Issues:

### F8. Docs polarity drift for 马头带剑 (and undocumented reserved id)

Type:
- Documentation drift

Severity:
- Low

Confidence:
- High

Evidence:
- `docs/en/patterns.md:105` says Polarity "Mixed"; `docs/zh-CN/patterns.md:86` says 吉凶参半; runtime is `PatternPolarity::Neutral` (serializes `"neutral"`, `"mixed"` accepted only as a deserialize alias — `model.rs:134-146`). Also `LingChangTuoWu` exists as a public reserved `PatternId` but appears in neither language's catalog table.

Minimal safe fix:
- Update both docs tables: 马头带剑 → Neutral/中性 (note the mixed alias), add a "reserved ids without detectors" row/note for 铃昌陀武. Both languages in one PR (AGENTS.md bilingual rule).

Files likely touched:
- `docs/en/patterns.md`, `docs/zh-CN/patterns.md`.

PR recommendation:
- Immediate / Small (docs-only).

### F9. i18n claim-key coverage test skips the patterns corpus

Type:
- Missing test (weak guard)

Severity:
- Low

Confidence:
- High

Evidence:
- `crates/iztro-i18n/src/lib.rs:354-366` — `every_pilot_claim_text_exists_in_both_locales` iterates `quan_shu_rules()` only. The two `IztroPatternCatalog` claim keys happen to exist in both `.ftl` files today, but nothing guards the next one.

Minimal safe fix:
- Iterate `iztro::rules::classical::classical_rules()` instead.

Files likely touched:
- `crates/iztro-i18n/src/lib.rs` (test only).

PR recommendation:
- Immediate / Small.

---

## 7. Selected-frame and temporal analysis audit

- **Overlays additive**: `Chart` exposes no mutators; `HoroscopeChart::push_layer` appends; the partial stack builder moves natal in unchanged and only appends layers in fixed scope order. ✅
- **Branch = stable coordinate, palace name = frame**: `EffectiveChartState` resolves `branch_of_palace` from exactly one frame (natal or one `TemporalPalaceLayout`); a non-natal frame with a missing layout is a hard `ChartError::MissingHoroscopePalaceLayout`, never a silent natal fallback. Projection carries `natal_identity` + `active_frame` separately; the facade derives `active_frame_scope` from the single canonical mapping (流年 view → Yearly frame, never Age). ✅
- **Selected vs source/layer reads**: the two helper families are explicit, documented, and routed through `rules::query`/`EffectiveChartState`. `star_matches_for_scope` correctly refuses flow-star equivalence for Natal/Age frames, so temporal flow stars can't satisfy natal-name conditions in a natal frame. ✅
- **Descendant scope leak**: prevented by `analysis_scopes_for_layer_key` truncation + `EffectiveChartState` active-scope filtering + `scope_is_visible`; exercised by `analysis_layer.rs::analysis_scopes_truncate_to_each_layer` and the selected-facade validation tests. ✅
- **Descendant target-coordinate leak**: **not prevented** — F1 (§3). This is the one real hole.
- Frame/context constructors: the lenient `horoscope(...)` constructor fails closed (effective `None` → selected helpers return nothing) and is documented as compat-only; the strict production constructor panics (F11).

---

## 8. GUI/read-model audit

- `iztro-gui` consumes: `by_solar`, `static_temporal_chart_view(_from_chart)`, `temporal_selection_for_*`, `detect_static_temporal_analysis_layers_from_chart`, `classical_rule_metadata`, projections. A full import scan shows no placement, overlay-building, branch arithmetic (`offset`), 三方四正 derivation, or rule evaluation in GUI code; 三方四正 highlighting reads the prepared `surround` field (`app.rs:884-888`, with tests `*_reads_prepared_surround_only`). ✅
- Analysis cache (`analysis.rs`) is per-`AnalysisLayerKey`, in-memory, cleared on new chart, never persisted; source text resolved once per rule id via metadata; the user-facing stream is QuanShu-only via `user_facing()` (guardrail-tested on the iztro side). Highlight projection reads only structured `involved_*`/`Evidence` fields and is conservatively empty for unmappable variants. ✅
- Rolling day/hour navigation reads month lengths from prepared month snapshots via the cache — core decides 29/30-day months, not the GUI. ✅
- i18n: GUI labels route through `iztro-i18n` typed helpers; `iztro` never depends on `iztro-i18n` (Cargo-verified). Projections additionally carry conventional zh labels from `core::labels::zh_cn` — documented additive-label convention, not runtime i18n; consistent with `docs/en/architecture.md`. ✅
- One weak spot:

### F2. GUI derivation-guard test does not scan `static_chart_screen/`

Type:
- Missing test (weak guard)

Severity:
- Medium (the guard is the boundary's tripwire)

Confidence:
- High

Evidence:
- `crates/iztro-gui/src/app.rs:2764` — `std::fs::read_dir(&src_dir)` is non-recursive; the `static_chart_screen/` subdirectory (7 files, incl. `palace.rs` at 701 lines, `inspector.rs`, `temporal.rs`) is never checked against the 27-symbol forbidden list. Manual review of those files found no violations today.

Why it matters:
- The most render-heavy files are exactly the ones outside the tripwire; a future `.offset(` or `build_decadal_frame` there would land unnoticed.

Minimal safe fix:
- Recursive walk (small stack-based loop; no new dep), assert the `checked` count covers a known minimum.

Files likely touched:
- `crates/iztro-gui/src/app.rs` (test only).

PR recommendation:
- Immediate / Small.

---

## 9. Missing invariants and tests (consolidated)

Priority order:

1. **Ancestor-layer content stability** (F1): Monthly/Daily key equality across Monthly/Daily/Hourly selections, fixture straddling a 节气. *Would have caught the highest-risk issue in this audit.*
2. **Detector registration** (F5): every `PatternId` has exactly one detector or is allowlisted.
3. **Overlay-scope wiring** (F4): metadata `applicable_scopes` ↔ evaluator context arms, both directions.
4. **GUI guard recursion** (F2).
5. **Diagnostics scope filter** (F7 unit test).
6. **i18n claim-key coverage over all corpora** (F9).

Already well-covered (no action): natal immutability & layer validation (`effective_chart_state.rs`, model API shape), temporal-vs-natal palace-name separation (projection active-frame tests, temporal layout fixtures), scope truncation, pattern/classical query duplication (structurally routed via `rules::query` — modulo F3/F6), source-text leakage (inventory + hit-ref tests), metadata registry consistency (registry/guardrail tests), duplicate rule ids, GUI fact derivation (guard test, modulo F2).

---

## 10. MCP readiness assessment

**Design: ready. Implementation: after F1.**

Stable enough to expose (all `Result`-returning, serializable, deterministic):

| Proposed tool | Backing API |
|---|---|
| `generate_chart` | `by_solar` / `by_lunar` (+`_with_options_report` for diagnostics) → `NatalFacadeSnapshot` / `ChartDiagnosticSnapshot` |
| `chart_stack_snapshot` | `ChartStackSnapshot` (renderer-neutral layers) |
| `static_chart_view` | `static_temporal_chart_view(_from_chart)` + `temporal_selection_for_solar_moment` ("today") |
| `analyze_layers` | `analysis_layers_for_selection` + `detect_static_temporal_analysis_layers_from_chart` (compact `ClassicalRuleHitRef` + `PatternDetection`) |
| `rule_metadata` / `pattern_catalog` | `classical_rule_metadata`, `pattern_specs` / `pattern_source_metadata` (verbatim provenance, display notes) |
| `horoscope_facade` | `HoroscopeFacadeSnapshot::from_horoscope_chart` (upstream-shaped export) |

Do **not** expose yet:

- Panicking context constructors (`*::horoscope_with_frame`, `pattern_spec`) — F11.
- Raw `evaluate_classical_in_context` / `detect_analysis_layer` with caller-supplied contexts (the unvalidated context/key contract, `detect.rs:27-31`); the batch facade is the safe wrapper.
- `core::placement` builders, `build_*_layer` primitives, `features`/`reading` scaffolds (placeholder semantics), `ClassicalRulePanelView::developer()` diagnostics stream (fine for a dev flag, not the default surface).
- Anything keyed by `AnalysisLayerKey` until F1 lands — agent-side caching would bake the instability in.

Note: `PatternDetection`/`AnalysisLayerResult` are `Serialize`-only (borrowed `&'static` name) — fine for MCP responses (one-way), worth remembering for Python bindings (return dicts, don't round-trip).

---

## 11. Recommended PR sequence

| # | Title | Goal | Files | Risk | Tests | Review |
|---|---|---|---|---|---|---|
| 1 | test(gui): recursive derivation-guard scan | F2 | `iztro-gui/src/app.rs` (test) | None | GUI suite | Normal |
| 2 | test(rules): detector-registration + overlay-scope wiring invariants | F5, F4, F7 | `patterns/mod.rs` (detector table), `classical/{evaluator,metadata,engine}.rs`, guardrail tests | Low (mechanical table refactor) | `patterns`, `classical_rule_guardrails`, full `-p iztro` | Normal |
| 3 | fix(analysis): per-key canonical temporal context in batch facade | F1 | `analysis/selected.rs`, `core/placement/overlay/selected_temporal.rs`, `tests/analysis_selected.rs` | **Medium** — temporal semantics; write the red test first | New stability tests + full workspace | **Deeper review** (temporal/domain reviewer) |
| 4 | refactor(rules): move relation module to shared `rules::relation` | F3, F6, part of F10 | `rules/{relation,query,mod}.rs`, `pattern/{relation,query,mod}.rs`, `classical/{evidence,evaluator}.rs`, `lib.rs` | Low (moves + re-exports; serde unchanged) | Full `-p iztro`, snapshot suites must be byte-identical | Normal |
| 5 | docs+test: pattern polarity sync, reserved-id note, i18n corpus-wide claim keys | F8, F9 | `docs/{en,zh-CN}/patterns.md`, `iztro-i18n/src/lib.rs` (test) | None | i18n suite | Normal |
| 6 | (defer) cleanup: `PatternSourceMetadata.work` typing, features relation rename | F12, F10 | registry, features | Low | existing | Normal |

Order rationale: 1–2 are pure tripwires and make 3–4 safer to review; 3 is the only behavior change and should land before any MCP work; 4 is mechanical; 5–6 are hygiene.

---

## 12. Explicit non-findings (clean areas — skip on next audit unless touched)

- **`core` → `rules`/`analysis`/`projection` dependency direction**: clean; grep of all `core` imports found none. `RuleEvaluationContext` in `core` is ADR-sanctioned transitional, correctly free of rule identity.
- **No rule-identity flags in contexts**: no `is_pattern`/`is_classical` or equivalents anywhere.
- **Natal immutability**: `Chart` has no mutating API; overlays append-only; `EffectiveChartState` read-only with natal-scope + duplicate-layer validation.
- **Temporal palace-name frames**: never conflated with natal — projection carries both identities; a missing frame layout fails loudly; `star_matches_for_scope` blocks flow-star equivalence in Natal/Age frames.
- **Scope truncation for cached layers**: correct and tested (the F1 issue is target coordinates, not scopes).
- **Pattern/classical engine separation**: pattern catalogue entries create no classical rules; `user_facing()` work-filter guarded with a non-vacuous test; `PatternShapeMatched` is a deliberate id-reference, not engine coupling.
- **Source text verbatim discipline**: enforced by `classical_source_inventory.rs` (text equality, ids, ordering, links) + hit-ref compactness tests; `ClassicalRuleHitRef` structurally cannot carry source text.
- **Evaluator dispatch drift**: triple-guarded (handled-ids ↔ corpus ↔ live arms).
- **GUI fact derivation**: none found (import scan + manual read + guard test, modulo F2's scan hole); highlights are GUI-local and never persisted; analysis results never persisted; settings vs charts stores separated as documented.
- **Determinism/ordering**: claims, source hits, detections, panel views all deterministically sorted; `fmt`/`clippy -D warnings`/full workspace tests green.
- **projection/render**: no `rules`/`analysis` imports; render is a pure `ChartStackSnapshot` consumer.
- **i18n boundary**: `iztro` independent of `iztro-i18n`; typed key helpers only; en fallback tested.
