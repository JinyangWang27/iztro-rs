# 架构

`iztro-rs` 采用分层架构。每一层都有清晰职责，并尽量避免把相邻层的职责混在一起。

当前实现把命盘事实、renderer-friendly read model、渲染、本地化、应用前端、特征提取、规则与叙事分开。这个边界是刻意设计的：排盘应保持确定性并由 fixtures 校验，GUI、TUI、MCP 和未来 3D 视图应消费 read model，而不是重新推导排盘布局。

## 1. Core Chart Layer

Core Chart Layer 保存确定性的排盘事实。它不包含解盘文章、报告排版、CLI 输出格式、GUI 假设、renderer geometry 或运行时语言选择。

示例内容：

- 出生上下文和历法配置；
- 由 `lunar-lite` 拥有并被直接使用的底层天干、地支、干支与四柱值对象；`lunar-lite` 历法转换置于 `core/calendar` 适配器之后；
- `core` 自己拥有的纳音与五行局逻辑；
- 十二宫；
- 有类型本命星曜；
- 无类型装饰性 runtime 星曜事实；
- 亮度；
- 本命四化；
- 仅模型的 horoscope overlays；
- scoped temporal star placements；
- temporal mutagen activations；
- method profile 与 chart-plane metadata。

这一层的输出是结构化星盘对象，通常是 `Chart`，或仅模型的叠加层 wrapper，例如 `HoroscopeChart`。

### 本命事实与时间叠加层

本命星盘事实一旦构建即不可变。`Chart::stars()` 只返回有类型本命 `StarPlacement`；`Palace::decorative_stars()` 保存长生/博士/岁前/将前十二神等无类型装饰性 runtime facts。这些 surface 必须保持分离。

时间事实是 additive overlays。`HoroscopeChart` 包裹一个不可变本命 `Chart`，并持有零个或多个 `TemporalLayer`。每个时间层记录自己的 `Scope`、`TemporalContext`、按地支标记的 scoped star placements、scoped decorative facts 与 `MutagenActivation`。时间层不能复制本命落点，也不能修改本命宫名。四化激活保持 activation facts，而不是伪装成星曜。

## 2. Snapshot / Read Model Layer

Snapshot Layer 把 core 命盘事实转换成 renderer-neutral read models。

主要 read model 是 `ChartStackSnapshot`：

```text
Chart / HoroscopeChart
  -> ChartStackSnapshot
     -> ChartLayerSnapshot[]
        -> PalaceLayerCellSnapshot[]
```

它把传统十二宫格表示为 x/y 坐标，并把本命/时间层表示为 stack layers。这样未来 renderer 可以用同一份数据展示为：

- 纯文本列表；
- 终端 UI 宫格；
- 2D 宫位图；
- 文墨天机风格交互盘；
- 未来 3D stacked view，其中 z 轴表示 本命 / 大限 / 流年 / 流月 / 流日 / 流时。

Renderer 应消费 `ChartStackSnapshot`，而不是直接遍历 `Chart`。GUI 应通过选择不同时间视图并重新生成 snapshot 来更新显示，而不是修改本命盘。

### Facade snapshots 与 GUI view models

`HoroscopeFacadeSnapshot` 及相关 facade DTO 是兼容性/导出 payload。它们应保留稳定的 machine-readable fields、确定性顺序，以及为兼容和可读性提供的附加中文标签，但不应变成 UI layout code 或 runtime localization infrastructure。

文墨天机风格静态盘由专门面向 GUI 的 read model `StaticChartViewSnapshot` 支撑。它由既有 chart/facade facts 派生，支持本命-only 或选定 temporal overlays，并包含：

- 每宫的传统 4x4 宫格位置；
- 地支、天干、宫名、星曜、亮度、四化、装饰星家族、星曜类别和 scope 的 display-ready labels；
- 以 `StarCategory` 分组的星曜列表；
- 本命/大限/小限/流年/流月/流日/流时 scope selector 状态；
- 当前视图选定的本命/时间叠加层，且与本命事实分离；
- 预留的 `HighlightView` annotations，供未来 feature/rule layers 填充。

