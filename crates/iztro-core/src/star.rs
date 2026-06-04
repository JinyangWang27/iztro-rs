use serde::{Deserialize, Serialize};

/// Stable identifiers for stars represented in chart facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarName {
    /// Zi Wei star (紫微).
    ZiWei,
    /// Tian Ji star (天机).
    TianJi,
    /// Tai Yang star (太阳).
    TaiYang,
    /// Wu Qu star (武曲).
    WuQu,
    /// Tian Tong star (天同).
    TianTong,
    /// Lian Zhen star (廉贞).
    LianZhen,
    /// Tian Fu star (天府).
    TianFu,
    /// Tai Yin star (太阴).
    TaiYin,
    /// Tan Lang star (贪狼).
    TanLang,
    /// Ju Men star (巨门).
    JuMen,
    /// Tian Xiang star (天相).
    TianXiang,
    /// Tian Liang star (天梁).
    TianLiang,
    /// Qi Sha star (七杀).
    QiSha,
    /// Po Jun star (破军).
    PoJun,
    /// Zuo Fu star (左辅).
    ZuoFu,
    /// You Bi star (右弼).
    YouBi,
    /// Wen Chang star (文昌).
    WenChang,
    /// Wen Qu star (文曲).
    WenQu,
    /// Tian Kui star (天魁).
    TianKui,
    /// Tian Yue star (天钺).
    TianYue,
    /// Lu Cun star (禄存).
    LuCun,
    /// Tian Ma star (天马).
    TianMa,
    /// Qing Yang star (擎羊).
    QingYang,
    /// Tuo Luo star (陀罗).
    TuoLuo,
    /// Huo Xing star (火星).
    HuoXing,
    /// Ling Xing star (铃星).
    LingXing,
    /// Di Kong star (地空).
    DiKong,
    /// Di Jie star (地劫).
    DiJie,
}

/// Factual metadata for the fourteen major stars.
const MAJOR_STAR_METADATA: [StarMetadata; 14] = [
    StarMetadata::new("zi_wei", "紫微", StarName::ZiWei, StarKind::Major),
    StarMetadata::new("tian_ji", "天机", StarName::TianJi, StarKind::Major),
    StarMetadata::new("tai_yang", "太阳", StarName::TaiYang, StarKind::Major),
    StarMetadata::new("wu_qu", "武曲", StarName::WuQu, StarKind::Major),
    StarMetadata::new("tian_tong", "天同", StarName::TianTong, StarKind::Major),
    StarMetadata::new("lian_zhen", "廉贞", StarName::LianZhen, StarKind::Major),
    StarMetadata::new("tian_fu", "天府", StarName::TianFu, StarKind::Major),
    StarMetadata::new("tai_yin", "太阴", StarName::TaiYin, StarKind::Major),
    StarMetadata::new("tan_lang", "贪狼", StarName::TanLang, StarKind::Major),
    StarMetadata::new("ju_men", "巨门", StarName::JuMen, StarKind::Major),
    StarMetadata::new("tian_xiang", "天相", StarName::TianXiang, StarKind::Major),
    StarMetadata::new("tian_liang", "天梁", StarName::TianLiang, StarKind::Major),
    StarMetadata::new("qi_sha", "七杀", StarName::QiSha, StarKind::Major),
    StarMetadata::new("po_jun", "破军", StarName::PoJun, StarKind::Major),
];

/// Factual metadata for supported fourteen minor stars.
const MINOR_STAR_METADATA: [StarMetadata; 14] = [
    StarMetadata::new("zuo_fu", "左辅", StarName::ZuoFu, StarKind::Soft),
    StarMetadata::new("you_bi", "右弼", StarName::YouBi, StarKind::Soft),
    StarMetadata::new("wen_chang", "文昌", StarName::WenChang, StarKind::Soft),
    StarMetadata::new("wen_qu", "文曲", StarName::WenQu, StarKind::Soft),
    StarMetadata::new("tian_kui", "天魁", StarName::TianKui, StarKind::Soft),
    StarMetadata::new("tian_yue", "天钺", StarName::TianYue, StarKind::Soft),
    StarMetadata::new("lu_cun", "禄存", StarName::LuCun, StarKind::LuCun),
    StarMetadata::new("tian_ma", "天马", StarName::TianMa, StarKind::TianMa),
    StarMetadata::new("qing_yang", "擎羊", StarName::QingYang, StarKind::Tough),
    StarMetadata::new("tuo_luo", "陀罗", StarName::TuoLuo, StarKind::Tough),
    StarMetadata::new("huo_xing", "火星", StarName::HuoXing, StarKind::Tough),
    StarMetadata::new("ling_xing", "铃星", StarName::LingXing, StarKind::Tough),
    StarMetadata::new("di_kong", "地空", StarName::DiKong, StarKind::Tough),
    StarMetadata::new("di_jie", "地劫", StarName::DiJie, StarKind::Tough),
];

