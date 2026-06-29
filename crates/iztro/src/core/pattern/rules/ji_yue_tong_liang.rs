//! 机月同梁 — Tian Ji (天机), Tai Yin (太阴), Tian Tong (天同), Tian Liang (天梁)
//! gathered through the 三方四正 of the Life palace.
//!
//! The base formation requires all four stars in the Life 三方四正. When the
//! formation is complete, a [`PatternStatus::Fulfilled`] detection is emitted; an
//! incomplete formation is not a near-pattern and produces no detection. No
//! weakening/breaker policy is modeled yet, so only `Fulfilled` is ever emitted.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternScope, PatternStatus, PatternStrength,
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
    _request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let chart = ctx.chart;
    let Some(life) = chart.life_palace() else {
        return;
    };
    let anchor = life.branch();

    let found = stars_in_san_fang_si_zheng(chart, anchor, &REQUIRED);

    // The base formation requires all four stars within the Life 三方四正. An
    // incomplete formation is not a near-pattern, so emit nothing.
    let all_present = REQUIRED
        .iter()
        .all(|star| found.iter().any(|(found_star, _)| found_star == star));
    if !all_present {
        return;
    }

    let involved_stars: Vec<StarName> = REQUIRED.to_vec();

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

    out.push(PatternDetection {
        id: PatternId::JiYueTongLiang,
        name_zh: NAME_ZH,
        family: PatternFamily::MajorStarCombination,
        polarity: PatternPolarity::Auspicious,
        status: PatternStatus::Fulfilled,
        strength: PatternStrength::Medium,
        scope: PatternScope::Natal,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces: branches,
        involved_stars,
        involved_mutagens: Vec::new(),
        evidence,
        missing_conditions: Vec::new(),
        weakening_factors: Vec::new(),
        breaking_factors: Vec::new(),
    });
}
