# ADR 0007：历法引擎——迁回 `lunar-lite`

## 状态

已接受。取代 [ADR 0006](0006-calendar-engine-tyme4rs-adapter.md)。

## 背景

[ADR 0006](0006-calendar-engine-tyme4rs-adapter.md) 用 `tyme4rs` 替换了
`lunar-lite`，以获得精确瞬时的「立春」分界与更丰富的节气数据，并将底层干支值对象
复制到 `core/model/ganzhi`。

此后有两点变化：

- `lunar-lite`（1.1.0）现已提供与 tyme 兼容的天文后端及广泛的 oracle 测试，重新成为
  可靠的底层历法/干支 crate。
- 仅 `tyme4rs` 提供的精确瞬时「立春」分界是与 `iztro@2.5.8` 唯一一处有记录偏离的
  来源（`year_divide_exact_2000_02_04` → `己卯` 而非上游 `庚辰`）。恢复 `lunar-lite`
  的日期级分界即可消除该偏离。

维护一份复制的干支模型与第二个历法依赖带来了成本，却没有净兼容性收益。

## 决策

- 依赖 `lunar-lite`（`= 1.1.0`，启用 `serde` feature）作为底层历法/干支 crate。移除
  `tyme4rs` 依赖。
- 删除复制的 `core/model/ganzhi.rs`。直接使用
  `lunar_lite::{HeavenlyStem, EarthlyBranch, StemBranch, FourPillars,
  StemBranchError, HEAVENLY_STEMS, EARTHLY_BRANCHES}`；`core` 继续 re-export 这些类型，
  公开 API 不变。
- 将 `lunar-lite` 置于内部 `core/calendar` 适配器之后。公历/农历转换、农历日期归一化、
  农历月天数与四柱均委托给 `lunar-lite`（`solar_to_lunar`、`lunar_to_solar`、
  `normalize_lunar_date`、`lunar_month_days`、
  `four_pillars_from_solar_date_with_options`）。适配器移除了独立的 `tyme.rs` 引擎包装与
  `policy.rs` 年/月柱模块：`lunar-lite` 的四柱 API 通过
  `StemBranchOptions { year: YearDivide, month: MonthDivide }` 推导全部四柱。
- 紫微斗数排盘策略保留在 `iztro-rs`（`ChartCalculationConfig`、`SolarTimePolicy`、
  `ApparentSolarTimeConfig`、`YearBoundary`、`LeapMonthBoundary`、`NominalAgeBoundary`）。
  `YearBoundary` 映射到 `lunar-lite` 的 `YearDivide`（`ChineseNewYearEve → Normal`、
  `LiChun → Exact`）；月柱使用 `MonthDivide::Normal`（五虎遁）。真太阳时作为 `iztro-rs`
  策略在历法转换**之前**应用；历法只接收已解析的本地日期与时辰索引。
- `YearBoundary::LiChun` 为**日期级**，与 `lunar-lite` 的 `YearDivide::Exact` 及
  `iztro@2.5.8` 一致：立春当日属于新干支年，与钟表时间无关。精确瞬时的钟表路径
  （`solar_to_lunar_with_resolved_datetime`）已移除。

## 影响

- 运行时历法事实来自 `lunar-lite`。
- 公开/领域 API 不变（`core` 仍 re-export 相同的干支类型），但它们由 `lunar-lite` 拥有，
  不再复制。
- `year_divide_exact_2000_02_04` supported-field fixture 重新与上游 `庚辰` 一致；有意偏离
  标注被移除，所有 fixture 用例保持与上游严格一致。
- 不再提供时刻级（精确瞬时）「立春」分界。如日后需要，正确做法是让 `lunar-lite` 暴露
  节气瞬时基元，并在其上新增一个精确瞬时的 `YearBoundary` 变体，而非重新引入第二个历法
  依赖。此项暂缓。
- `lunar-lite` 的 `ChildLimit`/起运与完整八字解读仍待单独兼容性分析后再定，暂缓。