该 view model 保持 renderer-neutral。它可以描述某宫或某星需要高亮，但不选择 CSS class、颜色、canvas 坐标、相机位置、动画或 3D geometry。

桌面 GUI 通过 `crates/iztro-i18n` 把当前 surface 渲染为英文或简体中文；本地化字符串不成为内部模型 identity。

## 3. Runtime Localization Layer

运行时本地化是 presentation boundary，不是排盘职责。

`crates/iztro-i18n` 负责：

- 当前 runtime locales：`en-US` 与 `zh-Hans`；
- Fluent 资源加载与格式化；
- fallback 行为，以 `en-US` 为默认 fallback；
- `star_name`、`palace_name`、`mutagen`、`temporal_label` 等 typed helpers；
- 从领域 enum/value object 到本地化标签的稳定 key mapping。

Core domain types 必须保持稳定 enum / value object，不能变成本地化字符串。GUI 可以调用 `i18n.star_name(star_name)`，但排盘和兼容性测试仍应断言 `StarName`、`EarthlyBranch`、`MutagenActivation` 等 typed facts。

Facade/export DTO 可以保留附加 zh-CN 标签以提高兼容性和可读性，但它们不是通用 runtime i18n 机制。

## 4. Render Layer

Render Layer 把 snapshot/read-model 数据转换为面向人的展示格式。

第一个具体 renderer 是 `render` crate 的纯文本 chart-stack renderer。它消费 `ChartStackSnapshot`，用于 demo 和调试。它不生成命盘事实、不推导时间层、不本地化术语、不评估规则，也不做解读。

未来 renderer 可以包括 CLI、TUI、web、GUI、SVG/HTML 或 3D view。它们都应是 snapshot/read-model consumers。

## 5. Application and Tooling Layer

应用 surface 是 facts、snapshots、features、claims 和 annotations 的消费者，不应成为另一套 chart engine。

推荐顺序：

1. **Static GUI**：验证十二宫图、保存盘流程、时间控制、hover/click 高亮和 i18n。
2. **TUI**：在同一组 snapshot 上提供轻量终端视图和调试 surface。
3. **MCP server/tooling**：在 facade/query surface 稳定后，为 coding agents 暴露结构化 facts、snapshots、features、pattern hits、claims 和 evidence。
4. **Timeline/3D views**：在静态 chart model 稳定后，消费可复用的 static chart frames 和 structured highlights。

前端可以选择不同交互模型，但不能解析已渲染文本来恢复事实，也不能复制安星或规则逻辑。

## 6. Feature Extraction Layer

Feature Extraction Layer 把星盘转换成语义特征图。

重要维度包括：

- 历法和边界设置；
- 十二宫特征；
- 星曜落点和星曜语义；
- 四化流向；
- 对宫、三方四正等宫位关系；
- 格局与组合；
- 大限、流年、流月、流日、流时等时间激活；
- 强弱评分和反证。

这一层的目标不是写文章，而是暴露可供规则引擎评估的特征。

本层第一个只读切片是 `core::pattern`：它把传统格局识别为基于命盘事实的结构化、可解释事实，既不修改命盘，也不产生文字。规则目录与保证见 [`patterns.md`](patterns.md)。

## 7. Rule Engine Layer

Rule Engine Layer 把特征映射成结构化判断。

规则不应该直接输出最终解盘文本。一条规则应输出：

- 领域；
- 主题；
- 极性；
- 强度；
- 证据；
- 反证；
- 来源元数据。

这样规则匹配可以测试，也方便多个规则先聚合，再生成报告。

### 分层分析协调

