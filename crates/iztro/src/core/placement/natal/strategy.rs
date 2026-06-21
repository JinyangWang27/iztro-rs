//! High-level natal star-placement strategies.
//!
//! Two layers of "strategy" exist in natal placement:
//!
//! 1. **Per-star-family placers** ([`MajorStarPlacer`], [`MinorStarPlacer`],
//!    [`AdjectiveStarPlacer`], [`DecorativeStarPlacer`]). Each is a strategy for
//!    one star family and knows nothing about the others.
//! 2. **The orchestration strategy** ([`NatalStarPlacementStrategy`], defined
//!    here). It owns the *whole* supported-star pipeline: given a minimal natal
//!    chart and the full natal input, it places every currently supported natal
//!    star family in the correct order.
//!
//! This split gives one clean extension point. Future Zhongzhou (中州) chart
//! algorithms — 中州地盘 (Earth chart) and 中州人盘 (Human chart) — should be
//! added as **new** [`NatalStarPlacementStrategy`] implementations (for example
//! `ZhongzhouEarthNatalPlacementStrategy` / `ZhongzhouHumanNatalPlacementStrategy`),
//! not by expanding conditional logic inside
//! [`crate::core::placement::natal::supported`]. The builder selects a strategy;
//! the strategy decides which placers (or entirely different placement rules) to
//! run.
//!
//! The default [`DeterministicNatalStarPlacementStrategy`] composes the four
//! deterministic placers and remains TS `iztro` 2.5.8-compatible for the
//! supported chart surface.

use crate::core::error::ChartError;
use crate::core::model::chart::Chart;
use crate::core::placement::natal::adjective::{
    AdjectiveStarPlacementInput, AdjectiveStarPlacer, DeterministicAdjectiveStarPlacer,
};
use crate::core::placement::natal::decorative::{
    DecorativeStarPlacementInput, DecorativeStarPlacer, DeterministicDecorativeStarPlacer,
};
use crate::core::placement::natal::input::NatalChartWithSupportedStarsInput;
use crate::core::placement::natal::major::{
    DeterministicMajorStarPlacer, MajorStarPlacementInput, MajorStarPlacer,
};
use crate::core::placement::natal::minor::{
    DeterministicMinorStarPlacer, MinorStarPlacementInput, MinorStarPlacer,
};

/// Orchestrates placement of all currently supported natal stars.
///
/// Implementations receive a minimal natal chart (palace layout, life/body
/// palace, palace stems, and the five-element bureau already derived) together
/// with the full [`NatalChartWithSupportedStarsInput`], and return the chart
/// with every supported natal star family placed.
///
/// This trait is intentionally object-safe so callers can hold a
/// `&dyn NatalStarPlacementStrategy` and pick an algorithm at runtime.
pub trait NatalStarPlacementStrategy {
    /// Places every currently supported natal star family onto `chart`.
    fn place_supported_stars(
        &self,
        chart: Chart,
        input: &NatalChartWithSupportedStarsInput,
    ) -> Result<Chart, ChartError>;
}

/// A [`NatalStarPlacementStrategy`] composed from one placer per star family.
///
/// The supported-star pipeline runs the placers in a fixed order — major, then
/// minor, then adjective, then decorative — because later families depend on the
/// positions of earlier ones (for example 三台/八座 derive from the placed
/// 左辅/右弼). Swapping in a different per-family placer changes that family's
/// rule without touching the orchestration.
///
/// For an entirely different algorithm where the *orchestration itself* differs
/// (different families, ordering, or anchors — as future 中州地盘 / 中州人盘
/// support may require), implement [`NatalStarPlacementStrategy`] directly
/// instead of composing this type.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct CompositeNatalStarPlacementStrategy<M, N, A, D> {
    major: M,
    minor: N,
    adjective: A,
    decorative: D,
}

impl<M, N, A, D> CompositeNatalStarPlacementStrategy<M, N, A, D> {
    /// Creates a composite strategy from one placer per star family.
    pub const fn new(major: M, minor: N, adjective: A, decorative: D) -> Self {
        Self {
            major,
            minor,
            adjective,
            decorative,
        }
    }
}

impl<M, N, A, D> NatalStarPlacementStrategy for CompositeNatalStarPlacementStrategy<M, N, A, D>
where
    M: MajorStarPlacer,
    N: MinorStarPlacer,
    A: AdjectiveStarPlacer,
    D: DecorativeStarPlacer,
{
    fn place_supported_stars(
        &self,
        chart: Chart,
        input: &NatalChartWithSupportedStarsInput,
    ) -> Result<Chart, ChartError> {
        let five_element_bureau = chart
            .five_element_bureau()
            .expect("minimal natal chart should derive a five-element bureau");

        let with_major_stars = self.major.place_major_stars(
            chart,
            MajorStarPlacementInput::new(
                input.lunar_day(),
                five_element_bureau,
                input.birth_year_stem(),
            ),
        )?;

        let with_minor_stars = self.minor.place_minor_stars(
            with_major_stars,
            MinorStarPlacementInput::new_with_birth_time_variant(
                input.lunar_month(),
                input.birth_context().birth_time_variant(),
                input.birth_year_stem(),
                input.birth_year_branch(),
            ),
        )?;

        let with_adjective_stars = self.adjective.place_adjective_stars(
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

        self.decorative.place_decorative_stars(
            with_adjective_stars,
            DecorativeStarPlacementInput::new(input.birth_year_stem(), input.birth_year_branch()),
        )
    }
}

/// The default deterministic natal star-placement strategy.
///
/// Composes the four deterministic per-family placers
/// ([`DeterministicMajorStarPlacer`], [`DeterministicMinorStarPlacer`],
/// [`DeterministicAdjectiveStarPlacer`], [`DeterministicDecorativeStarPlacer`])
/// and reproduces the supported natal chart exactly as TS `iztro` 2.5.8 does for
/// the supported chart surface.
///
/// This is the strategy used by
/// [`build_natal_chart_with_supported_stars`](crate::core::placement::natal::supported::build_natal_chart_with_supported_stars).
/// It is a thin newtype over [`CompositeNatalStarPlacementStrategy`] so the
/// public name stays stable and free of generic parameters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DeterministicNatalStarPlacementStrategy(
    CompositeNatalStarPlacementStrategy<
        DeterministicMajorStarPlacer,
        DeterministicMinorStarPlacer,
        DeterministicAdjectiveStarPlacer,
        DeterministicDecorativeStarPlacer,
    >,
);

impl NatalStarPlacementStrategy for DeterministicNatalStarPlacementStrategy {
    fn place_supported_stars(
        &self,
        chart: Chart,
        input: &NatalChartWithSupportedStarsInput,
    ) -> Result<Chart, ChartError> {
        self.0.place_supported_stars(chart, input)
    }
}
