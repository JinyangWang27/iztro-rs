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
  temporal placement into natal facts. An empty `PatternScope::Combined(vec![])`
  is never permitted by the scope guard.
- **Conservative**: a rule emits a detection only when its structural
  conditions are clearly met by modeled chart facts. Rules that depend on
  brightness never emit when a star's brightness is `Unknown`.

## Detection flow

`detect_patterns(ctx, request)` runs every registered rule, then filters and
deterministically sorts the results by scope, family, id, anchor, and involved
palaces. `PatternDetectionRequest` controls which scopes, statuses, and families
are returned.

## Rule catalog

| Pattern (格局) | `PatternId` | Family | Polarity | Condition |
| --- | --- | --- | --- | --- |
| 紫府朝垣 | `ZiFuChaoYuan` | `MajorStarCombination` | Auspicious | 紫微 and 天府 both in the Life 三方四正 (weakened by a 煞星 in an involved palace). |
| 机月同梁 | `JiYueTongLiang` | `MajorStarCombination` | Mixed | 天机/太阴/天同/天梁 gathered through the Life 三方四正 (partial support behind `include_partial`). |
| 羊陀夹忌 | `YangTuoJiaJi` | `ShaJi` | Inauspicious | 擎羊 and 陀罗 clamp (夹) the palace holding a natal 化忌 star. |
| 左右夹命 | `ZuoYouJiaMing` | `AuxiliaryStarCombination` | Auspicious | 左辅 and 右弼 occupy the two palaces clamping (夹) the Life palace, one on each side. |
| 昌曲夹命 | `ChangQuJiaMing` | `AuxiliaryStarCombination` | Auspicious | 文昌 and 文曲 clamp (夹) the Life palace, one on each side. |
| 日月并明 | `RiYueBingMing` | `MajorStarCombination` | Auspicious | 太阳 and 太阴 are both present and each in a clearly bright state (庙/旺/得/利). |
| 日月反背 | `RiYueFanBei` | `MajorStarCombination` | Inauspicious | 太阳 and 太阴 are both present and each in a clearly dim/fallen state (不/陷). |

### Clamp (夹) rules

The clamp-based rules (羊陀夹忌, 左右夹命, 昌曲夹命) share the branch-level
`clamp_branches` relation: the two palaces clamping an anchor are its `-1` and
`+1` neighbours. The shared `query::clamp_pair_matches` helper checks that both
clamp palaces are occupied — one by each required star — in either orientation,
and records each clamp as a `PalaceRelation { relation: ClampedBy }` from the
anchor palace to the clamping palace.

### Brightness rules

日月并明 and 日月反背 read the existing `Brightness` model through the
`query::is_bright` and `query::is_dim` helpers. `is_bright` accepts 庙/旺/得/利
only; `is_dim` accepts 不/陷 only. `平` (Flat) is treated as neutral and
`Unknown` is never bright or dim, so neither rule emits on an uncalculated or
neutral brightness.

## Status

This layer is intentionally narrow and conservative. New patterns are added one
at a time with positive/negative rule tests and source-grounded conditions.
Narrative readings, scoring beyond the coarse `PatternStrength`, and LLM-assisted
interpretation remain out of scope here and belong to later layers.
