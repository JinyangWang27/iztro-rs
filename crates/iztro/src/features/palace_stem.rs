//! Palace-stem role and mutagen-flow facts derived from natal chart facts.
//!
//! Every palace carries a Heavenly Stem (assigned by the 起五行寅例 rule from the
//! birth-year stem). Two school-independent families of facts follow purely from
//! those palace stems and the shared 十干四化 table:
//!
//! - **Palace-stem roles** — structural roles a palace plays because of its stem.
//!   The only role modeled here is [`PalaceStemRole::BirthYearStemOrigin`]: a
//!   palace whose stem equals the birth-year stem. In classical 飞星 practice this
//!   palace is known as 来因宫; this module exposes it by its structural invariant
//!   rather than by school-specific terminology.
//! - **Palace-stem mutagen flows** — for each palace, its stem transforms four
//!   natal stars via the 十干四化 table ([`stem_mutagen_targets`]). Each flow
//!   records the source palace/stem and the natal star/palace the mutagen lands
//!   on. These are *derived relations*, not placed stars: nothing is added to the
//!   chart, and the flows are recomputed from existing facts on demand.
//!
//! This module is the school-independent foundation that later 飞星派 / 钦天四化
//! profiles can consume. It performs no interpretation and models no school-specific
//! constructs (向心 / 离心 / 禄忌交战 / 双忌 are intentionally out of scope). The only
//! derived relation it exposes today is 自化 (self-transform), defined conservatively
//! as a flow whose mutagen lands back in its own source palace branch.

use crate::core::{
    Chart, ChartError, EarthlyBranch, HeavenlyStem, Mutagen, Palace, PalaceName, StarName,
    stem_mutagen_targets,
};
use serde::{Deserialize, Serialize};

/// A structural role a palace plays because of its Heavenly Stem.
///
/// Roles describe an invariant of the palace stem, not a school-specific
/// interpretation. Only one role is modeled so far; the enum is left open for
/// future stem-derived roles.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PalaceStemRole {
    /// The palace whose stem equals the birth-year stem (来因宫).
    ///
    /// Named for the structural invariant (`palace.stem == birth_year.stem`)
    /// rather than the school-specific 来因宫 gloss. Most birth-year stems yield
    /// one such palace, but 辛 and 壬 yield two, so callers must treat this as a
    /// set rather than assume a single palace.
    BirthYearStemOrigin,
}

/// A palace assigned a [`PalaceStemRole`] together with the facts that justify it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PalaceStemRoleAssignment {
    /// The structural role this palace plays.
    pub role: PalaceStemRole,
    /// Branch of the assigned palace.
    pub branch: EarthlyBranch,
    /// Name of the assigned palace.
    pub palace_name: PalaceName,
    /// Heavenly Stem of the assigned palace.
    pub palace_stem: HeavenlyStem,
    /// Reference stem the role is defined against (the birth-year stem).
    pub reference_stem: HeavenlyStem,
}

/// The source side of a palace-stem mutagen flow: the palace whose stem drives it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PalaceStemSource {
    /// Branch of the source palace.
    pub branch: EarthlyBranch,
    /// Name of the source palace.
    pub palace_name: PalaceName,
    /// Heavenly Stem of the source palace, used to look up the mutagen targets.
    pub stem: HeavenlyStem,
}

/// The landing side of a palace-stem mutagen flow: the natal star/palace hit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MutagenFlowTarget {
    /// Which of the four transformations the source stem applies.
    pub mutagen: Mutagen,
    /// The natal star the mutagen transforms.
    pub star: StarName,
    /// Branch of the palace the target star occupies natally.
    pub branch: EarthlyBranch,
    /// Name of the palace the target star occupies natally.
    pub palace_name: PalaceName,
}

/// A single palace-stem mutagen flow: `source palace stem -> mutagen -> target
/// star/palace`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PalaceStemMutagenFlow {
    /// The palace whose stem produces the flow.
    pub source: PalaceStemSource,
    /// The natal star/palace the flow lands on.
    pub target: MutagenFlowTarget,
}

impl PalaceStemMutagenFlow {
    /// Returns whether this flow is a self-transform (自化).
    ///
    /// Conservative first version: a flow self-transforms when its mutagen lands
    /// back in the branch of its own source palace. Directional refinements
    /// (向心 / 离心) are intentionally not modeled here.
    pub fn is_self_transform(&self) -> bool {
        self.source.branch == self.target.branch
    }
}