`analysis` 模块是一个轻量协调层，组合 Feature Extraction Layer 的 `core::pattern` 格局检测与 Rule Engine Layer 的 `rules::classical` 评估，提供**可缓存的逐层**检测。它位于 `core` 之外，正是因为 `core` 不得依赖 `rules`，而分层 API 两者都需要。它不新增任何解读：`detect_analysis_layer` 针对一个 `AnalysisLayerKey` 返回紧凑的 `ClassicalRuleHitRef`（经 `classical_rule_metadata` 解析回逐字原文）以及结构化的 `PatternDetection`，把分组、缓存与渲染留给消费方。API 及“最深层”跨层归属策略见 [`rules/rule-engine.md`](rules/rule-engine.md)。

## 8. Narrative Layer

Narrative Layer 把结构化判断渲染成人类可读报告。

第一版应支持确定性模板。未来可以加入可选的 LLM 润色，但 LLM 不应直接负责原始星盘解读。

## Method profiles

多派别兼容应通过可组合的 method profile 实现，而不是一个巨大的 school enum。

一个 method profile 可以指定：

- 历法策略；
- 排盘算法策略；
- 安星策略；
- 四化策略；
- 特征提取策略；
- 规则集选择；
- 叙事风格。

这允许类似 `全书排盘 + 三合特征 + 基础四化规则 + 技术型报告` 的组合。

## 输入计算策略

`ChartAlgorithmKind`、`ChartPlane` 与 `ChartCalculationConfig` 是相互独立的三个维度，不可混为一谈：

- `ChartAlgorithmKind` 是排盘算法派别（全书 / 中州 / …）。
- `ChartPlane` 是某派别内的盘面变体（天盘 / 地盘 / 人盘）。
- `ChartCalculationConfig` 是在排盘*之前*应用的输入计算策略。

`ChartCalculationConfig` 当前包含 `SolarTimePolicy`、`YearBoundary`、`LeapMonthBoundary` 与 `NominalAgeBoundary`。`YearBoundary` 与 `LeapMonthBoundary` 影响本命输入归一化：`YearBoundary::ChineseNewYearEve` 表示上一干支年持续到除夕结束，新干支年从正月初一开始；`YearBoundary::LiChun` 使用立春，按日期粒度判定（`lunar-lite` 的 `YearDivide::Exact`）。`LeapMonthBoundary` 映射旧的 `fix_leap` 分界。`NominalAgeBoundary` 只属于 runtime/horoscope：它影响虚岁解析，不影响本命排盘。

`Chart` 仍然是不可变的命盘事实聚合。计算诊断通过 generation reports 与 diagnostic snapshots 暴露，而不是存入 `Chart`。这些报告让解析后的钟表时间、真太阳时校正、年分界影响、闰月策略映射与虚岁解析可检查，同时不改变普通 `Chart` 序列化。

用户始终输入出生的钟表时间（时钟时间）。计算策略决定该钟表时间如何转换为时辰：

```text
原始出生日期 + 民用钟表时间
  -> 可选的真太阳时（apparent solar time）校正
  -> 解析后的本地日期/时间
  -> 推导时辰 / 时辰序号
  -> 现有的本命排盘流程
```

真太阳时是一种输入计算策略。它在排盘之前，使用时区与经度对出生钟表时间进行归一化。它**不**定义新的算法，也**不**定义新的盘面。经度校正是精确的：

```text
timezone_meridian_degrees = utc_offset_hours * 15
longitude_correction_minutes = 4 * (longitude_degrees - timezone_meridian_degrees)
resolved_time = clock_time + longitude_correction_minutes + equation_of_time_minutes
```

当校正后的时间跨越午夜时，解析后的公历日期会移动到相邻的一天。这些策略运行在现有排盘流程之前，不定义新的算法、盘面、本命定盘锚点或安星器。

## 证据优先

每个解盘判断都应能追溯到星盘证据。这有利于调试、审查、规则调权和未来经验校验。
