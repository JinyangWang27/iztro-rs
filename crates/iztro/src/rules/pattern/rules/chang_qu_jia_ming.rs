//! 昌曲夹命 — Wen Chang (文昌) and Wen Qu (文曲) clamping the Life palace.
//!
//! Conservative condition: the two palaces clamping (夹) the Life palace are
//! occupied — one by 文昌 and the other by 文曲, in either orientation. Runtime
//! flow 昌/曲 stars match only within their requested temporal scope.

use crate::core::{PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternStatus, PatternStrength,
};
use crate::rules::pattern::query::{
    effective_branch_of_palace, effective_clamp_pair_matches, pattern_scope_for,
};
use crate::rules::pattern::relation::PalaceRelation;

const NAME_ZH: &str = "昌曲夹命";

/// Detects 昌曲夹命 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        let Some(anchor) = effective_branch_of_palace(ctx, scope, PalaceName::Life) else {
            continue;
        };

        let Some([(low_star, low_branch), (high_star, high_branch)]) =
            effective_clamp_pair_matches(ctx, scope, anchor, StarName::WenChang, StarName::WenQu)
        else {
            continue;
        };

        let mut involved_palaces = vec![anchor, low_branch, high_branch];
        involved_palaces.sort_by_key(|branch| branch.index());
        involved_palaces.dedup();

        out.push(PatternDetection {
            id: PatternId::ChangQuJiaMing,
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
