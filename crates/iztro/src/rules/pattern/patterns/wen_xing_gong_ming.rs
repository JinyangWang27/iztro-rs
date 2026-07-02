//! 文星拱命 — Wen Chang (文昌) and Wen Qu (文曲) in the Life palace 三方四正.
//!
//! 成格: both 文昌 and 文曲 appear within the selected Life palace 三方四正.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::sanfang::selected_stars_in_san_fang_si_zheng;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::query::selected_branch_of_palace;

/// Detects 文星拱命 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(base) = detect_base_formation(ctx, request) else {
        return;
    };
    emit::push_detection(out, base, IntegrityAssessment::fulfilled());
}

/// 成格: 文昌、文曲皆现于命宫三方四正。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let anchor = selected_branch_of_palace(ctx, PalaceName::Life)?;

    let found =
        selected_stars_in_san_fang_si_zheng(ctx, anchor, &[StarName::WenChang, StarName::WenQu]);
    if selected_stars_in_san_fang_si_zheng(ctx, anchor, &[StarName::WenChang]).is_empty()
        || selected_stars_in_san_fang_si_zheng(ctx, anchor, &[StarName::WenQu]).is_empty()
    {
        return None;
    }

    let mut branches: Vec<EarthlyBranch> = found.iter().map(|(_, branch)| *branch).collect();
    sort_dedup_branches(&mut branches);
    let mut stars: Vec<StarName> = found.iter().map(|(star, _)| *star).collect();
    stars.sort();
    stars.dedup();

    Some(FormationMatch {
        id: PatternId::WenXingGongMing,
        scope,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces: branches.clone(),
        involved_stars: stars.clone(),
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSanFangSiZheng {
            stars,
            anchor,
            branches,
        }],
    })
}
