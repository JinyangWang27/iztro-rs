//! Convenient public facade entry points over strongly typed chart builders.

use crate::calendar::resolve_lunar_date;
use crate::error::ChartError;
use crate::model::calendar::{BirthContext, CalendarDate, Gender};
use crate::model::chart::Chart;
use crate::model::ganzhi::{EarthlyBranch, HeavenlyStem};
use crate::model::profile::MethodProfile;
use crate::placement::natal::input::NatalChartWithSupportedStarsInput;
use crate::placement::natal::life_body::{LunarDay, LunarMonth};
use crate::placement::natal::supported::build_natal_chart_with_supported_stars;

/// Typed lunar-date request for the iztro-compatible natal chart facade.
///
/// This mirrors iztro's `byLunar` conceptually while keeping explicit Rust
/// domain types. The birth year stem and branch remain explicit because
/// year-to-ganzhi derivation is not implemented yet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LunarChartRequest {
    lunar_year: i32,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    birth_time: EarthlyBranch,
    gender: Gender,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
    is_leap_month: bool,
    fix_leap: bool,
    method_profile: MethodProfile,
}

impl LunarChartRequest {
    /// Starts building a typed lunar chart request.
    ///
    /// Set each required field on the returned builder, then call
    /// [`LunarChartRequestBuilder::build`].
    pub fn builder() -> LunarChartRequestBuilder {
        LunarChartRequestBuilder::default()
    }

