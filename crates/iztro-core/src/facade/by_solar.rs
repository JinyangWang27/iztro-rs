//! Minimal solar-date facade over the supported chart-generation slice.
//!
//! [`by_solar`] mirrors iztro's `astro.bySolar(...)` conceptually through the
//! typed [`SolarChartRequest`]. It is an adaptor layer: it validates the solar
//! input, converts it to lunar facts with the internal ICU4X-backed calendar
//! adapter, then delegates to [`by_lunar`] so chart construction reuses exactly
//! the same supported slice. No new star-placement logic lives here.

use crate::calendar::solar_to_lunar;
use crate::error::ChartError;
use crate::facade::by_lunar::{LunarChartRequest, by_lunar};
use crate::model::calendar::{BirthTime, Gender, SolarDay, SolarMonth};
use crate::model::chart::Chart;
use crate::model::ganzhi::EarthlyBranch;
use crate::model::profile::MethodProfile;

/// Typed solar-date request for the iztro-compatible natal chart facade.
///
/// This mirrors iztro's `bySolar` conceptually while keeping explicit Rust
/// domain types. Unlike [`LunarChartRequest`], the birth year stem/branch and the
/// lunar date are derived during calendar conversion rather than supplied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolarChartRequest {
    solar_year: i32,
    solar_month: SolarMonth,
    solar_day: SolarDay,
    birth_time: BirthTime,
    gender: Gender,
    fix_leap: bool,
    method_profile: MethodProfile,
}

impl SolarChartRequest {
    /// Starts building a typed solar chart request.
    ///
    /// Set each required field on the returned builder, then call
    /// [`SolarChartRequestBuilder::build`].
    pub fn builder() -> SolarChartRequestBuilder {
        SolarChartRequestBuilder::default()
    }

    /// Returns the provided solar (Gregorian) year.
    pub const fn solar_year(&self) -> i32 {
        self.solar_year
    }

    /// Returns the validated solar month.
    pub const fn solar_month(&self) -> SolarMonth {
        self.solar_month
    }

    /// Returns the validated solar day.
    pub const fn solar_day(&self) -> SolarDay {
        self.solar_day
    }

    /// Returns the birth time branch.
    pub const fn birth_time(&self) -> EarthlyBranch {
        self.birth_time.branch()
    }

    /// Returns the full birth-time variant.
    pub const fn birth_time_variant(&self) -> BirthTime {
        self.birth_time
    }

    /// Returns the gender marker.
    pub const fn gender(&self) -> Gender {
        self.gender
    }

    /// Returns whether leap-month adjustment is applied (调整闰月).
    pub const fn fix_leap(&self) -> bool {
        self.fix_leap
    }

    /// Returns the method profile metadata.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }
}

/// Builder for [`SolarChartRequest`].
///
/// Each field is optional until set; [`build`](SolarChartRequestBuilder::build)
/// fails with [`ChartError::MissingRequiredInput`] when a required field is
/// missing. `fix_leap` defaults to `true`, matching upstream `iztro@2.5.8`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SolarChartRequestBuilder {
    solar_year: Option<i32>,
    solar_month: Option<SolarMonth>,
    solar_day: Option<SolarDay>,
    birth_time: Option<BirthTime>,
    gender: Option<Gender>,
    fix_leap: Option<bool>,
    method_profile: Option<MethodProfile>,
}

impl SolarChartRequestBuilder {
    /// Sets the solar (Gregorian) year.
    pub fn solar_year(mut self, value: i32) -> Self {
        self.solar_year = Some(value);
        self
    }

    /// Sets the validated solar month.
    pub fn solar_month(mut self, value: SolarMonth) -> Self {
        self.solar_month = Some(value);
        self
    }

    /// Sets the validated solar day.
    pub fn solar_day(mut self, value: SolarDay) -> Self {
        self.solar_day = Some(value);
        self
    }

    /// Sets the birth time branch.
    pub fn birth_time(mut self, value: EarthlyBranch) -> Self {
        self.birth_time = Some(BirthTime::from_branch(value));
        self
    }

    /// Sets the full birth-time variant.
    pub fn birth_time_variant(mut self, value: BirthTime) -> Self {
        self.birth_time = Some(value);
        self
    }

    /// Sets the birth time from an upstream `iztro` `timeIndex`.
    pub fn iztro_time_index(mut self, value: u8) -> Result<Self, ChartError> {
        self.birth_time = Some(BirthTime::from_iztro_time_index(value)?);
        Ok(self)
    }

    /// Sets the gender marker.
    pub fn gender(mut self, value: Gender) -> Self {
        self.gender = Some(value);
        self
    }

    /// Sets whether leap-month adjustment is applied (调整闰月).
    ///
    /// Defaults to `true` when unset, matching upstream `iztro@2.5.8`.
    pub fn fix_leap(mut self, value: bool) -> Self {
        self.fix_leap = Some(value);
        self
    }

    /// Sets the method profile metadata.
    pub fn method_profile(mut self, value: MethodProfile) -> Self {
        self.method_profile = Some(value);
        self
    }

    /// Builds the immutable request, requiring every field except `fix_leap`.
    pub fn build(self) -> Result<SolarChartRequest, ChartError> {
        Ok(SolarChartRequest {
            solar_year: self.solar_year.ok_or(ChartError::MissingRequiredInput {
                field: "solar_year",
            })?,
            solar_month: self.solar_month.ok_or(ChartError::MissingRequiredInput {
                field: "solar_month",
            })?,
            solar_day: self
                .solar_day
                .ok_or(ChartError::MissingRequiredInput { field: "solar_day" })?,
            birth_time: self.birth_time.ok_or(ChartError::MissingRequiredInput {
                field: "birth_time",
            })?,
            gender: self
                .gender
                .ok_or(ChartError::MissingRequiredInput { field: "gender" })?,
            fix_leap: self.fix_leap.unwrap_or(true),
            method_profile: self
                .method_profile
                .ok_or(ChartError::MissingRequiredInput {
                    field: "method_profile",
                })?,
        })
    }
}

/// Builds a natal chart with currently supported natal stars from a solar date.
///
/// This facade validates the Gregorian/solar date, converts it to Chinese
/// lunisolar facts with the internal ICU4X-backed adapter, derives the
/// birth-year stem/branch from the cyclic year, sets `is_leap_month` from the
/// conversion and `fix_leap` from the request, then delegates to [`by_lunar`].
/// It performs no chart construction of its own, so it preserves the exact
/// `by_lunar` supported slice (including leap-month behavior).
pub fn by_solar(request: SolarChartRequest) -> Result<Chart, ChartError> {
    let conversion = solar_to_lunar(
        request.solar_year(),
        request.solar_month(),
        request.solar_day(),
    )?;

    let lunar_request = LunarChartRequest::builder()
        .lunar_year(conversion.lunar_year())
        .lunar_month(conversion.lunar_month())
        .lunar_day(conversion.lunar_day())
        .birth_time_variant(request.birth_time_variant())
        .gender(request.gender())
        .birth_year_stem(conversion.birth_year_stem())
        .birth_year_branch(conversion.birth_year_branch())
        .is_leap_month(conversion.is_leap_month())
        .fix_leap(request.fix_leap())
        .method_profile(request.method_profile().clone())
        .build()?;

    by_lunar(lunar_request)
}
