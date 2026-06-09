//! Typed inputs for the natal chart builders.
//!
//! Solar-to-lunar conversion and year-to-ganzhi derivation are not implemented,
//! so callers supply the already-known non-leap lunar month (and lunar day where
//! needed) alongside the explicit birth year stem and branch.

use crate::model::calendar::BirthContext;
use crate::model::ganzhi::{EarthlyBranch, HeavenlyStem};
use crate::model::profile::MethodProfile;
use crate::placement::natal::life_body::{LunarDay, LunarMonth};

/// Inputs required by the minimal natal chart builder.
///
/// Solar-to-lunar conversion is not implemented here. Callers must provide the
/// already-known non-leap lunar month alongside the typed birth context. Birth
/// year stem derivation from a Gregorian date is likewise deferred, so the year
/// stem is supplied explicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NatalChartInput {
    birth_context: BirthContext,
    method_profile: MethodProfile,
    lunar_month: LunarMonth,
    birth_year_stem: HeavenlyStem,
}

impl NatalChartInput {
    /// Creates input for the minimal natal chart builder.
    pub const fn new(
        birth_context: BirthContext,
        method_profile: MethodProfile,
        lunar_month: LunarMonth,
        birth_year_stem: HeavenlyStem,
    ) -> Self {
        Self {
            birth_context,
            method_profile,
            lunar_month,
            birth_year_stem,
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
}

/// Inputs required by the natal chart builder with fourteen major stars.
///
/// Solar-to-lunar conversion is not implemented here. Callers must provide the
/// already-known non-leap lunar month and lunar day. Birth year stem derivation
/// from a Gregorian date is likewise deferred, so the year stem is supplied
/// explicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NatalChartWithMajorStarsInput {
    birth_context: BirthContext,
    method_profile: MethodProfile,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    birth_year_stem: HeavenlyStem,
}

impl NatalChartWithMajorStarsInput {
    /// Creates input for the natal chart builder with fourteen major stars.
    pub const fn new(
        birth_context: BirthContext,
        method_profile: MethodProfile,
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        birth_year_stem: HeavenlyStem,
    ) -> Self {
        Self {
            birth_context,
            method_profile,
            lunar_month,
            lunar_day,
            birth_year_stem,
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
}

/// Inputs required by the natal chart builder with all currently supported stars.
///
/// Solar-to-lunar conversion is not implemented here. Callers must provide the
/// already-known non-leap lunar month and lunar day. Birth year stem and branch
/// derivation are likewise deferred, so both are supplied explicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NatalChartWithSupportedStarsInput {
    birth_context: BirthContext,
    method_profile: MethodProfile,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
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

    /// Returns the birth year Heavenly Stem.
    pub const fn birth_year_stem(&self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth year Earthly Branch.
    pub const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }
}
