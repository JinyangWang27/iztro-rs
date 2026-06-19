//! Read-only chart query helpers shared by pattern rules.
//!
//! These helpers borrow chart facts and never mutate them. They centralize
//! common lookups so individual rules stay small and consistent.

use crate::core::pattern::relation::{clamp_branches, san_fang_si_zheng};
use crate::core::{Brightness, Chart, EarthlyBranch, StarKind, StarName, StarPlacement};

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

/// Returns the two stars clamping (夹) the palace at `anchor` when `left_star`
/// and `right_star` occupy its two clamp palaces, one on each side.
///
/// The two clamp palaces are the `-1` and `+1` neighbours of `anchor` (see
/// [`clamp_branches`]); the lower-offset neighbour is the "low" clamp and the
/// higher-offset neighbour is the "high" clamp. Either orientation matches:
/// `left_star` in the low clamp with `right_star` in the high clamp, or the
/// reverse. The returned array is always ordered `[(low_star, low_branch),
/// (high_star, high_branch)]` so callers get a stable clamp ordering. Returns
/// `None` unless both clamp palaces are occupied, one by each requested star.
pub fn clamp_pair_matches(
    chart: &Chart,
    anchor: EarthlyBranch,
    left_star: StarName,
    right_star: StarName,
) -> Option<[(StarName, EarthlyBranch); 2]> {
    let [low, high] = clamp_branches(anchor);

    if palace_has_star(chart, low, left_star) && palace_has_star(chart, high, right_star) {
        Some([(left_star, low), (right_star, high)])
    } else if palace_has_star(chart, low, right_star) && palace_has_star(chart, high, left_star) {
        Some([(right_star, low), (left_star, high)])
    } else {
        None
    }
}

/// Returns whether `brightness` is a clearly bright/auspicious state
/// (庙/旺/得/利).
///
/// Conservative by design: `Flat` (平) is treated as neutral, and `Weak`,
/// `Trapped`, and `Unknown` are never bright. Rules that depend on brightness
/// must not emit when brightness is `Unknown`; this helper returning `false`
/// for `Unknown` enforces that for the bright case.
pub fn is_bright(brightness: Brightness) -> bool {
    matches!(
        brightness,
        Brightness::Temple
            | Brightness::Prosperous
            | Brightness::Advantage
            | Brightness::Favourable
    )
}

/// Returns whether `brightness` is a clearly dim/fallen state (不/陷).
///
/// Conservative by design: only `Weak` (不) and `Trapped` (陷) count. `Flat`
/// (平) is neutral, and `Unknown` is never dim, so a rule gated on this helper
/// never emits on an uncalculated brightness.
pub fn is_dim(brightness: Brightness) -> bool {
    matches!(brightness, Brightness::Weak | Brightness::Trapped)
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
