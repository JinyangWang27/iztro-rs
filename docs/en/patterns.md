# Pattern (格局) Detection

`core::pattern` is a **read-only analytical layer** over already-computed chart
facts. It recognizes classical Zi Wei Dou Shu patterns (格局) as structured,
explainable facts and never produces narrative prose.

## Guarantees

- **Read-only**: detection never mutates `Chart`, `Palace`, `StarPlacement`,
  `TemporalLayer`, or `MutagenActivation`. It only inspects them.
- **Structured, not narrative**: a `PatternDetection` carries an id, family,
  polarity, status, strength, scope, anchor, involved palaces/stars/mutagens,
  and machine-checkable `evidence` / conditions. It contains no reading text.
- **Temporal facts stay overlays**: a temporal `PatternScope` never folds
  temporal placement into natal facts. Scope-aware queries read natal
  `Chart` facts for `Scope::Natal` and read `TemporalLayer` placements,
  mutagen activations, and `TemporalPalaceLayout` facts for non-natal scopes.
  An empty `PatternScope::Combined(vec![])` is never permitted by the scope
  guard.
- **Conservative**: a rule emits a detection only when its structural
  conditions are clearly met by modeled chart facts. Rules that depend on
  brightness never emit when a star's brightness is `Unknown`.
- **Source-backed where modeled**: QuanShu Volume 1 catalogues `定富局`,
  `定贵局`, `定贫贱局`, and `定杂局` are tracked as `pattern_rule` source
  inventory. Only structurally clear entries become executable
  `PatternDetection`s; the rest remain source inventory until their conditions
  are modeled.

## Detection flow

`detect_patterns(ctx, request)` runs every registered rule, then filters and
deterministically sorts the results by scope, family, id, anchor, and involved
palaces. `PatternDetectionRequest` controls which scopes, statuses, and families
are returned.

When a detector is requested for a temporal scope, it reads only that scope's
visible overlay facts plus the scope's temporal palace labels. Base-star
conditions for 文昌/文曲/擎羊/陀罗/天马 may match the corresponding runtime
flow-star identity in that same scope (for example 流昌, 月曲, 日羊), and the
detection records the actual matched runtime `StarName`. Temporal 四化 are read
from `MutagenActivation` facts; they are never modeled as fake stars or attached
to natal `StarPlacement`s.

## Status model

A `PatternDetection` is emitted **only when the base pattern formation exists**.
Incomplete or near formations are not detected — there is no `Partial` / 近格
status and no "near-pattern" output. `PatternStatus` therefore always describes
an existing base formation:

- `Fulfilled` (成格): the base structure exists and no modeled weakening or
  breaker condition applies.
- `Weakened` (成而减力): the base structure exists but modeled weakening factors
  reduce its strength.
- `Broken` (破格): the base structure exists but a modeled breaker condition
  invalidates or severely damages it.

`Broken` means a formed structure damaged by a modeled breaker — not a missing
required condition and not structurally impossible source wording. Source entries
whose wording is structurally impossible or otherwise unmodelable stay source
inventory only; they are never emitted as `Broken`.

`PatternDetectionRequest` exposes `include_weakened` / `include_broken` so a
GUI/pattern panel can choose to show damaged-but-formed patterns.

## Rule catalog

