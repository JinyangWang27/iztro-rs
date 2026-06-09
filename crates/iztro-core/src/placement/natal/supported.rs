//! Natal chart builders that place stars on top of the minimal natal chart.

use crate::error::ChartError;
use crate::model::chart::Chart;
use crate::placement::natal::adjective::{
    AdjectiveStarPlacementInput, AdjectiveStarPlacer, DeterministicAdjectiveStarPlacer,
};
use crate::placement::natal::input::{
    NatalChartInput, NatalChartWithMajorStarsInput, NatalChartWithSupportedStarsInput,
};
use crate::placement::natal::major::{
    DeterministicMajorStarPlacer, MajorStarPlacementInput, MajorStarPlacer,
};
use crate::placement::natal::minimal::build_minimal_natal_chart;
use crate::placement::natal::minor::{
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
/// natal-scope fourteen major stars, the supported fourteen minor stars, then
/// the full default-algorithm 38 natal adjective/helper stars.
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
        MinorStarPlacementInput::new(
            input.lunar_month(),
            input.birth_context().birth_time(),
            input.birth_year_stem(),
            input.birth_year_branch(),
        ),
    )?;

    DeterministicAdjectiveStarPlacer.place_adjective_stars(
        with_minor_stars,
        AdjectiveStarPlacementInput::new(
            input.lunar_month(),
            input.lunar_day(),
            input.birth_context().birth_time(),
            input.birth_year_stem(),
            input.birth_year_branch(),
        ),
    )
}
