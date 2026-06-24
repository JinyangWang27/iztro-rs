//! Compact structural snapshots for chart diagnostics.
//!
//! These snapshots copy existing chart facts for debugging and invariant
//! inspection. They are not compatibility exports or interpretation models.

use crate::core::model::ganzhi::{EarthlyBranch, HeavenlyStem};
use crate::core::model::{
    bureau::FiveElementBureau,
    chart::{Chart, PalaceName},
    profile::{ChartAlgorithmKind, ChartPlane},
};
use serde::{Deserialize, Serialize};

/// Lightweight structural diagnostic view of a natal chart.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChartDiagnosticSnapshot {
    /// Chart algorithm family used to generate the chart.
    pub algorithm: ChartAlgorithmKind,
    /// Chart plane represented by the chart.
    pub chart_plane: ChartPlane,
    /// Number of palaces present in the chart.
    pub palace_count: usize,
    /// Branch containing the Life Palace, if present.
    pub life_palace_branch: Option<EarthlyBranch>,
    /// Branch containing the Body Palace, if present.
    pub body_palace_branch: Option<EarthlyBranch>,
    /// Five-element bureau, if calculated.
    pub five_element_bureau: Option<FiveElementBureau>,
    /// Ordered structural facts for each palace.
    pub palaces: Vec<PalaceDiagnosticSnapshot>,
}

impl ChartDiagnosticSnapshot {
    pub(crate) fn from_chart(chart: &Chart) -> Self {
        let palaces = chart
            .palaces()
            .iter()
            .map(|palace| PalaceDiagnosticSnapshot {
                name: palace.name(),
                branch: palace.branch(),
                stem: palace.stem(),
                star_count: palace.stars().len(),
                decorative_star_count: palace.decorative_stars().len(),
            })
            .collect();

        Self {
            algorithm: chart.algorithm_kind(),
            chart_plane: chart.chart_plane(),
            palace_count: chart.palaces().len(),
            life_palace_branch: chart.life_palace().map(|palace| palace.branch()),
            body_palace_branch: chart.body_palace_branch(),
            five_element_bureau: chart.five_element_bureau(),
            palaces,
        }
    }
}

/// Lightweight structural diagnostic view of one natal palace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PalaceDiagnosticSnapshot {
    /// Canonical palace name.
    pub name: PalaceName,
    /// Earthly Branch occupied by the palace.
    pub branch: EarthlyBranch,
    /// Heavenly Stem assigned to the palace.
    pub stem: HeavenlyStem,
    /// Number of typed stars placed in the palace.
    pub star_count: usize,
    /// Number of decorative runtime stars placed in the palace.
    pub decorative_star_count: usize,
}
