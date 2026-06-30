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
    PatternStatus, PatternStrength,
};
use crate::core::pattern::query::{
    branch_of_palace_for_scope, pattern_scope_for, stars_in_san_fang_si_zheng_for_scope,
};
use crate::core::{EarthlyBranch, PalaceName, StarName};

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
    for &scope in &request.scopes {
        let Some(anchor) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
            continue;
        };

        let found = stars_in_san_fang_si_zheng_for_scope(ctx, scope, anchor, &REQUIRED);

        let all_present = REQUIRED
            .iter()
            .all(|star| found.iter().any(|(found_star, _)| found_star == star));
        if !all_present {
            continue;
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
            scope: pattern_scope_for(scope),
            anchor: PatternAnchor::Palace(anchor),
            involved_palaces: branches,
            involved_stars,
            involved_mutagens: Vec::new(),
            evidence,
            weakening_factors: Vec::new(),
            breaking_factors: Vec::new(),
        });
    }
}
