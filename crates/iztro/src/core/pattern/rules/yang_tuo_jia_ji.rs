//! 羊陀夹忌 — Qing Yang (擎羊) and Tuo Luo (陀罗) clamping the palace that holds a
//! 化忌 (Ji mutagen) star.
//!
//! Conservative condition: a natal star carries 化忌, and the two palaces
//! clamping (夹) that star's palace are occupied — one by 擎羊 and the other by
//! 陀罗. This rule reads only natal facts; the 化忌 is taken from the natal star
//! placement, never from a temporal overlay.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternScope, PatternStatus, PatternStrength,
};
use crate::core::pattern::query::find_star_branch;
use crate::core::pattern::relation::{PalaceRelation, clamp_branches};
use crate::core::{Mutagen, Scope, StarName};

const NAME_ZH: &str = "羊陀夹忌";

/// Detects 羊陀夹忌 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    _request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let chart = ctx.chart;

    // Find the natal star carrying 化忌, if any.
    let Some(ji) = chart
        .stars()
        .into_iter()
        .find(|fact| fact.placement().mutagen() == Some(Mutagen::Ji))
    else {
        return;
    };
    let target_star = ji.placement().name();
    let target_branch = ji.palace().branch();

    let [low, high] = clamp_branches(target_branch);

    let Some(qing_yang_branch) = find_star_branch(chart, StarName::QingYang) else {
        return;
    };
    let Some(tuo_luo_branch) = find_star_branch(chart, StarName::TuoLuo) else {
        return;
    };

    // Qing Yang and Tuo Luo must occupy the two distinct clamp palaces.
    let clamps_target = qing_yang_branch != tuo_luo_branch
        && (qing_yang_branch == low || qing_yang_branch == high)
        && (tuo_luo_branch == low || tuo_luo_branch == high);
    if !clamps_target {
        return;
    }

    let mut involved_palaces = vec![low, high, target_branch];
    involved_palaces.sort_by_key(|branch| branch.index());
    involved_palaces.dedup();

    out.push(PatternDetection {
        id: PatternId::YangTuoJiaJi,
        name_zh: NAME_ZH,
        family: PatternFamily::ShaJi,
        polarity: PatternPolarity::Inauspicious,
        status: PatternStatus::Fulfilled,
        strength: PatternStrength::Medium,
        scope: PatternScope::Natal,
        anchor: PatternAnchor::Palace(target_branch),
        involved_palaces,
        involved_stars: vec![StarName::QingYang, StarName::TuoLuo, target_star],
        involved_mutagens: vec![Mutagen::Ji],
        evidence: vec![
            PatternEvidence::StarInPalace {
                star: StarName::QingYang,
                branch: qing_yang_branch,
            },
            PatternEvidence::StarInPalace {
                star: StarName::TuoLuo,
                branch: tuo_luo_branch,
            },
            PatternEvidence::MutagenOnStar {
                star: target_star,
                mutagen: Mutagen::Ji,
                scope: Scope::Natal,
                branch: target_branch,
            },
            PatternEvidence::PalaceRelation {
                from: qing_yang_branch,
                to: target_branch,
                relation: PalaceRelation::ClampedBy,
            },
            PatternEvidence::PalaceRelation {
                from: tuo_luo_branch,
                to: target_branch,
                relation: PalaceRelation::ClampedBy,
            },
        ],
        missing_conditions: Vec::new(),
        weakening_factors: Vec::new(),
        breaking_factors: Vec::new(),
    });
}
