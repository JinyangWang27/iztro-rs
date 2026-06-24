//! Owned GanZhi value objects: Heavenly Stems (天干), Earthly Branches (地支),
//! their valid sexagenary-cycle pairing (干支), and the four pillars (四柱).
//!
//! These are `iztro-rs`'s own public/domain value objects. The runtime calendar
//! engine adapter ([`crate::core::calendar`]) converts its calendar facts into
//! these types at the adapter boundary, so no third-party calendar type ever
//! leaks into the public or domain API.
//!
//! The shapes intentionally mirror the previous `lunar-lite` primitives (same
//! variants, method names, and serde representation) so the placement, nayin,
//! bureau, label, pattern, and view layers consume them unchanged.

use thiserror::Error;

/// Canonical cyclic ordering of the ten Heavenly Stems.
pub const HEAVENLY_STEMS: [HeavenlyStem; 10] = [
    HeavenlyStem::Jia,
    HeavenlyStem::Yi,
    HeavenlyStem::Bing,
    HeavenlyStem::Ding,
    HeavenlyStem::Wu,
    HeavenlyStem::Ji,
    HeavenlyStem::Geng,
    HeavenlyStem::Xin,
    HeavenlyStem::Ren,
    HeavenlyStem::Gui,
];

/// Canonical cyclic ordering of the twelve Earthly Branches.
pub const EARTHLY_BRANCHES: [EarthlyBranch; 12] = [
    EarthlyBranch::Zi,
    EarthlyBranch::Chou,
    EarthlyBranch::Yin,
    EarthlyBranch::Mao,
    EarthlyBranch::Chen,
    EarthlyBranch::Si,
    EarthlyBranch::Wu,
    EarthlyBranch::Wei,
    EarthlyBranch::Shen,
    EarthlyBranch::You,
    EarthlyBranch::Xu,
    EarthlyBranch::Hai,
];

/// One of the ten Heavenly Stems.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeavenlyStem {
    /// Jia stem (甲).
    Jia,
    /// Yi stem (乙).
    Yi,
    /// Bing stem (丙).
    Bing,
    /// Ding stem (丁).
    Ding,
    /// Wu stem (戊).
    Wu,
    /// Ji stem (己).
    Ji,
    /// Geng stem (庚).
    Geng,
    /// Xin stem (辛).
    Xin,
    /// Ren stem (壬).
    Ren,
    /// Gui stem (癸).
    Gui,
}

impl HeavenlyStem {
    /// Returns this stem's zero-based position in [`HEAVENLY_STEMS`].
    pub const fn index(self) -> usize {
        match self {
            Self::Jia => 0,
            Self::Yi => 1,
            Self::Bing => 2,
            Self::Ding => 3,
            Self::Wu => 4,
            Self::Ji => 5,
            Self::Geng => 6,
            Self::Xin => 7,
            Self::Ren => 8,
            Self::Gui => 9,
        }
    }

    /// Returns the stem at `index`, wrapping with modulo arithmetic.
    pub fn from_index(index: usize) -> Self {
        HEAVENLY_STEMS[index % HEAVENLY_STEMS.len()]
    }

    /// Returns the stem offset by `delta`, wrapping in both directions.
    pub fn offset(self, delta: isize) -> Self {
        let len = HEAVENLY_STEMS.len() as isize;
        let index = (self.index() as isize + delta).rem_euclid(len) as usize;
        Self::from_index(index)
    }
}

/// One of the twelve Earthly Branches.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EarthlyBranch {
    /// Zi branch (子).
    Zi,
    /// Chou branch (丑).
    Chou,
    /// Yin branch (寅).
    Yin,
    /// Mao branch (卯).
    Mao,
    /// Chen branch (辰).
    Chen,
    /// Si branch (巳).
    Si,
    /// Wu branch (午).
    Wu,
    /// Wei branch (未).
    Wei,
    /// Shen branch (申).
    Shen,
    /// You branch (酉).
    You,
    /// Xu branch (戌).
    Xu,
    /// Hai branch (亥).
    Hai,
}

impl EarthlyBranch {
    /// Returns this branch's zero-based position in [`EARTHLY_BRANCHES`].
    pub const fn index(self) -> usize {
        match self {
            Self::Zi => 0,
            Self::Chou => 1,
            Self::Yin => 2,
            Self::Mao => 3,
            Self::Chen => 4,
            Self::Si => 5,
            Self::Wu => 6,
            Self::Wei => 7,
            Self::Shen => 8,
            Self::You => 9,
            Self::Xu => 10,
            Self::Hai => 11,
        }
    }

    /// Returns the branch at `index`, wrapping with modulo arithmetic.
    pub fn from_index(index: usize) -> Self {
        EARTHLY_BRANCHES[index % EARTHLY_BRANCHES.len()]
    }

    /// Returns the branch offset by `delta`, wrapping in both directions.
    pub fn offset(self, delta: isize) -> Self {
        let len = EARTHLY_BRANCHES.len() as isize;
        let index = (self.index() as isize + delta).rem_euclid(len) as usize;
        Self::from_index(index)
    }
}

/// Errors from constructing a [`StemBranch`].
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum StemBranchError {
    /// The stem and branch do not share a position in the sexagenary cycle
    /// (their indices have different parity).
    #[error("Invalid stem-branch pair: {stem:?} - {branch:?}")]
    InvalidStemBranchPair {
        /// The offending Heavenly Stem.
        stem: HeavenlyStem,
        /// The offending Earthly Branch.
        branch: EarthlyBranch,
    },
}

