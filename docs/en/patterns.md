# Pattern (格局) Detection

`rules::pattern` is the **read-only pattern rule engine** over
already-computed chart facts. It recognizes classical Zi Wei Dou Shu patterns
(格局) as structured, explainable facts and never produces narrative prose.

## Guarantees

- **Read-only**: detection never mutates `Chart`, `Palace`, `StarPlacement`,
  `TemporalLayer`, or `MutagenActivation`. It only inspects them.
- **Structured, not narrative**: a `PatternDetection` carries an id, family,
  polarity, status, strength, scope, anchor, involved palaces/stars/mutagens,
  and machine-checkable `evidence` / conditions. It contains no reading text.
- **Metadata is separated by purpose**: `PatternSourceMetadata` is verified
  source provenance only. `PatternDisplayMetadata` is runtime/display metadata:
  display name, aliases, condition note, source note, and interpretation note.
  Display notes may explain a normalized runtime convention, but they are not
  evidence and they do not create claims.
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

## Metadata convention

When adding or maintaining a pattern, keep three lines separate:

1. **Condition** -> detector logic and structured `PatternEvidence`.
2. **Source** -> verified provenance in `PatternSourceMetadata`, or a display
   source note when the cited line is only an explanatory note and not verified
   provenance for the runtime id.
3. **Claim** -> display/docs only until a rule-engine claim is explicitly
   accepted. `PatternDetection` itself does not carry narrative claims.

In pattern docs and display metadata, `加会` means present in the anchor
palace's `三方四正`: the anchor palace, opposite palace, and two trine palaces.

`RiChuFuSang` remains the stable public `PatternId` for compatibility with the
source inventory. Its runtime display name is `日照雷门`, with `日出扶桑格` as a
display alias. The verified QuanShu source provenance remains source-facing as
`日出扶桑 日在卯守命是也，守官禄宫亦然`.

## Detection flow

`detect_patterns(ctx, request)` runs every registered rule, then filters and
deterministically sorts the results by scope, family, id, anchor, and involved
palaces. `PatternDetectionRequest` controls which scopes, statuses, and families
are returned.

When a detector is requested for a temporal scope, it reads that scope's
visible facts (selected-state detectors also see natal facts projected into the
temporal palace frame) plus the scope's temporal palace labels.

