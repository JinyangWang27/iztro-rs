# 格局检测（Pattern Detection）

`rules::pattern` 是构建在既有命盘事实之上的**只读格局规则引擎**。它把传统紫微斗数格局识别为
结构化、可解释的事实，并且不产生任何叙述性文字。

## 保证

- **只读**：检测过程绝不修改 `Chart`、`Palace`、`StarPlacement`、`TemporalLayer`
  或 `MutagenActivation`，只读取它们。
- **结构化、非叙述**：一个 `PatternDetection` 携带 id、family、polarity、status、
  strength、scope、anchor，以及涉及的宫位／星曜／四化和可机器校验的 `evidence`／
  条件，不包含任何解读文本。
- **元数据按用途分离**：`PatternSourceMetadata` 只放已核验的出处来源。
  `PatternDisplayMetadata` 是运行时／展示元数据：显示名、别名、条件说明、出处说明与
  解读说明。展示说明可以解释归一化口径，但不是证据，也不会创造判断。
- **时间事实保持为叠加层**：时间性 `PatternScope` 绝不把流曜安置折叠进本命事实。
  scope-aware 查询在 `Scope::Natal` 读取本命 `Chart` 事实，在非本命 scope 读取
  `TemporalLayer` 的星曜落点、四化激活与 `TemporalPalaceLayout` 事实。空的
  `PatternScope::Combined(vec![])` 永远不会通过 scope 守卫。
- **保守**：只有当结构条件被已建模的命盘事实清晰满足时，规则才会产生检测。依赖亮度
  的规则在星曜亮度为 `Unknown` 时绝不产出。
- **有出处则显式记录**：《紫微斗数全书》卷一末尾的 `定富局`、`定贵局`、`定贫贱局`、
  `定杂局` 已作为 `pattern_rule` source inventory 登记。只有结构条件清晰且已建模的条目
  会成为可执行 `PatternDetection`；其余条目先保留为出处清单。

## 元数据约定

新增或维护格局时，三条线必须分开：

1. **条件** -> 检测器逻辑与结构化 `PatternEvidence`。
2. **出处** -> 已核验出处放入 `PatternSourceMetadata`；若只是解释运行时口径、而不是
   runtime id 的已核验出处，则放入展示用 source note。
3. **判断** -> 只放在展示／文档中；除非 rule-engine claim 被明确接受，否则不进入
   `PatternDetection`。

在格局文档和展示元数据中，`加会` 指出现在锚点宫位的 `三方四正`：本宫、对宫与两组三合宫。

`RiChuFuSang` 保留为稳定公开 `PatternId`，以兼容既有出处清单。运行时显示名为
`日照雷门`，展示别名为 `日出扶桑格`。已核验的全书出处仍按 source-facing 名称保留为
`日出扶桑 日在卯守命是也，守官禄宫亦然`。

## 检测流程

`detect_patterns(ctx, request)` 运行每条已注册规则，然后按 scope、family、id、
anchor、涉及宫位对结果进行过滤与确定性排序。`PatternDetectionRequest` 控制返回哪些
scope、status 与 family。

当检测器被请求到某个时间 scope 时，它只读取该 scope 可见的叠加事实与时间宫名。
文昌／文曲／擎羊／陀罗／天马等基础星曜条件，可以在同一 scope 内匹配对应的 runtime
流曜身份（例如流昌、月曲、日羊）；检测结果会记录实际命中的 runtime `StarName`。
时间四化只读取 `MutagenActivation` 事实，不会伪装成星曜，也不会写回本命
`StarPlacement`。

## 状态模型

只有当**基础格局结构成立**时才会产出 `PatternDetection`。条件不足或近似的格局不会被
检测——不存在 `Partial`／近格 状态，也没有「近格」输出。因此 `PatternStatus` 始终描述
一个已成立的基础结构：

- `Fulfilled`（成格）：基础结构成立，且无已建模的减力或破格条件。
- `Weakened`（成而减力）：基础结构成立，但有已建模的减力因素削弱其力量。
- `Broken`（破格）：基础结构成立，但有已建模的破格条件使其失效或严重受损。

