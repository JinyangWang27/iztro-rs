# 兼容性政策

`iztro-rs` 受 `iztro` 启发，并在适用范围内以 `iztro` 校验排盘行为。

## 兼容性的含义

兼容性意味着：

- 选定的排盘输出应与 `iztro` golden fixtures 保持一致；
- 差异应明确记录；
- Rust 公开模型应尽量保留相同的星盘事实；
- 测试应显式说明兼容目标，而不是隐含假设。

## 兼容性不意味着

兼容性不要求：

- 内部架构完全一致；
- 公开 API 名称完全一致；
- 数据表示仍然是字符串优先；
- 解盘叙事输出完全一致；
- 第一版支持 `iztro` 的全部功能。

## 兼容目标

当前兼容目标为：

- `iztro` npm package version `2.5.8`。

后续兼容 fixture 如需更新目标版本，必须同时记录版本变化和预期输出差异。

本地检查上游行为时，可使用 `tools/iztro-reference` 下固定版本的 npm reference workspace：

```bash
npm ci --prefix tools/iztro-reference
```

已提交的 fixture JSON 仍是兼容性的 source of truth。

## 当前支持 surface 概览

当前 fixture-backed chart-generation surface 包括：

- 强类型 `by_lunar` 与 `by_solar` request facades；
- `lunar-lite` 1.0.0-backed 阳历转农历，以及 `by_solar` 的可配置年分界出生年/四柱事实推导；
- supported slice 的闰月 / `fix_leap` 行为，包括 `LeapMonthBoundary` 映射；
- `YearBoundary`、`LeapMonthBoundary` 与 `NominalAgeBoundary` 的 fixture-backed calculation configuration cases；
- 通过 `BirthTime` 建模上游 `timeIndex` `0..=12` 早晚子时；
- 出生年 `StemBranch`、十二宫、命宫、身宫、宫干、五行局；
- `by_solar` 星盘上的可选事实性 `lunar_lite::FourPillars`；
- `ChartProfile` 元数据，用于在生成的 chart 上保留 method profile 与 chart plane；
- typed palace lookup helpers 与 compact chart diagnostics，用于 invariant/debug surfaces；
- 已表示的有类型本命星曜、亮度与出生年四化；
- 无类型装饰性 runtime 星曜家族；
- explicit temporal contexts 下的 branch-tagged typed temporal flow-star placements；
- explicit contexts 下的大限与流年 temporal mutagen activation layers；
- typed `DecadalFrame`、`MonthlyPeriod`、`DailyPeriod`、`HourlyPeriod` 及其已实现 layer assembly；
- `build_full_horoscope_chart`：将大限、小限、流年、流月、流日、流时 layers 组装为一个 `HoroscopeChart`；
- 流年层的 yearly `yearlyDecStar`（岁前/将前十二神）作为 yearly-scope temporal decorative facts；
- `HoroscopeSupportedFieldsSnapshot`、`HoroscopeRuntime` 与 `HoroscopeFacadeSnapshot`，分别以 `horoscope.json`、`horoscope_runtime.json`、`horoscope_facade.json` fixture 校验已实现事实面；
- renderer-neutral `ChartStackSnapshot`、GUI-facing `StaticChartViewSnapshot` 与 deterministic plain text renderer demo。

项目现在提供一个 upstream-like horoscope facade snapshot，基于 `HoroscopeChart`、`HoroscopeSupportedFieldsSnapshot`、`NatalFacadeSnapshot` 与 `HoroscopeRuntime` 组合而成。它更接近 TS `FunctionalAstrolabe#horoscope` payload shape，但仍**不是**完整 package parity：内嵌 `astrolabe` 有意保持最小，只包含已建模的本命事实；完整上游 astrolabe helper/query 方法、本地化标签、八字字符串、大限 ranges、ages 数组、bindings、renderers、rules 与 narrative 仍延期。

## 星曜名称清单

`core` 保留两套相互独立的星曜 metadata surface：

- `represented_star_metadata_table().len() == 70` 保持严格边界：只覆盖当前由 chart facts 表示、由 Rust 代码安放、并由 fixtures 校验的星。其中四颗已表示杂曜受算法门控，只会在 `ChartAlgorithmKind::Zhongzhou` 下出现。
- `known_star_metadata_table().len() == 170` 清点更广的上游 `iztro@2.5.8` runtime 星曜名称宇宙，包含已表示星曜、装饰性 runtime arrays（`changsheng12`、`boshi12`、`suiqian12`、`jiangqian12`），以及大限、流年、流月、流日、流时的 horoscope flow-star names。

`represented_star_metadata_table()` 保持本命-only。装饰性 families 是无类型 `DecorativeStarPlacement`，不会出现在 `Chart::stars()`。Horoscope flow stars 是 typed、branch-tagged `ScopedStarPlacement`，存在于 `TemporalLayer` 内，不属于 natal represented metadata。

上游 locale key `xunzhong` / `旬中` 被有意排除，因为在 `iztro@2.5.8` 中没有找到内置的 `FunctionalStar` 构造或 `StarType` 分配。四化仍作为 `Mutagen` / `MutagenActivation` 事实存在，而不是 `StarName` variants。

