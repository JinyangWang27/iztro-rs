//! NaYin (纳音) five-element classes used by Zi Wei Dou Shu.
//!
//! NaYin is the traditional element assigned to each valid pair in the
//! sixty-pair stem-branch cycle. In this crate, the Life Palace stem-branch
//! pair's NaYin element determines the five-element bureau.
//!
//! Low-level stem/branch enums and sexagenary-cycle validation are provided by
//! `lunar-lite`; this module only owns the Zi Wei-specific NaYin lookup.

use lunar_lite::StemBranch;
use serde::{Deserialize, Serialize};

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

/// Returns the NaYin element for a stem-branch pair.
///
/// `lunar_lite::StemBranch` is valid by construction, so every pair maps to a
/// NaYin element. Adjacent cycle positions share one element, so the lookup
/// halves the pair's sexagenary-cycle index.
pub fn nayin_element(pair: StemBranch) -> NaYinElement {
    NAYIN_ELEMENTS[pair.cycle_index() / 2]
}
