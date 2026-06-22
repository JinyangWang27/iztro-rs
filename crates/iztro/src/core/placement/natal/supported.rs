//! Natal chart builders that place stars on top of the minimal natal chart.
//!
//! These builders own only the minimal-chart construction; the actual star
//! placement is delegated to a [`NatalStarPlacementStrategy`] (for supported
//! stars) or a [`MajorStarPlacer`] (for major-only charts).
//!
//! Chart planes are handled by re-anchoring, not by per-plane strategies. The
//! facade resolves a [`ChartPlane`](crate::core::model::profile::ChartPlane)
//! into a [`NatalChartAnchor`], the supported builder rebuilds an anchor-aware
//! minimal chart, and the existing deterministic strategy places stars:
//!
//! ```text
//! facade resolves ChartPlane -> NatalChartAnchor
//! supported builder builds an anchor-aware minimal chart
//! existing deterministic strategy places stars
//! ```
//!
//! Star placers never branch on the chart plane; see
//! [`build_natal_chart_with_supported_stars_using_anchor_and_strategy`].

use crate::core::error::ChartError;
use crate::core::model::chart::Chart;
use crate::core::placement::natal::input::{
    NatalChartInput, NatalChartWithMajorStarsInput, NatalChartWithSupportedStarsInput,
};
use crate::core::placement::natal::major::{
    DeterministicMajorStarPlacer, MajorStarPlacementInput, MajorStarPlacer,
};
use crate::core::placement::natal::minimal::{
    NatalChartAnchor, build_minimal_natal_chart, build_minimal_natal_chart_with_anchor,
};
use crate::core::placement::natal::strategy::{
    DeterministicNatalStarPlacementStrategy, NatalStarPlacementStrategy,
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
    build_natal_chart_with_major_stars_using(input, &DeterministicMajorStarPlacer)
}

/// Builds a natal chart with the fourteen major stars placed using `placer`.
///
/// Like [`build_natal_chart_with_major_stars`], but the major-star placement
/// strategy is injected, so alternative [`MajorStarPlacer`] implementations can
/// be used without duplicating the minimal-chart construction.
pub fn build_natal_chart_with_major_stars_using<P>(
    input: NatalChartWithMajorStarsInput,
    placer: &P,
) -> Result<Chart, ChartError>
where
    P: MajorStarPlacer + ?Sized,
{
    let chart = build_minimal_natal_chart(NatalChartInput::new(
        input.birth_context().clone(),
        input.method_profile().clone(),
        input.lunar_month(),
        input.birth_year_stem(),
        input.birth_year_branch(),
    ))?;
    let five_element_bureau = chart
        .five_element_bureau()
        .ok_or(ChartError::RequiredFiveElementBureauMissing)?;

    placer.place_major_stars(
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
///
/// Placement is delegated to the default
/// [`DeterministicNatalStarPlacementStrategy`], which remains TS `iztro` 2.5.8
/// -compatible for the supported chart surface. To place stars with a different
/// algorithm, use [`build_natal_chart_with_supported_stars_using`].
pub fn build_natal_chart_with_supported_stars(
    input: NatalChartWithSupportedStarsInput,
) -> Result<Chart, ChartError> {
    build_natal_chart_with_supported_stars_using(
        input,
        &DeterministicNatalStarPlacementStrategy::default(),
    )
}

/// Builds a natal chart with supported stars placed by `strategy`.
///
/// Like [`build_natal_chart_with_supported_stars`], but the high-level
/// [`NatalStarPlacementStrategy`] is injected. The minimal chart uses the
/// calculated Life Palace ([`NatalChartAnchor::CalculatedLifePalace`]); to
/// generate a re-anchored chart plane, use
/// [`build_natal_chart_with_supported_stars_using_anchor_and_strategy`].
pub fn build_natal_chart_with_supported_stars_using<S>(
    input: NatalChartWithSupportedStarsInput,
    strategy: &S,
) -> Result<Chart, ChartError>
where
    S: NatalStarPlacementStrategy + ?Sized,
{
    build_natal_chart_with_supported_stars_using_anchor_and_strategy(
        input,
        NatalChartAnchor::CalculatedLifePalace,
        strategy,
    )
}

/// Builds a natal chart with supported stars, re-anchoring the Life Palace.
///
/// Like [`build_natal_chart_with_supported_stars_using`], but the minimal chart
/// is built with an explicit [`NatalChartAnchor`]. This is how the centralized
/// facade dispatch generates the Zhongzhou 中州地盘 / 中州人盘 planes: it derives
/// the Life-palace anchor from the Heaven chart and rebuilds, rather than
/// mutating a completed chart or branching on the chart plane inside individual
/// star placers.
pub fn build_natal_chart_with_supported_stars_using_anchor_and_strategy<S>(
    input: NatalChartWithSupportedStarsInput,
    anchor: NatalChartAnchor,
    strategy: &S,
) -> Result<Chart, ChartError>
where
    S: NatalStarPlacementStrategy + ?Sized,
{
    let chart = build_minimal_natal_chart_with_anchor(
        NatalChartInput::new(
            input.birth_context().clone(),
            input.method_profile().clone(),
            input.lunar_month(),
            input.birth_year_stem(),
            input.birth_year_branch(),
        ),
        anchor,
    )?;

    strategy.place_supported_stars(chart, &input)
}
