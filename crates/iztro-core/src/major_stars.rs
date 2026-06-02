//! Major-star placement interface for future chart-generation algorithms.

use crate::{chart::Chart, error::ChartError};

/// Inputs reserved for future major-star placement algorithms.
///
/// The first vertical slice does not implement Zi Wei, Tian Fu, or complete
/// fourteen-major-star placement. This empty input type marks the module
/// boundary without pretending star placement is available.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct MajorStarPlacementInput;

/// Places major stars into a chart.
///
/// Implementations must preserve chart invariants and return a valid chart.
pub trait MajorStarPlacer {
    /// Places major stars in `chart` according to `input`.
    fn place_major_stars(
        &self,
        chart: Chart,
        input: MajorStarPlacementInput,
    ) -> Result<Chart, ChartError>;
}

/// Major-star placer that intentionally leaves the chart unchanged.
///
/// This is scaffolding for the natal chart pipeline. It does not implement any
/// real major-star placement algorithm.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct NoopMajorStarPlacer;

impl MajorStarPlacer for NoopMajorStarPlacer {
    fn place_major_stars(
        &self,
        chart: Chart,
        _input: MajorStarPlacementInput,
    ) -> Result<Chart, ChartError> {
        Ok(chart)
    }
}