/// Returns the source descriptor for a palace.
fn palace_stem_source(palace: &Palace) -> PalaceStemSource {
    PalaceStemSource {
        branch: palace.branch(),
        palace_name: palace.name(),
        stem: palace.stem(),
    }
}

/// Returns every [`PalaceStemRoleAssignment`] for the chart.
///
/// Palaces are visited in chart order, so the output is deterministic. Only the
/// [`PalaceStemRole::BirthYearStemOrigin`] role is emitted today.
pub fn palace_stem_role_assignments(
    chart: &Chart,
) -> Result<Vec<PalaceStemRoleAssignment>, ChartError> {
    let reference_stem = chart.birth_year().stem();

    let assignments = chart
        .palaces()
        .iter()
        .filter(|palace| palace.stem() == reference_stem)
        .map(|palace| PalaceStemRoleAssignment {
            role: PalaceStemRole::BirthYearStemOrigin,
            branch: palace.branch(),
            palace_name: palace.name(),
            palace_stem: palace.stem(),
            reference_stem,
        })
        .collect();

    Ok(assignments)
}

/// Returns the palaces whose stem equals the birth-year stem (来因宫).
///
/// Plural because 辛 and 壬 birth years yield two such palaces; most birth-year
/// stems yield one.
pub fn birth_year_stem_origin_palaces(
    chart: &Chart,
) -> Result<Vec<PalaceStemRoleAssignment>, ChartError> {
    Ok(palace_stem_role_assignments(chart)?
        .into_iter()
        .filter(|assignment| assignment.role == PalaceStemRole::BirthYearStemOrigin)
        .collect())
}

/// Returns every palace-stem mutagen flow for the chart.
///
/// Flows are ordered first by source palace (chart order) and then by mutagen in
/// canonical 禄 / 权 / 科 / 忌 order, so each palace contributes exactly four
/// flows and the whole sequence is deterministic.
///
/// Returns [`ChartError::RequiredStarMissing`] if a stem's mutagen target star is
/// not placed in the chart. This surfaces incomplete star inventories rather than
/// silently dropping a flow: the four 十干四化 targets are always expected to be
/// present in a fully placed natal chart.
pub fn palace_stem_mutagen_flows(chart: &Chart) -> Result<Vec<PalaceStemMutagenFlow>, ChartError> {
    let mut flows = Vec::with_capacity(chart.palaces().len() * 4);

    for palace in chart.palaces() {
        let source = palace_stem_source(palace);

        for (mutagen, star) in stem_mutagen_targets(source.stem) {
            let placement = chart
                .star(star)
                .ok_or(ChartError::RequiredStarMissing { star })?;

            flows.push(PalaceStemMutagenFlow {
                source,
                target: MutagenFlowTarget {
                    mutagen,
                    star,
                    branch: placement.palace().branch(),
                    palace_name: placement.palace().name(),
                },
            });
        }
    }

    Ok(flows)
}

/// Returns the palace-stem mutagen flows whose source is the named palace.
pub fn mutagen_flows_from_palace(
    chart: &Chart,
    palace: PalaceName,
) -> Result<Vec<PalaceStemMutagenFlow>, ChartError> {
    Ok(palace_stem_mutagen_flows(chart)?
        .into_iter()
        .filter(|flow| flow.source.palace_name == palace)
        .collect())
}

/// Returns the palace-stem mutagen flows that land in the named palace.
pub fn mutagen_flows_landing_in_palace(
    chart: &Chart,
    palace: PalaceName,
) -> Result<Vec<PalaceStemMutagenFlow>, ChartError> {
    Ok(palace_stem_mutagen_flows(chart)?
        .into_iter()
        .filter(|flow| flow.target.palace_name == palace)
        .collect())
}

/// Returns the palace-stem mutagen flows that self-transform (自化).
pub fn self_transforming_flows(chart: &Chart) -> Result<Vec<PalaceStemMutagenFlow>, ChartError> {
    Ok(palace_stem_mutagen_flows(chart)?
        .into_iter()
        .filter(PalaceStemMutagenFlow::is_self_transform)
        .collect())
}
