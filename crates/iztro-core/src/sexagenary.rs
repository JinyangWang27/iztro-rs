//! Sexagenary stem-branch pairs and their NaYin (纳音) five-element class.
//!
//! A sexagenary pair (干支) combines one of the ten Heavenly Stems with one of
//! the twelve Earthly Branches. Only sixty of the 120 combinations are valid:
//! the stem and branch must advance together, so their zero-based indices share
//! the same parity. Each valid pair maps to one of the five NaYin elements via
//! the classical 六十花甲子纳音 table.

use crate::{
    error::ChartError,
    ganzhi::{EarthlyBranch, HeavenlyStem},
};
use serde::{Deserialize, Serialize};

/// A Heavenly Stem paired with an Earthly Branch.
///
/// [`StemBranch::new`] performs no validation; use
/// [`StemBranch::try_new_valid`] or [`is_valid_sexagenary_pair`] when the pair
/// must belong to the sexagenary cycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct StemBranch {
    stem: HeavenlyStem,
    branch: EarthlyBranch,
}

impl StemBranch {
    /// Creates a stem-branch pair without checking sexagenary validity.
    pub const fn new(stem: HeavenlyStem, branch: EarthlyBranch) -> Self {
        Self { stem, branch }
    }

    /// Creates a stem-branch pair, rejecting combinations outside the cycle.
    pub fn try_new_valid(stem: HeavenlyStem, branch: EarthlyBranch) -> Result<Self, ChartError> {
        let pair = Self::new(stem, branch);
        if is_valid_sexagenary_pair(pair) {
            Ok(pair)
        } else {
            Err(ChartError::InvalidStemBranchPair { stem, branch })
        }
    }

    /// Returns the Heavenly Stem of this pair.
    pub const fn stem(&self) -> HeavenlyStem {
        self.stem
    }

    /// Returns the Earthly Branch of this pair.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }
}

/// Returns whether a stem-branch pair belongs to the sexagenary cycle.
///
/// In a valid pair the stem and branch indices share parity (both even or both
/// odd), e.g. `Jia(0)-Zi(0)` and `Yi(1)-Chou(1)` are valid while
/// `Jia(0)-Chou(1)` is not.
pub const fn is_valid_sexagenary_pair(pair: StemBranch) -> bool {
    pair.stem.index() % 2 == pair.branch.index() % 2
}

/// One of the five NaYin (纳音) elements.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NaYinElement {
    /// Metal (金).
    Metal,
    /// Fire (火).
    Fire,
    /// Wood (木).
    Wood,
    /// Earth (土).
    Earth,
    /// Water (水).
    Water,
}

/// NaYin element for each of the thirty consecutive stem-branch groups.
///
/// The sexagenary cycle pairs two adjacent positions (e.g. JiaZi + YiChou) under
/// a single NaYin. Index `g` covers cycle positions `2g` and `2g + 1`. This
/// table is the 六十花甲子纳音 element sequence; localized NaYin display names
/// (e.g. 海中金) are intentionally deferred.
// TODO: add localized NaYin display names (海中金, 炉中火, ...) when a
// presentation layer needs them.
const NAYIN_ELEMENTS: [NaYinElement; 30] = [
    NaYinElement::Metal, // JiaZi  / YiChou
    NaYinElement::Fire,  // BingYin / DingMao
    NaYinElement::Wood,  // WuChen / JiSi
    NaYinElement::Earth, // GengWu / XinWei
    NaYinElement::Metal, // RenShen / GuiYou
    NaYinElement::Fire,  // JiaXu  / YiHai
    NaYinElement::Water, // BingZi / DingChou
    NaYinElement::Earth, // WuYin  / JiMao
    NaYinElement::Metal, // GengChen / XinSi
    NaYinElement::Wood,  // RenWu  / GuiWei
    NaYinElement::Water, // JiaShen / YiYou
    NaYinElement::Earth, // BingXu / DingHai
    NaYinElement::Fire,  // WuZi   / JiChou
    NaYinElement::Wood,  // GengYin / XinMao
    NaYinElement::Water, // RenChen / GuiSi
    NaYinElement::Metal, // JiaWu  / YiWei
    NaYinElement::Fire,  // BingShen / DingYou
    NaYinElement::Wood,  // WuXu   / JiHai
    NaYinElement::Earth, // GengZi / XinChou
    NaYinElement::Metal, // RenYin / GuiMao
    NaYinElement::Fire,  // JiaChen / YiSi
    NaYinElement::Water, // BingWu / DingWei
    NaYinElement::Earth, // WuShen / JiYou
    NaYinElement::Metal, // GengXu / XinHai
    NaYinElement::Wood,  // RenZi  / GuiChou
    NaYinElement::Water, // JiaYin / YiMao
    NaYinElement::Earth, // BingChen / DingSi
    NaYinElement::Fire,  // WuWu   / JiWei
    NaYinElement::Wood,  // GengShen / XinYou
    NaYinElement::Water, // RenXu  / GuiHai
];

/// Returns the position of a valid pair within the sexagenary cycle (`0..60`).
fn sexagenary_index(pair: StemBranch) -> Option<usize> {
    (0..60).find(|&n| n % 10 == pair.stem.index() && n % 12 == pair.branch.index())
}

/// Returns the NaYin element for a stem-branch pair.
///
/// Returns [`ChartError::InvalidStemBranchPair`] when the pair is not part of
/// the sexagenary cycle.
pub fn nayin_element(pair: StemBranch) -> Result<NaYinElement, ChartError> {
    sexagenary_index(pair)
        .map(|index| NAYIN_ELEMENTS[index / 2])
        .ok_or(ChartError::InvalidStemBranchPair {
            stem: pair.stem,
            branch: pair.branch,
        })
}
