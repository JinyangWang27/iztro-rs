//! 极向离明 — Zi Wei (紫微) guarding the Life palace at 午.
//!
//! 成格: 紫微 sits in the Life palace and the Life palace branch is 午.
//! 破格: a 煞 ([`StarKind::Tough`]) star anywhere in the Life 三方四正 breaks the
//! formation ([`PatternStatus::Broken`]). When no breaker is present the formation
//! is fulfilled.
//!
//! [`PatternStatus::Broken`]: crate::rules::pattern::model::PatternStatus::Broken

use crate::core::{EarthlyBranch, PalaceName, StarKind, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternId,
};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::sanfang::san_fang_si_zheng;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::query::{
    selected_branch_of_palace, selected_palace_has_all_stars, selected_stars_in_palace,
};

/// Detects 极向离明 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(base) = detect_base_formation(ctx, request) else {
        return;
    };
    let integrity = assess_integrity(ctx, &base);
    emit::push_detection(out, base, integrity);
}

/// 成格: 紫微坐午宫命宫。Any 三方四正 breaker is folded into the involved facts so a
/// broken detection still reports what broke it.
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let branch = EarthlyBranch::Wu;
    if selected_branch_of_palace(ctx, PalaceName::Life) != Some(branch)
        || !selected_palace_has_all_stars(ctx, branch, &[StarName::ZiWei])
    {
        return None;
    }

    let mut involved_palaces = vec![branch];
    let mut involved_stars = vec![StarName::ZiWei];
    let mut evidence = vec![PatternEvidence::StarInPalace {
        star: StarName::ZiWei,
        branch,
    }];
    if let Some((star, breaker_branch)) = breaker(ctx, branch) {
        involved_palaces.push(breaker_branch);
        involved_stars.push(star);
        evidence.push(PatternEvidence::StarInPalace {
            star,
            branch: breaker_branch,
        });
    }
    sort_dedup_branches(&mut involved_palaces);
    involved_stars.sort();
    involved_stars.dedup();

    Some(FormationMatch {
        id: PatternId::JiXiangLiMing,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces,
        involved_stars,
        involved_mutagens: Vec::new(),
        evidence,
    })
}

/// 破格: a 煞星 in the 三方四正 breaks the formation.
fn assess_integrity(ctx: &PatternContext<'_>, base: &FormationMatch) -> IntegrityAssessment {
    let PatternAnchor::Palace(branch) = base.anchor else {
        return IntegrityAssessment::fulfilled();
    };
    match breaker(ctx, branch) {
        Some((star, breaker_branch)) => {
            IntegrityAssessment::broken(vec![PatternCondition::BrokenByStar {
                star,
                branch: breaker_branch,
            }])
        }
        None => IntegrityAssessment::fulfilled(),
    }
}

/// Returns the first 煞 ([`StarKind::Tough`]) star in the 三方四正 of `anchor`.
fn breaker(ctx: &PatternContext<'_>, anchor: EarthlyBranch) -> Option<(StarName, EarthlyBranch)> {
    san_fang_si_zheng(anchor).into_iter().find_map(|candidate| {
        selected_stars_in_palace(ctx, candidate)
            .into_iter()
            .find(|placement| placement.placement().kind() == StarKind::Tough)
            .map(|placement| (placement.placement().name(), candidate))
    })
}
