//! 羊陀夹忌 — Qing Yang (擎羊) and Tuo Luo (陀罗) clamping the palace that holds a
//! 化忌 (Ji mutagen) star.
//!
//! Conservative condition: a natal star carries 化忌, and the two palaces
//! clamping (夹) that star's palace are occupied — one by 擎羊 and the other by
//! 陀罗. Natal 化忌 is taken from natal placements; temporal 化忌 is read from
//! scoped [`MutagenActivation`] facts.

use crate::core::{Mutagen, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::display_metadata::pattern_display_metadata;
use crate::rules::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternStatus, PatternStrength,
};
use crate::rules::pattern::query::{
    find_star_branch_for_scope, mutagen_activations_for_scope, pattern_scope_for, scope_is_visible,
};
use crate::rules::pattern::relation::{PalaceRelation, clamp_branches};

/// Detects 羊陀夹忌 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        if !scope_is_visible(ctx, scope) {
            continue;
        }

        let Some((qing_yang, qing_yang_branch)) =
            find_star_branch_for_scope(ctx, scope, StarName::QingYang)
        else {
            continue;
        };
        let Some((tuo_luo, tuo_luo_branch)) =
            find_star_branch_for_scope(ctx, scope, StarName::TuoLuo)
        else {
            continue;
        };

        for (target_star, target_branch) in ji_targets(ctx, scope) {
            let [low, high] = clamp_branches(target_branch);

            let clamps_target = qing_yang_branch != tuo_luo_branch
                && (qing_yang_branch == low || qing_yang_branch == high)
                && (tuo_luo_branch == low || tuo_luo_branch == high);
            if !clamps_target {
                continue;
            }

            let mut involved_palaces = vec![low, high, target_branch];
            involved_palaces.sort_by_key(|branch| branch.index());
            involved_palaces.dedup();

            out.push(PatternDetection {
                id: PatternId::YangTuoJiaJi,
                name_zh: pattern_display_metadata(PatternId::YangTuoJiaJi).name_zh,
                family: PatternFamily::ShaJi,
                polarity: PatternPolarity::Inauspicious,
                status: PatternStatus::Fulfilled,
                strength: PatternStrength::Medium,
                scope: pattern_scope_for(scope),
                anchor: PatternAnchor::Palace(target_branch),
                involved_palaces,
                involved_stars: vec![qing_yang, tuo_luo, target_star],
                involved_mutagens: vec![Mutagen::Ji],
                evidence: vec![
                    PatternEvidence::StarInPalace {
                        star: qing_yang,
                        branch: qing_yang_branch,
                    },
                    PatternEvidence::StarInPalace {
                        star: tuo_luo,
                        branch: tuo_luo_branch,
                    },
                    PatternEvidence::MutagenOnStar {
                        star: target_star,
                        mutagen: Mutagen::Ji,
                        scope,
                        branch: target_branch,
                    },
                    PatternEvidence::PalaceRelation {
                        from: target_branch,
                        to: qing_yang_branch,
                        relation: PalaceRelation::ClampedBy,
                    },
                    PatternEvidence::PalaceRelation {
                        from: target_branch,
                        to: tuo_luo_branch,
                        relation: PalaceRelation::ClampedBy,
                    },
                ],
                weakening_factors: Vec::new(),
                breaking_factors: Vec::new(),
            });
        }
    }
}

fn ji_targets(
    ctx: &PatternContext<'_>,
    scope: Scope,
) -> Vec<(StarName, crate::core::EarthlyBranch)> {
    if scope == Scope::Natal {
        return ctx
            .chart()
            .stars()
            .into_iter()
            .filter(|fact| fact.placement().mutagen() == Some(Mutagen::Ji))
            .map(|fact| (fact.placement().name(), fact.palace().branch()))
            .collect();
    }

    mutagen_activations_for_scope(ctx, scope)
        .into_iter()
        .filter(|activation| activation.mutagen() == Mutagen::Ji)
        .map(|activation| (activation.target_star(), activation.target_branch()))
        .collect()
}
