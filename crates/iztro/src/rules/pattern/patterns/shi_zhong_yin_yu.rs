//! 石中隐玉 — Ju Men (巨门) guarding the Life palace at 子 or 午, with auxiliary
//! support in the Life 三方四正.
//!
//! Source-backed (斗数骨髓赋). 成格: the selected Life palace branch is 子 or 午,
//! 巨门 sits in it, and the explicit support set (禄存／左右／曲昌／魁钺 plus 化禄/权/科)
//! appears in the Life 三方四正.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::predicates::support::selected_support_in_san_fang_si_zheng;
use crate::rules::pattern::query::{selected_branch_of_palace, selected_star_in_palace};

/// Detects 石中隐玉 and appends any detection to `out`.
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

/// 成格: 子/午命宫，巨门坐命，命宫三方四正加会吉辅。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    use crate::core::EarthlyBranch::{Wu, Zi};

    let scope = requested_selected_scope(ctx, request)?;
    let life = selected_branch_of_palace(ctx, PalaceName::Life)?;
    if life != Zi && life != Wu {
        return None;
    }

    let ju_men = selected_star_in_palace(ctx, life, StarName::JuMen)?;
    let star = ju_men.placement().name();

    let support = selected_support_in_san_fang_si_zheng(ctx, life);
    if support.is_empty() {
        return None;
    }

    let mut involved_palaces = vec![life];
    involved_palaces.extend(support.branches.iter().copied());
    sort_dedup_branches(&mut involved_palaces);

    let mut involved_stars = vec![star];
    involved_stars.extend(support.involved_stars());
    involved_stars.sort();
    involved_stars.dedup();

    let mut evidence = vec![PatternEvidence::StarInPalace { star, branch: life }];
    evidence.extend(support.evidence());

    Some(FormationMatch {
        id: PatternId::ShiZhongYinYu,
        scope,
        anchor: PatternAnchor::Palace(life),
        involved_palaces,
        involved_stars,
        involved_mutagens: support.involved_mutagens(),
        evidence,
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
