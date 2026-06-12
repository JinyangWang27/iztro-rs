use crate::model::star::{StarCategory, StarName, star_metadata};
use lunar_lite::HeavenlyStem;
use serde::{Deserialize, Serialize};

/// Four transformations, also known as mutagens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mutagen {
    /// Lu transformation (化禄).
    Lu,
    /// Quan transformation (化权).
    Quan,
    /// Ke transformation (化科).
    Ke,
    /// Ji transformation (化忌).
    Ji,
}

/// Time scope for a chart fact or transformation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Natal chart (本命盘).
    Natal,
    /// Decadal period (大限).
    Decadal,
    /// Yearly period (流年).
    Yearly,
    /// Monthly period (流月).
    Monthly,
    /// Daily period (流日).
    Daily,
    /// Hourly period (流时).
    Hourly,
}

/// Returns the birth-year mutagen for any represented star, if supported.
///
/// The table mirrors iztro 2.5.8 Heavenly Stem mutagens in Lu, Quan, Ke, Ji
/// order for represented major stars and represented minor targets.
pub fn birth_year_star_mutagen(year_stem: HeavenlyStem, star: StarName) -> Option<Mutagen> {
    match year_stem {
        HeavenlyStem::Jia => match star {
            StarName::LianZhen => Some(Mutagen::Lu),
            StarName::PoJun => Some(Mutagen::Quan),
            StarName::WuQu => Some(Mutagen::Ke),
            StarName::TaiYang => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Yi => match star {
            StarName::TianJi => Some(Mutagen::Lu),
            StarName::TianLiang => Some(Mutagen::Quan),
            StarName::ZiWei => Some(Mutagen::Ke),
            StarName::TaiYin => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Bing => match star {
            StarName::TianTong => Some(Mutagen::Lu),
            StarName::TianJi => Some(Mutagen::Quan),
            StarName::WenChang => Some(Mutagen::Ke),
            StarName::LianZhen => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Ding => match star {
            StarName::TaiYin => Some(Mutagen::Lu),
            StarName::TianTong => Some(Mutagen::Quan),
            StarName::TianJi => Some(Mutagen::Ke),
            StarName::JuMen => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Wu => match star {
            StarName::TanLang => Some(Mutagen::Lu),
            StarName::TaiYin => Some(Mutagen::Quan),
            StarName::YouBi => Some(Mutagen::Ke),
            StarName::TianJi => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Ji => match star {
            StarName::WuQu => Some(Mutagen::Lu),
            StarName::TanLang => Some(Mutagen::Quan),
            StarName::TianLiang => Some(Mutagen::Ke),
            StarName::WenQu => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Geng => match star {
            StarName::TaiYang => Some(Mutagen::Lu),
            StarName::WuQu => Some(Mutagen::Quan),
            StarName::TaiYin => Some(Mutagen::Ke),
            StarName::TianTong => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Xin => match star {
            StarName::JuMen => Some(Mutagen::Lu),
            StarName::TaiYang => Some(Mutagen::Quan),
            StarName::WenQu => Some(Mutagen::Ke),
            StarName::WenChang => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Ren => match star {
            StarName::TianLiang => Some(Mutagen::Lu),
            StarName::ZiWei => Some(Mutagen::Quan),
            StarName::ZuoFu => Some(Mutagen::Ke),
            StarName::WuQu => Some(Mutagen::Ji),
            _ => None,
        },
        HeavenlyStem::Gui => match star {
            StarName::PoJun => Some(Mutagen::Lu),
            StarName::JuMen => Some(Mutagen::Quan),
            StarName::TaiYin => Some(Mutagen::Ke),
            StarName::TanLang => Some(Mutagen::Ji),
            _ => None,
        },
    }
}

/// Returns the birth-year mutagen for a represented major star, if supported.
pub fn birth_year_major_star_mutagen(year_stem: HeavenlyStem, star: StarName) -> Option<Mutagen> {
    if star_metadata(star).category() != StarCategory::Major {
        return None;
    }

    birth_year_star_mutagen(year_stem, star)
}