    /// Returns the provided lunar year.
    pub const fn lunar_year(&self) -> i32 {
        self.lunar_year
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(&self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the validated lunar day.
    pub const fn lunar_day(&self) -> LunarDay {
        self.lunar_day
    }

    /// Returns the birth time branch.
    pub const fn birth_time(&self) -> EarthlyBranch {
        self.birth_time
    }

    /// Returns the gender marker.
    pub const fn gender(&self) -> Gender {
        self.gender
    }

    /// Returns the explicit birth year Heavenly Stem.
    pub const fn birth_year_stem(&self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the explicit birth year Earthly Branch.
    pub const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }

    /// Returns whether the lunar month is a leap month (闰月).
    pub const fn is_leap_month(&self) -> bool {
        self.is_leap_month
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

/// Builder for [`LunarChartRequest`].
///
/// Each field is optional until set; [`build`](LunarChartRequestBuilder::build)
/// fails with [`ChartError::MissingRequiredInput`] if a required field is
/// missing, keeping construction explicit and deterministic.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LunarChartRequestBuilder {
    lunar_year: Option<i32>,
    lunar_month: Option<LunarMonth>,
    lunar_day: Option<LunarDay>,
    birth_time: Option<EarthlyBranch>,
    gender: Option<Gender>,
    birth_year_stem: Option<HeavenlyStem>,
    birth_year_branch: Option<EarthlyBranch>,
    is_leap_month: Option<bool>,
    fix_leap: Option<bool>,
    method_profile: Option<MethodProfile>,
}

impl LunarChartRequestBuilder {
    /// Sets the lunar year.
    pub fn lunar_year(mut self, value: i32) -> Self {
        self.lunar_year = Some(value);
        self
    }

    /// Sets the validated lunar month.
    pub fn lunar_month(mut self, value: LunarMonth) -> Self {
        self.lunar_month = Some(value);
        self
    }

    /// Sets the validated lunar day.
    pub fn lunar_day(mut self, value: LunarDay) -> Self {
        self.lunar_day = Some(value);
        self
    }

    /// Sets the birth time branch.
    pub fn birth_time(mut self, value: EarthlyBranch) -> Self {
        self.birth_time = Some(value);
        self
    }

    /// Sets the gender marker.
    pub fn gender(mut self, value: Gender) -> Self {
        self.gender = Some(value);
        self
    }

    /// Sets the explicit birth year Heavenly Stem.
    pub fn birth_year_stem(mut self, value: HeavenlyStem) -> Self {
        self.birth_year_stem = Some(value);
        self
    }

    /// Sets the explicit birth year Earthly Branch.
    pub fn birth_year_branch(mut self, value: EarthlyBranch) -> Self {
        self.birth_year_branch = Some(value);
        self
    }

    /// Sets whether the lunar month is a leap month (闰月).
    ///
    /// Defaults to `false` when unset, preserving the original non-leap
    /// behavior.
    pub fn is_leap_month(mut self, value: bool) -> Self {
        self.is_leap_month = Some(value);
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

    /// Builds the immutable request, requiring every field to be set.
    pub fn build(self) -> Result<LunarChartRequest, ChartError> {
        Ok(LunarChartRequest {
            lunar_year: self.lunar_year.ok_or(ChartError::MissingRequiredInput {
                field: "lunar_year",
            })?,
            lunar_month: self.lunar_month.ok_or(ChartError::MissingRequiredInput {
                field: "lunar_month",
            })?,
            lunar_day: self
                .lunar_day
                .ok_or(ChartError::MissingRequiredInput { field: "lunar_day" })?,
            birth_time: self.birth_time.ok_or(ChartError::MissingRequiredInput {
                field: "birth_time",
            })?,
            gender: self
                .gender
                .ok_or(ChartError::MissingRequiredInput { field: "gender" })?,
            birth_year_stem: self
                .birth_year_stem
                .ok_or(ChartError::MissingRequiredInput {
                    field: "birth_year_stem",
                })?,
            birth_year_branch: self
                .birth_year_branch
                .ok_or(ChartError::MissingRequiredInput {
                    field: "birth_year_branch",
                })?,
            is_leap_month: self.is_leap_month.unwrap_or(false),
            fix_leap: self.fix_leap.unwrap_or(true),
            method_profile: self
                .method_profile
                .ok_or(ChartError::MissingRequiredInput {
                    field: "method_profile",
                })?,
        })
    }
}

/// Builds a natal chart with currently supported natal stars from explicit lunar inputs.
///
/// This facade records the lunar date as chart input facts and delegates to the
/// existing strongly typed builder. It does not perform solar-to-lunar
/// conversion or year-to-ganzhi derivation; calendar conversion is provided by
/// [`by_solar`](crate::facade::by_solar::by_solar), which delegates here.
///
/// Leap-month behavior is explicit and upstream-compatible. The requested
/// `is_leap_month` is first resolved against the real calendar through
/// [`resolve_lunar_date`] (the flag is honored only when the requested month is
/// actually the year's leap month, mirroring upstream `lunar2solar`). The
/// recorded calendar date keeps the resolved lunar year/month/day, while
/// month-based star placement uses the effective month derived from
/// [`effective_lunar_month`] applied to the resolved leap state.
pub fn by_lunar(request: LunarChartRequest) -> Result<Chart, ChartError> {
    let resolved = resolve_lunar_date(
        request.lunar_year(),
        request.lunar_month(),
        request.lunar_day(),
        request.is_leap_month(),
    )?;

    let effective_lunar_month = effective_lunar_month(
        resolved.lunar_month(),
        resolved.lunar_day(),
        resolved.is_leap_month(),
        request.fix_leap(),
    )?;

    let birth_context = BirthContext::new(
        CalendarDate::lunar(
            resolved.lunar_year(),
            resolved.lunar_month().value(),
            resolved.lunar_day().value(),
        ),
        request.birth_time(),
        request.gender(),
    );

    build_natal_chart_with_supported_stars(NatalChartWithSupportedStarsInput::new(
        birth_context,
        request.method_profile().clone(),
        effective_lunar_month,
        resolved.lunar_day(),
        request.birth_year_stem(),
        request.birth_year_branch(),
    ))
}

/// Computes the effective lunar month used for month-based star placement.
///
/// Mirrors upstream `iztro@2.5.8` `fixLunarMonthIndex`: when the birth month is a
/// leap month, leap-month adjustment is enabled, and the lunar day is in the
/// second half of the month (after the 15th), the effective month advances by
/// one. Otherwise a leap month is treated as the same numeric month, and a
/// non-leap month is always used as-is. iztro-rs does not model the late
/// rat-hour (晚子时) variant, so that upstream guard is always satisfied here.
///
/// `is_leap_month` here is the **resolved** leap state from
/// [`resolve_lunar_date`], not the raw request flag, so an invalid leap request
/// (a month that is not actually leap that year) never advances the month.
///
/// A leap twelfth month would push the effective month past 12 into the next
/// lunar year, which is out of the supported slice, so it returns
/// [`ChartError::UnsupportedLeapMonthCombination`] rather than guessing.
fn effective_lunar_month(
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    is_leap_month: bool,
    fix_leap: bool,
) -> Result<LunarMonth, ChartError> {
    let needs_advance = is_leap_month && fix_leap && lunar_day.value() > 15;
    if !needs_advance {
        return Ok(lunar_month);
    }

    LunarMonth::new(lunar_month.value() + 1).map_err(|_| {
        ChartError::UnsupportedLeapMonthCombination {
            lunar_month: lunar_month.value(),
            lunar_day: lunar_day.value(),
        }
    })
}
