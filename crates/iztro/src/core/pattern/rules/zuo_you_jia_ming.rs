//! 左右夹命 — Zuo Fu (左辅) and You Bi (右弼) clamping the Life palace.
//!
//! Conservative condition: the two palaces clamping (夹) the Life palace are
//! occupied — one by 左辅 and the other by 右弼, in either orientation. This rule
//! reads only natal facts and never mutates them.

use crate::core::StarName;
use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternScope, PatternStatus, PatternStrength,
};
use crate::core::pattern::query::clamp_pair_matches;
use crate::core::pattern::relation::PalaceRelation;

const NAME_ZH: &str = "左右夹命";

/// Detects 左右夹命 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    _request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let chart = ctx.chart;
    let Some(life) = chart.life_palace() else {
        return;
    };
    let anchor = life.branch();

    let Some([(low_star, low_branch), (high_star, high_branch)]) =
        clamp_pair_matches(chart, anchor, StarName::ZuoFu, StarName::YouBi)
    else {
        return;
    };

    let mut involved_palaces = vec![anchor, low_branch, high_branch];
    involved_palaces.sort_by_key(|branch| branch.index());
    involved_palaces.dedup();

    out.push(PatternDetection {
        id: PatternId::ZuoYouJiaMing,
        name_zh: NAME_ZH,
        family: PatternFamily::AuxiliaryStarCombination,
        polarity: PatternPolarity::Auspicious,
        status: PatternStatus::Fulfilled,
        strength: PatternStrength::Medium,
        scope: PatternScope::Natal,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces,
        involved_stars: vec![StarName::ZuoFu, StarName::YouBi],
        involved_mutagens: Vec::new(),
        // `ClampedBy` reads from the Life palace to each clamping palace: Life is
        // the anchor, the clamp palaces are the related ones.
        evidence: vec![
            PatternEvidence::StarInPalace {
                star: low_star,
                branch: low_branch,
            },
            PatternEvidence::StarInPalace {
                star: high_star,
                branch: high_branch,
            },
            PatternEvidence::PalaceRelation {
                from: anchor,
                to: low_branch,
                relation: PalaceRelation::ClampedBy,
            },
            PatternEvidence::PalaceRelation {
                from: anchor,
                to: high_branch,
                relation: PalaceRelation::ClampedBy,
            },
        ],
        missing_conditions: Vec::new(),
        weakening_factors: Vec::new(),
        breaking_factors: Vec::new(),
    });
}
