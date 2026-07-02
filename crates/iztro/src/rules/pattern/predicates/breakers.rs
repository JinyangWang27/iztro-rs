//! Shared breaker-star discovery within a 三方四正.
//!
//! These helpers locate currently-modelled breaker stars. They only report where
//! a breaker star sits; whether that breaks, weakens, or does nothing to a
//! formation is decided by the named detector.

use crate::core::{EarthlyBranch, StarName};
use crate::rules::pattern::context::PatternContext;
use crate::rules::pattern::predicates::sanfang::san_fang_si_zheng;
use crate::rules::pattern::query::selected_stars_in_palace;

/// Returns each 地空/地劫 found within the 三方四正 of `anchor`, with its branch,
/// reading the selected (effective) view.
pub(crate) fn selected_kong_jie_in_san_fang_si_zheng(
    ctx: &PatternContext<'_>,
    anchor: EarthlyBranch,
) -> Vec<(StarName, EarthlyBranch)> {
    let mut found = Vec::new();
    for branch in san_fang_si_zheng(anchor) {
        for placement in selected_stars_in_palace(ctx, branch) {
            let star = placement.placement().name();
            if matches!(star, StarName::DiKong | StarName::DiJie) {
                found.push((star, branch));
            }
        }
    }
    found
}
