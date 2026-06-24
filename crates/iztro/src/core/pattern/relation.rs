//! Branch-level palace relation helpers for pattern detection.
//!
//! All helpers operate on [`EarthlyBranch`] using the canonical branch ordering
//! owned by `iztro-rs` (via [`EarthlyBranch::offset`]). They introduce no
//! second branch order. Offsets mirror the palace-level relations used elsewhere
//! in the crate (see [`crate::features::relations`]): opposite `+6`, trine `+4`
//! and `+8`, adjacent `±1`.

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::EarthlyBranch::{Chen, Mao, Shen, Si, Wei, Yin, You, Zi};

    #[test]
    fn opposite_is_offset_six() {
        assert_eq!(opposite(Yin), Shen);
        assert_eq!(opposite(Zi), EarthlyBranch::Wu);
    }

    #[test]
    fn trine_branches_are_self_plus_four_and_eight() {
        // 寅午戌 trine group, anchored on 寅.
        assert_eq!(
            trine_branches(Yin),
            [Yin, EarthlyBranch::Wu, EarthlyBranch::Xu]
        );
    }

    #[test]
    fn san_fang_si_zheng_is_self_opposite_and_trine() {
        let set = san_fang_si_zheng(Yin);
        assert!(set.contains(&Yin));
        assert!(set.contains(&Shen)); // opposite
        assert!(set.contains(&EarthlyBranch::Wu)); // trine +4
        assert!(set.contains(&EarthlyBranch::Xu)); // trine +8
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn is_in_san_fang_si_zheng_membership() {
        assert!(is_in_san_fang_si_zheng(Yin, Yin));
        assert!(is_in_san_fang_si_zheng(Yin, Shen));
        assert!(is_in_san_fang_si_zheng(Yin, EarthlyBranch::Wu));
        assert!(!is_in_san_fang_si_zheng(Yin, Mao));
        // Sanity: a few unrelated branches are excluded.
        for b in [Chen, Si, Wei, You] {
            assert!(!is_in_san_fang_si_zheng(Yin, b));
        }
    }

    #[test]
    fn is_in_trine_group_includes_anchor() {
        assert!(is_in_trine_group(Yin, Yin)); // self
        assert!(is_in_trine_group(Yin, EarthlyBranch::Wu)); // +4
        assert!(is_in_trine_group(Yin, EarthlyBranch::Xu)); // +8
        assert!(!is_in_trine_group(Yin, Shen)); // opposite is not a trine member
    }

    #[test]
    fn clamp_branches_are_neighbours() {
        assert_eq!(clamp_branches(Mao), [Yin, Chen]);
    }
}