`Broken` 指已成形结构被已建模破格条件破坏——既不表示缺少必要条件，也不表示原文措辞结构
上不可能。原文措辞结构上不可能或无法建模的条目，只保留为 source inventory，永不作为
`Broken` 产出。

`PatternDetectionRequest` 通过 `include_weakened`／`include_broken` 让 GUI／格局面板可
选择是否展示「已成形但受损」的格局。

## 规则目录

| 格局 | `PatternId` | Family | 吉凶 | 条件 |
| --- | --- | --- | --- | --- |
| 紫府朝垣 | `ZiFuChaoYuan` | `MajorStarCombination` | 吉 | 紫微与天府同在命宫三方四正（若涉及宫位有煞星则减力）。 |
| 机月同梁 | `JiYueTongLiang` | `MajorStarCombination` | 吉 | 天机／太阴／天同／天梁四星齐会于命宫三方四正；不齐则不产出。 |
| 羊陀夹忌 | `YangTuoJiaJi` | `ShaJi` | 凶 | 擎羊与陀罗夹住承载化忌的宫位；本命读取星曜自身四化，时间 scope 读取显式 `MutagenActivation`。 |
| 左右夹命 | `ZuoYouJiaMing` | `AuxiliaryStarCombination` | 吉 | 左辅与右弼分居命宫两侧夹宫，各占一边。 |
| 昌曲夹命 | `ChangQuJiaMing` | `AuxiliaryStarCombination` | 吉 | 文昌与文曲夹住命宫，各占一边。 |
| 日月并明 | `RiYueBingMing` | `MajorStarCombination` | 吉 | 太阳与太阴皆在盘，且各自处于明亮庙旺之位（庙／旺／得／利）。 |
| 日月反背 | `RiYueFanBei` | `MajorStarCombination` | 凶 | 太阳与太阴皆在盘，且各自处于失辉落陷之位（不／陷）。 |
| 金灿光辉 | `JinCanGuangHui` | `MajorStarCombination` | 吉 | 命宫在午，太阳在命宫，且太阳是该宫唯一主星。 |
| 日照雷门 | `RiChuFuSang` | `MajorStarCombination` | 吉 | 仅本命：出生时辰为卯至未，命宫在卯，太阳与天梁同在卯宫命宫，且命宫三方四正有禄存／左右／曲昌／魁钺或禄／权／科加会。公开 id 继续使用 `RiChuFuSang`；展示别名为 `日出扶桑格`。展示出处说明：`日出扶桑 日在卯守命是也，守官禄宫亦然（紫微斗数全书）`。 |
| 月落亥宫 | `YueLuoHaiGong` | `MajorStarCombination` | 吉 | 太阴在亥，且亥宫是命宫。 |
| 月生沧海 | `YueShengCangHai` | `MajorStarCombination` | 吉 | 太阴在子，且子宫是田宅宫。 |
| 马头带剑 | `MaTouDaiJian` | `ShaJi` | 平 | 天马与擎羊同宫；不限定为午宫格。 |
| 贪火相逢 | `TanHuoXiangFeng` | `ShaJi` | 吉 | 贪狼与火星同守命宫，且二者皆为已建模的明亮状态。 |
| 武曲守垣 | `WuQuShouYuan` | `MajorStarCombination` | 吉 | 武曲在命宫，且命宫地支为卯。 |
| 财与囚仇 | `CaiYuQiuChou` | `MajorStarCombination` | 凶 | 武曲与廉贞同宫，且该宫为命宫或身宫。 |
| 马落空亡 | `MaLuoKongWang` | `ShaJi` | 凶 | 天马与已建模空亡族星（旬空、空亡、截路、截空）同宫。 |
| 命里逢空 | `MingLiFengKong` | `ShaJi` | 凶 | 地空（DiKong）与／或地劫（DiJie）守命。已建模空亡族星（旬空/空亡/截路/截空）**不**属本格。 |
| 禄逢冲破 | `LuFengChongPo` | `ShaJi` | 凶 | 禄存或化禄坐命（命宫本宫），且该禄被命宫三方四正中的地空或地劫冲破。状态为 `Broken`。 |
| 文星拱命 | `WenXingGongMing` | `AuxiliaryStarCombination` | 吉 | 文昌与文曲皆在命宫三方四正。 |
| 天机巳亥 | `TianJiSiHai` | `MajorStarCombination` | 凶 | 命宫地支为巳或亥，且天机坐守命宫本宫（而非仅在命宫三方四正）。 |
| 左右同宫 | `ZuoYouTongGong` | `AuxiliaryStarCombination` | 吉 | 仅本命：命宫或身宫地支为丑或未，左辅与右弼同在该锚点宫，且锚点三方四正另有禄存／左右／曲昌／魁钺或禄／权／科加会（更于吉星，超出左右同宫本身）。 |
| 明珠出海 | `MingZhuChuHai` | `MajorStarCombination` | 吉 | 命宫在未且无主星，太阳与天梁同在卯，太阴入庙旺于亥，命宫三方四正有禄存／左右／曲昌／魁钺或禄／权／科加会。可与命无正曜并存。展示出处说明：`三合明珠生旺地稳步蟾宫（斗数骨髓赋）`。 |
| 命无正曜 | `MingWuZhengYao` | `MajorStarCombination` | 平 | 命宫无主星。 |
| 极向离明 | `JiXiangLiMing` | `MajorStarCombination` | 吉 | 命宫在午且紫微在命宫；命宫三方四正无煞星则成格，有煞星则以破格产出。 |
| 府相朝垣 | `FuXiangChaoYuan` | `MajorStarCombination` | 吉 | 天府与天相分居财帛宫与官禄宫（各占一宫），或天府坐命且天相在命宫三方四正；另需命宫三方四正有禄存／左右／曲昌／魁钺或禄／权／科加会。展示出处说明：`府相朝垣 见前批注（紫微斗数全书）`。 |
| 石中隐玉 | `ShiZhongYinYu` | `MajorStarCombination` | 吉 | 命宫在子或午，巨门坐命，命宫三方四正有禄存／左右／曲昌／魁钺或禄／权／科加会。出处：`子午巨门石中隐玉，明禄暗禄锦上添花`（斗数骨髓赋）。 |
| 紫府夹命 | `ZiFuJiaMing` | `MajorStarCombination` | 吉 | 紫微与天府分居命宫两侧夹宫，任一方向皆可；本实现不额外要求加会。出处：`紫府夹命为贵格`（卷三·论诸星同垣）。 |
| 贞杀同宫 | `LianZhenQiShaTongGong` | `MajorStarCombination` | 平 | 命宫在丑或未，廉贞与七杀同守命宫。别名 `廉贞七杀同宫`。当前只识别 廉贞七杀同守丑未命宫 结构；出处同时讨论庙旺与陷地化忌之别，本检测器不展开法律纠纷等现代断语。出处：`廉贞七杀居庙旺反为积富之人 杀居午奇格，若陷地化忌，贫贱残疾`（卷三·论诸星同垣）。 |
| 天乙拱命 | `TianYiGongMing` | `AuxiliaryStarCombination` | 吉 | 天魁、天钺一在命宫、一在迁移宫相对拱照（仅命宫／对宫轴线）。公开运行时显示名为天乙拱命，出处名／别名为坐贵向贵。出处：`坐贵向贵 谓魁钺在命迭相坐拱是也`（定贵局）。 |
| 擎羊入庙 | `QingYangRuMiao` | `ShaJi` | 吉 | 命宫在辰戌丑未，擎羊守命，且命宫三方四正有禄存／左右／曲昌／魁钺或禄／权／科加会。加会为成格必要条件（`辰戍丑未守命遇吉是也` /`加吉万论`），无吉 support 不产出。公开运行时显示名为擎羊入庙，出处名／别名为羊刃入庙。出处：`羊刃入庙 辰戍丑未守命遇吉是也`（定贵局）。 |

