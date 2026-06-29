# ADR 0007: Calendar Engine — Back to `lunar-lite`

## Status

Accepted. Supersedes [ADR 0006](0006-calendar-engine-tyme4rs-adapter.md).

## Context

[ADR 0006](0006-calendar-engine-tyme4rs-adapter.md) replaced `lunar-lite`
with `tyme4rs` to obtain an exact-instant `立春` (LiChun) boundary and richer
solar-term data, and duplicated the low-level GanZhi value objects into
`core/model/ganzhi`.

Two things changed since then:

- `lunar-lite` (1.2.1) now ships a tyme-compatible astronomical backend, a
  broad oracle test suite, and a public datetime-level `立春` primitive
  (`lunar_lite::li_chun_datetime(year) -> SolarTermDateTime`). It is a faithful
  lower-level calendar/GanZhi crate again, and the exact-instant boundary no
  longer requires a second calendar dependency.
- Maintaining a duplicated GanZhi model and a second calendar dependency added
  cost without a net benefit.

## Decision

- Depend on `lunar-lite` (`= 1.2.1`, `serde` feature) as the canonical
  lower-level calendar/GanZhi crate. Remove the `tyme4rs` dependency.
- Delete the duplicated `core/model/ganzhi.rs`. Use
  `lunar_lite::{HeavenlyStem, EarthlyBranch, StemBranch, FourPillars,
  StemBranchError, HEAVENLY_STEMS, EARTHLY_BRANCHES}` directly; `core`
  re-exports them so the public API is unchanged. `FourPillars` is the
  `lunar-lite` value object, not an `iztro-rs`-owned type.
- Keep `lunar-lite` behind the internal `core/calendar` adapter. Solar/lunar
  conversion, lunar-date normalization, and lunar month-day counts are
  delegated to `lunar-lite` (`solar_to_lunar`, `lunar_to_solar`,
  `normalize_lunar_date`, `lunar_month_days`). For the four pillars the adapter
  takes **only** the day and hour pillars from
  `four_pillars_from_solar_date_with_options`; it recomputes the year and month
  pillars itself in a small `core/calendar/year_boundary.rs` module so that the
  `立春` boundary can be datetime-level (`lunar-lite`'s `YearDivide::Exact`
  remains date-level for upstream compatibility).
- Keep Zi Wei Dou Shu chart-calculation policy in `iztro-rs`
  (`ChartCalculationConfig`, `SolarTimePolicy`, `ApparentSolarTimeConfig`,
  `YearBoundary`, `LeapMonthBoundary`, `NominalAgeBoundary`). The month pillar
  uses the normal 五虎遁 derived from the effective year stem. Apparent solar
  time is applied as `iztro-rs` policy *before* the calendar conversion runs;
  the calendar receives the already-resolved local date, 时辰 index, and clock
  hour/minute.
- `YearBoundary::LiChun` is **datetime-level**, powered by
  `lunar_lite::li_chun_datetime`: the resolved birth instant is compared
  against the exact `立春` instant for that Gregorian year. Clock-time APIs
  preserve the resolved hour/minute through
  `solar_to_lunar_with_resolved_datetime` and compare that exact instant.
  Legacy `BirthTime` / `timeIndex` APIs carry no clock minutes, so they compare
  using the representative `lunar-lite` midpoint for the supplied 时辰
  (`hour = max(timeIndex * 2 - 1, 0)`, `minute = 30`).
  `YearBoundary::ChineseNewYearEve` uses the lunar-new-year boundary.

## Consequences

- Runtime calendar facts come from `lunar-lite`.
- The public/domain API is unchanged (`core` re-exports the same GanZhi
  types), but they are now owned by `lunar-lite` rather than duplicated.
- `YearBoundary::LiChun` stays datetime-level. This intentionally diverges from
  `iztro@2.5.8` (date-level) for births before the exact `立春` instant on the
  `立春` day. The single affected supported-field fixture case
  (`year_divide_exact_2000_02_04`, a birth at 08:00 on 2000-02-04, before the
  20:40:24 instant) keeps the corrected `iztro-rs` result (`己卯`) and is
  annotated as an intentional divergence.
- The exact-instant `立春` primitive is now sourced from `lunar-lite`
  (`li_chun_datetime`) rather than from a second calendar dependency.
- `lunar-lite`'s `ChildLimit`/起运 and full BaZi interpretation remain
  deferred pending separate compatibility analysis.
