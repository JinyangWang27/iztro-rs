//! Apparent-solar-time resolver.
//!
//! This resolves a raw civil birth date plus wall-clock time into a resolved
//! local solar date/time and 时辰, applying the configured input calculation
//! policy. It runs *before* chart generation and never touches the chart
//! algorithm, the chart plane, the natal anchor, or any star placer.
//!
//! ```text
//! raw birth date + civil clock time
//!   -> optional apparent solar time adjustment
//!   -> resolved local date/time
//!   -> derive time branch / time index
//!   -> existing natal chart generation
//! ```

use crate::core::error::ChartError;
use crate::core::facade::static_temporal_chart_view::time_index_for_hour;
use crate::core::model::calendar::{BirthTime, SolarDate};
use lunar_lite::EarthlyBranch;

use super::config::{
    ChartCalculationConfig, ClockBirthTime, EquationOfTimePolicy, SolarTimePolicy,
};

/// The resolved local birth date/time after applying the calculation policy.
///
/// All adjustment measurements are reported for diagnostics. They are `None`
/// under [`SolarTimePolicy::ClockTime`], where no adjustment is applied.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedBirthDateTime {
    input_date: SolarDate,
    input_time: ClockBirthTime,
    resolved_date: SolarDate,
    resolved_hour: u8,
    resolved_minute: u8,
    resolved_time_index: u8,
    resolved_time_branch: EarthlyBranch,
    longitude_correction_minutes: Option<f64>,
    equation_of_time_minutes: Option<f64>,
    total_adjustment_minutes: f64,
}

impl ResolvedBirthDateTime {
    /// Returns the original input date.
    pub const fn input_date(&self) -> SolarDate {
        self.input_date
    }

    /// Returns the original input clock time.
    pub const fn input_time(&self) -> ClockBirthTime {
        self.input_time
    }

    /// Returns the resolved local solar date.
    pub const fn resolved_date(&self) -> SolarDate {
        self.resolved_date
    }

    /// Returns the resolved local hour (`0..=23`).
    pub const fn resolved_hour(&self) -> u8 {
        self.resolved_hour
    }

    /// Returns the resolved local minute (`0..=59`).
    pub const fn resolved_minute(&self) -> u8 {
        self.resolved_minute
    }

    /// Returns the derived `iztro` `timeIndex` (`0..=12`) for the resolved hour.
    ///
    /// This preserves the early-Zi (`0`) versus late-Zi (`12`) distinction.
    pub const fn resolved_time_index(&self) -> u8 {
        self.resolved_time_index
    }

    /// Returns the derived 时辰 (time branch) for the resolved hour.
    pub const fn resolved_time_branch(&self) -> EarthlyBranch {
        self.resolved_time_branch
    }

    /// Returns the applied longitude correction in minutes, if any.
    pub const fn longitude_correction_minutes(&self) -> Option<f64> {
        self.longitude_correction_minutes
    }

    /// Returns the applied equation-of-time minutes, if any.
    pub const fn equation_of_time_minutes(&self) -> Option<f64> {
        self.equation_of_time_minutes
    }

    /// Returns the total adjustment applied to the clock time, in minutes.
    pub const fn total_adjustment_minutes(&self) -> f64 {
        self.total_adjustment_minutes
    }

    /// Returns the full birth-time variant for the resolved hour.
    pub fn resolved_birth_time(&self) -> Result<BirthTime, ChartError> {
        BirthTime::from_iztro_time_index(self.resolved_time_index)
    }
}

