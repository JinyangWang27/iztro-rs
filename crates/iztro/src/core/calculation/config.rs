//! Input calculation policy domain types.
//!
//! These types model *how birth clock time is interpreted* before a chart is
//! generated. They are a separate axis from
//! [`ChartAlgorithmKind`](crate::core::model::profile::ChartAlgorithmKind) (the
//! algorithm family) and [`ChartPlane`](crate::core::model::profile::ChartPlane)
//! (the plane variant). Apparent solar time is an input calculation policy: it
//! normalises the birth clock time; it does not define a new algorithm and does
//! not define a new chart plane.

use crate::core::error::ChartError;
use serde::{Deserialize, Serialize};

/// Inclusive minute bound for the most western real-world UTC offset (`-12:00`).
const MIN_UTC_OFFSET_MINUTES: i32 = -12 * 60;
/// Inclusive minute bound for the most eastern real-world UTC offset (`+14:00`).
const MAX_UTC_OFFSET_MINUTES: i32 = 14 * 60;

/// A validated geographic longitude in degrees, east-positive.
///
/// Valid range is `-180.0..=180.0`. West longitudes are negative.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Longitude {
    degrees: f64,
}

impl Longitude {
    /// Creates a validated longitude from degrees (east-positive).
    pub fn new(degrees: f64) -> Result<Self, ChartError> {
        if degrees.is_nan() || !(-180.0..=180.0).contains(&degrees) {
            return Err(ChartError::InvalidLongitude { value: degrees });
        }
        Ok(Self { degrees })
    }

    /// Returns the longitude in degrees (east-positive).
    pub const fn degrees(self) -> f64 {
        self.degrees
    }
}

/// A validated UTC offset stored as whole minutes east of UTC.
///
/// Valid range is `-720..=840` minutes (`-12:00..=+14:00`), covering every
/// real-world civil time zone.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct UtcOffset {
    minutes: i32,
}

impl UtcOffset {
    /// Creates a validated UTC offset from whole minutes east of UTC.
    pub const fn from_minutes(minutes: i32) -> Result<Self, ChartError> {
        if minutes < MIN_UTC_OFFSET_MINUTES || minutes > MAX_UTC_OFFSET_MINUTES {
            return Err(ChartError::InvalidUtcOffset { minutes });
        }
        Ok(Self { minutes })
    }

    /// Creates a validated UTC offset from whole hours east of UTC.
    pub const fn from_hours(hours: i32) -> Result<Self, ChartError> {
        Self::from_minutes(hours * 60)
    }

    /// Returns the offset in whole minutes east of UTC.
    pub const fn minutes(self) -> i32 {
        self.minutes
    }

    /// Returns the offset in hours east of UTC as a real number.
    pub fn hours(self) -> f64 {
        f64::from(self.minutes) / 60.0
    }

    /// Returns the central meridian of this offset in degrees east.
    ///
    /// This is `utc_offset_hours * 15`, the longitude where civil clock time and
    /// apparent solar time coincide for the offset.
    pub fn meridian_degrees(self) -> f64 {
        self.hours() * 15.0
    }
}

/// A wall-clock birth time with its civil UTC offset.
///
/// The user always supplies the birth *clock* time. If apparent solar time is
/// disabled, the 时辰 (time branch) is derived directly from this clock time. If
/// apparent solar time is enabled, the clock time is adjusted first using the
/// time zone and longitude, then the 时辰 is derived.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ClockBirthTime {
    hour: u8,
    minute: u8,
    timezone: UtcOffset,
}

impl ClockBirthTime {
    /// Creates a validated wall-clock birth time.
    ///
    /// `hour` must be `0..=23` and `minute` must be `0..=59`.
    pub const fn new(hour: u8, minute: u8, timezone: UtcOffset) -> Result<Self, ChartError> {
        if hour > 23 || minute > 59 {
            return Err(ChartError::InvalidClockTime { hour, minute });
        }
        Ok(Self {
            hour,
            minute,
            timezone,
        })
    }

