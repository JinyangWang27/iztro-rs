//! Source-backed QuanShu Volume 1 pattern detections.
//!
//! These predicates cover only structurally clear entries from 定富局, 定贵局,
//! and 定贫贱局. The source inventory records more entries than this module
//! executes; unclear, referenced, and temporal entries remain metadata-only.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::metadata::pattern_source_metadata;
use crate::core::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternStatus, PatternStrength,
};
use crate::core::pattern::query::{
    effective_branch_of_palace, effective_palace_has_all_stars, effective_star_in_palace,
    effective_stars_in_palace, find_star_for_scope, is_bright,
    modeled_void_star_in_palace_for_scope, pattern_scope_for,
};
use crate::core::{EarthlyBranch, PalaceName, Scope, StarName};

/// Detects the supported QuanShu Volume 1 source-backed patterns.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        detect_jin_can_guang_hui(ctx, scope, out);
        detect_yue_luo_hai_gong(ctx, scope, out);
        detect_yue_sheng_cang_hai(ctx, scope, out);
        detect_ma_tou_dai_jian(ctx, scope, out);
        detect_tan_huo_xiang_feng(ctx, scope, out);
        detect_wu_qu_shou_yuan(ctx, scope, out);
        detect_cai_yu_qiu_chou(ctx, scope, out);
        detect_ma_luo_kong_wang(ctx, scope, out);
    }
}

fn detect_jin_can_guang_hui(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let branch = EarthlyBranch::Wu;
    if effective_branch_of_palace(ctx, scope, PalaceName::Life) != Some(branch) {
        return;
    }
    let Some(tai_yang) = effective_star_in_palace(ctx, scope, branch, StarName::TaiYang) else {
        return;
    };
    if effective_stars_in_palace(ctx, branch)
        .iter()
        .filter(|star| star.placement().kind() == crate::core::StarKind::Major)
        .count()
        != 1
    {
        return;
    }

    push_single_star(
        out,
        scope,
        PatternId::JinCanGuangHui,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        tai_yang.placement().name(),
    );
}

fn detect_yue_luo_hai_gong(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let branch = EarthlyBranch::Hai;
    if effective_branch_of_palace(ctx, scope, PalaceName::Life) != Some(branch) {
        return;
    }
    let Some(tai_yin) = effective_star_in_palace(ctx, scope, branch, StarName::TaiYin) else {
        return;
    };

    push_single_star(
        out,
        scope,
        PatternId::YueLuoHaiGong,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        tai_yin.placement().name(),
    );
}

fn detect_yue_sheng_cang_hai(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let branch = EarthlyBranch::Zi;
    if effective_branch_of_palace(ctx, scope, PalaceName::Property) != Some(branch) {
        return;
    }
    let Some(tai_yin) = effective_star_in_palace(ctx, scope, branch, StarName::TaiYin) else {
        return;
    };

    push_single_star(
        out,
        scope,
        PatternId::YueShengCangHai,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        tai_yin.placement().name(),
    );
}

fn detect_ma_tou_dai_jian(ctx: &PatternContext<'_>, scope: Scope, out: &mut Vec<PatternDetection>) {
    let Some(tian_ma) = find_star_for_scope(ctx, scope, StarName::TianMa) else {
        return;
    };
    let branch = tian_ma.branch();
    let Some(qing_yang) = find_star_for_scope(ctx, scope, StarName::QingYang) else {
        return;
    };
    if qing_yang.branch() != branch {
        return;
    }

    push_same_palace(
        out,
        scope,
        PatternId::MaTouDaiJian,
        PatternFamily::ShaJi,
        PatternPolarity::Neutral,
        branch,
        vec![tian_ma.placement().name(), qing_yang.placement().name()],
    );
}

fn detect_tan_huo_xiang_feng(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let Some(branch) = effective_branch_of_palace(ctx, scope, PalaceName::Life) else {
        return;
    };
    if !effective_palace_has_all_stars(ctx, scope, branch, &[StarName::TanLang, StarName::HuoXing])
    {
        return;
    }

    let Some(tan_lang) = effective_star_in_palace(ctx, scope, branch, StarName::TanLang) else {
        return;
    };
    let Some(huo_xing) = effective_star_in_palace(ctx, scope, branch, StarName::HuoXing) else {
        return;
    };
    if !is_bright(tan_lang.placement().brightness())
        || !is_bright(huo_xing.placement().brightness())
    {
        return;
    }

    push_same_palace(
        out,
        scope,
        PatternId::TanHuoXiangFeng,
        PatternFamily::ShaJi,
        PatternPolarity::Auspicious,
        branch,
        vec![tan_lang.placement().name(), huo_xing.placement().name()],
    );
}

