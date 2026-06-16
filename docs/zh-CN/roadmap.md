# 路线图

本路线图刻意保守。项目应先建立稳定架构和兼容性测试，再扩展解读深度。

## Phase 0：文档与架构

- [x] 项目规格。
- [x] 中英文 README。
- [x] 架构文档。
- [x] 兼容性政策。
- [x] 规则引擎设计。
- [x] 术语表。
- [x] 关键决策 ADR。

## Phase 1：Rust workspace 脚手架

- [x] 创建 Rust workspace。
- [x] 添加核心 crates：
  - [x] `core`；
  - [x] `features`；
  - [x] `rules`；
  - [x] `reading`；
  - [x] `iztro-cli`。
- [x] 添加格式化、clippy、测试 CI。
- [x] 添加序列化和 fixture-based test 基础设施。

`core` 的源码树按领域模块组织：`model`（值对象、星曜事实与不可变星盘事实）、`placement`（确定性安星与叠加层激活构建器）、`facade`（对外的 iztro 兼容入口）、`feature`（为未来派生事实提取保留的边界）。crate 错误类型保留在 crate 根部。这只是内部重组；公开 API 与排盘行为均未改变。

## Phase 2：核心星盘模型

- [x] 定义天干、地支、宫位、星曜、四化、作用范围、性别、历法配置。
- [x] 定义星盘、宫位和星曜落点模型。
- [x] 定义大限与运限模型。
- [x] 确保已实现模型强类型且可序列化。
- [x] 将上游 `iztro@2.5.8` runtime 星曜名称清单与已表示星盘事实分开维护。

大限与运限模型以叠加层形式定义：`HoroscopeChart` 包裹不可变的本命 `Chart`，并持有
零个或多个 `TemporalLayer`，每个层带有非本命的 `Scope`、强类型的 `TemporalContext`、
按范围划分的 `StarPlacement` 和 `MutagenActivation`。这些只是调用方显式提供的模型
事实；时间范围安星与历法推导仍推迟到 Phase 3。这些模型之上的前两个时间算法现已
提供：`build_yearly_mutagen_layer` 从显式的流年干支生成流年 `TemporalLayer` 四化
激活，`build_decadal_mutagen_layer` 则从显式的大限干支及起运年龄生成大限版。两者都
复用共享的天干四化表，作用于本命盘中已存在的星曜。它们仅为叠加层——不安放流曜、
不修改本命、不推导历法/年龄区间/大限命宫/大限宫位——四化仍作为 `MutagenActivation`
事实，而非独立星曜。

星曜 metadata 有意拆成两层。`represented_star_metadata_table().len() == 70` 覆盖已安放
且有 fixture 覆盖的本命星，其中包含受算法门控的中州派特有杂曜。
`known_star_metadata_table().len() == 170` 记录上游 `iztro@2.5.8` runtime 星曜名称条目，
包括已表示本命星、装饰性 runtime 数组以及 horoscope 流曜名称。已表示 metadata 保持
仅本命；装饰性 runtime 条目是 known 的无类型 runtime 事实，流曜则是通过
`TemporalLayer` 安放的 known 有类型时间事实。`xunzhong` / `旬中` 因只属于 locale 而
被排除；已支持本命 `getAdjectiveStar` 切片以外的神煞安星、
上游 runtime 查询助手、亮度扩展，以及把四化建模为星曜，仍然延期。完整 horoscope stack 组装
（大限、小限、流年、流月、流日、流时组装为一个 `HoroscopeChart`）已实现，流年层并附带
`yearlyDecStar`（岁前/将前十二神）作为流年范围的时间性装饰事实，但仅为已支持事实面的模型级组装。

## Phase 3：排盘兼容性

