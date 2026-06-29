# ADR 0007：历法引擎——迁回 `lunar-lite`

## 状态

已接受。取代 [ADR 0006](0006-calendar-engine-tyme4rs-adapter.md)。

## 背景

[ADR 0006](0006-calendar-engine-tyme4rs-adapter.md) 用 `tyme4rs` 替换了
`lunar-lite`，以获得精确瞬时的「立春」分界与更丰富的节气数据，并将底层干支值对象
复制到 `core/model/ganzhi`。

此后有两点变化：

- `lunar-lite`（1.2.1）现已提供与 tyme 兼容的天文后端、广泛的 oracle 测试，以及
  公开的时刻级「立春」基元
  （`lunar_lite::li_chun_datetime(year) -> SolarTermDateTime`）。它重新成为可靠的
  底层历法/干支 crate，且精确瞬时分界不再需要第二个历法依赖。
- 维护一份复制的干支模型与第二个历法依赖带来了成本，却没有净收益。

## 决策

- 依赖 `lunar-lite`（`1.2.1`，启用 `serde` feature）作为底层历法/干支 crate。移除
  `tyme4rs` 依赖。
- 删除复制的 `core/model/ganzhi.rs`。直接使用
  `lunar_lite::{HeavenlyStem, EarthlyBranch, StemBranch, FourPillars,
  StemBranchError, HEAVENLY_STEMS, EARTHLY_BRANCHES}`；`core` 继续 re-export 这些类型，
  公开 API 不变。`FourPillars` 是 `lunar-lite` 的值对象，并非 `iztro-rs` 自有类型。
- 将 `lunar-lite` 置于内部 `core/calendar` 适配器之后。公历/农历转换、农历日期归一化与
  农历月天数均委托给 `lunar-lite`（`solar_to_lunar`、`lunar_to_solar`、
  `normalize_lunar_date`、`lunar_month_days`）。四柱方面，适配器**仅**从
  `four_pillars_from_solar_date_with_options` 取用日柱与时柱；年柱与月柱由适配器自身在
  小模块 `core/calendar/year_boundary.rs` 中重新计算，以便「立春」分界可达时刻级
  （`lunar-lite` 的 `YearDivide::Exact` 出于上游兼容仍为日期级）。
- 紫微斗数排盘策略保留在 `iztro-rs`（`ChartCalculationConfig`、`SolarTimePolicy`、
  `ApparentSolarTimeConfig`、`YearBoundary`、`LeapMonthBoundary`、`NominalAgeBoundary`）。
  月柱使用由有效年干推出的正常五虎遁。真太阳时作为 `iztro-rs` 策略在历法转换**之前**
  应用；历法接收已解析的本地日期、时辰索引以及钟表时分。
- `YearBoundary::LiChun` 为**时刻级**，由 `lunar_lite::li_chun_datetime` 驱动：将已解析的
  出生时刻与该公历年精确的立春时刻比较。钟表时间 API 通过
  `solar_to_lunar_with_resolved_datetime` 保留已解析的时分并按精确时刻比较；旧式
  `BirthTime` / `timeIndex` API 不携带钟表分钟，故按 `lunar-lite` 对该时辰的代表性中点
  比较（`hour = max(timeIndex * 2 - 1, 0)`、`minute = 30`）。
  `YearBoundary::ChineseNewYearEve` 使用农历新年分界。

## 影响

- 运行时历法事实来自 `lunar-lite`。
- 公开/领域 API 不变（`core` 仍 re-export 相同的干支类型），但它们由 `lunar-lite` 拥有，
  不再复制。
- `YearBoundary::LiChun` 保持时刻级。对于立春当日、立春精确时刻之前出生者，这有意偏离
  `iztro@2.5.8`（日期级）。唯一受影响的 supported-field fixture 用例
  （`year_divide_exact_2000_02_04`，即 2000-02-04 08:00 出生，早于 20:40:24 的立春时刻）
  保持修正后的 `iztro-rs` 结果 `己卯`，并标注为有意偏离。
- 精确瞬时的「立春」基元现由 `lunar-lite`（`li_chun_datetime`）提供，而非第二个历法依赖。
- `lunar-lite` 的 `ChildLimit`/起运与完整八字解读仍待单独兼容性分析后再定，暂缓。
