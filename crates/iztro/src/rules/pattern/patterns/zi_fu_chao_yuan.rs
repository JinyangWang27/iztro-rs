//! 紫府朝垣 — Zi Wei (紫微) and Tian Fu (天府) arranged around the Life palace.
//!
//! 成格: both 紫微 and 天府 fall within the 三方四正 of the Life palace.
//! 减力: an adverse [`StarKind::Tough`] star also sitting in one of the involved
//! palaces weakens the formation ([`PatternStatus::Weakened`], 成而减力).
//!
//! [`PatternStatus::Weakened`]: crate::rules::pattern::model::PatternStatus::Weakened

use crate::core::{EarthlyBranch, PalaceName, Scope, StarKind, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternId,
};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::predicates::sanfang::stars_in_san_fang_si_zheng_for_scope;
use crate::rules::pattern::query::{branch_of_palace_for_scope, stars_in_palace_for_scope};

const REQUIRED: [StarName; 2] = [StarName::ZiWei, StarName::TianFu];

/// Detects 紫府朝垣 and appends any detection to `out`.
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

/// 成格: 紫微 and 天府 both fall within the Life 三方四正.
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let anchor = branch_of_palace_for_scope(ctx, scope, PalaceName::Life)?;

    let found = stars_in_san_fang_si_zheng_for_scope(ctx, scope, anchor, &REQUIRED);
    let has_ziwei = found.iter().any(|(star, _)| *star == StarName::ZiWei);
    let has_tianfu = found.iter().any(|(star, _)| *star == StarName::TianFu);
    if !(has_ziwei && has_tianfu) {
        return None;
    }

    let mut branches: Vec<EarthlyBranch> = found.iter().map(|(_, branch)| *branch).collect();
    branches.sort_by_key(|branch| branch.index());
    branches.dedup();

    let involved_stars: Vec<StarName> = REQUIRED
        .iter()
        .copied()
        .filter(|star| found.iter().any(|(found_star, _)| found_star == star))
        .collect();

    Some(FormationMatch {
        id: PatternId::ZiFuChaoYuan,
        scope,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces: branches.clone(),
        involved_stars,
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSanFangSiZheng {
            stars: REQUIRED.to_vec(),
            anchor,
            branches,
        }],
    })
}

/// 减力: any 煞 ([`StarKind::Tough`]) star inside an involved palace weakens it.
fn assess_integrity(ctx: &PatternContext<'_>, base: &FormationMatch) -> IntegrityAssessment {
    let mut weakening_factors: Vec<PatternCondition> = Vec::new();
    for &branch in &base.involved_palaces {
        for placement in stars_in_palace_for_scope(ctx, base.scope, branch) {
            if placement.placement().kind() == StarKind::Tough {
                weakening_factors.push(PatternCondition::WeakenedByStar {
                    star: placement.placement().name(),
                    branch,
                });
            }
        }
    }

    if weakening_factors.is_empty() {
        IntegrityAssessment::fulfilled()
    } else {
        IntegrityAssessment::weakened(weakening_factors)
    }
}