- [x] 实现最小 `by_lunar` 入口。
- [x] 实现最小 `by_solar` 入口。
- [x] 将当前排盘切片拆成小的确定性模块进行移植或重写。
- [x] 加入与选定 `iztro` 输出对齐的 implemented-slice golden tests。
- [x] 记录 implemented slice 的已知差异。
- [x] 添加默认算法本命杂曜。iztro 2.5.8 默认算法的全部 38 颗杂曜均已安放；逐星落点基准见兼容性文档「默认算法本命杂曜全集」。
- [x] 添加中州派特有本命杂曜。`ChartAlgorithmKind::Zhongzhou` 根据上游 iztro 2.5.8 fixtures 安放龙德/截空/劫煞/大耗，省略默认截路/空亡，并保留中州派天伤/天使互换。
- [x] 安放装饰性 runtime 星曜家族。`by_lunar` 将长生/博士/岁前/将前十二神作为每宫无类型的 `DecorativeStarPlacement` 安放，并与 `Chart::stars()` 分离。岁破 known，且在中州派下可替代第七个岁前位置，但不是额外的第十三个岁前 placement。
- [x] 安放 scoped flow stars。`build_flow_star_layer` 通过规范化的 `FlowStarScope` + `FlowStarBase` identity，将大限/流年/流月/流日/流时流曜（以及流年年解）安放为带地支标签的 `ScopedStarPlacement`。
- [x] 添加阳历转农历与闰月行为。`by_solar` 通过内部 `lunar-lite` 适配器将公历日期转换为农历事实并委托给 `by_lunar`；`by_lunar` 为已支持切片携带显式的 `is_leap_month`/`fix_leap` 语义。两者均以 `iztro@2.5.8` fixtures 校验。日历后端类型不出现在公开 API 中。
- [x] 添加早晚子时变体。`BirthTime` 建模上游 `iztro` `timeIndex` `0..=12`，保留早子时（`0`）与晚子时（`12`），同时让按地支传入的 request API 保持向后兼容。
- [x] 添加 fixture-backed 流月 period 与 layer 组装。`build_monthly_period` 保留流月干支和流月命宫为独立事实，`build_monthly_horoscope_layer` 组装流月流曜、流月四化和流月宫名布局。
- [x] 添加 fixture-backed 流日 period 与 layer 组装。`build_daily_period` 保留流日干支和流日命宫为独立事实，`build_daily_horoscope_layer` 组装流日流曜、流日四化和流日宫名布局。
- [x] 添加 fixture-backed 流时 period 与 layer 组装。`build_hourly_period` 保留流时干支和流时命宫为独立事实，`build_hourly_horoscope_layer` 组装流时流曜、流时四化和流时宫名布局。
- [x] 添加完整 horoscope stack 组装。`build_full_horoscope_chart`（输入 `HoroscopeStackInput`）按确定顺序把大限、小限、流年、流月、流日、流时层组装为一个 `HoroscopeChart`，并按推导的虚岁选取大限 period。仅为模型级组装，非上游 `FunctionalAstrolabe#horoscope` 载荷对齐或查询助手。
- [x] 添加流年 `yearlyDecStar`（岁前/将前十二神）作为流年范围的时间性装饰事实（`build_yearly_decorative_star_placements`）。无类型：不进入 `Chart::stars()` 或本命 `Palace::decorative_stars()`。
- [ ] 添加完整八字输出、上游 runtime 查询助手、bindings、特征提取、规则与叙事。

当前核心切片：`by_lunar` 接受显式农历输入以及显式出生年干、年支，生成确定性的本命星盘事实，并用选定的 `iztro` 2.5.8 fixtures 校验 minimal chart 字段、十四主星、十四颗已支持辅星、完整默认算法的 38 颗本命杂曜/辅助星，以及中州派 40 颗本命杂曜/辅助星输出。默认/非中州派输出保持 14 主星 + 14 辅星 + 38 杂曜/辅助星 = 66 颗本命星；中州派输出为 14 主星 + 14 辅星 + 40 杂曜/辅助星 = 68 颗本命星。已表示 metadata table 为 70 颗，因为默认专属与中州派专属本命杂曜都属于已表示星曜。装饰性 runtime 家族（长生/博士/岁前/将前十二神）与 scoped 流曜现在作为独立事实安放（见下文）。`by_solar` 增加了最小的 `lunar-lite` 阳历转农历转换并委托给 `by_lunar`，后者现在为已支持切片建模 fixture 支持的闰月行为（`is_leap_month`/`fix_leap`）与早晚子时变体（`BirthTime` / `timeIndex` `0..=12`）。流月、流日与流时 period 与 layer 组装已有 fixture-backed 覆盖，完整 horoscope 组装与流年 `yearlyDecStar` 时间性装饰事实现已实现；完整八字输出、上游 runtime 查询助手、bindings、特征提取、规则与叙事仍然推迟。四化仍作为安星上的 `Mutagen` 事实，而非独立星曜。

