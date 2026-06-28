# ADR 0006: Calendar Engine — `tyme4rs` Behind an Internal Adapter

## Status

Superseded by [ADR 0007](0007-calendar-engine-back-to-lunar-lite.md).

`iztro-rs` has migrated the calendar engine back to `lunar-lite`. The
decision below — adopting `tyme4rs` and duplicating the GanZhi value
objects in `core/model/ganzhi` — no longer reflects the codebase. In
particular, the datetime-level (exact-instant) `立春` boundary and its
intentional `year_divide_exact_2000_02_04` divergence have been reverted to
`lunar-lite`'s date-level boundary, which matches `iztro@2.5.8`. This record
is kept for history; see ADR 0007 for the current decision.

## Context

`iztro-rs` previously used `lunar-lite-rs` as its runtime Chinese-calendar
engine for solar-to-lunar conversion, leap-month handling, and four-pillar
(干支) derivation. Two problems motivated a change:

- `tyme4rs` is a broader and more mature calendar library (solar terms, lunar
  calendar, sexagenary cycle, Julian day) with exact-instant solar-term data.
- `lunar-lite-rs` resolved the `立春` (LiChun) year boundary at **date**
  granularity, which is semantically wrong around the exact LiChun instant: a
  birth before the LiChun moment on the LiChun day should still belong to the
  previous Ganzhi year.

In addition, `lunar-lite-rs` owned the low-level `HeavenlyStem`,
`EarthlyBranch`, `StemBranch`, and `FourPillars` primitives, which were
re-exported directly from `iztro-rs`'s public API, leaking a third-party type
through the public/domain surface.

Fixture parity with `iztro@2.5.8` remains the first compatibility target.

## Decision

- Replace `lunar-lite-rs` as the runtime calendar engine with `tyme4rs`
  (`= 1.5.0`).
- Keep `tyme4rs` behind an internal `core/calendar` adapter. `core/calendar/tyme.rs`
  is the only module permitted to depend on `tyme4rs`; every `tyme4rs` value is
  converted into an `iztro-rs`-owned type at that boundary.
- Production source code depends on `tyme4rs` only from `core/calendar/tyme.rs`.
  Integration tests must not import `tyme4rs` directly; they should use committed
  fixture facts or `iztro-rs` APIs that cross the internal adapter boundary.
- `iztro-rs` owns the public/domain stem, branch, stem-branch, and four-pillar
  value objects (`core/model/ganzhi`). `tyme4rs` types never appear in
  public/domain APIs.
- Keep Zi Wei Dou Shu chart-calculation policy in `iztro-rs`
  (`ChartCalculationConfig`, `SolarTimePolicy`, `ApparentSolarTimeConfig`,
  `YearBoundary`, `LeapMonthBoundary`, `NominalAgeBoundary`). Apparent solar time
  is applied as `iztro-rs` policy *before* a `tyme4rs::SolarTime` is built.
- The year pillar (lunar-new-year / LiChun boundary) and month pillar (五虎遁)
  are derived in `core/calendar/policy.rs`, not in `tyme4rs`; the unambiguous day
  and hour pillars (continuous day count and 五鼠遁 hour, including the 晚子时 day
  roll) come from `tyme4rs`.
- `YearBoundary::LiChun` is **datetime-level**: the resolved birth instant is
  compared against the exact `立春` instant. Legacy `BirthTime` / `timeIndex`
  APIs compare using the representative synthesized midpoint for the supplied
  时辰, because they do not carry clock minutes. Clock-time APIs preserve the
  resolved hour/minute and compare that exact resolved instant against LiChun.

## Consequences

- Runtime calendar facts come from `tyme4rs`.
- Public/domain APIs keep `iztro-rs`-owned types; no `tyme4rs` leak.
- Adapter-boundary tests may exercise `tyme4rs` through `core/calendar/tyme.rs`,
  but integration tests stay adapter-agnostic.
- The LiChun boundary can be datetime-level. This intentionally diverges from
  `iztro@2.5.8` (date-level) for births before the exact `立春` instant on the
  `立春` day. The single affected supported-field fixture case
  (`year_divide_exact_2000_02_04`) is updated to the corrected `iztro-rs` result
  (`己卯`) and annotated as an intentional divergence; all other cases keep strict
  upstream parity. The fixture-priority order remains:
  1. `iztro@2.5.8` supported-field fixture parity;
  2. correct calendar semantics from `tyme4rs`;
  3. old `lunar-lite-rs` behavior.
- `tyme4rs::tyme::eightchar::ChildLimit` (起运) adoption and full BaZi
  interpretation remain deferred pending separate compatibility analysis.