### 保留但未检测的 id

`LingChangTuoWu`（铃昌陀武）目前作为稳定公开 `PatternId` 与 registry 条目保留，供后续
实现使用。当前未注册 detector，因此不会出现在 `detect_patterns` 的运行时输出中。

### 全书出处格局目录

《紫微斗数全书》卷一末尾包含四组显式格局目录：

- `定富局`
- `定贵局`
- `定贫贱局`
- `定杂局`

这些章节属于有出处依据的格局材料。其出处条目记录在
`crates/iztro/rule-corpus/quan-shu/source/volume-01.toml`，使用
`category = "pattern_rule"` 与 `status = "segmented"`。运行时代码不解析该 inventory。

**一个格局只有唯一的规范运行时身份：其 `PatternId`，由 `rules::pattern` 检测。**《紫微斗数全书》
卷一格局目录条目是这些规范格局的**古籍出处来源（source provenance）**，不会创造第二个运行时身份：

- `rules::pattern` 负责结构检测，产出 `PatternDetection` 事实。这是唯一识别格局之处。
- `rules::pattern::metadata::pattern_source_metadata(pattern_id)` 把全书出处引用（work、
  `source_id`、原文逐字文本、目录分组）挂到已实现的 `PatternId` 上，供 GUI 或文档展示出处。
  这仅为出处来源，不代表存在独立的 classical 运行时规则。