四组装饰性 runtime 家族由 `by_lunar` 作为无类型 `DecorativeStarPlacement` 安放到独立
的 `Palace::decorative_stars()` collection 中，因此 `Chart::stars()` 仍只包含有类型
星曜，66/68 的计数不变。它们只能通过 `try_known_star_metadata` resolve（没有
`StarKind`）。上游只输出 12 个岁前条目：岁破 known，且在中州派下可替代大耗，但不
会作为额外岁前条目安放。

流曜 runtime identity 通过 `FlowStarScope` + `FlowStarBase` 规范化：`YunKui`、
`LiuKui`、`YueKui`、`RiKui`、`ShiKui` 等带范围的上游名称仍保留为彼此独立的
`StarName` 变体，以保证 serde/runtime fidelity；`flow_star_name(scope, base)` 暴露
它们共享的 identity。`build_flow_star_layer` 现在据此将十颗 matrix 流曜（加流年年解）
作为有类型、带地支标签的 `ScopedStarPlacement` 安放到 `TemporalLayer` 中。这样安放
流曜不会改变 metadata table 数量、不会改变本命 `by_lunar` 输出，也不会把时间范围四化
建模为星曜；`represented_star_metadata_table()` 保持仅本命（70）。

## Phase 4：特征提取

- [x] 提取宫位特征。
- [x] 提取星曜特征。
- [x] 提取本命四化流向。
- [x] 提取宫位关系、三方四正、对宫。
- [ ] 添加强弱评分占位接口。
- [ ] 添加时间激活接口。

首个切片已实现：`features` 的 `BasicFeatureExtractor` 将确定性星盘事实转换为结构化的宫位特征、星曜特征、本命四化流向和宫位循环关系。星曜特征保留所有落点星曜事实；宫位与领域的映射是可选元数据，目前仅限五个直接的宫位—领域映射（命宫、官禄宫、财帛宫、夫妻宫、疾厄宫），其他宫位的星曜不带领域。此阶段仅做特征提取——不做规则匹配、不产出判断、不做解读、不生成叙事。强弱评分与时间激活接口仍然推迟。

## Phase 5：规则引擎骨架

- [ ] 定义规则 schema。
- [ ] 从 TOML 加载规则。
- [ ] 用规则匹配提取后的特征。
- [ ] 输出带证据和来源元数据的结构化判断。
- [ ] 为规则匹配添加确定性单元测试。

## Phase 6：基础确定性解读

- [ ] 添加少量 seed rules。
- [ ] 为性格、事业、财富、关系生成领域判断。
- [ ] 从结构化判断渲染确定性报告。
- [ ] 叙事保持简洁、证据优先。

## Phase 7：多方法扩展

- [ ] 添加 method profiles。
- [ ] 支持多个排盘或特征提取策略。
- [ ] 添加不同派别或解读风格的可选规则集。
- [ ] 保持 profile 组合显式且可测试。

## Phase 8：绑定与应用

- [ ] CLI。
- [ ] Python bindings。
- [ ] WebAssembly bindings。
- [ ] TUI 前端，推迟到核心模型和报告结构稳定后再设计。
- [ ] GUI 前端，推迟到核心模型和报告结构稳定后再设计。
- [ ] 可选 LLM 叙事润色。

应用前端会被刻意推迟。核心 crates 应保持 UI 无关、确定性、可序列化，使未来 CLI、TUI、GUI、WASM 和 Python 前端可以直接消费星盘、特征、判断、证据和报告结构，而不需要解析自然语言文本。

## 发布策略

`0.1.0` 之前 API 可以自由变更。`0.1.0` 之后，破坏性变更应记录在 `CHANGELOG.md`，必要时补充 ADR。
