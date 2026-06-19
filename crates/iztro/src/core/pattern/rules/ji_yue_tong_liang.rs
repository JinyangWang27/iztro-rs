//! 机月同梁 — Tian Ji (天机), Tai Yin (太阴), Tian Tong (天同), Tian Liang (天梁)
//! gathered through the 三方四正 of the Life palace.
//!
//! [`PatternStatus::Fulfilled`] when all four stars appear in the Life 三方四正.
//! When some are missing, nothing is emitted unless `request.include_partial`,
//! in which case a [`PatternStatus::Partial`] (近格 / 条件不足) detection is emitted
//! with the missing stars recorded in `missing_conditions`.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternFamily, PatternId,
    PatternPolarity, PatternScope, PatternStatus, PatternStrength,
};
use crate::core::pattern::query::stars_in_san_fang_si_zheng;
use crate::core::{EarthlyBranch, StarName};

const NAME_ZH: &str = "机月同梁";
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
    let chart = ctx.chart;
    let Some(life) = chart.life_palace() else {
        return;
    };
    let anchor = life.branch();

    let found = stars_in_san_fang_si_zheng(chart, anchor, &REQUIRED);

    let involved_stars: Vec<StarName> = REQUIRED
        .iter()
        .copied()
        .filter(|star| found.iter().any(|(found_star, _)| found_star == star))
        .collect();
    let missing: Vec<StarName> = REQUIRED
        .iter()
        .copied()
        .filter(|star| !involved_stars.contains(star))
        .collect();

    let all_present = missing.is_empty();
    if !all_present && !request.include_partial {
        return;
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

    let missing_conditions: Vec<PatternCondition> = missing
        .iter()
        .map(|star| PatternCondition::RequiresStar { star: *star })
        .collect();

    let status = if all_present {
        PatternStatus::Fulfilled
    } else {
        PatternStatus::Partial
    };

    out.push(PatternDetection {
        id: PatternId::JiYueTongLiang,
        name_zh: NAME_ZH,
        family: PatternFamily::MajorStarCombination,
        polarity: PatternPolarity::Mixed,
        status,
        strength: PatternStrength::Medium,
        scope: PatternScope::Natal,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces: branches,
        involved_stars,
        involved_mutagens: Vec::new(),
        evidence,
        missing_conditions,
        weakening_factors: Vec::new(),
        breaking_factors: Vec::new(),
    });
}