    /// Returns the clock hour (`0..=23`).
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// Returns the clock minute (`0..=59`).
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// Returns the civil UTC offset for the clock time.
    pub const fn timezone(self) -> UtcOffset {
        self.timezone
    }

    /// Returns the clock time as minutes since midnight (`0..=1439`).
    pub const fn minutes_since_midnight(self) -> i32 {
        self.hour as i32 * 60 + self.minute as i32
    }
}

/// Equation-of-time policy for apparent solar time.
///
/// The equation of time is the difference between apparent solar time and mean
/// solar time. It is independent of longitude.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EquationOfTimePolicy {
    /// Equation-of-time minutes are treated as zero. Only the exact longitude
    /// correction is applied. This is the supported default.
    #[default]
    Disabled,
    /// A deterministic equation-of-time approximation. Not implemented yet;
    /// resolving with this policy returns
    /// [`ChartError::UnsupportedEquationOfTimePolicy`].
    Approximate,
}

/// Configuration for the apparent-solar-time calculation policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApparentSolarTimeConfig {
    /// Birth-place longitude used for the exact longitude correction.
    pub longitude: Longitude,
    /// Equation-of-time policy applied on top of the longitude correction.
    pub equation_of_time: EquationOfTimePolicy,
}

impl ApparentSolarTimeConfig {
    /// Creates an apparent-solar-time configuration.
    pub const fn new(longitude: Longitude, equation_of_time: EquationOfTimePolicy) -> Self {
        Self {
            longitude,
            equation_of_time,
        }
    }
}

/// How birth clock time is interpreted when deriving the 时辰.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SolarTimePolicy {
    /// Derive the 时辰 directly from the supplied clock time. This is the
    /// default and reproduces existing chart-generation behaviour.
    #[default]
    ClockTime,
    /// Adjust the clock time to apparent solar time (using time zone and
    /// longitude) before deriving the 时辰.
    ApparentSolarTime(ApparentSolarTimeConfig),
}

/// 年分界: the effective astrological year boundary (`yearDivide`).
///
/// This mirrors upstream TS `iztro@2.5.8` `yearDivide`. It selects which boundary
/// separates one cyclic birth year (and year pillar) from the next. It is an
/// input calculation policy for a supported field; it does not define a new
/// algorithm or chart plane.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum YearBoundary {
    /// 年分界：按除夕. The cyclic year changes at the lunar new year (正月初一),
    /// upstream `yearDivide: 'normal'`. This is the default and preserves
    /// existing iztro-rs behaviour.
    #[default]
    ChineseNewYearEve,

    /// 年分界：按立春. The cyclic year changes at 立春 (LiChun), upstream
    /// `yearDivide: 'exact'`. `iztro-rs` resolves this at **datetime**
    /// granularity via `lunar_lite::li_chun_datetime`: a birth before the exact
    /// 立春 instant on the 立春 day keeps the previous Ganzhi year, while a birth
    /// at or after it advances. This intentionally diverges from upstream
    /// `iztro@2.5.8`, which is date-level.
    LiChun,
}

/// 闰月分界: how a leap month (闰月) is attributed to a numeric month (`fixLeap`).
///
/// This mirrors upstream TS `iztro@2.5.8` `fixLeap`. It controls whether the
/// second half of a leap month advances month-based placement to the next month.
/// It is an input calculation policy for a supported field; it does not define a
/// new algorithm or chart plane.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeapMonthBoundary {
    /// 闰月分界：算上月. The whole leap month is treated as its own numeric
    /// month; the second half is not advanced. Equivalent to upstream
    /// `fixLeap: false`.
    AsPreviousMonth,

    /// 闰月分界：月中分界. The leap month splits at the 15th: day `<= 15` stays in
    /// the month, day `>= 16` advances to the next month. Equivalent to upstream
    /// `fixLeap: true`. This is the default and preserves existing iztro-rs
    /// behaviour.
    #[default]
    MidMonth,
}

