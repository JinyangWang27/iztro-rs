//! Typed inputs for the natal chart builders.
//!
//! These low-level builders are calendar-agnostic: they accept an
//! already-resolved **effective** lunar month (and lunar day where needed)
//! alongside the explicit birth-year stem and branch. Calendar conversion and
//! leap-month normalization happen one layer up, in the facade/calendar adapter
//! (`by_solar` and `by_lunar`), so the year stem and branch are supplied
//! explicitly.

use crate::core::model::calendar::BirthContext;
use lunar_lite::{EarthlyBranch, HeavenlyStem};
use crate::core::model::profile::MethodProfile;
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};

/// Inputs required by the minimal natal chart builder.
///
/// This builder is calendar-agnostic: callers provide the already-resolved
/// effective lunar month (leap-month normalization happens upstream in the
/// facade/calendar layer) alongside the typed birth context. Year-to-ganzhi
/// derivation is deferred, so the year stem is supplied explicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NatalChartInput {
    birth_context: BirthContext,
    method_profile: MethodProfile,
    lunar_month: LunarMonth,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
}

impl NatalChartInput {
    /// Creates input for the minimal natal chart builder.
    pub const fn new(
        birth_context: BirthContext,
        method_profile: MethodProfile,
        lunar_month: LunarMonth,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self {
            birth_context,
            method_profile,
            lunar_month,
            birth_year_stem,
            birth_year_branch,
        }
    }

    /// Returns the typed birth context.
    pub const fn birth_context(&self) -> &BirthContext {
        &self.birth_context
    }

    /// Returns the method profile metadata.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(&self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the birth year Heavenly Stem used for palace stem assignment.
    pub const fn birth_year_stem(&self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth year Earthly Branch.
    pub const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }
}

/// Inputs required by the natal chart builder with fourteen major stars.
///
/// This builder is calendar-agnostic: callers provide the already-resolved
/// effective lunar month and lunar day (leap-month normalization happens upstream
/// in the facade/calendar layer). Year-to-ganzhi derivation is deferred, so the
/// year stem is supplied explicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NatalChartWithMajorStarsInput {
    birth_context: BirthContext,
    method_profile: MethodProfile,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
}

impl NatalChartWithMajorStarsInput {
    /// Creates input for the natal chart builder with fourteen major stars.
    pub const fn new(
        birth_context: BirthContext,
        method_profile: MethodProfile,
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self {
            birth_context,
            method_profile,
            lunar_month,
            lunar_day,
            birth_year_stem,
            birth_year_branch,
        }
    }

    /// Returns the typed birth context.
    pub const fn birth_context(&self) -> &BirthContext {
        &self.birth_context
    }

    /// Returns the method profile metadata.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(&self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the validated lunar day.
    pub const fn lunar_day(&self) -> LunarDay {
        self.lunar_day
    }

    /// Returns the birth year Heavenly Stem used for palace stem assignment.
    pub const fn birth_year_stem(&self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth year Earthly Branch.
    pub const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }
}

/// Inputs required by the natal chart builder with all currently supported stars.
///
/// This builder is calendar-agnostic: callers provide the already-resolved
/// effective lunar month and lunar day (leap-month normalization happens upstream
/// in the facade/calendar layer). Year-to-ganzhi derivation is deferred, so the
/// year stem and branch are supplied explicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NatalChartWithSupportedStarsInput {
    birth_context: BirthContext,
    method_profile: MethodProfile,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    daily_star_offset: u8,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
}

impl NatalChartWithSupportedStarsInput {
    /// Creates input for the natal chart builder with supported stars.
    pub const fn new(
        birth_context: BirthContext,
        method_profile: MethodProfile,
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self::new_with_daily_star_offset(
            birth_context,
            method_profile,
            lunar_month,
            lunar_day,
            lunar_day.value() - 1,
            birth_year_stem,
            birth_year_branch,
        )
    }

    /// Creates input for the supported-star builder with an explicit daily-star offset.
    ///
    /// The explicit offset is used by the facade to preserve upstream late-Zi
    /// `fixLunarDayIndex` behavior separately from the major-star lunar day.
    #[allow(clippy::too_many_arguments)]
    pub const fn new_with_daily_star_offset(
        birth_context: BirthContext,
        method_profile: MethodProfile,
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        daily_star_offset: u8,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self {
            birth_context,
            method_profile,
            lunar_month,
            lunar_day,
            daily_star_offset,
            birth_year_stem,
            birth_year_branch,
        }
    }

    /// Returns the typed birth context.
    pub const fn birth_context(&self) -> &BirthContext {
        &self.birth_context
    }

    /// Returns the method profile metadata.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(&self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the validated lunar day.
    pub const fn lunar_day(&self) -> LunarDay {
        self.lunar_day
    }

    /// Returns the daily-star offset used for day-based adjective placement.
    pub const fn daily_star_offset(&self) -> u8 {
        self.daily_star_offset
    }

    /// Returns the birth year Heavenly Stem.
    pub const fn birth_year_stem(&self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth year Earthly Branch.
    pub const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }
}
