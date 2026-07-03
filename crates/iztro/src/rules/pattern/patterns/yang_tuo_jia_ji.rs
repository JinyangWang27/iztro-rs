//! 羊陀夹忌 — Qing Yang (擎羊) and Tuo Luo (陀罗) clamping the palace that holds a
//! 化忌 (Ji mutagen) star.
//!
//! 成格: a star carries 化忌, and the two palaces clamping (夹) that star's palace
//! are occupied — one by 擎羊 and the other by 陀罗. Natal 化忌 is read from natal
//! placements; temporal 化忌 from scoped [`MutagenActivation`] facts.
//!
//! Star matching is exact per scope: the natal scope queries 擎羊/陀罗 exactly,
//! while temporal scopes query the scope-specific flow blades explicitly (运羊/
//! 运陀, 流羊/流陀, …) via [`StarFamily::exact_member_for_scope`]. There is no hidden
//! base↔flow equivalence — 擎羊 never silently matches 流羊.
//! 减力/破格: no weakening/breaker policy is modeled, so integrity is always
//! fulfilled.
//!
//! [`MutagenActivation`]: crate::core::MutagenActivation

use crate::core::{EarthlyBranch, Mutagen, Scope, StarFamily, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::predicates::clamp::clamp_branches;
use crate::rules::pattern::query::{
    find_star_branch_for_scope, mutagen_activations_for_scope, scope_is_visible,
};
use crate::rules::relation::PalaceRelation;

/// Detects 羊陀夹忌 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        for base in detect_base_formations(ctx, scope) {
            let integrity = assess_integrity(ctx, &base);
            emit::push_detection(out, base, integrity);
        }
    }
}

/// 成格: every 化忌-carrying palace clamped by 擎羊 and 陀罗 in this scope.
fn detect_base_formations(ctx: &PatternContext<'_>, scope: Scope) -> Vec<FormationMatch> {
    let mut out = Vec::new();
    if !scope_is_visible(ctx, scope) {
        return out;
    }

    // Exact per-scope blade identities: 擎羊/陀罗 for natal, the scope-specific
    // flow blades (运羊/运陀, 流羊/流陀, …) for temporal scopes. Base blades never
    // silently match flow blades.
    let yang_star = StarFamily::Yang.exact_member_for_scope(scope);
    let tuo_star = StarFamily::Tuo.exact_member_for_scope(scope);

    let Some((qing_yang, qing_yang_branch)) = find_star_branch_for_scope(ctx, scope, yang_star)
    else {
        return out;
    };
    let Some((tuo_luo, tuo_luo_branch)) = find_star_branch_for_scope(ctx, scope, tuo_star) else {
        return out;
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

        out.push(FormationMatch {
            id: PatternId::YangTuoJiaJi,
            scope,
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
        });
    }

    out
}

fn ji_targets(ctx: &PatternContext<'_>, scope: Scope) -> Vec<(StarName, EarthlyBranch)> {
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

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
