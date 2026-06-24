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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
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

/// The input calculation policy applied before chart generation.
///
/// This is a separate axis from the algorithm family and the chart plane. With
/// the default policy, the clock-time API derives the 时辰 from the supplied
/// clock time and produces the same chart as the legacy time-index API for the
/// same 时辰.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChartCalculationConfig {
    /// Policy controlling how birth clock time becomes a 时辰.
    pub solar_time: SolarTimePolicy,
}

impl ChartCalculationConfig {
    /// Creates a calculation config from an explicit solar-time policy.
    pub const fn new(solar_time: SolarTimePolicy) -> Self {
        Self { solar_time }
    }

    /// Creates the default clock-time calculation config.
    pub const fn clock_time() -> Self {
        Self {
            solar_time: SolarTimePolicy::ClockTime,
        }
    }

    /// Creates an apparent-solar-time calculation config.
    pub const fn apparent_solar_time(config: ApparentSolarTimeConfig) -> Self {
        Self {
            solar_time: SolarTimePolicy::ApparentSolarTime(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longitude_accepts_in_range() {
        assert_eq!(Longitude::new(120.0).expect("valid").degrees(), 120.0);
        assert_eq!(Longitude::new(-180.0).expect("valid").degrees(), -180.0);
        assert_eq!(Longitude::new(180.0).expect("valid").degrees(), 180.0);
    }

    #[test]
    fn longitude_rejects_out_of_range() {
        assert_eq!(
            Longitude::new(180.5),
            Err(ChartError::InvalidLongitude { value: 180.5 }),
        );
        assert_eq!(
            Longitude::new(-181.0),
            Err(ChartError::InvalidLongitude { value: -181.0 }),
        );
    }

    #[test]
    fn utc_offset_accepts_real_world_range() {
        assert_eq!(UtcOffset::from_hours(8).expect("valid").minutes(), 480);
        assert_eq!(UtcOffset::from_hours(-12).expect("valid").minutes(), -720);
        assert_eq!(UtcOffset::from_hours(14).expect("valid").minutes(), 840);
    }

    #[test]
    fn utc_offset_rejects_out_of_range() {
        assert_eq!(
            UtcOffset::from_minutes(841),
            Err(ChartError::InvalidUtcOffset { minutes: 841 }),
        );
        assert_eq!(
            UtcOffset::from_hours(-13),
            Err(ChartError::InvalidUtcOffset { minutes: -780 }),
        );
    }

    #[test]
    fn utc_offset_meridian_is_offset_hours_times_fifteen() {
        assert_eq!(
            UtcOffset::from_hours(8).expect("valid").meridian_degrees(),
            120.0
        );
        assert_eq!(
            UtcOffset::from_hours(0).expect("valid").meridian_degrees(),
            0.0
        );
    }

    #[test]
    fn clock_birth_time_accepts_valid_time() {
        let tz = UtcOffset::from_hours(8).expect("valid offset");
        let clock = ClockBirthTime::new(1, 5, tz).expect("valid clock time");
        assert_eq!(clock.hour(), 1);
        assert_eq!(clock.minute(), 5);
        assert_eq!(clock.minutes_since_midnight(), 65);
    }

    #[test]
    fn clock_birth_time_rejects_invalid_time() {
        let tz = UtcOffset::from_hours(8).expect("valid offset");
        assert_eq!(
            ClockBirthTime::new(24, 0, tz),
            Err(ChartError::InvalidClockTime {
                hour: 24,
                minute: 0,
            }),
        );
        assert_eq!(
            ClockBirthTime::new(0, 60, tz),
            Err(ChartError::InvalidClockTime {
                hour: 0,
                minute: 60,
            }),
        );
    }

    #[test]
    fn calculation_config_defaults_to_clock_time() {
        assert_eq!(
            ChartCalculationConfig::default().solar_time,
            SolarTimePolicy::ClockTime,
        );
    }
}
