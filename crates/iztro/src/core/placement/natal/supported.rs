//! Natal chart builders that place stars on top of the minimal natal chart.

use crate::core::error::ChartError;
use crate::core::model::chart::Chart;
use crate::core::placement::natal::adjective::{
    AdjectiveStarPlacementInput, AdjectiveStarPlacer, DeterministicAdjectiveStarPlacer,
};
use crate::core::placement::natal::decorative::{
    DecorativeStarPlacementInput, DecorativeStarPlacer, DeterministicDecorativeStarPlacer,
};
use crate::core::placement::natal::input::{
    NatalChartInput, NatalChartWithMajorStarsInput, NatalChartWithSupportedStarsInput,
};
use crate::core::placement::natal::major::{
    DeterministicMajorStarPlacer, MajorStarPlacementInput, MajorStarPlacer,
};
use crate::core::placement::natal::minimal::build_minimal_natal_chart;
use crate::core::placement::natal::minor::{
    DeterministicMinorStarPlacer, MinorStarPlacementInput, MinorStarPlacer,
};

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
        input.birth_year_branch(),
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

/// Builds a natal chart with all currently supported natal stars placed.
///
/// This public builder preserves the minimal natal chart facts, places the
/// natal-scope fourteen major stars, the supported fourteen minor stars, the
/// algorithm-selected natal adjective/helper stars, and the natal decorative
/// runtime families. Typed stars remain available through [`Chart::stars`],
/// while decorative runtime entries use the separate decorative fact surface.
/// Temporal scopes beyond natal, horoscope placement, feature extraction,
/// rule-engine output, and narrative output remain out of scope.
pub fn build_natal_chart_with_supported_stars(
    input: NatalChartWithSupportedStarsInput,
) -> Result<Chart, ChartError> {
    let chart = build_minimal_natal_chart(NatalChartInput::new(
        input.birth_context().clone(),
        input.method_profile().clone(),
        input.lunar_month(),
        input.birth_year_stem(),
        input.birth_year_branch(),
    ))?;
    let five_element_bureau = chart
        .five_element_bureau()
        .expect("minimal natal chart should derive a five-element bureau");
    let with_major_stars = DeterministicMajorStarPlacer.place_major_stars(
        chart,
        MajorStarPlacementInput::new(
            input.lunar_day(),
            five_element_bureau,
            input.birth_year_stem(),
        ),
    )?;

    let with_minor_stars = DeterministicMinorStarPlacer.place_minor_stars(
        with_major_stars,
        MinorStarPlacementInput::new_with_birth_time_variant(
            input.lunar_month(),
            input.birth_context().birth_time_variant(),
            input.birth_year_stem(),
            input.birth_year_branch(),
        ),
    )?;

    let with_adjective_stars = DeterministicAdjectiveStarPlacer.place_adjective_stars(
        with_minor_stars,
        AdjectiveStarPlacementInput::new_with_daily_star_offset(
            input.lunar_month(),
            input.lunar_day(),
            input.daily_star_offset(),
            input.birth_context().birth_time_variant(),
            input.birth_year_stem(),
            input.birth_year_branch(),
        ),
    )?;

    DeterministicDecorativeStarPlacer.place_decorative_stars(
        with_adjective_stars,
        DecorativeStarPlacementInput::new(input.birth_year_stem(), input.birth_year_branch()),
    )
}
