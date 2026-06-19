# 格局检测（Pattern Detection）

`core::pattern` 是构建在既有命盘事实之上的**只读分析层**。它把传统紫微斗数格局识别为
结构化、可解释的事实，并且不产生任何叙述性文字。

## 保证

- **只读**：检测过程绝不修改 `Chart`、`Palace`、`StarPlacement`、`TemporalLayer`
  或 `MutagenActivation`，只读取它们。
- **结构化、非叙述**：一个 `PatternDetection` 携带 id、family、polarity、status、
  strength、scope、anchor，以及涉及的宫位／星曜／四化和可机器校验的 `evidence`／
  条件，不包含任何解读文本。
- **时间事实保持为叠加层**：时间性 `PatternScope` 绝不把流曜安置折叠进本命事实。
  空的 `PatternScope::Combined(vec![])` 永远不会通过 scope 守卫。
- **保守**：只有当结构条件被已建模的命盘事实清晰满足时，规则才会产生检测。依赖亮度
  的规则在星曜亮度为 `Unknown` 时绝不产出。

## 检测流程

`detect_patterns(ctx, request)` 运行每条已注册规则，然后按 scope、family、id、
anchor、涉及宫位对结果进行过滤与确定性排序。`PatternDetectionRequest` 控制返回哪些
scope、status 与 family。

## 规则目录

| 格局 | `PatternId` | Family | 吉凶 | 条件 |
| --- | --- | --- | --- | --- |
| 紫府朝垣 | `ZiFuChaoYuan` | `MajorStarCombination` | 吉 | 紫微与天府同在命宫三方四正（若涉及宫位有煞星则减力）。 |
| 机月同梁 | `JiYueTongLiang` | `MajorStarCombination` | 吉凶参半 | 天机／太阴／天同／天梁会于命宫三方四正（`include_partial` 下支持近格）。 |
| 羊陀夹忌 | `YangTuoJiaJi` | `ShaJi` | 凶 | 擎羊与陀罗夹住承载本命化忌之星的宫位。 |
| 左右夹命 | `ZuoYouJiaMing` | `AuxiliaryStarCombination` | 吉 | 左辅与右弼分居命宫两侧夹宫，各占一边。 |
| 昌曲夹命 | `ChangQuJiaMing` | `AuxiliaryStarCombination` | 吉 | 文昌与文曲夹住命宫，各占一边。 |
| 日月并明 | `RiYueBingMing` | `MajorStarCombination` | 吉 | 太阳与太阴皆在盘，且各自处于明亮庙旺之位（庙／旺／得／利）。 |
| 日月反背 | `RiYueFanBei` | `MajorStarCombination` | 凶 | 太阳与太阴皆在盘，且各自处于失辉落陷之位（不／陷）。 |

### 夹宫规则

夹宫类规则（羊陀夹忌、左右夹命、昌曲夹命）共用宫位级的 `clamp_branches` 关系：夹住某
锚点的两个宫位是其 `-1` 与 `+1` 邻宫。共享的 `query::clamp_pair_matches` 辅助函数检查
两个夹宫是否各被一颗所需星曜占据（接受任一朝向），并以从锚点宫到夹宫的
`PalaceRelation { relation: ClampedBy }` 记录每次夹宫。

### 亮度规则

日月并明与日月反背通过 `query::is_bright` 与 `query::is_dim` 辅助函数读取既有的
`Brightness` 模型。`is_bright` 只接受 庙／旺／得／利；`is_dim` 只接受 不／陷。`平`
（Flat）视为中性，`Unknown` 既不算亮也不算暗，因此两条规则都不会在未计算或中性亮度上
产出。

## 现状

本层刻意保持狭窄且保守。新格局逐条加入，并配有正例／负例规则测试以及有出处依据的条件。
叙述性解读、超出粗粒度 `PatternStrength` 的评分，以及 LLM 辅助解读都不在本层范围内，
属于后续层级。