/// A valid Heavenly Stem / Earthly Branch pair in the sexagenary cycle (六十甲子).
///
/// Only sixty of the 120 possible combinations are valid: the stem and branch
/// indices must share parity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize)]
pub struct StemBranch {
    stem: HeavenlyStem,
    branch: EarthlyBranch,
}

impl<'de> serde::Deserialize<'de> for StemBranch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawStemBranch {
            stem: HeavenlyStem,
            branch: EarthlyBranch,
        }

        let raw = RawStemBranch::deserialize(deserializer)?;
        StemBranch::try_new(raw.stem, raw.branch).map_err(serde::de::Error::custom)
    }
}

impl StemBranch {
    /// Creates a valid stem-branch pair.
    ///
    /// Returns an error if the stem and branch do not belong to the same
    /// sexagenary-cycle position.
    pub fn try_new(stem: HeavenlyStem, branch: EarthlyBranch) -> Result<Self, StemBranchError> {
        if stem.index() % 2 == branch.index() % 2 {
            Ok(Self { stem, branch })
        } else {
            Err(StemBranchError::InvalidStemBranchPair { stem, branch })
        }
    }

    /// Creates a stem-branch pair from a zero-based sexagenary-cycle index.
    ///
    /// Index `0` is JiaZi, `1` is YiChou, ..., `59` is GuiHai. The input wraps
    /// modulo 60.
    pub fn from_cycle_index(index: usize) -> Self {
        let index = index % 60;
        Self {
            stem: HeavenlyStem::from_index(index),
            branch: EarthlyBranch::from_index(index),
        }
    }

    /// Creates the stem-branch pair for a lunar year.
    ///
    /// Uses the conventional anchor `1984 = JiaZi`.
    pub fn from_lunar_year(year: i32) -> Self {
        let index = (year - 1984).rem_euclid(60) as usize;
        Self::from_cycle_index(index)
    }

    /// Returns the Heavenly Stem.
    pub const fn stem(&self) -> HeavenlyStem {
        self.stem
    }

    /// Returns the Earthly Branch.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the zero-based sexagenary-cycle index.
    pub fn cycle_index(&self) -> usize {
        (0..60)
            .find(|&index| index % 10 == self.stem.index() && index % 12 == self.branch.index())
            .expect("StemBranch invariant guarantees a valid cycle index")
    }
}

/// The four pillars (年柱, 月柱, 日柱, 时柱) of a date and time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FourPillars {
    /// Year pillar (年柱).
    pub yearly: StemBranch,
    /// Month pillar (月柱).
    pub monthly: StemBranch,
    /// Day pillar (日柱).
    pub daily: StemBranch,
    /// Hour pillar (时柱).
    pub hourly: StemBranch,
}

/// Returns the stem-branch of a Chinese lunar year.
///
/// This is useful when a domain rule needs the lunar birth-year stem/branch
/// rather than the four-pillar year pillar, which may use a LiChun boundary.
pub fn lunar_year_stem_branch(lunar_year: i32) -> StemBranch {
    StemBranch::from_lunar_year(lunar_year)
}

/// Returns the Heavenly Stem of a Chinese lunar year (生年干).
pub fn lunar_year_stem(lunar_year: i32) -> HeavenlyStem {
    StemBranch::from_lunar_year(lunar_year).stem()
}

/// Returns the Earthly Branch of a Chinese lunar year (生年支).
pub fn lunar_year_branch(lunar_year: i32) -> EarthlyBranch {
    StemBranch::from_lunar_year(lunar_year).branch()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stem_branch_rejects_mismatched_parity() {
        assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Chou).is_err());
        assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi).is_ok());
    }

    #[test]
    fn from_lunar_year_anchors_1984_jiazi() {
        let jiazi = StemBranch::from_lunar_year(1984);
        assert_eq!(jiazi.stem(), HeavenlyStem::Jia);
        assert_eq!(jiazi.branch(), EarthlyBranch::Zi);
        // 2024 is 甲辰 (JiaChen).
        let y2024 = StemBranch::from_lunar_year(2024);
        assert_eq!(y2024.stem(), HeavenlyStem::Jia);
        assert_eq!(y2024.branch(), EarthlyBranch::Chen);
        // 2023 is 癸卯 (GuiMao).
        let y2023 = StemBranch::from_lunar_year(2023);
        assert_eq!(y2023.stem(), HeavenlyStem::Gui);
        assert_eq!(y2023.branch(), EarthlyBranch::Mao);
    }

    #[test]
    fn cycle_index_round_trips() {
        for index in 0..60 {
            assert_eq!(StemBranch::from_cycle_index(index).cycle_index(), index);
        }
    }

    #[test]
    fn lunar_year_helpers_agree_with_pillar_accessors() {
        for year in [1850, 1984, 2000, 2023, 2150] {
            let pillar = StemBranch::from_lunar_year(year);
            assert_eq!(lunar_year_stem_branch(year), pillar);
            assert_eq!(lunar_year_stem(year), pillar.stem());
            assert_eq!(lunar_year_branch(year), pillar.branch());
        }
    }
}
