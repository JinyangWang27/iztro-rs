//! 左右夹命 — Zuo Fu (左辅) and You Bi (右弼) clamping the Life palace.
//!
//! 成格: the two palaces clamping (夹) the Life palace are occupied — one by 左辅
//! and the other by 右弼, in either orientation.
//! 减力/破格: no weakening/breaker policy is modeled, so integrity is always
//! fulfilled.

use crate::core::{PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::predicates::clamp::effective_clamp_pair_matches;
use crate::rules::pattern::query::effective_branch_of_palace;
use crate::rules::pattern::relation::PalaceRelation;

/// Detects 左右夹命 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        let Some(base) = detect_base_formation(ctx, scope) else {
            continue;
        };
        let integrity = assess_integrity(ctx, &base);
        emit::push_detection(out, base, integrity);
    }
}

/// 成格: 左辅 and 右弼 occupy the two palaces clamping the Life palace.
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let anchor = effective_branch_of_palace(ctx, scope, PalaceName::Life)?;

    let [(low_star, low_branch), (high_star, high_branch)] =
        effective_clamp_pair_matches(ctx, scope, anchor, StarName::ZuoFu, StarName::YouBi)?;

    let mut involved_palaces = vec![anchor, low_branch, high_branch];
    involved_palaces.sort_by_key(|branch| branch.index());
    involved_palaces.dedup();

    Some(FormationMatch {
        id: PatternId::ZuoYouJiaMing,
        scope,
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
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
