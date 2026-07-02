//! Branch-level palace relation vocabulary shared by rule engines.
//!
//! All helpers operate on [`EarthlyBranch`] using the canonical branch ordering
//! owned by `iztro-rs` (via [`EarthlyBranch::offset`]). They introduce no
//! second branch order. Offsets mirror the palace-level relations used elsewhere
//! in the crate (see [`crate::features::relations`]): opposite `+6`, trine `+4`
//! and `+8`, adjacent `±1`.
//!
//! This module is the canonical home for branch/palace relation vocabulary used
//! by both the pattern and classical rule engines. `rules::pattern::relation`
//! remains as a compatibility re-export.

use serde::{Deserialize, Serialize};

use crate::core::EarthlyBranch;

/// A relation between two palaces, expressed at the branch level.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PalaceRelation {
    /// Same palace.
    Same,
    /// Opposite palace (对宫), offset `+6`.
    Opposite,
    /// A trine member (三合), offset `+4` or `+8`.
    Trine,
    /// Within the three-sides-four-directions set (三方四正).
    SanFangSiZheng,
    /// Adjacent palace (邻宫), offset `±1`.
    Adjacent,
    /// One of the two palaces clamping a target (夹宫).
    ClampedBy,
}

/// Returns the opposite branch (对宫), offset `+6`.
pub fn opposite(branch: EarthlyBranch) -> EarthlyBranch {
    branch.offset(6)
}

/// Returns the trine (三合) group for `branch`: itself and the `+4`/`+8` members.
pub fn trine_branches(branch: EarthlyBranch) -> [EarthlyBranch; 3] {
    [branch, branch.offset(4), branch.offset(8)]
}

/// Returns the three-sides-four-directions (三方四正) set for `branch`:
/// itself, its opposite, and the two trine members.
pub fn san_fang_si_zheng(branch: EarthlyBranch) -> [EarthlyBranch; 4] {
    [branch, branch.offset(6), branch.offset(4), branch.offset(8)]
}

/// Returns whether `target` is the opposite palace of `anchor`.
pub fn is_opposite(anchor: EarthlyBranch, target: EarthlyBranch) -> bool {
    opposite(anchor) == target
}

/// Returns whether `target` is in the trine (三合) group of `anchor`.
///
/// The trine group includes `anchor` itself, so `is_in_trine_group(b, b)` is
/// `true`. The name makes that inclusion explicit, distinguishing it from
/// [`PalaceRelation::Same`].
pub fn is_in_trine_group(anchor: EarthlyBranch, target: EarthlyBranch) -> bool {
    trine_branches(anchor).contains(&target)
}

/// Returns whether `target` is in the 三方四正 of `anchor`.
///
/// The set includes `anchor` itself.
pub fn is_in_san_fang_si_zheng(anchor: EarthlyBranch, target: EarthlyBranch) -> bool {
    san_fang_si_zheng(anchor).contains(&target)
}

/// Returns the two branches clamping `branch` (夹宫): the `-1` and `+1` neighbours.
pub fn clamp_branches(branch: EarthlyBranch) -> [EarthlyBranch; 2] {
    [branch.offset(-1), branch.offset(1)]
}
