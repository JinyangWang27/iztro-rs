//! Source-backed QuanShu Volume 1 pattern detections.
//!
//! These predicates cover only structurally clear entries from 定富局, 定贵局,
//! and 定贫贱局. The source inventory records more entries than this module
//! executes; unclear, referenced, and temporal entries remain metadata-only.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::metadata::pattern_source_metadata;
use crate::core::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternScope, PatternStatus, PatternStrength,
};
use crate::core::pattern::query::{
    branch_of_palace, is_bright, major_star_count_in_palace, modeled_void_star_in_palace,
    palace_has_all_stars, palace_has_star,
};
use crate::core::{Chart, EarthlyBranch, PalaceName, StarName};

/// Detects the supported QuanShu Volume 1 source-backed patterns.
pub fn detect(
    ctx: &PatternContext<'_>,
    _request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    detect_jin_can_guang_hui(ctx.chart, out);
    detect_ri_chu_fu_sang(ctx.chart, out);
    detect_yue_luo_hai_gong(ctx.chart, out);
    detect_yue_sheng_cang_hai(ctx.chart, out);
    detect_ma_tou_dai_jian(ctx.chart, out);
    detect_tan_huo_xiang_feng(ctx.chart, out);
    detect_wu_qu_shou_yuan(ctx.chart, out);
    detect_cai_yu_qiu_chou(ctx.chart, out);
    detect_ma_luo_kong_wang(ctx.chart, out);
}

fn detect_jin_can_guang_hui(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let branch = EarthlyBranch::Wu;
    if branch_of_palace(chart, PalaceName::Life) != Some(branch) {
        return;
    }
    if !palace_has_star(chart, branch, StarName::TaiYang) {
        return;
    }
    if major_star_count_in_palace(chart, branch) != 1 {
        return;
    }

    push_single_star(
        out,
        PatternId::JinCanGuangHui,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        StarName::TaiYang,
    );
}

fn detect_ri_chu_fu_sang(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let branch = EarthlyBranch::Mao;
    if !palace_has_star(chart, branch, StarName::TaiYang) {
        return;
    }

    let is_life = branch_of_palace(chart, PalaceName::Life) == Some(branch);
    let is_career = branch_of_palace(chart, PalaceName::Career) == Some(branch);
    if !(is_life || is_career) {
        return;
    }

    push_single_star(
        out,
        PatternId::RiChuFuSang,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        StarName::TaiYang,
    );
}

fn detect_yue_luo_hai_gong(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let branch = EarthlyBranch::Hai;
    if branch_of_palace(chart, PalaceName::Life) != Some(branch) {
        return;
    }
    if !palace_has_star(chart, branch, StarName::TaiYin) {
        return;
    }

    push_single_star(
        out,
        PatternId::YueLuoHaiGong,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        StarName::TaiYin,
    );
}

fn detect_yue_sheng_cang_hai(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let branch = EarthlyBranch::Zi;
    if branch_of_palace(chart, PalaceName::Property) != Some(branch) {
        return;
    }
    if !palace_has_star(chart, branch, StarName::TaiYin) {
        return;
    }

    push_single_star(
        out,
        PatternId::YueShengCangHai,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        StarName::TaiYin,
    );
}

fn detect_ma_tou_dai_jian(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let Some(tian_ma) = chart.star(StarName::TianMa) else {
        return;
    };
    let branch = tian_ma.palace().branch();
    if !palace_has_star(chart, branch, StarName::QingYang) {
        return;
    }

    push_same_palace(
        out,
        PatternId::MaTouDaiJian,
        PatternFamily::ShaJi,
        PatternPolarity::Mixed,
        branch,
        vec![StarName::TianMa, StarName::QingYang],
    );
}

fn detect_tan_huo_xiang_feng(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let Some(life) = chart.life_palace() else {
        return;
    };
    let branch = life.branch();
    if !palace_has_all_stars(chart, branch, &[StarName::TanLang, StarName::HuoXing]) {
        return;
    }

    let Some(tan_lang) = chart.star(StarName::TanLang) else {
        return;
    };
    let Some(huo_xing) = chart.star(StarName::HuoXing) else {
        return;
    };
    if !is_bright(tan_lang.placement().brightness())
        || !is_bright(huo_xing.placement().brightness())
    {
        return;
    }

    push_same_palace(
        out,
        PatternId::TanHuoXiangFeng,
        PatternFamily::ShaJi,
        PatternPolarity::Auspicious,
        branch,
        vec![StarName::TanLang, StarName::HuoXing],
    );
}

fn detect_wu_qu_shou_yuan(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let branch = EarthlyBranch::Mao;
    if branch_of_palace(chart, PalaceName::Life) != Some(branch) {
        return;
    }
    if !palace_has_star(chart, branch, StarName::WuQu) {
        return;
    }

    push_single_star(
        out,
        PatternId::WuQuShouYuan,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        branch,
        StarName::WuQu,
    );
}

fn detect_cai_yu_qiu_chou(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let mut branches = Vec::new();
    if let Some(branch) = branch_of_palace(chart, PalaceName::Life) {
        branches.push(branch);
    }
    if let Some(branch) = chart.body_palace_branch() {
        branches.push(branch);
    }
    branches.sort_by_key(|branch| branch.index());
    branches.dedup();

    for branch in branches {
        if palace_has_all_stars(chart, branch, &[StarName::WuQu, StarName::LianZhen]) {
            push_same_palace(
                out,
                PatternId::CaiYuQiuChou,
                PatternFamily::MajorStarCombination,
                PatternPolarity::Inauspicious,
                branch,
                vec![StarName::WuQu, StarName::LianZhen],
            );
        }
    }
}

fn detect_ma_luo_kong_wang(chart: &Chart, out: &mut Vec<PatternDetection>) {
    let Some(tian_ma) = chart.star(StarName::TianMa) else {
        return;
    };
    let branch = tian_ma.palace().branch();
    let Some(void_star) = modeled_void_star_in_palace(chart, branch) else {
        return;
    };

    push_same_palace(
        out,
        PatternId::MaLuoKongWang,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        branch,
        vec![StarName::TianMa, void_star],
    );
}

fn push_single_star(
    out: &mut Vec<PatternDetection>,
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
        scope: PatternScope::Natal,
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
        scope: PatternScope::Natal,
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
    const EMITTED_SOURCE_BACKED_PATTERN_IDS: [PatternId; 9] = [
        PatternId::JinCanGuangHui,
        PatternId::RiChuFuSang,
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
