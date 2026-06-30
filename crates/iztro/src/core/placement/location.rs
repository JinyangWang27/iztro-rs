//! Shared branch-index formulas reused across placement passes.
//!
//! These reproduce `iztro` 2.5.8 location helpers (`src/star/location.ts`, MIT
//! licensed). They are intentionally pure branch arithmetic with no calendar
//! derivation: callers supply the already-known stem/branch facts. Natal minor
//! placement, decorative runtime placement, and temporal flow placement all
//! depend on this module rather than on one another.

use crate::core::model::bureau::FiveElementBureau;
use crate::core::model::calendar::Gender;
use lunar_lite::{EarthlyBranch, HeavenlyStem};

/// Returns the 天魁/天钺 branches for a Heavenly Stem (iztro `getKuiYueIndex`).
pub(crate) const fn kui_yue_branches(stem: HeavenlyStem) -> (EarthlyBranch, EarthlyBranch) {
    match stem {
        HeavenlyStem::Jia | HeavenlyStem::Wu | HeavenlyStem::Geng => {
            (EarthlyBranch::Chou, EarthlyBranch::Wei)
        }
        HeavenlyStem::Yi | HeavenlyStem::Ji => (EarthlyBranch::Zi, EarthlyBranch::Shen),
        HeavenlyStem::Xin => (EarthlyBranch::Wu, EarthlyBranch::Yin),
        HeavenlyStem::Bing | HeavenlyStem::Ding => (EarthlyBranch::Hai, EarthlyBranch::You),
        HeavenlyStem::Ren | HeavenlyStem::Gui => (EarthlyBranch::Mao, EarthlyBranch::Si),
    }
}

/// Returns the 禄存/擎羊/陀罗/天马 branches for a stem-branch pair
/// (iztro `getLuYangTuoMaIndex`).
pub(crate) fn lu_yang_tuo_ma_branches(
    stem: HeavenlyStem,
    branch: EarthlyBranch,
) -> (EarthlyBranch, EarthlyBranch, EarthlyBranch, EarthlyBranch) {
    let lu = match stem {
        HeavenlyStem::Jia => EarthlyBranch::Yin,
        HeavenlyStem::Yi => EarthlyBranch::Mao,
        HeavenlyStem::Bing | HeavenlyStem::Wu => EarthlyBranch::Si,
        HeavenlyStem::Ding | HeavenlyStem::Ji => EarthlyBranch::Wu,
        HeavenlyStem::Geng => EarthlyBranch::Shen,
        HeavenlyStem::Xin => EarthlyBranch::You,
        HeavenlyStem::Ren => EarthlyBranch::Hai,
        HeavenlyStem::Gui => EarthlyBranch::Zi,
    };
    let ma = match branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Shen,
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Yin,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Hai,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Si,
    };

    (lu, lu.offset(1), lu.offset(-1), ma)
}

/// Returns the 文昌/文曲 branches by Heavenly Stem
/// (iztro `getChangQuIndexByHeavenlyStem`).
///
/// This is the **flow** 昌曲 rule and is deliberately distinct from the natal
/// time-based 文昌文曲 rule (iztro `getChangQuIndex`).
pub(crate) const fn chang_qu_branches_by_stem(
    stem: HeavenlyStem,
) -> (EarthlyBranch, EarthlyBranch) {
    match stem {
        HeavenlyStem::Jia => (EarthlyBranch::Si, EarthlyBranch::You),
        HeavenlyStem::Yi => (EarthlyBranch::Wu, EarthlyBranch::Shen),
        HeavenlyStem::Bing | HeavenlyStem::Wu => (EarthlyBranch::Shen, EarthlyBranch::Wu),
        HeavenlyStem::Ding | HeavenlyStem::Ji => (EarthlyBranch::You, EarthlyBranch::Si),
        HeavenlyStem::Geng => (EarthlyBranch::Hai, EarthlyBranch::Mao),
        HeavenlyStem::Xin => (EarthlyBranch::Zi, EarthlyBranch::Yin),
        HeavenlyStem::Ren => (EarthlyBranch::Yin, EarthlyBranch::Zi),
        HeavenlyStem::Gui => (EarthlyBranch::Mao, EarthlyBranch::Hai),
    }
}

/// Returns the 红鸾/天喜 branches for a branch (iztro `getLuanXiIndex`):
/// 红鸾 counts backward from 卯 by the branch's index, 天喜 sits opposite.
pub(crate) fn luan_xi_branches(branch: EarthlyBranch) -> (EarthlyBranch, EarthlyBranch) {
    let hong_luan = EarthlyBranch::Mao.offset(-(branch.index() as isize));
    (hong_luan, hong_luan.offset(6))
}

/// Returns the 年解 branch for a year branch (iztro `getNianjieIndex`):
/// 解神 starts at 戌 on 子 and counts backward to the year branch.
pub(crate) fn nian_jie_branch(year_branch: EarthlyBranch) -> EarthlyBranch {
    const NIAN_JIE_BY_YEAR_BRANCH: [EarthlyBranch; 12] = [
        EarthlyBranch::Xu,
        EarthlyBranch::You,
        EarthlyBranch::Shen,
        EarthlyBranch::Wei,
        EarthlyBranch::Wu,
        EarthlyBranch::Si,
        EarthlyBranch::Chen,
        EarthlyBranch::Mao,
        EarthlyBranch::Yin,
        EarthlyBranch::Chou,
        EarthlyBranch::Zi,
        EarthlyBranch::Hai,
    ];

    // `EarthlyBranch::index()` is always 0..=11 (twelve branches), and the table
    // has exactly twelve entries, so this index is in bounds by construction.
    NIAN_JIE_BY_YEAR_BRANCH[year_branch.index()]
}

/// Returns the starting branch of the 长生十二神 for a five-element bureau
/// (iztro `getChangesheng12StartIndex`).
pub(crate) const fn changsheng_start_branch(bureau: FiveElementBureau) -> EarthlyBranch {
    match bureau {
        FiveElementBureau::Water2 => EarthlyBranch::Shen,
        FiveElementBureau::Wood3 => EarthlyBranch::Hai,
        FiveElementBureau::Metal4 => EarthlyBranch::Si,
        FiveElementBureau::Earth5 => EarthlyBranch::Shen,
        FiveElementBureau::Fire6 => EarthlyBranch::Yin,
    }
}

/// Returns whether the 长生/博士十二神 advance forward (顺行).
///
/// 阳男阴女顺行, 阴男阳女逆行: forward when the gender polarity matches the year
/// branch polarity (Male = 阳, branch 阳 when its index is even).
pub(crate) fn twelve_god_direction_forward(gender: Gender, year_branch: EarthlyBranch) -> bool {
    let branch_yang = year_branch.index() % 2 == 0;
    let gender_yang = matches!(gender, Gender::Male);
    branch_yang == gender_yang
}