/// Factual metadata for all currently represented stars.
const REPRESENTED_STAR_METADATA: [StarMetadata; 28] = [
    StarMetadata::new("zi_wei", "紫微", StarName::ZiWei, StarKind::Major),
    StarMetadata::new("tian_ji", "天机", StarName::TianJi, StarKind::Major),
    StarMetadata::new("tai_yang", "太阳", StarName::TaiYang, StarKind::Major),
    StarMetadata::new("wu_qu", "武曲", StarName::WuQu, StarKind::Major),
    StarMetadata::new("tian_tong", "天同", StarName::TianTong, StarKind::Major),
    StarMetadata::new("lian_zhen", "廉贞", StarName::LianZhen, StarKind::Major),
    StarMetadata::new("tian_fu", "天府", StarName::TianFu, StarKind::Major),
    StarMetadata::new("tai_yin", "太阴", StarName::TaiYin, StarKind::Major),
    StarMetadata::new("tan_lang", "贪狼", StarName::TanLang, StarKind::Major),
    StarMetadata::new("ju_men", "巨门", StarName::JuMen, StarKind::Major),
    StarMetadata::new("tian_xiang", "天相", StarName::TianXiang, StarKind::Major),
    StarMetadata::new("tian_liang", "天梁", StarName::TianLiang, StarKind::Major),
    StarMetadata::new("qi_sha", "七杀", StarName::QiSha, StarKind::Major),
    StarMetadata::new("po_jun", "破军", StarName::PoJun, StarKind::Major),
    StarMetadata::new("zuo_fu", "左辅", StarName::ZuoFu, StarKind::Soft),
    StarMetadata::new("you_bi", "右弼", StarName::YouBi, StarKind::Soft),
    StarMetadata::new("wen_chang", "文昌", StarName::WenChang, StarKind::Soft),
    StarMetadata::new("wen_qu", "文曲", StarName::WenQu, StarKind::Soft),
    StarMetadata::new("tian_kui", "天魁", StarName::TianKui, StarKind::Soft),
    StarMetadata::new("tian_yue", "天钺", StarName::TianYue, StarKind::Soft),
    StarMetadata::new("lu_cun", "禄存", StarName::LuCun, StarKind::LuCun),
    StarMetadata::new("tian_ma", "天马", StarName::TianMa, StarKind::TianMa),
    StarMetadata::new("qing_yang", "擎羊", StarName::QingYang, StarKind::Tough),
    StarMetadata::new("tuo_luo", "陀罗", StarName::TuoLuo, StarKind::Tough),
    StarMetadata::new("huo_xing", "火星", StarName::HuoXing, StarKind::Tough),
    StarMetadata::new("ling_xing", "铃星", StarName::LingXing, StarKind::Tough),
    StarMetadata::new("di_kong", "地空", StarName::DiKong, StarKind::Tough),
    StarMetadata::new("di_jie", "地劫", StarName::DiJie, StarKind::Tough),
];

/// Coarse palace grouping for placed stars.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarCategory {
    /// Fourteen major stars.
    Major,
    /// Minor stars, including supportive and tough stars.
    Minor,
    /// Miscellaneous symbolic markers.
    Adjective,
}

/// iztro-compatible fine star type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarKind {
    /// Fourteen major stars (14 主星).
    Major,
    /// Supportive soft stars (14辅星 6 吉星).
    Soft,
    /// Tough stars (14辅星 6 凶星).
    Tough,
    /// Lu Cun star (禄存).
    #[serde(rename = "lucun")]
    LuCun,
    /// Tian Ma star (天马).
    #[serde(rename = "tianma")]
    TianMa,
    /// Miscellaneous adjective stars.
    Adjective,
    /// Flower stars (桃花星).
    Flower,
    /// Helper stars.
    Helper,
}