- `rules::classical` **不**为每个全书格局目录条目创建平行的 source-hit/claim 规则，
  `evaluate_classical` 也不消费 pattern 检测。`rule-corpus/patterns/rules.toml` 只放项目自有的
  pattern 派生 classical 规则（`work = "iztro_pattern_catalog"`、`source_id = "pattern.*"`）。

现代教材（如中州派一脉）可为归一化解读与更严格的条件设计提供参考，但同样不创造独立的格局身份。
未实现、只写「见前批注」或依赖限运的条目继续只保留在 source inventory。

### 夹宫规则

夹宫类规则（羊陀夹忌、左右夹命、昌曲夹命）共用宫位级的 `clamp_branches` 关系：夹住某
锚点的两个宫位是其 `-1` 与 `+1` 邻宫。共享的 scoped 夹宫辅助函数检查两个夹宫是否
各被一颗所需星曜或同 scope 的流曜等价星占据（接受任一朝向），并以从锚点宫到夹宫的
`PalaceRelation { relation: ClampedBy }` 记录每次夹宫。

### 亮度规则

日月并明与日月反背通过 `query::is_bright` 与 `query::is_dim` 辅助函数读取既有的
`Brightness` 模型。`is_bright` 只接受 庙／旺／得／利；`is_dim` 只接受 不／陷。`平`
（Flat）视为中性，`Unknown` 既不算亮也不算暗，因此两条规则都不会在未计算或中性亮度上
产出。

## 现状

本层刻意保持狭窄且保守。新格局逐条加入，并配有正例／负例规则测试以及有出处依据的条件。
`PatternDetection` 只产出结构化事实，且格局具有唯一规范身份（`PatternId`）。classical
runtime（`rules::classical`）只为项目自有的 pattern 派生规则产出 claim，不把全书格局目录条目
镜像成重复的 source-hit/claim 规则。现有目录可以通过 core 事实评估已支持的时间叠加层，
但这**不是**完整的古法限运解读。全书格局扩展仍保持暂停；叙述性解读、超出粗粒度
`PatternStrength` 的评分，以及 LLM 辅助解读都不在本层范围内，属于后续层级。

## 开发者清单

新增一个格局时：

- [ ] 新增 `PatternId` 变体，并更新 `PatternId::ALL` 与穷尽性测试。
- [ ] 新增 `PatternDisplayMetadata`（显示名、别名、说明）；只有已核验出处才新增
  `PatternSourceMetadata`。
- [ ] 新增聚焦检测器，并填充 `involved_palaces`、`involved_stars`、
  `involved_mutagens` 与结构化 evidence。
- [ ] 新增正例和反例集成测试；若有减力或破格，断言 status 与 evidence。
- [ ] 公开目录变化时同步更新中英文格局文档。
