//! Serializable calculation-resolution diagnostics.
//!
//! These snapshots report how input/runtime calculation policies were resolved
//! without becoming part of the immutable [`Chart`](crate::core::model::chart::Chart)
//! serde shape.

use crate::core::calculation::{
    EquationOfTimePolicy, LeapMonthBoundary, NominalAgeBoundary, YearBoundary,
};
use crate::core::model::chart::{HoroscopeLunarDate, HoroscopeSolarDate};
use crate::core::model::ganzhi::{EarthlyBranch, StemBranch};
use serde::{Deserialize, Serialize};

/// Calculation facts resolved while generating a natal chart.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartCalculationDiagnosticSnapshot {
    /// Birth clock-time resolution, including apparent-solar-time corrections.
    pub birth_time: BirthTimeResolutionSnapshot,
    /// Effective birth-year resolution under the selected 年分界 policy.
    pub year_boundary: YearBoundaryDiagnosticSnapshot,
    /// Leap-month policy mapping and input leap-state facts.
    pub leap_month_boundary: LeapMonthBoundaryDiagnosticSnapshot,
}

/// Birth input calendar kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BirthInputCalendarKind {
    /// Gregorian/solar birth-date input.
    Solar,
    /// Chinese-lunisolar birth-date input.
    Lunar,
}

/// Birth clock-time resolution facts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BirthTimeResolutionSnapshot {
    /// Input calendar kind.
    pub input_calendar: BirthInputCalendarKind,
    /// Input date formatted as `YYYY-MM-DD`.
    pub input_date: String,
    /// Input clock time formatted as `HH:MM`.
    pub input_clock_time: String,
    /// Civil UTC offset in minutes east of UTC.
    pub timezone_offset_minutes: i32,
    /// Solar-time calculation policy resolved for this input.
    pub solar_time_policy: SolarTimePolicyDiagnostic,
    /// Birth-place longitude in degrees east, when apparent solar time is used.
    pub longitude_degrees: Option<f64>,
    /// Applied longitude correction in minutes, when apparent solar time is used.
    pub longitude_correction_minutes: Option<f64>,
    /// Applied equation-of-time minutes, when configured.
    pub equation_of_time_minutes: Option<f64>,
    /// Total clock-time adjustment in minutes.
    pub total_adjustment_minutes: f64,
    /// Resolved solar date formatted as `YYYY-MM-DD`; absent for lunar input.
    pub resolved_solar_date: Option<String>,
    /// Resolved clock time formatted as `HH:MM`.
    pub resolved_clock_time: String,
    /// Resolved upstream `iztro` time index (`0..=12`).
    pub resolved_time_index: u8,
    /// Resolved 时辰 branch.
    pub resolved_time_branch: EarthlyBranch,
}

/// Diagnostic form of the solar-time policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SolarTimePolicyDiagnostic {
    /// Clock time was used directly.
    ClockTime,
    /// Apparent solar time was used.
    ApparentSolarTime {
        /// Birth-place longitude in degrees east.
        longitude_degrees: f64,
        /// Equation-of-time policy applied with the longitude correction.
        equation_of_time: EquationOfTimePolicy,
    },
}

/// Effective birth-year facts under the 年分界 policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YearBoundaryDiagnosticSnapshot {
    /// Selected 年分界 policy.
    pub policy: YearBoundary,
    /// Effective cyclic birth-year stem/branch used by chart generation.
    pub effective_birth_year: StemBranch,
}

/// Leap-month policy facts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeapMonthBoundaryDiagnosticSnapshot {
    /// Selected 闰月分界 policy.
    pub policy: LeapMonthBoundary,
    /// Legacy `fix_leap` flag passed into the existing chart-generation path.
    pub legacy_fix_leap: bool,
    /// Whether the normalized input month is a leap lunar month.
    pub input_is_leap_month: bool,
}

/// Runtime/horoscope calculation facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeCalculationDiagnosticSnapshot {
    /// Target Gregorian/solar date.
    pub target_solar_date: HoroscopeSolarDate,
    /// Target Chinese-lunisolar date.
    pub target_lunar_date: HoroscopeLunarDate,
    /// Selected 虚岁分界 policy.
    pub nominal_age_boundary: NominalAgeBoundary,
    /// Resolved nominal age used for decadal and nominal-age layers.
    pub resolved_nominal_age: u8,
}
