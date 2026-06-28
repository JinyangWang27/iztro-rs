# ADR 0007: Calendar Engine — Back to `lunar-lite`

## Status

Accepted. Supersedes [ADR 0006](0006-calendar-engine-tyme4rs-adapter.md).

## Context

[ADR 0006](0006-calendar-engine-tyme4rs-adapter.md) replaced `lunar-lite`
with `tyme4rs` to obtain an exact-instant `立春` (LiChun) boundary and richer
solar-term data, and duplicated the low-level GanZhi value objects into
`core/model/ganzhi`.

Two things changed since then:

- `lunar-lite` (1.1.0) now ships a tyme-compatible astronomical backend and a
  broad oracle test suite, so it is a faithful lower-level calendar/GanZhi
  crate again.
- The `tyme4rs`-only exact-instant `立春` boundary was the single source of a
  documented divergence from `iztro@2.5.8` (`year_divide_exact_2000_02_04` →
  `己卯` instead of upstream `庚辰`). Restoring `lunar-lite`'s date-level
  boundary removes that divergence.

Maintaining a duplicated GanZhi model and a second calendar dependency added
cost without a net compatibility benefit.

## Decision

- Depend on `lunar-lite` (`= 1.1.0`, `serde` feature) as the canonical
  lower-level calendar/GanZhi crate. Remove the `tyme4rs` dependency.
- Delete the duplicated `core/model/ganzhi.rs`. Use
  `lunar_lite::{HeavenlyStem, EarthlyBranch, StemBranch, FourPillars,
  StemBranchError, HEAVENLY_STEMS, EARTHLY_BRANCHES}` directly; `core`
  re-exports them so the public API is unchanged.
- Keep `lunar-lite` behind the internal `core/calendar` adapter. Solar/lunar
  conversion, lunar-date normalization, lunar month-day counts, and the four
  pillars are delegated to `lunar-lite`
  (`solar_to_lunar`, `lunar_to_solar`, `normalize_lunar_date`,
  `lunar_month_days`, `four_pillars_from_solar_date_with_options`). The
  adapter retired the separate `tyme.rs` engine wrapper and the
  `policy.rs` year/month-pillar module: `lunar-lite`'s four-pillar API derives
  all four pillars from `StemBranchOptions { year: YearDivide, month:
  MonthDivide }`.
- Keep Zi Wei Dou Shu chart-calculation policy in `iztro-rs`
  (`ChartCalculationConfig`, `SolarTimePolicy`, `ApparentSolarTimeConfig`,
  `YearBoundary`, `LeapMonthBoundary`, `NominalAgeBoundary`). `YearBoundary`
  maps to `lunar-lite`'s `YearDivide` (`ChineseNewYearEve → Normal`,
  `LiChun → Exact`); the month uses `MonthDivide::Normal` (五虎遁). Apparent
  solar time is applied as `iztro-rs` policy *before* the calendar conversion
  runs; the calendar receives the already-resolved local date and 时辰 index.
- `YearBoundary::LiChun` is **date-level**, matching `lunar-lite`'s
  `YearDivide::Exact` and `iztro@2.5.8`: the `立春` day belongs to the new
  Ganzhi year regardless of clock time. The exact-instant clock path
  (`solar_to_lunar_with_resolved_datetime`) is removed.

## Consequences

- Runtime calendar facts come from `lunar-lite`.
- The public/domain API is unchanged (`core` re-exports the same GanZhi
  types), but they are now owned by `lunar-lite` rather than duplicated.
- The `year_divide_exact_2000_02_04` supported-field fixture again matches
  upstream `庚辰`; the intentional-divergence annotation is removed and every
  fixture case keeps strict upstream parity.
- A datetime-level (exact-instant) `立春` boundary is no longer available. If
  it is needed later, the right path is to have `lunar-lite` expose a
  solar-term-instant primitive and to add an exact-instant `YearBoundary`
  variant on top of it, rather than reintroducing a second calendar
  dependency. This is deferred.
- `lunar-lite`'s `ChildLimit`/起运 and full BaZi interpretation remain
  deferred pending separate compatibility analysis.
