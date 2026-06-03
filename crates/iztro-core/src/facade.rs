//! Convenient public facade entry points over strongly typed chart builders.

use crate::{
    calendar::{BirthContext, CalendarDate, Gender},
    chart::Chart,
    error::ChartError,
    ganzhi::{EarthlyBranch, HeavenlyStem},
    life_body::{LunarDay, LunarMonth},
    natal::{NatalChartWithMajorStarsInput, build_natal_chart_with_major_stars},
    profile::MethodProfile,
};

/// Typed lunar-date request for the iztro-compatible natal chart facade.
///
/// This mirrors iztro's `byLunar` conceptually while keeping explicit Rust
/// domain types. The birth year stem remains explicit because year-to-ganzhi
/// derivation is not implemented yet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LunarChartRequest {
    lunar_year: i32,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    birth_time: EarthlyBranch,
    gender: Gender,
    birth_year_stem: HeavenlyStem,
    method_profile: MethodProfile,
}

impl LunarChartRequest {
    /// Creates a typed lunar chart request.
    pub const fn new(
        lunar_year: i32,
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        birth_time: EarthlyBranch,
        gender: Gender,
        birth_year_stem: HeavenlyStem,
        method_profile: MethodProfile,
    ) -> Self {
        Self {
            lunar_year,
            lunar_month,
            lunar_day,
            birth_time,
            gender,
            birth_year_stem,
            method_profile,
        }
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

    /// Returns the method profile metadata.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }
}

/// Builds a natal chart with fourteen major stars from explicit lunar inputs.
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

    build_natal_chart_with_major_stars(NatalChartWithMajorStarsInput::new(
        birth_context,
        request.method_profile().clone(),
        request.lunar_month(),
        request.lunar_day(),
        request.birth_year_stem(),
    ))
}
