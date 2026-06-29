//! Pure branch/star derivation helpers for adjective-star placement.
//!
//! Each helper reproduces an iztro 2.5.8 branch formula and depends only on its
//! ganzhi/algorithm/gender arguments — no chart state.

use crate::core::model::calendar::Gender;
use crate::core::model::profile::ChartAlgorithmKind;
use lunar_lite::{EarthlyBranch, HeavenlyStem};

pub(super) fn hua_gai_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Chen,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Chou,
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Xu,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Wei,
    }
}

pub(super) fn gu_chen_gua_su_branches(
    year_branch: EarthlyBranch,
) -> (EarthlyBranch, EarthlyBranch) {
    match year_branch {
        EarthlyBranch::Yin | EarthlyBranch::Mao | EarthlyBranch::Chen => {
            (EarthlyBranch::Si, EarthlyBranch::Chou)
        }
        EarthlyBranch::Si | EarthlyBranch::Wu | EarthlyBranch::Wei => {
            (EarthlyBranch::Shen, EarthlyBranch::Chen)
        }
        EarthlyBranch::Shen | EarthlyBranch::You | EarthlyBranch::Xu => {
            (EarthlyBranch::Hai, EarthlyBranch::Wei)
        }
        EarthlyBranch::Hai | EarthlyBranch::Zi | EarthlyBranch::Chou => {
            (EarthlyBranch::Yin, EarthlyBranch::Xu)
        }
    }
}

pub(super) fn po_sui_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Zi | EarthlyBranch::Wu | EarthlyBranch::Mao | EarthlyBranch::You => {
            EarthlyBranch::Si
        }
        EarthlyBranch::Yin | EarthlyBranch::Shen | EarthlyBranch::Si | EarthlyBranch::Hai => {
            EarthlyBranch::You
        }
        EarthlyBranch::Chen | EarthlyBranch::Xu | EarthlyBranch::Chou | EarthlyBranch::Wei => {
            EarthlyBranch::Chou
        }
    }
}

/// 咸池 (iztro `getHuagaiXianchiIndex`): the peach-blossom branch of the birth
/// year branch's 三合 (triad) family — the opposite member from 华盖.
pub(super) fn xian_chi_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Mao,
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::You,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Wu,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Zi,
    }
}

/// 天官 (iztro `getYearlyStarIndex`): a fixed branch per birth year stem.
pub(super) fn tian_guan_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia => EarthlyBranch::Wei,
        HeavenlyStem::Yi => EarthlyBranch::Chen,
        HeavenlyStem::Bing => EarthlyBranch::Si,
        HeavenlyStem::Ding => EarthlyBranch::Yin,
        HeavenlyStem::Wu => EarthlyBranch::Mao,
        HeavenlyStem::Ji => EarthlyBranch::You,
        HeavenlyStem::Geng => EarthlyBranch::Hai,
        HeavenlyStem::Xin => EarthlyBranch::You,
        HeavenlyStem::Ren => EarthlyBranch::Xu,
        HeavenlyStem::Gui => EarthlyBranch::Wu,
    }
}

/// 天厨 (iztro `getYearlyStarIndex`): a fixed branch per birth year stem.
pub(super) fn tian_chu_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia => EarthlyBranch::Si,
        HeavenlyStem::Yi => EarthlyBranch::Wu,
        HeavenlyStem::Bing => EarthlyBranch::Zi,
        HeavenlyStem::Ding => EarthlyBranch::Si,
        HeavenlyStem::Wu => EarthlyBranch::Wu,
        HeavenlyStem::Ji => EarthlyBranch::Shen,
        HeavenlyStem::Geng => EarthlyBranch::Yin,
        HeavenlyStem::Xin => EarthlyBranch::Wu,
        HeavenlyStem::Ren => EarthlyBranch::You,
        HeavenlyStem::Gui => EarthlyBranch::Hai,
    }
}

/// 天福 adjective star (iztro `getYearlyStarIndex` `tianfuIndex`): a fixed
/// branch per birth year stem. Distinct from the major star 天府.
pub(super) fn tian_fu_adj_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia => EarthlyBranch::You,
        HeavenlyStem::Yi => EarthlyBranch::Shen,
        HeavenlyStem::Bing => EarthlyBranch::Zi,
        HeavenlyStem::Ding => EarthlyBranch::Hai,
        HeavenlyStem::Wu => EarthlyBranch::Mao,
        HeavenlyStem::Ji => EarthlyBranch::Yin,
        HeavenlyStem::Geng => EarthlyBranch::Wu,
        HeavenlyStem::Xin => EarthlyBranch::Si,
        HeavenlyStem::Ren => EarthlyBranch::Wu,
        HeavenlyStem::Gui => EarthlyBranch::Si,
    }
}

