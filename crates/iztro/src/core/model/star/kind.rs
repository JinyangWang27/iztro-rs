use serde::{Deserialize, Serialize};

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
///
/// The derived [`Ord`]/[`PartialOrd`] follow the variant declaration order and
/// exist only to give facade/export snapshots a stable, deterministic star
/// ordering key (see [`crate::core::model::chart::facade_snapshot`]). They carry
/// no astrological ranking meaning and do not affect placement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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
