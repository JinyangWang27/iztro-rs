//! Soul master (命主) and body master (身主) derivation.
//!
//! These are deterministic Zi Wei Dou Shu facts matching upstream `iztro`:
//!
//! - the soul master (命主) is selected by the **Life Palace branch** (命宫地支);
//! - the body master (身主) is selected by the **birth-year branch** (年支).
//!
//! Both resolve to a [`StarName`]; callers map that to a Chinese label through
//! [`crate::core::labels::zh_cn::star_name_zh`].

use crate::core::model::ganzhi::EarthlyBranch;
use crate::core::model::star::StarName;

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
