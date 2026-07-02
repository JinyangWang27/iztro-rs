//! 擎羊入庙 (羊刃入庙) — Qing Yang (擎羊) guarding the Life palace at 辰/戌/丑/未, with
//! auxiliary support in the Life 三方四正.
//!
//! Source-backed (定贵局). 成格: the selected Life palace branch is 辰/戌/丑/未, 擎羊
//! sits in it, and the explicit support set (禄存／左右／曲昌／魁钺 plus 化禄/权/科)
//! appears in the Life 三方四正. The support is **constitutive** here: the source
//! (辰戍丑未守命遇吉是也 / 加吉万论) makes 遇吉 part of the base formation, so no
//! support means no detection.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::predicates::support::selected_support_in_san_fang_si_zheng;
use crate::rules::pattern::query::{selected_branch_of_palace, selected_star_in_palace};

/// The 四墓 (四库) branches where 擎羊 is considered 入庙.
const RU_MIAO_BRANCHES: [EarthlyBranch; 4] = [
    EarthlyBranch::Chen,
    EarthlyBranch::Xu,
    EarthlyBranch::Chou,
    EarthlyBranch::Wei,
];

/// Detects 擎羊入庙 and appends any detection to `out`.
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

/// 成格: 辰戌丑未命宫，擎羊坐命，命宫三方四正加会吉辅。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let life = selected_branch_of_palace(ctx, PalaceName::Life)?;
    if !RU_MIAO_BRANCHES.contains(&life) {
        return None;
    }

    let qing_yang = selected_star_in_palace(ctx, life, StarName::QingYang)?;
    let star = qing_yang.placement().name();

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
        id: PatternId::QingYangRuMiao,
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
