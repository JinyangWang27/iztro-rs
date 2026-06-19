//! Read-only chart query helpers shared by pattern rules.
//!
//! These helpers borrow chart facts and never mutate them. They centralize
//! common lookups so individual rules stay small and consistent.

use crate::core::pattern::relation::san_fang_si_zheng;
use crate::core::{Chart, EarthlyBranch, StarKind, StarName, StarPlacement};

/// Returns the typed star placements in the palace at `branch`.
pub fn stars_in_palace(chart: &Chart, branch: EarthlyBranch) -> Vec<&StarPlacement> {
    chart
        .palaces()
        .iter()
        .filter(|palace| palace.branch() == branch)
        .flat_map(|palace| palace.stars().iter())
        .collect()
}

/// Returns whether `star` occupies the palace at `branch`.
pub fn palace_has_star(chart: &Chart, branch: EarthlyBranch, star: StarName) -> bool {
    stars_in_palace(chart, branch)
        .iter()
        .any(|placement| placement.name() == star)
}

/// Returns the branch of the palace containing `star`, if present.
pub fn find_star_branch(chart: &Chart, star: StarName) -> Option<EarthlyBranch> {
    chart.star(star).map(|fact| fact.palace().branch())
}

/// Returns each requested star found within the 三方四正 of `anchor`, paired with
/// the branch it was found in.
pub fn stars_in_san_fang_si_zheng(
    chart: &Chart,
    anchor: EarthlyBranch,
    stars: &[StarName],
) -> Vec<(StarName, EarthlyBranch)> {
    let mut found = Vec::new();
    for branch in san_fang_si_zheng(anchor) {
        for placement in stars_in_palace(chart, branch) {
            if stars.contains(&placement.name()) {
                found.push((placement.name(), branch));
            }
        }
    }
    found
}

/// Returns whether any adverse "煞星" (sha star) sits in the palace at `branch`.
///
/// Conservative by design: this uses the already-modeled [`StarKind::Tough`]
/// classification, which currently covers 擎羊 (QingYang), 陀罗 (TuoLuo),
/// 火星 (HuoXing), 铃星 (LingXing), 地空 (DiKong), and 地劫 (DiJie). No stars or
/// aliases are invented here.
pub fn any_sha_star_in_palace(chart: &Chart, branch: EarthlyBranch) -> bool {
    stars_in_palace(chart, branch)
        .iter()
        .any(|placement| placement.kind() == StarKind::Tough)
}
