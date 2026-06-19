//! 紫府朝垣 — Zi Wei (紫微) and Tian Fu (天府) arranged around the Life palace.
//!
//! Conservative condition: both 紫微 and 天府 fall within the 三方四正 of the Life
//! palace. The detection is [`PatternStatus::Fulfilled`] when this holds, or
//! [`PatternStatus::Weakened`] (成而减力) when an adverse [`StarKind::Tough`] star
//! also sits in one of the involved palaces.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternFamily, PatternId,
    PatternPolarity, PatternScope, PatternStatus, PatternStrength,
};
use crate::core::pattern::query::stars_in_palace;
use crate::core::{EarthlyBranch, StarKind, StarName};

const NAME_ZH: &str = "紫府朝垣";
const REQUIRED: [StarName; 2] = [StarName::ZiWei, StarName::TianFu];

/// Detects 紫府朝垣 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    _request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let chart = ctx.chart;
    let Some(life) = chart.life_palace() else {
        return;
    };
    let anchor = life.branch();

    let found = crate::core::pattern::query::stars_in_san_fang_si_zheng(chart, anchor, &REQUIRED);
    let has_ziwei = found.iter().any(|(star, _)| *star == StarName::ZiWei);
    let has_tianfu = found.iter().any(|(star, _)| *star == StarName::TianFu);
    if !(has_ziwei && has_tianfu) {
        return;
    }

    let mut branches: Vec<EarthlyBranch> = found.iter().map(|(_, branch)| *branch).collect();
    branches.sort_by_key(|branch| branch.index());
    branches.dedup();

    let involved_stars: Vec<StarName> = REQUIRED
        .iter()
        .copied()
        .filter(|star| found.iter().any(|(found_star, _)| found_star == star))
        .collect();

    // Weakening: adverse (煞) stars sharing one of the involved palaces.
    let mut weakening_factors: Vec<PatternCondition> = Vec::new();
    for &branch in &branches {
        for placement in stars_in_palace(chart, branch) {
            if placement.kind() == StarKind::Tough {
                weakening_factors.push(PatternCondition::WeakenedByStar {
                    star: placement.name(),
                    branch,
                });
            }
        }
    }

    let status = if weakening_factors.is_empty() {
        PatternStatus::Fulfilled
    } else {
        PatternStatus::Weakened
    };

    out.push(PatternDetection {
        id: PatternId::ZiFuChaoYuan,
        name_zh: NAME_ZH,
        family: PatternFamily::MajorStarCombination,
        polarity: PatternPolarity::Auspicious,
        status,
        strength: PatternStrength::Medium,
        scope: PatternScope::Natal,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces: branches.clone(),
        involved_stars,
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSanFangSiZheng {
            stars: REQUIRED.to_vec(),
            anchor,
            branches,
        }],
        missing_conditions: Vec::new(),
        weakening_factors,
        breaking_factors: Vec::new(),
    });
}
