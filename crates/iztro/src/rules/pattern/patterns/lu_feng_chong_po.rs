//! 禄逢冲破 — a 禄 base in the Life palace broken by 地空/地劫 in its 三方四正.
//!
//! 成格: 禄存 or a 化禄-carrying star (natal or temporal activation) sits in the Life
//! palace itself.
//! 破格: 地空/地劫 within the Life 三方四正 break the formation. This pattern is only
//! emitted when a breaker is present, so its status is always
//! [`PatternStatus::Broken`]. Arbitrary 煞星 or 空亡族 stars are not accepted.
//!
//! [`PatternStatus::Broken`]: crate::rules::pattern::model::PatternStatus::Broken

use crate::core::{EarthlyBranch, Mutagen, PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternId,
};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::breakers::selected_kong_jie_in_san_fang_si_zheng;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::query::{selected_branch_of_palace, selected_stars_in_palace};

/// Detects 禄逢冲破 and appends any detection to `out`.
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

/// 成格 + breaker discovery: 禄坐命 with 地空/地劫 冲破 in the 三方四正. Both the 禄
/// base and the breakers are folded into the involved facts.
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let life_branch = selected_branch_of_palace(ctx, PalaceName::Life)?;

    let lu_base = selected_lu_base_in_life(ctx, life_branch)?;

    let breakers = selected_kong_jie_in_san_fang_si_zheng(ctx, life_branch);
    if breakers.is_empty() {
        return None;
    }

    let mut involved_palaces = vec![life_branch];
    involved_palaces.extend(breakers.iter().map(|(_, branch)| *branch));
    sort_dedup_branches(&mut involved_palaces);

    let mut involved_stars = vec![lu_base.star];
    involved_stars.extend(breakers.iter().map(|(star, _)| *star));
    involved_stars.sort();
    involved_stars.dedup();

    let mut involved_mutagens = Vec::new();
    if let Some(mutagen) = lu_base.mutagen {
        involved_mutagens.push(mutagen);
    }

    let mut breaker_stars: Vec<StarName> = breakers.iter().map(|(star, _)| *star).collect();
    breaker_stars.sort();
    breaker_stars.dedup();
    let mut breaker_branches: Vec<EarthlyBranch> =
        breakers.iter().map(|(_, branch)| *branch).collect();
    sort_dedup_branches(&mut breaker_branches);

    let mut evidence = vec![lu_base.evidence];
    evidence.push(PatternEvidence::StarsInSanFangSiZheng {
        stars: breaker_stars,
        anchor: life_branch,
        branches: breaker_branches,
    });

    Some(FormationMatch {
        id: PatternId::LuFengChongPo,
        scope,
        anchor: PatternAnchor::Palace(life_branch),
        involved_palaces,
        involved_stars,
        involved_mutagens,
        evidence,
    })
}

/// 破格: the 地空/地劫 冲破 stars, as breaking factors.
fn assess_integrity(ctx: &PatternContext<'_>, base: &FormationMatch) -> IntegrityAssessment {
    let PatternAnchor::Palace(life_branch) = base.anchor else {
        return IntegrityAssessment::fulfilled();
    };
    let breaking_factors = selected_kong_jie_in_san_fang_si_zheng(ctx, life_branch)
        .into_iter()
        .map(|(star, branch)| PatternCondition::BrokenByStar { star, branch })
        .collect();
    IntegrityAssessment::broken(breaking_factors)
}

struct LuBase {
    star: StarName,
    mutagen: Option<Mutagen>,
    evidence: PatternEvidence,
}

/// Returns a 禄 base (禄存 or 化禄) sitting in the Life palace itself.
fn selected_lu_base_in_life(
    ctx: &PatternContext<'_>,
    life_branch: EarthlyBranch,
) -> Option<LuBase> {
    for placement in selected_stars_in_palace(ctx, life_branch) {
        let star = placement.placement().name();
        if star == StarName::LuCun {
            return Some(LuBase {
                star,
                mutagen: None,
                evidence: PatternEvidence::StarInPalace {
                    star,
                    branch: life_branch,
                },
            });
        }
        if placement.placement().mutagen() == Some(Mutagen::Lu) {
            return Some(LuBase {
                star,
                mutagen: Some(Mutagen::Lu),
                evidence: PatternEvidence::MutagenOnStar {
                    star,
                    mutagen: Mutagen::Lu,
                    scope: placement.source_scope(),
                    branch: life_branch,
                },
            });
        }
    }

    if let Some(state) = ctx.effective() {
        for activation in state.mutagen_activations() {
            let activation_fact = activation.activation();
            if activation_fact.mutagen() == Mutagen::Lu
                && activation_fact.target_branch() == life_branch
            {
                return Some(LuBase {
                    star: activation_fact.target_star(),
                    mutagen: Some(Mutagen::Lu),
                    evidence: PatternEvidence::MutagenOnStar {
                        star: activation_fact.target_star(),
                        mutagen: Mutagen::Lu,
                        scope: activation.source_scope(),
                        branch: life_branch,
                    },
                });
            }
        }
    }

    None
}