/// Resolves a civil birth date and clock time into a local solar date/time and
/// 时辰, applying the configured calculation policy.
///
/// Under [`SolarTimePolicy::ClockTime`] the clock time is used directly. Under
/// [`SolarTimePolicy::ApparentSolarTime`] the clock time is shifted by the exact
/// longitude correction (and equation-of-time minutes); when the shifted time
/// crosses midnight the resolved solar date moves to the adjacent day.
pub(crate) fn resolve_birth_datetime(
    date: SolarDate,
    birth_time: ClockBirthTime,
    config: &ChartCalculationConfig,
) -> Result<ResolvedBirthDateTime, ChartError> {
    let timezone_meridian_degrees = birth_time.timezone().meridian_degrees();
    let year = date.year();
    let month = date.month().value();
    let day = date.day().value();

    let base_days = days_from_civil(year, i32::from(month), i32::from(day));
    if civil_from_days(base_days) != (year, i32::from(month), i32::from(day)) {
        return Err(ChartError::InvalidSolarDate { year, month, day });
    }

    let (longitude_correction_minutes, equation_of_time_minutes, total_adjustment_minutes) =
        match config.solar_time {
            SolarTimePolicy::ClockTime => (None, None, 0.0),
            SolarTimePolicy::ApparentSolarTime(apparent) => {
                let equation_of_time = match apparent.equation_of_time {
                    EquationOfTimePolicy::Disabled => 0.0,
                    EquationOfTimePolicy::Approximate => {
                        return Err(ChartError::UnsupportedEquationOfTimePolicy);
                    }
                };
                let longitude_correction =
                    4.0 * (apparent.longitude.degrees() - timezone_meridian_degrees);
                (
                    Some(longitude_correction),
                    Some(equation_of_time),
                    longitude_correction + equation_of_time,
                )
            }
        };

    // Resolve to whole minutes. Apparent-solar-time precision finer than a
    // minute is not meaningful for 时辰 derivation; the un-rounded total is
    // still reported for diagnostics.
    let adjusted_minutes =
        (f64::from(birth_time.minutes_since_midnight()) + total_adjustment_minutes).round() as i64;
    let day_offset = adjusted_minutes.div_euclid(MINUTES_PER_DAY);
    let minutes_in_day = adjusted_minutes.rem_euclid(MINUTES_PER_DAY);
    let resolved_hour = (minutes_in_day / 60) as u8;
    let resolved_minute = (minutes_in_day % 60) as u8;

    let (resolved_year, resolved_month, resolved_day) = civil_from_days(base_days + day_offset);
    let resolved_date = SolarDate::new(resolved_year, resolved_month as u8, resolved_day as u8)?;

    let resolved_time_index = time_index_for_hour(resolved_hour);
    let resolved_time_branch = BirthTime::from_iztro_time_index(resolved_time_index)?.branch();

    Ok(ResolvedBirthDateTime {
        input_date: date,
        input_time: birth_time,
        resolved_date,
        resolved_hour,
        resolved_minute,
        resolved_time_index,
        resolved_time_branch,
        longitude_correction_minutes,
        equation_of_time_minutes,
        total_adjustment_minutes,
    })
}

/// Minutes in a civil day.
const MINUTES_PER_DAY: i64 = 24 * 60;

/// Days since the Unix epoch for a proleptic Gregorian date.
///
/// Howard Hinnant's `days_from_civil`. `m` is `1..=12` and `d` is `1..=31`.
const fn days_from_civil(y: i32, m: i32, d: i32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y } as i64;
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400; // [0, 399]
    let m = m as i64;
    let d = d as i64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe - 719468
}

