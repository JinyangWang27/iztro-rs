//! Shared branch-index formulas reused across placement passes.
//!
//! These reproduce `iztro` 2.5.8 location helpers (`src/star/location.ts`, MIT
//! licensed). They are intentionally pure branch arithmetic with no calendar
//! derivation: callers supply the already-known stem/branch facts. Natal minor
//! placement, decorative runtime placement, and temporal flow placement all
//! depend on this module rather than on one another.

use crate::model::bureau::FiveElementBureau;
use crate::model::calendar::Gender;
use crate::model::ganzhi::{EarthlyBranch, HeavenlyStem};

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