## 天地人三盘兼容性边界

`ChartPlane` 是 Rust-side 维度，与 `ChartAlgorithmKind` 分离。`Heaven` 是默认盘别，并保留 default / Zhongzhou algorithms 的既有 fixture-backed 输出。生成的 `Chart` 通过 `ChartProfile` 保留所选盘别。

中州地盘与人盘是 Rust extension behaviour，不是上游 `iztro@2.5.8` parity targets。TS `iztro@2.5.8` 不暴露可比较的 Earth/Human chart-plane generation，因此这些盘别通过内部结构 invariant、anchor resolver tests、diagnostic snapshots 和架构文档保护，而不是通过 TS golden fixtures 校验。

支持组合是显式的：`QuanShu + Heaven`、`Zhongzhou + Heaven`、`Zhongzhou + Earth`、`Zhongzhou + Human`，以及兼容性的 `Placeholder + Heaven`。请求 `QuanShu + Earth/Human` 或 `Placeholder + Earth/Human` 会返回 `ChartError::UnsupportedChartPlane`。

## 公开 facade 兼容性

`by_lunar` 与 `by_solar` 是 `iztro-rs` 的 iztro-compatible facade 入口。它们概念上对应 iztro 的 `astro.byLunar(...)` 与 `astro.bySolar(...)`，但使用强类型 `LunarChartRequest` 与 `SolarChartRequest`，而不是 JavaScript 风格位置参数。

`by_lunar` 把传入农历日期记录为 chart input facts，并委托给 supported-star natal chart builder。它通过 `is_leap_month` 与 `fix_leap` 携带显式闰月语义；无效闰月请求会按真实历法 normalizer 处理，而不是盲目回显。出生年干支仍是显式 `by_lunar` inputs，并校验为 chart 上保留的出生年 `StemBranch`。

出生时间由 `BirthTime` 表示，对齐上游 `iztro` `timeIndex` `0..=12`。`EarlyZi` (`0`) 与 `LateZi` (`12`) 都投影到 `EarthlyBranch::Zi`；按地支传入的 request setters 继续把 `Zi` 映射为早子时，以保持向后兼容。

`by_solar` 校验阳历日期，通过内部 `lunar-lite` adapter 转换为农历事实，并通过 `lunar-lite` 1.0.0 的可配置 year-boundary four-pillar API 推导事实性本命四柱。它设置 `is_leap_month` 与 `fix_leap`，委托 `by_lunar` 安星，并把结果 `lunar_lite::FourPillars` 保留在 `Chart`。它本身不添加安星逻辑。显式 invariant：当 `Chart::four_pillars()` 存在时，其年柱等于 `Chart::birth_year()`。

`ChartCalculationConfig` 是 input/runtime calculation-policy 维度，不是算法或盘面维度。它包含 `SolarTimePolicy`、`YearBoundary`、`LeapMonthBoundary` 与 `NominalAgeBoundary`。`YearBoundary` 将上游 `yearDivide: "normal"` 映射到 `ChineseNewYearEve`，将 `"exact"` 映射到 `LiChun`；`ChineseNewYearEve` 表示上一年持续到除夕结束，新干支年从正月初一开始。`LeapMonthBoundary` 将上游 `fixLeap: false` 映射到 `AsPreviousMonth`，将 `fixLeap: true` 映射到 `MidMonth`。`NominalAgeBoundary` 将上游 `ageDivide: "normal"` 映射到 `NaturalYear`，将 `"birthday"` 映射到 `Birthday`，且只用于 runtime/full-horoscope 虚岁解析。

Generation reports 会为这个 supported surface 暴露计算诊断 snapshots。它们是 iztro-rs 的调试/export surface，不是完整上游 package parity 声明。`iztro-rs` 对 fixture-backed calculation configuration cases 具备与上游 `iztro@2.5.8` 的 supported-field parity；它不是每个上游 TS feature 的完整 drop-in semantic clone。

`by_lunar` 保持保守：它接受显式出生年干支，但不会从农历 input 伪造月柱、日柱或时柱，因此该 slice 中 `by_lunar` chart 的 `Chart::four_pillars()` 为 `None`。未来 PR 可决定 `by_lunar` 是否接受显式 `FourPillars`，或通过规范化阳历日期推导。

`lunar-lite` 拥有 canonical low-level stem/branch、sexagenary-cycle 和 four-pillar primitives（`HeavenlyStem`、`EarthlyBranch`、`StemBranch`、`FourPillars`），并由 `core` re-export。`core` 拥有紫微斗数特有的纳音与五行局逻辑。

## 运限层模型

`core` 定义 model-only horoscope overlays：`HoroscopeChart` 包裹不可变本命 `Chart`，并持有零个或多个 `TemporalLayer`。每个层带有非本命 `Scope`、typed `TemporalContext`、scoped `StarPlacement` 与 `MutagenActivation`。这些模型只承载调用方显式提供的时间事实，且 layer 不复制本命 placements。