fn detect_wu_qu_shou_yuan(ctx: &PatternContext<'_>, scope: Scope, out: &mut Vec<PatternDetection>) {
    let branch = EarthlyBranch::Mao;
    if effective_branch_of_palace(ctx, scope, PalaceName::Life) != Some(branch) {
        return;
    }
    let Some(wu_qu) = effective_star_in_palace(ctx, scope, branch, StarName::WuQu) else {
        return;
    };

    push_single_star(
        out,
        scope,
        PatternId::WuQuShouYuan,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        wu_qu.placement().name(),
    );
}

fn detect_cai_yu_qiu_chou(ctx: &PatternContext<'_>, scope: Scope, out: &mut Vec<PatternDetection>) {
    let mut branches = Vec::new();
    if let Some(branch) = effective_branch_of_palace(ctx, scope, PalaceName::Life) {
        branches.push(branch);
    }
    if scope == Scope::Natal {
        if let Some(branch) = ctx.chart.body_palace_branch() {
            branches.push(branch);
        }
    }
    branches.sort_by_key(|branch| branch.index());
    branches.dedup();

    for branch in branches {
        if effective_palace_has_all_stars(ctx, scope, branch, &[StarName::WuQu, StarName::LianZhen])
        {
            push_same_palace(
                out,
                scope,
                PatternId::CaiYuQiuChou,
                PatternFamily::MajorStarCombination,
                PatternPolarity::Inauspicious,
                branch,
                vec![StarName::WuQu, StarName::LianZhen],
            );
        }
    }
}

fn detect_ma_luo_kong_wang(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let Some(tian_ma) = find_star_for_scope(ctx, scope, StarName::TianMa) else {
        return;
    };
    let branch = tian_ma.branch();
    let Some(void_star) = modeled_void_star_in_palace_for_scope(ctx, scope, branch) else {
        return;
    };

    push_same_palace(
        out,
        scope,
        PatternId::MaLuoKongWang,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        branch,
        vec![tian_ma.placement().name(), void_star],
    );
}

fn push_single_star(
    out: &mut Vec<PatternDetection>,
    scope: Scope,
    id: PatternId,
    family: PatternFamily,
    polarity: PatternPolarity,
    branch: EarthlyBranch,
    star: StarName,
) {
    let metadata = pattern_source_metadata(id).expect("source-backed pattern metadata");
    out.push(PatternDetection {
        id,
        name_zh: metadata.name_zh,
        family,
        polarity,
        status: PatternStatus::Fulfilled,
        strength: PatternStrength::Medium,
        scope: pattern_scope_for(scope),
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: vec![star],
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarInPalace { star, branch }],
        weakening_factors: Vec::new(),
        breaking_factors: Vec::new(),
    });
}

fn push_same_palace(
    out: &mut Vec<PatternDetection>,
    scope: Scope,
    id: PatternId,
    family: PatternFamily,
    polarity: PatternPolarity,
    branch: EarthlyBranch,
    stars: Vec<StarName>,
) {
    let metadata = pattern_source_metadata(id).expect("source-backed pattern metadata");
    out.push(PatternDetection {
        id,
        name_zh: metadata.name_zh,
        family,
        polarity,
        status: PatternStatus::Fulfilled,
        strength: PatternStrength::Medium,
        scope: pattern_scope_for(scope),
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: stars.clone(),
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSamePalace { stars, branch }],
        weakening_factors: Vec::new(),
        breaking_factors: Vec::new(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `PatternId`s [`detect`] can emit via [`push_single_star`] /
    /// [`push_same_palace`]. Each is passed to
    /// `pattern_source_metadata(id).expect(..)`, so a missing entry panics at
    /// runtime. Keep in sync with the [`detect`] call order. Test-only and private
    /// so this detector detail stays out of the public API.
    const EMITTED_SOURCE_BACKED_PATTERN_IDS: [PatternId; 8] = [
        PatternId::JinCanGuangHui,
        PatternId::YueLuoHaiGong,
        PatternId::YueShengCangHai,
        PatternId::MaTouDaiJian,
        PatternId::TanHuoXiangFeng,
        PatternId::WuQuShouYuan,
        PatternId::CaiYuQiuChou,
        PatternId::MaLuoKongWang,
    ];

    /// Guards the `.expect("source-backed pattern metadata")` calls in
    /// [`push_single_star`] / [`push_same_palace`]: every emitted id must resolve
    /// to source metadata. (Inventory-reference correctness for source-backed
    /// patterns is checked separately in `tests/classical_source_inventory.rs`,
    /// which owns the test-only inventory loader.)
    #[test]
    fn every_emitted_pattern_has_source_metadata() {
        for id in EMITTED_SOURCE_BACKED_PATTERN_IDS {
            assert!(
                pattern_source_metadata(id).is_some(),
                "emitted pattern {id:?} has no source metadata; detection would panic",
            );
        }
    }
}
