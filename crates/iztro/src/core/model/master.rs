//! Soul master (命主) and body master (身主) derivation.
//!
//! These are deterministic Zi Wei Dou Shu facts matching upstream `iztro`:
//!
//! - the soul master (命主) is selected by the **Life Palace branch** (命宫地支);
//! - the body master (身主) is selected by the **birth-year branch** (年支).
//!
//! Both resolve to a [`StarName`]; callers map that to a Chinese label through
//! [`crate::core::labels::zh_cn::star_name_zh`].

use crate::core::model::star::StarName;
use lunar_lite::EarthlyBranch;

/// Returns the soul master (命主) star for a Life Palace branch (命宫地支).
pub const fn soul_master(life_palace_branch: EarthlyBranch) -> StarName {
    match life_palace_branch {
        EarthlyBranch::Zi => StarName::TanLang,
        EarthlyBranch::Chou | EarthlyBranch::Hai => StarName::JuMen,
        EarthlyBranch::Yin | EarthlyBranch::Xu => StarName::LuCun,
        EarthlyBranch::Mao | EarthlyBranch::You => StarName::WenQu,
        EarthlyBranch::Chen | EarthlyBranch::Shen => StarName::LianZhen,
        EarthlyBranch::Si | EarthlyBranch::Wei => StarName::WuQu,
        EarthlyBranch::Wu => StarName::PoJun,
    }
}

/// Returns the body master (身主) star for a birth-year branch (年支).
pub const fn body_master(birth_year_branch: EarthlyBranch) -> StarName {
    match birth_year_branch {
        EarthlyBranch::Zi | EarthlyBranch::Wu => StarName::HuoXing,
        EarthlyBranch::Chou | EarthlyBranch::Wei => StarName::TianXiang,
        EarthlyBranch::Yin | EarthlyBranch::Shen => StarName::TianLiang,
        EarthlyBranch::Mao | EarthlyBranch::You => StarName::TianTong,
        EarthlyBranch::Chen | EarthlyBranch::Xu => StarName::WenChang,
        EarthlyBranch::Si | EarthlyBranch::Hai => StarName::TianJi,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soul_master_matches_life_palace_branch_table() {
        let pairs = [
            (EarthlyBranch::Zi, StarName::TanLang),
            (EarthlyBranch::Chou, StarName::JuMen),
            (EarthlyBranch::Hai, StarName::JuMen),
            (EarthlyBranch::Yin, StarName::LuCun),
            (EarthlyBranch::Xu, StarName::LuCun),
            (EarthlyBranch::Mao, StarName::WenQu),
            (EarthlyBranch::You, StarName::WenQu),
            (EarthlyBranch::Chen, StarName::LianZhen),
            (EarthlyBranch::Shen, StarName::LianZhen),
            (EarthlyBranch::Si, StarName::WuQu),
            (EarthlyBranch::Wei, StarName::WuQu),
            (EarthlyBranch::Wu, StarName::PoJun),
        ];
        for (branch, expected) in pairs {
            assert_eq!(soul_master(branch), expected);
        }
    }

    #[test]
    fn body_master_matches_birth_year_branch_table() {
        let pairs = [
            (EarthlyBranch::Zi, StarName::HuoXing),
            (EarthlyBranch::Wu, StarName::HuoXing),
            (EarthlyBranch::Chou, StarName::TianXiang),
            (EarthlyBranch::Wei, StarName::TianXiang),
            (EarthlyBranch::Yin, StarName::TianLiang),
            (EarthlyBranch::Shen, StarName::TianLiang),
            (EarthlyBranch::Mao, StarName::TianTong),
            (EarthlyBranch::You, StarName::TianTong),
            (EarthlyBranch::Chen, StarName::WenChang),
            (EarthlyBranch::Xu, StarName::WenChang),
            (EarthlyBranch::Si, StarName::TianJi),
            (EarthlyBranch::Hai, StarName::TianJi),
        ];
        for (branch, expected) in pairs {
            assert_eq!(body_master(branch), expected);
        }
    }
}