/// Inverse of [`days_from_civil`]: proleptic Gregorian date for a Unix day count.
const fn civil_from_days(z: i64) -> (i32, i32, i32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as i32, d as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::calculation::config::{
        ApparentSolarTimeConfig, ClockBirthTime, Longitude, UtcOffset,
    };

    fn utc_plus_8() -> UtcOffset {
        UtcOffset::from_hours(8).expect("valid offset")
    }

    fn clock(hour: u8, minute: u8) -> ClockBirthTime {
        ClockBirthTime::new(hour, minute, utc_plus_8()).expect("valid clock time")
    }

    fn solar(year: i32, month: u8, day: u8) -> SolarDate {
        SolarDate::new(year, month, day).expect("valid solar date")
    }

    fn resolve(
        date: SolarDate,
        birth: ClockBirthTime,
        config: &ChartCalculationConfig,
    ) -> ResolvedBirthDateTime {
        resolve_birth_datetime(date, birth, config).expect("resolution should succeed")
    }

    fn apparent(longitude: f64) -> ChartCalculationConfig {
        ChartCalculationConfig::apparent_solar_time(ApparentSolarTimeConfig::new(
            Longitude::new(longitude).expect("valid longitude"),
            EquationOfTimePolicy::Disabled,
        ))
    }

    #[test]
    fn civil_day_round_trip() {
        for &(y, m, d) in &[
            (1999, 12, 31),
            (2000, 1, 1),
            (2000, 2, 29),
            (2000, 3, 1),
            (1990, 6, 15),
        ] {
            let z = days_from_civil(y, m, d);
            assert_eq!(civil_from_days(z), (y, m, d));
        }
    }

    #[test]
    fn clock_time_policy_keeps_input_datetime() {
        let date = solar(2000, 1, 1);
        let resolved = resolve(date, clock(1, 5), &ChartCalculationConfig::clock_time());

        assert_eq!(resolved.resolved_date(), date);
        assert_eq!(resolved.resolved_hour(), 1);
        assert_eq!(resolved.resolved_minute(), 5);
        assert_eq!(resolved.resolved_time_branch(), EarthlyBranch::Chou);
        assert_eq!(resolved.longitude_correction_minutes(), None);
        assert_eq!(resolved.equation_of_time_minutes(), None);
        assert_eq!(resolved.total_adjustment_minutes(), 0.0);
    }

    #[test]
    fn apparent_solar_time_at_timezone_meridian_has_zero_longitude_correction() {
        let date = solar(2000, 1, 1);
        let resolved = resolve(date, clock(12, 0), &apparent(120.0));

        assert_eq!(resolved.longitude_correction_minutes(), Some(0.0));
        assert_eq!(resolved.total_adjustment_minutes(), 0.0);
        assert_eq!(resolved.resolved_date(), date);
        assert_eq!(resolved.resolved_hour(), 12);
        assert_eq!(resolved.resolved_minute(), 0);
    }

    #[test]
    fn apparent_solar_time_east_of_timezone_meridian_moves_time_later() {
        let date = solar(2000, 1, 1);
        let resolved = resolve(date, clock(12, 0), &apparent(135.0));

        assert_eq!(resolved.longitude_correction_minutes(), Some(60.0));
        assert_eq!(resolved.resolved_date(), date);
        assert_eq!(resolved.resolved_hour(), 13);
        assert_eq!(resolved.resolved_minute(), 0);
    }

    #[test]
    fn apparent_solar_time_west_of_timezone_meridian_moves_time_earlier() {
        let date = solar(2000, 1, 1);
        let resolved = resolve(date, clock(12, 0), &apparent(105.0));

        assert_eq!(resolved.longitude_correction_minutes(), Some(-60.0));
        assert_eq!(resolved.resolved_date(), date);
        assert_eq!(resolved.resolved_hour(), 11);
        assert_eq!(resolved.resolved_minute(), 0);
    }

    #[test]
    fn apparent_solar_time_can_cross_previous_day() {
        let resolved = resolve(solar(2000, 1, 1), clock(0, 30), &apparent(105.0));

        assert_eq!(resolved.resolved_date(), solar(1999, 12, 31));
        assert_eq!(resolved.resolved_hour(), 23);
        assert_eq!(resolved.resolved_minute(), 30);
    }

    #[test]
    fn apparent_solar_time_can_cross_next_day() {
        let resolved = resolve(solar(2000, 1, 1), clock(23, 30), &apparent(135.0));

        assert_eq!(resolved.resolved_date(), solar(2000, 1, 2));
        assert_eq!(resolved.resolved_hour(), 0);
        assert_eq!(resolved.resolved_minute(), 30);
    }

    #[test]
    fn apparent_solar_time_can_change_time_branch() {
        // 01:05 at UTC+8 with longitude 105E corrects by -60 minutes to 00:05,
        // moving the 时辰 from Chou (丑) to Zi (子).
        let clock_branch = resolve(
            solar(2000, 1, 1),
            clock(1, 5),
            &ChartCalculationConfig::clock_time(),
        )
        .resolved_time_branch();
        assert_eq!(clock_branch, EarthlyBranch::Chou);

        let resolved = resolve(solar(2000, 1, 1), clock(1, 5), &apparent(105.0));
        assert_eq!(resolved.resolved_hour(), 0);
        assert_eq!(resolved.resolved_minute(), 5);
        assert_eq!(resolved.resolved_time_branch(), EarthlyBranch::Zi);
        assert_eq!(resolved.resolved_time_index(), 0);
    }

    #[test]
    fn approximate_equation_of_time_is_unsupported() {
        let config = ChartCalculationConfig::apparent_solar_time(ApparentSolarTimeConfig::new(
            Longitude::new(120.0).expect("valid longitude"),
            EquationOfTimePolicy::Approximate,
        ));
        let result = resolve_birth_datetime(solar(2000, 1, 1), clock(12, 0), &config);
        assert_eq!(result, Err(ChartError::UnsupportedEquationOfTimePolicy));
    }

    #[test]
    fn invalid_input_date_is_rejected() {
        let result = resolve_birth_datetime(
            SolarDate::new(2001, 2, 29).expect("range-valid parts"),
            clock(12, 0),
            &ChartCalculationConfig::clock_time(),
        );
        assert_eq!(
            result,
            Err(ChartError::InvalidSolarDate {
                year: 2001,
                month: 2,
                day: 29,
            }),
        );
    }
}
