//! 机月同梁 — Tian Ji (天机), Tai Yin (太阴), Tian Tong (天同), Tian Liang (天梁)
//! gathered through the 三方四正 of the Life palace.
//!
//! 成格: all four stars fall within the Life 三方四正. An incomplete formation is
//! not a near-pattern and produces no detection.
//! 减力/破格: no weakening/breaker policy is modeled, so integrity is always
//! [`IntegrityAssessment::fulfilled`].

use crate::core::{EarthlyBranch, PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::predicates::sanfang::effective_stars_in_san_fang_si_zheng;
use crate::rules::pattern::query::effective_branch_of_palace;

const REQUIRED: [StarName; 4] = [
    StarName::TianJi,
    StarName::TaiYin,
    StarName::TianTong,
    StarName::TianLiang,
];

/// Detects 机月同梁 and appends any detection to `out`.
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

/// 成格: 天机、太阴、天同、天梁 all present in the Life 三方四正.
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let anchor = effective_branch_of_palace(ctx, scope, PalaceName::Life)?;

    let found = effective_stars_in_san_fang_si_zheng(ctx, scope, anchor, &REQUIRED);
    let all_present = REQUIRED
        .iter()
        .all(|star| found.iter().any(|(found_star, _)| found_star == star));
    if !all_present {
        return None;
    }

    let mut branches: Vec<EarthlyBranch> = found.iter().map(|(_, branch)| *branch).collect();
    branches.sort_by_key(|branch| branch.index());
    branches.dedup();

    let evidence: Vec<PatternEvidence> = found
        .iter()
        .map(|(star, branch)| PatternEvidence::StarInPalace {
            star: *star,
            branch: *branch,
        })
        .collect();

    Some(FormationMatch {
        id: PatternId::JiYueTongLiang,
        scope,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces: branches,
        involved_stars: REQUIRED.to_vec(),
        involved_mutagens: Vec::new(),
        evidence,
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
