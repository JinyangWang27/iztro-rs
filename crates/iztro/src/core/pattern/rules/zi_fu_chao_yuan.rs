//! 紫府朝垣 — Zi Wei (紫微) and Tian Fu (天府) arranged around the Life palace.
//!
//! Conservative condition: both 紫微 and 天府 fall within the 三方四正 of the Life
//! palace. The detection is [`PatternStatus::Fulfilled`] when this holds, or
//! [`PatternStatus::Weakened`] (成而减力) when an adverse [`StarKind::Tough`] star
//! also sits in one of the involved palaces.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternFamily, PatternId,
    PatternPolarity, PatternStatus, PatternStrength,
};
use crate::core::pattern::query::{
    branch_of_palace_for_scope, pattern_scope_for, stars_in_palace_for_scope,
};
use crate::core::{EarthlyBranch, PalaceName, StarKind, StarName};

const NAME_ZH: &str = "紫府朝垣";
const REQUIRED: [StarName; 2] = [StarName::ZiWei, StarName::TianFu];

/// Detects 紫府朝垣 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        let Some(anchor) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
            continue;
        };

        let found = crate::core::pattern::query::stars_in_san_fang_si_zheng_for_scope(
            ctx, scope, anchor, &REQUIRED,
        );
        let has_ziwei = found.iter().any(|(star, _)| *star == StarName::ZiWei);
        let has_tianfu = found.iter().any(|(star, _)| *star == StarName::TianFu);
        if !(has_ziwei && has_tianfu) {
            continue;
        }

        let mut branches: Vec<EarthlyBranch> = found.iter().map(|(_, branch)| *branch).collect();
        branches.sort_by_key(|branch| branch.index());
        branches.dedup();

        let involved_stars: Vec<StarName> = REQUIRED
            .iter()
            .copied()
            .filter(|star| found.iter().any(|(found_star, _)| found_star == star))
            .collect();

        let mut weakening_factors: Vec<PatternCondition> = Vec::new();
        for &branch in &branches {
            for placement in stars_in_palace_for_scope(ctx, scope, branch) {
                if placement.placement().kind() == StarKind::Tough {
                    weakening_factors.push(PatternCondition::WeakenedByStar {
                        star: placement.placement().name(),
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
            scope: pattern_scope_for(scope),
            anchor: PatternAnchor::Palace(anchor),
            involved_palaces: branches.clone(),
            involved_stars,
            involved_mutagens: Vec::new(),
            evidence: vec![PatternEvidence::StarsInSanFangSiZheng {
                stars: REQUIRED.to_vec(),
                anchor,
                branches,
            }],
            weakening_factors,
            breaking_factors: Vec::new(),
        });
    }
}
