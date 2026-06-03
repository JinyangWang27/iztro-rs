use serde::{Deserialize, Serialize};

/// Stable identifiers for stars represented in chart facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarName {
    /// Zi Wei star.
    ZiWei,
    /// Tian Ji star.
    TianJi,
    /// Tai Yang star.
    TaiYang,
    /// Wu Qu star.
    WuQu,
    /// Tian Tong star.
    TianTong,
    /// Lian Zhen star.
    LianZhen,
    /// Tian Fu star.
    TianFu,
    /// Tai Yin star.
    TaiYin,
    /// Tan Lang star.
    TanLang,
    /// Ju Men star.
    JuMen,
    /// Tian Xiang star.
    TianXiang,
    /// Tian Liang star.
    TianLiang,
    /// Qi Sha star.
    QiSha,
    /// Po Jun star.
    PoJun,
}

/// Broad star category used by feature extractors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarCategory {
    /// Fourteen major stars.
    Major,
    /// Supportive or secondary stars.
    Minor,
    /// Malefic stars.
    Malefic,
    /// Auxiliary stars.
    Auxiliary,
    /// Miscellaneous symbolic markers.
    Adjective,
}

/// Factual metadata for a represented star.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StarMetadata {
    key: &'static str,
    chinese_name: &'static str,
    name: StarName,
    category: StarCategory,
}

impl StarMetadata {
    /// Creates factual star metadata.
    pub const fn new(
        key: &'static str,
        chinese_name: &'static str,
        name: StarName,
        category: StarCategory,
    ) -> Self {
        Self {
            key,
            chinese_name,
            name,
            category,
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

    /// Returns the broad star category.
    pub const fn category(&self) -> StarCategory {
        self.category
    }
}

/// A star's brightness or strength state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Brightness {
    /// Temple brightness.
    Temple,
    /// Prosperous brightness.
    Prosperous,
    /// Advantageous brightness.
    Advantage,
    /// Favourable brightness.
    Favourable,
    /// Flat brightness.
    Flat,
    /// Weak brightness.
    Weak,
    /// Trapped brightness.
    Trapped,
    /// Brightness has not been calculated.
    Unknown,
}
