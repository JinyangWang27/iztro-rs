//! Minimal natal chart pipeline for the first algorithmic vertical slice.

use crate::{
    builder::build_empty_chart,
    bureau::five_element_bureau_from_life_palace,
    calendar::BirthContext,
    chart::{Chart, PALACE_COUNT, Palace},
    error::ChartError,
    ganzhi::{EarthlyBranch, HeavenlyStem},
    life_body::LunarDay,
    life_body::{LunarBirthContext, LunarMonth, calculate_life_body_palace_indices},
    major_stars::{DeterministicMajorStarPlacer, MajorStarPlacementInput, MajorStarPlacer},
    palace::PalaceName,
    palace_stems::palace_stem_for_branch,
    profile::MethodProfile,
    sexagenary::StemBranch,
};

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

/// Builds a minimal natal chart with calculated Life Palace layout.
///
/// The builder calculates the Life and Body palaces, assigns each palace its
/// Heavenly Stem from the birth year stem via 起五行寅例, remaps palace names
/// relative to the Life Palace, and derives the five-element bureau from the
/// Life Palace stem-branch pair. Stars are intentionally left empty;
/// leap-month handling and full chart generation are deferred.
pub fn build_minimal_natal_chart(input: NatalChartInput) -> Result<Chart, ChartError> {
    let empty_chart = build_empty_chart(
        input.birth_context().clone(),
        input.method_profile().clone(),
    )?;
    let indices = calculate_life_body_palace_indices(LunarBirthContext::new(
        input.lunar_month(),
        input.birth_context().birth_time(),
    ))?;
    let life_branch = indices.life_palace_branch();
    let year_stem = input.birth_year_stem();

    let palaces = empty_chart
        .palaces()
        .iter()
        .map(|palace| {
            let branch = palace.branch();
            Palace::new(
                palace_name_relative_to_life_branch(branch, life_branch),
                branch,
                palace_stem_for_branch(year_stem, branch),
                palace.stars().to_vec(),
            )
        })
        .collect();

    let life_pair = StemBranch::new(palace_stem_for_branch(year_stem, life_branch), life_branch);
    let five_element_bureau = five_element_bureau_from_life_palace(life_pair)?;

    Chart::try_new(
        input.birth_context().clone(),
        input.method_profile().clone(),
        palaces,
        Some(indices.body_palace_branch()),
        Some(five_element_bureau),
    )
}

/// Builds a natal chart with the fourteen major stars placed.
///
/// This public builder preserves the minimal natal chart facts, derives the
/// five-element bureau through [`build_minimal_natal_chart`], and then places
/// the natal-scope fourteen major stars using [`DeterministicMajorStarPlacer`].
/// Minor stars, adjective stars, temporal scopes beyond natal, and narrative
/// output remain out of scope.
pub fn build_natal_chart_with_major_stars(
    input: NatalChartWithMajorStarsInput,
) -> Result<Chart, ChartError> {
    let chart = build_minimal_natal_chart(NatalChartInput::new(
        input.birth_context().clone(),
        input.method_profile().clone(),
        input.lunar_month(),
        input.birth_year_stem(),
    ))?;
    let five_element_bureau = chart
        .five_element_bureau()
        .expect("minimal natal chart should derive a five-element bureau");

    DeterministicMajorStarPlacer.place_major_stars(
        chart,
        MajorStarPlacementInput::new(
            input.lunar_day(),
            five_element_bureau,
            input.birth_year_stem(),
        ),
    )
}

fn palace_name_relative_to_life_branch(
    branch: EarthlyBranch,
    life_branch: EarthlyBranch,
) -> PalaceName {
    let offset = (life_branch.index() + PALACE_COUNT - branch.index()) % PALACE_COUNT;

    PalaceName::from_index(offset)
}
