//! 左右夹命 — Zuo Fu (左辅) and You Bi (右弼) clamping the Life palace.
//!
//! Conservative condition: the two palaces clamping (夹) the Life palace are
//! occupied — one by 左辅 and the other by 右弼, in either orientation.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternStatus, PatternStrength,
};
use crate::core::pattern::query::{
    branch_of_palace_for_scope, clamp_pair_matches_for_scope, pattern_scope_for,
};
use crate::core::pattern::relation::PalaceRelation;
use crate::core::{PalaceName, StarName};

const NAME_ZH: &str = "左右夹命";

/// Detects 左右夹命 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        let Some(anchor) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
            continue;
        };

        let Some([(low_star, low_branch), (high_star, high_branch)]) =
            clamp_pair_matches_for_scope(ctx, scope, anchor, StarName::ZuoFu, StarName::YouBi)
        else {
            continue;
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
            scope: pattern_scope_for(scope),
            anchor: PatternAnchor::Palace(anchor),
            involved_palaces,
            involved_stars: vec![low_star, high_star],
            involved_mutagens: Vec::new(),
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
            weakening_factors: Vec::new(),
            breaking_factors: Vec::new(),
        });
    }
}
