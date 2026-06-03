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
}

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
    /// Fourteen major stars.
    Major,
    /// Supportive soft stars.
    Soft,
    /// Tough stars.
    Tough,
    /// Lu Cun star (禄存).
    #[serde(rename = "lucun")]
    LuCun,
    /// Tian Ma star (天马).
    #[serde(rename = "tianma")]
    TianMa,
    /// Miscellaneous adjective stars.
    Adjective,
    /// Flower stars.
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
