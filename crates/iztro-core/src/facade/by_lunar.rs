//! Convenient public facade entry points over strongly typed chart builders.

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
/// conversion, leap-month handling, or year-to-ganzhi derivation.
pub fn by_lunar(request: LunarChartRequest) -> Result<Chart, ChartError> {
    let birth_context = BirthContext::new(
        CalendarDate::lunar(
            request.lunar_year(),
            request.lunar_month().value(),
            request.lunar_day().value(),
        ),
        request.birth_time(),
        request.gender(),
    );

    build_natal_chart_with_supported_stars(NatalChartWithSupportedStarsInput::new(
        birth_context,
        request.method_profile().clone(),
        request.lunar_month(),
        request.lunar_day(),
        request.birth_year_stem(),
        request.birth_year_branch(),
    ))
}
