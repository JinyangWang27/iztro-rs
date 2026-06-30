//! 日月并明 — Tai Yang (太阳) and Tai Yin (太阴) both shining brightly.
//!
//! Conservative condition: both 太阳 and 太阴 are present and each sits in a
//! clearly bright/auspicious brightness state (庙/旺/得/利) per the existing
//! [`Brightness`] model. If either star's brightness is `Unknown` (or merely
//! `Flat`/dim), nothing is emitted — the rule never guesses an uncalculated
//! brightness. This reads only natal facts and never mutates them.
//!
//! [`Brightness`]: crate::core::Brightness

use crate::core::StarName;
use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternStatus, PatternStrength,
};
use crate::core::pattern::query::{find_star_for_scope, is_bright, pattern_scope_for};

const NAME_ZH: &str = "日月并明";

/// Detects 日月并明 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        let Some(sun) = find_star_for_scope(ctx, scope, StarName::TaiYang) else {
            continue;
        };
        let Some(moon) = find_star_for_scope(ctx, scope, StarName::TaiYin) else {
            continue;
        };

        if !is_bright(sun.placement().brightness()) || !is_bright(moon.placement().brightness()) {
            continue;
        }

        let sun_branch = sun.branch();
        let moon_branch = moon.branch();

        let mut involved_palaces = vec![sun_branch, moon_branch];
        involved_palaces.sort_by_key(|branch| branch.index());
        involved_palaces.dedup();

        out.push(PatternDetection {
            id: PatternId::RiYueBingMing,
            name_zh: NAME_ZH,
            family: PatternFamily::MajorStarCombination,
            polarity: PatternPolarity::Auspicious,
            status: PatternStatus::Fulfilled,
            strength: PatternStrength::Medium,
            scope: pattern_scope_for(scope),
            anchor: PatternAnchor::Chart,
            involved_palaces,
            involved_stars: vec![sun.placement().name(), moon.placement().name()],
            involved_mutagens: Vec::new(),
            evidence: vec![
                PatternEvidence::StarInPalace {
                    star: sun.placement().name(),
                    branch: sun_branch,
                },
                PatternEvidence::StarInPalace {
                    star: moon.placement().name(),
                    branch: moon_branch,
                },
            ],
            weakening_factors: Vec::new(),
            breaking_factors: Vec::new(),
        });
    }
}
