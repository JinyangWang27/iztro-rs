//! 府相朝垣 — Tian Fu (天府) and Tian Xiang (天相) supporting the Life palace, with
//! auxiliary support.
//!
//! 成格 (two alternative base forms, kept together in this file):
//! - Form A: 天府 and 天相 occupy the Wealth and Career palaces, one in each.
//! - Form B: 天府 sits in the Life palace and 天相 appears in the Life 三方四正.
//!
//! Either form additionally requires auxiliary support in the Life 三方四正.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::sanfang::selected_stars_in_san_fang_si_zheng;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::predicates::support::selected_support_in_san_fang_si_zheng;
use crate::rules::pattern::query::{selected_branch_of_palace, selected_palace_has_all_stars};

/// Detects 府相朝垣 and appends any detection to `out`.
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

/// 成格: a 府相 base form (A or B) plus auxiliary support in the Life 三方四正.
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let life = selected_branch_of_palace(ctx, PalaceName::Life)?;

    let base = selected_fu_xiang_base_form(ctx, life)?;

    let support = selected_support_in_san_fang_si_zheng(ctx, life);
    if support.is_empty() {
        return None;
    }

    let mut involved_palaces = base.palaces.clone();
    involved_palaces.extend(support.branches.iter().copied());
    sort_dedup_branches(&mut involved_palaces);

    let mut involved_stars = vec![StarName::TianFu, StarName::TianXiang];
    involved_stars.extend(support.involved_stars());
    involved_stars.sort();
    involved_stars.dedup();

    let mut evidence = base.evidence;
    evidence.extend(support.evidence());

    Some(FormationMatch {
        id: PatternId::FuXiangChaoYuan,
        scope,
        anchor: PatternAnchor::Palace(life),
        involved_palaces,
        involved_stars,
        involved_mutagens: support.involved_mutagens(),
        evidence,
    })
}

struct FuXiangBase {
    palaces: Vec<EarthlyBranch>,
    evidence: Vec<PatternEvidence>,
}

/// Returns the matched 府相 base formation, restricted to the two maintained forms:
///
/// A. 天府 and 天相 occupy the Wealth and Career palaces, one in each.
/// B. 天府 sits in the Life palace, and 天相 appears in the Life 三方四正.
fn selected_fu_xiang_base_form(
    ctx: &PatternContext<'_>,
    life: EarthlyBranch,
) -> Option<FuXiangBase> {
    let wealth = selected_branch_of_palace(ctx, PalaceName::Wealth);
    let career = selected_branch_of_palace(ctx, PalaceName::Career);
    if let (Some(wealth), Some(career)) = (wealth, career) {
        let fu_at = |branch| selected_palace_has_all_stars(ctx, branch, &[StarName::TianFu]);
        let xiang_at = |branch| selected_palace_has_all_stars(ctx, branch, &[StarName::TianXiang]);
        if fu_at(wealth) && xiang_at(career) {
            return Some(FuXiangBase {
                palaces: vec![wealth, career],
                evidence: vec![
                    PatternEvidence::StarInPalace {
                        star: StarName::TianFu,
                        branch: wealth,
                    },
                    PatternEvidence::StarInPalace {
                        star: StarName::TianXiang,
                        branch: career,
                    },
                ],
            });
        }
        if fu_at(career) && xiang_at(wealth) {
            return Some(FuXiangBase {
                palaces: vec![wealth, career],
                evidence: vec![
                    PatternEvidence::StarInPalace {
                        star: StarName::TianFu,
                        branch: career,
                    },
                    PatternEvidence::StarInPalace {
                        star: StarName::TianXiang,
                        branch: wealth,
                    },
                ],
            });
        }
    }

    if selected_palace_has_all_stars(ctx, life, &[StarName::TianFu]) {
        let xiang = selected_stars_in_san_fang_si_zheng(ctx, life, &[StarName::TianXiang]);
        if let Some((_, xiang_branch)) = xiang.first().copied() {
            return Some(FuXiangBase {
                palaces: vec![life, xiang_branch],
                evidence: vec![
                    PatternEvidence::StarInPalace {
                        star: StarName::TianFu,
                        branch: life,
                    },
                    PatternEvidence::StarInPalace {
                        star: StarName::TianXiang,
                        branch: xiang_branch,
                    },
                ],
            });
        }
    }

    None
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
