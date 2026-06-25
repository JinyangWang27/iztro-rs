//! Hand-coded predicates for the pilot classical rules.
//!
//! These reuse the read-only chart query helpers from [`crate::core::pattern`]
//! (clamp matching, brightness classification, star lookup) so the classical rule
//! engine introduces no second copy of that logic. Each predicate returns the
//! typed facts a rule needs to build machine-readable evidence; it never builds a
//! claim or any prose.

use crate::core::pattern::query::{clamp_pair_matches, find_star_branch, is_dim, stars_in_palace};
use crate::core::{Brightness, Chart, EarthlyBranch, StarName};
use crate::rules::classical::void::{VoidKind, VoidPolicy};

/// 天马 sharing a palace with a modeled 空亡-family star.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TianMaVoid {
    /// The branch 天马 occupies (shared with the void star).
    pub tian_ma_branch: EarthlyBranch,
    /// The kind of void affecting 天马.
    pub void_kind: VoidKind,
}

/// Returns the 空亡 fact affecting 天马, if any void star counted by `policy`
/// shares 天马's palace.
///
/// Conservative: only modeled 空亡-family stars qualify (see [`VoidKind`]); 天空,
/// 地空, and 地劫 are never treated as 空亡.
pub fn tian_ma_affected_by_void(chart: &Chart, policy: VoidPolicy) -> Option<TianMaVoid> {
    let branch = find_star_branch(chart, StarName::TianMa)?;
    stars_in_palace(chart, branch).into_iter().find_map(|placement| {
        VoidKind::from_star(placement.name())
            .filter(|kind| policy.includes(*kind))
            .map(|void_kind| TianMaVoid {
                tian_ma_branch: branch,
                void_kind,
            })
    })
}

/// Two stars clamping (夹) the Life palace, one on each adjacent branch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LifeClamp {
    /// The Life palace branch being clamped.
    pub life_branch: EarthlyBranch,
    /// The clamping star in the lower-offset (`-1`) neighbour, with its branch.
    pub low: (StarName, EarthlyBranch),
    /// The clamping star in the higher-offset (`+1`) neighbour, with its branch.
    pub high: (StarName, EarthlyBranch),
}

/// Returns the clamp fact when `a` and `b` occupy the two palaces clamping the
/// Life palace, one on each side (either orientation). Reuses
/// [`clamp_pair_matches`].
pub fn stars_clamp_life(chart: &Chart, a: StarName, b: StarName) -> Option<LifeClamp> {
    let life_branch = chart.life_palace()?.branch();
    let [low, high] = clamp_pair_matches(chart, life_branch, a, b)?;
    Some(LifeClamp {
        life_branch,
        low,
        high,
    })
}

/// 太阳 and 太阴 both in clearly dim/fallen (不/陷) brightness states.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RiYueDim {
    /// 太阳's branch and dim brightness.
    pub sun: (EarthlyBranch, Brightness),
    /// 太阴's branch and dim brightness.
    pub moon: (EarthlyBranch, Brightness),
}

/// Returns the brightness facts when both 太阳 and 太阴 are present and each is
/// [`is_dim`]. Conservative: `Unknown`/`Flat` never qualify.
pub fn sun_and_moon_dim(chart: &Chart) -> Option<RiYueDim> {
    let sun = chart.star(StarName::TaiYang)?;
    let moon = chart.star(StarName::TaiYin)?;
    let sun_brightness = sun.placement().brightness();
    let moon_brightness = moon.placement().brightness();
    if is_dim(sun_brightness) && is_dim(moon_brightness) {
        Some(RiYueDim {
            sun: (sun.palace().branch(), sun_brightness),
            moon: (moon.palace().branch(), moon_brightness),
        })
    } else {
        None
    }
}