/// 截路 (iztro `getYearlyStarIndex`): a fixed branch per 五鼠遁-style stem pair
/// (stem index mod 5).
pub(super) fn jie_lu_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia | HeavenlyStem::Ji => EarthlyBranch::Shen,
        HeavenlyStem::Yi | HeavenlyStem::Geng => EarthlyBranch::Wu,
        HeavenlyStem::Bing | HeavenlyStem::Xin => EarthlyBranch::Chen,
        HeavenlyStem::Ding | HeavenlyStem::Ren => EarthlyBranch::Yin,
        HeavenlyStem::Wu | HeavenlyStem::Gui => EarthlyBranch::Zi,
    }
}

/// 空亡 (iztro `getYearlyStarIndex`): the branch one step forward from 截路,
/// also fixed per stem pair (stem index mod 5).
pub(super) fn kong_wang_branch(year_stem: HeavenlyStem) -> EarthlyBranch {
    match year_stem {
        HeavenlyStem::Jia | HeavenlyStem::Ji => EarthlyBranch::You,
        HeavenlyStem::Yi | HeavenlyStem::Geng => EarthlyBranch::Wei,
        HeavenlyStem::Bing | HeavenlyStem::Xin => EarthlyBranch::Si,
        HeavenlyStem::Ding | HeavenlyStem::Ren => EarthlyBranch::Mao,
        HeavenlyStem::Wu | HeavenlyStem::Gui => EarthlyBranch::Chou,
    }
}

pub(super) fn zhongzhou_jie_kong_branch(
    year_branch: EarthlyBranch,
    jie_lu: EarthlyBranch,
    kong_wang: EarthlyBranch,
) -> EarthlyBranch {
    if year_branch.index() % 2 == 0 {
        jie_lu
    } else {
        kong_wang
    }
}

pub(super) fn zhongzhou_jie_sha_adj_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    match year_branch {
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Si,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Shen,
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Hai,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Yin,
    }
}

pub(super) fn tian_shang_tian_shi_branches(
    algorithm: ChartAlgorithmKind,
    gender: Gender,
    year_branch: EarthlyBranch,
    life_branch: EarthlyBranch,
) -> (EarthlyBranch, EarthlyBranch) {
    let default_tian_shang = life_branch.offset(5);
    let default_tian_shi = life_branch.offset(7);
    let same_yinyang = year_branch.index() % 2
        == match gender {
            Gender::Male => 0,
            Gender::Female => 1,
        };

    if algorithm == ChartAlgorithmKind::Zhongzhou && !same_yinyang {
        (default_tian_shi, default_tian_shang)
    } else {
        (default_tian_shang, default_tian_shi)
    }
}

/// 旬空 (旬中空亡, iztro `getYearlyStarIndex` `xunkongIndex`).
///
/// The birth year stem-branch pair sits inside one of the six 甲-旬 (sexagenary
/// decades); two branches are absent from that decade and form the 旬空 pair.
/// iztro picks the void branch whose阴阳 polarity matches the birth year branch
/// (yang year branch → yang void branch, yin → yin).
///
/// iztro computes a base palace index `year_branch_palace + 癸 - stem + 1`, then
/// advances one palace when the base palace parity differs from the year
/// branch's. Palace indices and branch indices differ by the fixed offset of 寅
/// (2), so parity is preserved and the rule translates directly to branch
/// space: `base = year_branch_index + 10 - stem_index (mod 12)`.
pub(super) fn xun_kong_branch(
    year_stem: HeavenlyStem,
    year_branch: EarthlyBranch,
) -> EarthlyBranch {
    let stem = year_stem.index() as isize;
    let year_branch_index = year_branch.index() as isize;
    // 癸 is stem index 9, and the iztro formula adds a further +1.
    let mut index = (year_branch_index + 10 - stem).rem_euclid(12);
    let year_polarity = year_branch_index.rem_euclid(2);
    if year_polarity != index.rem_euclid(2) {
        index = (index + 1).rem_euclid(12);
    }
    EarthlyBranch::from_index(index as usize)
}