/// 虚岁分界: when the nominal age (虚岁) increments (`ageDivide`).
///
/// This mirrors upstream TS `iztro@2.5.8` `ageDivide`. It is a runtime/horoscope
/// calculation policy: it affects nominal-age resolution for 小限 and decadal
/// selection only, never natal chart generation. It does not define a new
/// algorithm or chart plane.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NominalAgeBoundary {
    /// 虚岁分界：按自然年. The nominal age increments at the natural-year boundary
    /// (the lunar new year), upstream `ageDivide: 'normal'`. This is the default
    /// and preserves existing iztro-rs runtime behaviour.
    #[default]
    NaturalYear,

    /// 虚岁分界：按生日. The nominal age increments at the (lunar) birthday,
    /// upstream `ageDivide: 'birthday'`.
    Birthday,
}

/// The input calculation policy applied before chart generation.
///
/// This is a separate axis from the algorithm family and the chart plane. With
/// the default policy, the clock-time API derives the 时辰 from the supplied
/// clock time and produces the same chart as the legacy time-index API for the
/// same 时辰.
///
/// [`year_boundary`](Self::year_boundary) and
/// [`leap_month_boundary`](Self::leap_month_boundary) affect natal chart
/// generation; [`nominal_age_boundary`](Self::nominal_age_boundary) affects
/// runtime/horoscope nominal-age resolution only. All default to the values that
/// preserve existing iztro-rs behaviour and fixtures.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChartCalculationConfig {
    /// Policy controlling how birth clock time becomes a 时辰.
    pub solar_time: SolarTimePolicy,
    /// 年分界 policy controlling the effective cyclic birth year.
    pub year_boundary: YearBoundary,
    /// 闰月分界 policy controlling leap-month month attribution.
    pub leap_month_boundary: LeapMonthBoundary,
    /// 虚岁分界 policy controlling runtime nominal-age increments.
    pub nominal_age_boundary: NominalAgeBoundary,
}

impl ChartCalculationConfig {
    /// Creates a calculation config from an explicit solar-time policy.
    ///
    /// The boundary policies use their defaults
    /// ([`YearBoundary::ChineseNewYearEve`], [`LeapMonthBoundary::MidMonth`],
    /// [`NominalAgeBoundary::NaturalYear`]).
    pub const fn new(solar_time: SolarTimePolicy) -> Self {
        Self {
            solar_time,
            year_boundary: YearBoundary::ChineseNewYearEve,
            leap_month_boundary: LeapMonthBoundary::MidMonth,
            nominal_age_boundary: NominalAgeBoundary::NaturalYear,
        }
    }

    /// Creates the default clock-time calculation config.
    pub const fn clock_time() -> Self {
        Self::new(SolarTimePolicy::ClockTime)
    }

    /// Creates an apparent-solar-time calculation config.
    pub const fn apparent_solar_time(config: ApparentSolarTimeConfig) -> Self {
        Self::new(SolarTimePolicy::ApparentSolarTime(config))
    }

    /// Returns a copy with the 年分界 policy replaced.
    pub const fn with_year_boundary(mut self, year_boundary: YearBoundary) -> Self {
        self.year_boundary = year_boundary;
        self
    }

    /// Returns a copy with the 闰月分界 policy replaced.
    pub const fn with_leap_month_boundary(
        mut self,
        leap_month_boundary: LeapMonthBoundary,
    ) -> Self {
        self.leap_month_boundary = leap_month_boundary;
        self
    }

    /// Returns a copy with the 虚岁分界 policy replaced.
    pub const fn with_nominal_age_boundary(
        mut self,
        nominal_age_boundary: NominalAgeBoundary,
    ) -> Self {
        self.nominal_age_boundary = nominal_age_boundary;
        self
    }
}