**Star matching is exact by default.** An exact `StarName` condition matches only
the exact runtime `StarName`. Flow stars such as 流曲 (`LiuQu`) are *independent*
`StarName` identities from their base stars such as 文曲 (`WenQu`): a base-star
condition never silently matches a flow star, and a selected temporal frame that
makes 流曲 visible does **not** make it satisfy a 文曲 condition. `StarFamily`
records taxonomy/lineage (文曲 and 流曲 share `StarFamily::Qu`), **not** equality;
family-level matching is used only by detectors that explicitly ask for it via
`StarSelector::Family`. A detector that intentionally wants the flow blade of a
given layer (for example 羊陀夹忌 matching 流羊/流陀 in a yearly scope) resolves the
exact per-scope identity explicitly through `StarFamily::exact_member_for_scope`, and
the detection records the actual matched runtime `StarName`. Temporal 四化 are
read from `MutagenActivation` facts; they are never modeled as fake stars or
attached to natal `StarPlacement`s.

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
| 日照雷门 | `RiChuFuSang` | `MajorStarCombination` | Auspicious | Natal-only: birth time is Mao through Wei, Life palace is Mao, 太阳 and 天梁 both occupy Mao Life, and Life 三方四正 has explicit 禄存／左右／曲昌／魁钺 or 禄/权/科 support. Public id retained from `RiChuFuSang`; display alias `日出扶桑格`. Display source note: `日出扶桑 日在卯守命是也，守官禄宫亦然（紫微斗数全书）`. |
| 月落亥宫 | `YueLuoHaiGong` | `MajorStarCombination` | Auspicious | 太阴 is in Hai, and Hai is the Life palace. |
| 月生沧海 | `YueShengCangHai` | `MajorStarCombination` | Auspicious | 太阴 is in Zi, and Zi is the Property palace. |
| 马头带剑 | `MaTouDaiJian` | `ShaJi` | Neutral | 天马 and 擎羊 share one palace. This does not impose a Wu-only interpretation. |
| 贪火相逢 | `TanHuoXiangFeng` | `ShaJi` | Auspicious | 贪狼 and 火星 share the Life palace, and both have clearly bright states. |
| 武曲守垣 | `WuQuShouYuan` | `MajorStarCombination` | Auspicious | 武曲 is in the Life palace and the Life palace branch is Mao. |
| 财与囚仇 | `CaiYuQiuChou` | `MajorStarCombination` | Inauspicious | 武曲 and 廉贞 share the Life or Body palace. |
| 马落空亡 | `MaLuoKongWang` | `ShaJi` | Inauspicious | 天马 shares a palace with a modeled 空亡-family star (旬空、空亡、截路、截空). |
| 命里逢空 | `MingLiFengKong` | `ShaJi` | Inauspicious | 地空 (DiKong) and/or 地劫 (DiJie) occupy the Life palace. The modeled 空亡-family stars (旬空/空亡/截路/截空) are **not** this pattern. |
| 禄逢冲破 | `LuFengChongPo` | `ShaJi` | Inauspicious | 禄存 or 化禄 sits in the Life palace itself, and that 禄 base is broken (冲破) by 地空 or 地劫 in the Life 三方四正. Status is `Broken`. |
| 文星拱命 | `WenXingGongMing` | `AuxiliaryStarCombination` | Auspicious | 文昌 and 文曲 both appear in Life 三方四正. |
| 天机巳亥 | `TianJiSiHai` | `MajorStarCombination` | Inauspicious | Life palace branch is Si or Hai, and 天机 occupies the Life palace itself (not merely the Life 三方四正). |
| 左右同宫 | `ZuoYouTongGong` | `AuxiliaryStarCombination` | Auspicious | Natal-only: the Life or Body palace branch is Chou or Wei, 左辅 and 右弼 share that anchor palace, and there is additional 禄存／左右／曲昌／魁钺 or 禄/权/科 support in the anchor 三方四正 beyond the base 左右 pair (`更于吉星`). |
| 明珠出海 | `MingZhuChuHai` | `MajorStarCombination` | Auspicious | Life palace is Wei with no major star, 太阳 and 天梁 both sit in Mao, 太阴 sits bright in Hai, and the Life 三方四正 carries explicit 禄存／左右／曲昌／魁钺 or 禄/权/科 support. May coexist with 命无正曜. Display source note: `三合明珠生旺地稳步蟾宫（斗数骨髓赋）`. |
| 命无正曜 | `MingWuZhengYao` | `MajorStarCombination` | Neutral | Life palace has no major star. |
| 极向离明 | `JiXiangLiMing` | `MajorStarCombination` | Auspicious | Life is Wu and 紫微 is in Life. Fulfilled when Life 三方四正 has no tough star; broken when a tough star appears. |
| 府相朝垣 | `FuXiangChaoYuan` | `MajorStarCombination` | Auspicious | Either 天府 and 天相 occupy the Wealth and Career palaces (one in each), or 天府 sits in the Life palace with 天相 in the Life 三方四正; additionally the Life 三方四正 carries explicit 禄存／左右／曲昌／魁钺 or 禄/权/科 support. Display source note: `府相朝垣 见前批注（紫微斗数全书）`. |
| 石中隐玉 | `ShiZhongYinYu` | `MajorStarCombination` | Auspicious | Selected Life palace branch is Zi or Wu, 巨门 sits in Life, and the Life 三方四正 carries explicit 禄存／左右／曲昌／魁钺 or 禄/权/科 support. Source: `子午巨门石中隐玉，明禄暗禄锦上添花` (斗数骨髓赋). |
| 紫府夹命 | `ZiFuJiaMing` | `MajorStarCombination` | Auspicious | 紫微 and 天府 occupy the two palaces clamping (夹) the selected Life palace, in either orientation. No extra support is required. Source: `紫府夹命为贵格` (卷三·论诸星同垣). |
| 贞杀同宫 | `LianZhenQiShaTongGong` | `MajorStarCombination` | Neutral | Selected Life palace branch is Chou or Wei and 廉贞 and 七杀 both occupy it. Alias `廉贞七杀同宫`. Recognises the 廉贞七杀同守丑未命宫 structure only; the source distinguishes 庙旺 from 陷地化忌 cases, but this detector does not infer later modern claims. Source: `廉贞七杀居庙旺反为积富之人 杀居午奇格，若陷地化忌，贫贱残疾` (卷三·论诸星同垣). |
| 天乙拱命 | `TianYiGongMing` | `AuxiliaryStarCombination` | Auspicious | 天魁 in the selected Life palace with 天钺 in its opposite (迁移) palace, or the reverse — the Life/opposite axis only. Runtime display name 天乙拱命; source-facing name / alias 坐贵向贵. Source: `坐贵向贵 谓魁钺在命迭相坐拱是也` (定贵局). |
| 擎羊入庙 | `QingYangRuMiao` | `ShaJi` | Auspicious | Selected Life palace branch is Chen/Xu/Chou/Wei, 擎羊 sits in Life, and the Life 三方四正 carries explicit 禄存／左右／曲昌／魁钺 or 禄/权/科 support. The support is constitutive (`辰戍丑未守命遇吉是也` / `加吉万论`): no support means no detection. Runtime display name 擎羊入庙; source-facing name / alias 羊刃入庙. Source: `羊刃入庙 辰戍丑未守命遇吉是也` (定贵局). |

### Reserved ids without detectors

`LingChangTuoWu` (铃昌陀武) is retained as a stable public `PatternId` and
registry entry for future implementation. It currently has no registered detector
and therefore never appears in `detect_patterns` output.

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
detected by `rules::pattern`.** QuanShu Volume 1 pattern catalogue entries are the
*ancient source provenance* for those canonical patterns — they do not create a
second runtime identity:

- `rules::pattern` performs the structural detection and emits
  `PatternDetection` facts. This is the only place a pattern is recognized.
- `rules::pattern::metadata::pattern_source_metadata(pattern_id)` attaches the
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
are occupied — one by each required exact star identity — in either orientation,
and record each clamp as a `PalaceRelation { relation: ClampedBy }` from the
anchor palace to the clamping palace. Matching is exact: 昌曲夹命 requires exact
文昌 + exact 文曲 and is not satisfied by 流昌/流曲. 羊陀夹忌 keeps its temporal
semantics by querying the exact per-scope blades (擎羊/陀罗 natally, 流羊/流陀 in a
yearly scope, …) explicitly rather than relying on any base↔flow equivalence.

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

## Developer checklist

When adding one pattern:

- [ ] Add the `PatternId` variant and update `PatternId::ALL` plus its exhaustive
  test.
- [ ] Add `PatternDisplayMetadata` with display name, aliases, and notes; add
  `PatternSourceMetadata` only for verified source provenance.
- [ ] Add a focused detector that populates `involved_palaces`,
  `involved_stars`, `involved_mutagens`, and structured evidence.
- [ ] Add positive and negative integration tests, including status/evidence
  assertions for weakened or broken cases.
- [ ] Update English and Chinese pattern docs when the public catalog changes.