| Pattern (格局) | `PatternId` | Family | Polarity | Condition |
| --- | --- | --- | --- | --- |
| 紫府朝垣 | `ZiFuChaoYuan` | `MajorStarCombination` | Auspicious | 紫微 and 天府 both in the Life 三方四正 (weakened by a 煞星 in an involved palace). |
| 机月同梁 | `JiYueTongLiang` | `MajorStarCombination` | Auspicious | 天机/太阴/天同/天梁 all gathered through the Life 三方四正. An incomplete set emits nothing. |
| 羊陀夹忌 | `YangTuoJiaJi` | `ShaJi` | Inauspicious | 擎羊 and 陀罗 clamp (夹) the palace holding 化忌: natal uses a natal star's attached mutagen; temporal scopes use explicit `MutagenActivation`. |
| 左右夹命 | `ZuoYouJiaMing` | `AuxiliaryStarCombination` | Auspicious | 左辅 and 右弼 occupy the two palaces clamping (夹) the Life palace, one on each side. |
| 昌曲夹命 | `ChangQuJiaMing` | `AuxiliaryStarCombination` | Auspicious | 文昌 and 文曲 clamp (夹) the Life palace, one on each side. |
| 日月并明 | `RiYueBingMing` | `MajorStarCombination` | Auspicious | 太阳 and 太阴 are both present and each in a clearly bright state (庙/旺/得/利). |
| 日月反背 | `RiYueFanBei` | `MajorStarCombination` | Inauspicious | 太阳 and 太阴 are both present and each in a clearly dim/fallen state (不/陷). |
| 金灿光辉 | `JinCanGuangHui` | `MajorStarCombination` | Auspicious | Life palace is Wu, 太阳 is there, and 太阳 is the only major star in that palace. |
| 日出扶桑 | `RiChuFuSang` | `MajorStarCombination` | Auspicious | 太阳 is in Mao, and Mao is the Life or Career palace. |
| 月落亥宫 | `YueLuoHaiGong` | `MajorStarCombination` | Auspicious | 太阴 is in Hai, and Hai is the Life palace. |
| 月生沧海 | `YueShengCangHai` | `MajorStarCombination` | Auspicious | 太阴 is in Zi, and Zi is the Property palace. |
| 马头带剑 | `MaTouDaiJian` | `ShaJi` | Mixed | 天马 and 擎羊 share one palace. This does not impose a Wu-only interpretation. |
| 贪火相逢 | `TanHuoXiangFeng` | `ShaJi` | Auspicious | 贪狼 and 火星 share the Life palace, and both have clearly bright states. |
| 武曲守垣 | `WuQuShouYuan` | `MajorStarCombination` | Auspicious | 武曲 is in the Life palace and the Life palace branch is Mao. |
| 财与囚仇 | `CaiYuQiuChou` | `MajorStarCombination` | Inauspicious | 武曲 and 廉贞 share the Life or Body palace. |
| 马落空亡 | `MaLuoKongWang` | `ShaJi` | Inauspicious | 天马 shares a palace with a modeled 空亡-family star (旬空、空亡、截路、截空). |

### QuanShu source-backed catalogues

The end of QuanShu Volume 1 has explicit pattern catalogues:

- `定富局`
- `定贵局`
- `定贫贱局`
- `定杂局`

These sections are source-backed pattern material. Their source entries live in
`crates/iztro/rule-corpus/quan-shu/source/volume-01.toml` with
`category = "pattern_rule"` and `status = "segmented"`. Runtime code does not
parse that inventory.

**A 格局/pattern has exactly one canonical runtime identity: its `PatternId`,
detected by `core::pattern`.** QuanShu Volume 1 pattern catalogue entries are the
*ancient source provenance* for those canonical patterns — they do not create a
second runtime identity:

- `core::pattern` performs the structural detection and emits
  `PatternDetection` facts. This is the only place a pattern is recognized.
- `core::pattern::metadata::pattern_source_metadata(pattern_id)` attaches the
  QuanShu source citation (work, `source_id`, verbatim source text, catalogue
  group) to an implemented `PatternId`, so a GUI or docs layer can display the
  provenance. This is provenance only.
- `rules::classical` does **not** create a parallel source-hit/claim rule for
  each QuanShu pattern catalogue entry, and `evaluate_classical` does not consume
  pattern detections. `rule-corpus/patterns/rules.toml` holds project-owned
  pattern-derived classical rules only (`work = "iztro_pattern_catalog"`,
  `source_id = "pattern.*"`).

Modern textbooks (e.g. Zhongzhou-style) may inform normalized interpretation and
stricter condition design, but they do not create separate pattern identities
either. Unimplemented, referenced, or temporal catalogue entries stay recorded as
source inventory only.

### Clamp (夹) rules

The clamp-based rules (羊陀夹忌, 左右夹命, 昌曲夹命) share the branch-level
`clamp_branches` relation: the two palaces clamping an anchor are its `-1` and
`+1` neighbours. The shared scoped clamp helpers check that both clamp palaces
are occupied — one by each required star or same-scope flow-star equivalent — in
either orientation, and record each clamp as a
`PalaceRelation { relation: ClampedBy }` from the anchor palace to the clamping
palace.

### Brightness rules

日月并明 and 日月反背 read the existing `Brightness` model through the
`query::is_bright` and `query::is_dim` helpers. `is_bright` accepts 庙/旺/得/利
only; `is_dim` accepts 不/陷 only. `平` (Flat) is treated as neutral and
`Unknown` is never bright or dim, so neither rule emits on an uncalculated or
neutral brightness.

## Status

This layer is intentionally narrow and conservative. New patterns are added one
at a time with positive/negative rule tests and source-grounded conditions.
`PatternDetection`s are structured facts only, with the pattern's single
canonical identity (`PatternId`). The classical rule runtime
(`rules::classical`) emits claims for project-owned pattern-derived rules only;
it does not mirror QuanShu pattern catalogue entries as duplicate source-hit/claim
rules. The existing catalogue can evaluate supported overlay layers through core
facts, but this is **not** full classical temporal interpretation. QuanShu
catalogue expansion remains paused; narrative readings, scoring beyond the
coarse `PatternStrength`, and LLM-assisted interpretation remain out of scope
here and belong to later layers.
