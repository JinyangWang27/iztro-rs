# 当前项目状态

本文总结当前已实现的主要事实面。英文文档仍是工程规格的规范来源；本文是对应中文状态说明，并以中文术语表达紫微斗数领域概念。

## 兼容目标

当前排盘兼容目标为 `iztro@2.5.8`。

兼容性是 fixture-driven 且有边界的：项目仅声明已支持事实面的兼容性，不声明完整上游 API parity、完整 horoscope facade payload parity、完整序列化 parity 或解读叙事 parity。

## 已实现的本命排盘事实面

当前已支持：

- 强类型 request facade：`by_lunar` 与 `by_solar`；
- `lunar-lite` 1.0.0-backed 阳历转农历，以及 `by_solar` 的 normal-boundary 四柱事实推导；
- 闰月与 `fix_leap` 行为的 supported slice；
- `BirthTime` / 上游 `timeIndex` `0..=12`，包括早子时与晚子时；
- `Chart::birth_year()` 出生年干支事实；
- `by_solar` 星盘上的可选事实性 `Chart::four_pillars()`；
- `ChartProfile` 元数据：在 `Chart` 上保留 `MethodProfile` 与 `ChartPlane`；
- 按宫名和地支查询的 typed palace lookup helpers，以及 invariant-sensitive 的 required lookup variants；
- `Chart::diagnostic_snapshot()`，用于结构诊断和 invariant 调试；
- 十二宫布局、命宫、身宫、宫干、五行局；
- 已表示的有类型本命星曜、亮度和出生年四化；
- 本命装饰性 runtime 星曜家族，作为无类型 decorative facts；
- 大限/流年/流月/流日/流时等时间层的 scoped flow-star placements；
- decadal、age、yearly、monthly、daily、hourly 层的 typed period / layer / mutagen / palace-layout 事实；
- `build_full_horoscope_chart` 将大限、小限、流年、流月、流日、流时组装为一个 `HoroscopeChart`；
- `HoroscopeSupportedFieldsSnapshot`、`HoroscopeRuntime` 与 `HoroscopeFacadeSnapshot`，用于已建模 horoscope fact surface 的 fixture-backed 导出。

默认 / 非中州本命输出保持 66 颗有类型本命星。中州本命输出保持 68 颗有类型本命星。`represented_star_metadata_table().len() == 70` 保持本命星事实边界；`known_star_metadata_table().len() == 170` 清点更广的上游 runtime 星曜名称宇宙。

## 天地人三盘

`ChartPlane` 与 `ChartAlgorithmKind` 是两个独立维度。`Heaven`（天盘）是默认值，并保持既有 fixture-backed 输出。

中州派支持：

- `Zhongzhou + Heaven`：中州算法的天盘，不等同于全书算法；
- `Zhongzhou + Earth`：将命宫重新锚定到天盘身宫地支；
- `Zhongzhou + Human`：将命宫重新锚定到天盘福德宫（`PalaceName::Spirit`）地支。

地盘与人盘通过带锚点的最小星盘重建生成，而不是修改已完成的 `Chart`。重新锚定后，宫名、宫干与五行局会根据新的命宫重新推导；身宫地支保持原始计算值。具体锚点分派由 `core::placement::natal::plane::resolve_natal_chart_anchor` 处理，`by_lunar` facade 只负责 request adaptation 和构造 Heaven chart closure。

`Zhongzhou + Earth` 与 `Zhongzhou + Human` 是 Rust 扩展行为，不是上游 `iztro@2.5.8` parity target，因为 TS `iztro@2.5.8` 不暴露这些 chart planes。它们由结构 invariant、anchor resolver 测试、diagnostic snapshot 和架构文档保护。

## 输入计算策略

`ChartCalculationConfig` 是第三个维度，与 `ChartAlgorithmKind`、`ChartPlane` 相互独立。它决定出生钟表时间在排盘*之前*如何转换为时辰。钟表时间入口 `by_solar_with_options` / `by_lunar_with_options` 先通过 `core::calculation::resolve_birth_datetime` 解析输入，再委托给现有的 `by_solar` / `by_lunar` 流程，因此 `Chart` 的序列化保持不变。

计算策略现在包含 `SolarTimePolicy`、`YearBoundary`、`LeapMonthBoundary` 与 `NominalAgeBoundary`。默认值保持既有行为：钟表时间、农历新年干支年分界（`ChineseNewYearEve`：上一年持续到除夕结束，新年从正月初一开始）、闰月月中分界、按自然年虚岁。`YearBoundary` 与 `LeapMonthBoundary` 影响本命输入归一化；`NominalAgeBoundary` 只影响 runtime/full-horoscope 虚岁解析。

钟表时间 facade 现在也暴露 report APIs：`by_solar_with_options_report`、`by_lunar_with_options_report`、`resolve_solar_birth_input`、`resolve_lunar_birth_input` 与 `build_full_horoscope_chart_report`。它们在生成 chart 或 horoscope 的同时返回计算诊断 snapshot，记录解析后的钟表时间、真太阳时经度/均时差校正、有效出生年、闰月 `fix_leap` 映射与解析后的虚岁，同时保持普通 `Chart` 序列化不变。

默认策略（`SolarTimePolicy::ClockTime`）直接由钟表时间推导时辰。`SolarTimePolicy::ApparentSolarTime` 应用精确的经度校正（`4 * (经度 − 时区中央经线)` 分钟，经度差先做跨越对日线的归一化），并可能使解析后的公历日期跨越午夜。`EquationOfTimePolicy::Approximate` 尚未实现，返回 `ChartError::UnsupportedEquationOfTimePolicy`。农历日期输入拒绝真太阳时（`ChartError::ApparentSolarTimeRequiresSolarDate`）。

注意：`ChartError` 现在派生 `PartialEq` 但不再派生 `Eq`，因为输入计算策略的校验错误可能携带浮点经度值。

## 运行时本地化

运行时本地化已经通过 `crates/iztro-i18n` 实现。当前支持：

- 默认 GUI / runtime locale：`en-US`；
- 首个中文 locale：`zh-Hans`；
- Fluent 资源编译期打包；
- typed label helpers，用于星曜、宫位、四化、时间范围、亮度和共享 UI 文案；
- 缺失翻译时 fallback 到英文，再 fallback 到可见占位符；
- `iztro-gui` 使用 `iztro-i18n` 渲染当前 English / Simplified Chinese 界面。

Core 模型保持语言中立。Facade/export DTO 可以保留附加 zh-CN `*_zh` 标签用于兼容和可读性，但这些标签不是内部事实来源。

## 当前不是完整 parity 的部分

仍然延期：

- 完整上游 facade 序列化 parity；
- 完整上游 astrolabe helper/query 方法；
- 上游本地化日期字符串、完整本命 localized labels、BaZi 字符串、大限 ranges、ages 数组；
- 完整八字解读、十神、藏干、五行评分、喜用神、成格等；
- temporal decorative arrays beyond yearly `yearlyDecStar`；
- bindings、TUI、MCP、WASM frontend；
- 完整规则评估与确定性解读；
- 叙事或 LLM-assisted prose。

## 近期方向

近期工作应继续小步推进：

1. 保持 compatibility fixture-backed，避免大范围改写安星逻辑。
2. 扩展 `iztro-i18n` 覆盖面和 UI 字符串审计。
3. 继续改善 Iced static chart GUI 的保存、时间导航、布局和本地化体验。
4. 将 TUI / CLI / MCP 作为现有 snapshots 和 view models 的消费者，而不是新 chart engine。
5. 在事实面稳定后再扩展 feature extraction、pattern/rule output 和 narrative。