Yearly / decadal mutagen overlay builders 根据显式 stem-branch/context facts 生成 `TemporalLayer`，把对应 Heavenly Stem 的四化应用到本命盘中实际存在的 represented stars 上。它们不推导历法事实、不安放 flow stars、不修改本命 placements，也不做 interpretation。

Scoped flow-star builder (`build_flow_star_layer`) 为一个 explicit `TemporalContext` 安放 horoscope flow stars（流曜）。该 placement 是 branch-based，不执行 horoscope palace-name derivation。

`build_full_horoscope_chart` 将 decadal、age、yearly、monthly、daily、hourly layers 组装为一个 `HoroscopeChart`。这只是已支持字段的 model-level stack assembly，不复刻完整上游 `FunctionalAstrolabe#horoscope` payload shape。

`HoroscopeRuntime` 提供 typed Rust equivalents for upstream runtime helper slice。`age_palace`、`palace`、`surround_palaces` 按地支投影；本命宫名/宫干/星曜保持可用，时间宫名作为 additive label，不覆盖本命 identity。`has_horoscope_stars` 等查询 helpers 是 fixture-backed against `iztro@2.5.8`。Full facade payload parity 仍延期。

## Runtime 星曜家族安放

Typed stars 与 decorative runtime entries 是分离事实面，`Chart::stars()` 只返回 typed `StarPlacement`。

长生/博士/岁前/将前十二神四个 decorative families 建模为无类型 `DecorativeStarPlacement`，而不是 fake-typed `StarPlacement`。它们存在于 `Palace::decorative_stars()`，并通过 `Chart::decorative_stars()` / `Chart::decorative_star()` 读取。

因为 decorative entries 是分离事实，default / non-Zhongzhou 的 `Chart::stars()` 仍为 66 个 typed natal placements，Zhongzhou 的 `Chart::stars()` 仍为 68 个 typed natal placements。

## Snapshot 与 render 兼容性

`ChartStackSnapshot` 是 renderer-neutral read model，不是上游 `iztro` facade payload，也不打算匹配上游 JSON shape。

它保留：

- birth context 与 method profile 等 chart identity fields；
- 出生年干支与可选事实性本命四柱；
- 本命命宫/身宫地支与五行局；
- 传统十二宫 visual grid positions；
- natal layer first, then temporal layers 的 stacked layer model；
- typed natal stars、decorative stars、scoped temporal stars、mutagen activations 的分离 cell sections。

`render` 的 plain text renderer 消费 `ChartStackSnapshot` 用于 demo 和 debugging。Renderer output 是确定性的，但不属于与上游 `iztro` 的 chart-generation compatibility。

## 当前 fixtures

Fixtures 有意保持 supported-field-only。它们覆盖当前 natal、decorative、flow-star、solar/lunar conversion、leap-month、calculation configuration、rat-hour、horoscope runtime 和 facade snapshot slices，并在适用处对齐 `iztro@2.5.8`。`iztro-rs` 对 fixture-backed calculation configuration cases 具备 supported-field parity；这不是对每个上游 TS feature 的完整 drop-in semantic clone 声明。

关键 fixture groups 包括：

- minimal natal chart facts；
- major stars；
- minor stars；
- default 与 Zhongzhou adjective/helper stars；
- runtime decorative families；
- flow stars；
- horoscope monthly / daily / hourly period/layer cases；
- full horoscope stack assembly cases；
- normalized full-horoscope supported-fields snapshot cases；
- horoscope facade payload snapshot cases；
- e2e supported `by_lunar` cases；
- e2e supported `by_solar` cases；
- leap-month behavior；
- rat-hour behavior。

具体 fixture files 位于 `crates/iztro/fixtures/iztro/`。Regeneration scripts 位于 `tools/iztro-reference`。

## 本地化标签

Rust 内部领域模型保持语言中立：天干、地支、宫位、星曜、四化、亮度、星类与 families 都是强类型 enums，并以稳定 machine-readable keys 序列化。

Facade/export natal astrolabe snapshots 可以附加 conventional Chinese (`zh-CN`) labels，例如 `branch`/`branch_zh`、`name`/`name_zh`、`stem`/`stem_zh`。这些标签由 deterministic `core::labels::zh_cn` 查表生成，绝不替代 canonical identity，因此兼容性断言仍只校验 machine-readable fields。

Runtime GUI/application localization 由 `crates/iztro-i18n` 单独处理。它当前通过 Fluent resources 与 typed label helpers 支持 English (`en-US`) 和 Simplified Chinese (`zh-Hans`)。完整上游 localized-string parity、更多 locales 和 BaZi localized output 仍延期。

## 延期的兼容性工作

延期 surfaces 包括：

- full upstream facade serialization parity；
- 当前 `iztro-i18n` English / Simplified Chinese surface 之外的 additional locales 和 complete upstream localized-string parity；
- factual `by_solar` natal four pillars 之外的 full BaZi interpretation/output；
- bindings；
- temporal activation feature extraction；
- rules；
- narrative and interpretation output。
