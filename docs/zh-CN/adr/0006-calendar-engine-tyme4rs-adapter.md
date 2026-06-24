# ADR 0006：历法引擎——`tyme4rs` 置于内部适配器之后

## 状态

已接受。

## 背景

`iztro-rs` 此前使用 `lunar-lite-rs` 作为运行时中国历法引擎，负责公历转农历、闰月处理和四柱（干支）推导。有两点促成了变更：

- `tyme4rs` 是更广泛、更成熟的历法库（节气、农历、六十甲子、儒略日），并具备精确到瞬时的节气数据。
- `lunar-lite-rs` 在**日期**粒度上判定「立春」年分界，这在精确立春时刻附近语义错误：立春当日、立春时刻之前出生，仍应属于上一个干支年。

此外，`lunar-lite-rs` 拥有底层 `HeavenlyStem`、`EarthlyBranch`、`StemBranch`、`FourPillars` 基础类型，并被 `iztro-rs` 公开 API 直接 re-export，使第三方类型泄漏到公开/领域层。

与 `iztro@2.5.8` 的 fixture 一致性仍是首要兼容目标。

## 决策

- 用 `tyme4rs`（`= 1.5.0`）替换 `lunar-lite-rs` 作为运行时历法引擎。
- 将 `tyme4rs` 置于内部 `core/calendar` 适配器之后。`core/calendar/tyme.rs` 是唯一允许依赖 `tyme4rs` 的模块；每个 `tyme4rs` 值都在该边界转换为 `iztro-rs` 自有类型。
- 生产源码只允许 `core/calendar/tyme.rs` 直接依赖 `tyme4rs`。集成测试不得直接 import `tyme4rs`；应使用已提交 fixture facts，或通过 `iztro-rs` API 穿过内部适配器边界。
- `iztro-rs` 拥有公开/领域层的天干、地支、干支与四柱值对象（`core/model/ganzhi`）。`tyme4rs` 类型绝不出现在公开/领域 API 中。
- 紫微斗数排盘策略保留在 `iztro-rs`（`ChartCalculationConfig`、`SolarTimePolicy`、`ApparentSolarTimeConfig`、`YearBoundary`、`LeapMonthBoundary`、`NominalAgeBoundary`）。真太阳时作为 `iztro-rs` 策略在构造 `tyme4rs::SolarTime` **之前**应用。
- 年柱（农历新年 / 立春分界）与月柱（五虎遁）在 `core/calendar/policy.rs` 推导，而非在 `tyme4rs` 中；无歧义的日柱与时柱（连续日数、五鼠遁时柱，含晚子时换日）来自 `tyme4rs`。
- `YearBoundary::LiChun` 为**时刻级**：以解析后的出生时刻与精确「立春」时刻比较。旧 `BirthTime` / `timeIndex` API 不携带钟表分钟，因此使用该时辰的代表性合成中点比较；钟表时间 API 保留解析后的 hour/minute，并以该精确解析时刻比较立春。

## 影响

- 运行时历法事实来自 `tyme4rs`。
- 公开/领域 API 保留 `iztro-rs` 自有类型；不泄漏 `tyme4rs`。
- 适配器边界测试可经由 `core/calendar/tyme.rs` 覆盖 `tyme4rs` 行为；集成测试保持适配器无关。
- 立春分界可达时刻级。对立春当日、立春时刻之前出生的情形，这有意偏离 `iztro@2.5.8`（日期级）。唯一受影响的 supported-field fixture 用例（`year_divide_exact_2000_02_04`）已更新为修正后的 `iztro-rs` 结果（`己卯`），并标注为有意偏离；其余用例保持与上游严格一致。Fixture 优先级顺序不变：
  1. `iztro@2.5.8` supported-field fixture 一致性；
  2. 来自 `tyme4rs` 的正确历法语义；
  3. 旧的 `lunar-lite-rs` 行为。
- `tyme4rs::tyme::eightchar::ChildLimit`（起运）采用与完整八字解读仍待单独兼容性分析后再定，暂缓。