impl StarKind {
    /// Returns the coarse palace grouping for this fine star type.
    pub const fn category(self) -> StarCategory {
        match self {
            Self::Major => StarCategory::Major,
            Self::Soft | Self::Tough | Self::LuCun | Self::TianMa => StarCategory::Minor,
            Self::Adjective | Self::Flower | Self::Helper => StarCategory::Adjective,
        }
    }
}

/// Factual metadata for a represented star.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StarMetadata {
    key: &'static str,
    chinese_name: &'static str,
    name: StarName,
    kind: StarKind,
}

impl StarMetadata {
    /// Creates factual star metadata.
    pub const fn new(
        key: &'static str,
        chinese_name: &'static str,
        name: StarName,
        kind: StarKind,
    ) -> Self {
        Self {
            key,
            chinese_name,
            name,
            kind,
        }
    }

    /// Returns the stable internal key.
    pub const fn key(&self) -> &'static str {
        self.key
    }

    /// Returns the Chinese display name.
    pub const fn chinese_name(&self) -> &'static str {
        self.chinese_name
    }

    /// Returns the typed star identifier.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the iztro-compatible fine star type.
    pub const fn kind(&self) -> StarKind {
        self.kind
    }

    /// Returns the coarse palace grouping.
    pub const fn category(&self) -> StarCategory {
        self.kind.category()
    }
}

/// Returns factual metadata for the fourteen major stars.
pub const fn major_star_metadata_table() -> &'static [StarMetadata; 14] {
    &MAJOR_STAR_METADATA
}

/// Returns factual metadata for the supported fourteen minor stars.
pub const fn minor_star_metadata_table() -> &'static [StarMetadata; 14] {
    &MINOR_STAR_METADATA
}

/// Returns factual metadata for all currently represented stars.
pub const fn represented_star_metadata_table() -> &'static [StarMetadata; 28] {
    &REPRESENTED_STAR_METADATA
}

/// Returns factual metadata for one represented major star, if represented.
pub fn try_major_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    let index = match star {
        StarName::ZiWei => 0,
        StarName::TianJi => 1,
        StarName::TaiYang => 2,
        StarName::WuQu => 3,
        StarName::TianTong => 4,
        StarName::LianZhen => 5,
        StarName::TianFu => 6,
        StarName::TaiYin => 7,
        StarName::TanLang => 8,
        StarName::JuMen => 9,
        StarName::TianXiang => 10,
        StarName::TianLiang => 11,
        StarName::QiSha => 12,
        StarName::PoJun => 13,
        _ => return None,
    };

    Some(&MAJOR_STAR_METADATA[index])
}

/// Returns factual metadata for one represented minor star, if represented.
pub fn try_minor_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    let index = match star {
        StarName::ZuoFu => 0,
        StarName::YouBi => 1,
        StarName::WenChang => 2,
        StarName::WenQu => 3,
        StarName::TianKui => 4,
        StarName::TianYue => 5,
        StarName::LuCun => 6,
        StarName::TianMa => 7,
        StarName::QingYang => 8,
        StarName::TuoLuo => 9,
        StarName::HuoXing => 10,
        StarName::LingXing => 11,
        StarName::DiKong => 12,
        StarName::DiJie => 13,
        _ => return None,
    };

    Some(&MINOR_STAR_METADATA[index])
}

/// Returns factual metadata for one represented star, if represented.
pub fn try_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    try_major_star_metadata(star).or_else(|| try_minor_star_metadata(star))
}

/// Returns factual metadata for one represented major star.
pub fn major_star_metadata(star: StarName) -> &'static StarMetadata {
    try_major_star_metadata(star).expect("star is not a represented major star")
}

/// Returns factual metadata for one represented minor star.
pub fn minor_star_metadata(star: StarName) -> &'static StarMetadata {
    try_minor_star_metadata(star).expect("star is not a represented minor star")
}

/// Returns factual metadata for one represented star.
pub fn star_metadata(star: StarName) -> &'static StarMetadata {
    try_star_metadata(star).expect("star is not represented")
}

/// A star's brightness or strength state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Brightness {
    /// Temple brightness (庙).
    Temple,
    /// Prosperous brightness (旺).
    Prosperous,
    /// Advantageous brightness (得).
    Advantage,
    /// Favourable brightness (利).
    Favourable,
    /// Flat brightness (平).
    Flat,
    /// Weak brightness (不).
    Weak,
    /// Trapped brightness (陷).
    Trapped,
    /// Brightness has not been calculated.
    Unknown,
}
