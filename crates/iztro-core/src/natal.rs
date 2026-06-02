//! Minimal natal chart pipeline for the first algorithmic vertical slice.

use crate::{
    builder::build_empty_chart,
    calendar::BirthContext,
    chart::{Chart, PALACE_COUNT, Palace},
    error::ChartError,
    ganzhi::EarthlyBranch,
    life_body::{LunarBirthContext, LunarMonth, calculate_life_body_palace_indices},
    palace::PalaceName,
    profile::MethodProfile,
};

/// Inputs required by the minimal natal chart builder.
///
/// Solar-to-lunar conversion is not implemented here. Callers must provide the
/// already-known non-leap lunar month alongside the typed birth context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NatalChartInput {
    birth_context: BirthContext,
    method_profile: MethodProfile,
    lunar_month: LunarMonth,
}

impl NatalChartInput {
    /// Creates input for the minimal natal chart builder.
    pub const fn new(
        birth_context: BirthContext,
        method_profile: MethodProfile,
        lunar_month: LunarMonth,
    ) -> Self {
        Self {
            birth_context,
            method_profile,
            lunar_month,
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
}

/// Builds a minimal natal chart with calculated Life Palace layout.
///
/// The builder intentionally leaves stars empty and keeps the existing
/// placeholder heavenly-stem assignment from [`build_empty_chart`]. Real palace
/// stem placement, leap-month handling, and full chart generation are deferred.
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

    let palaces = empty_chart
        .palaces()
        .iter()
        .map(|palace| {
            Palace::new(
                palace_name_relative_to_life_branch(palace.branch(), life_branch),
                palace.branch(),
                palace.stem(),
                palace.stars().to_vec(),
            )
        })
        .collect();

    Chart::try_new(
        input.birth_context().clone(),
        input.method_profile().clone(),
        palaces,
        Some(indices.body_palace_branch()),
    )
}

fn palace_name_relative_to_life_branch(
    branch: EarthlyBranch,
    life_branch: EarthlyBranch,
) -> PalaceName {
    let offset = (life_branch.index() + PALACE_COUNT - branch.index()) % PALACE_COUNT;

    PalaceName::from_index(offset)
}
