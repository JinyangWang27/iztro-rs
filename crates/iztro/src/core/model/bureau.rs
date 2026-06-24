//! Five-element bureau (五行局) derived from the Life Palace stem-branch pair.
//!
//! The Life Palace pair determines a NaYin element, and the NaYin element
//! selects the five-element bureau used by later star placement:
//!
//! ```text
//! Water -> Water2 (水二局)
//! Wood  -> Wood3  (木三局)
//! Metal -> Metal4 (金四局)
//! Earth -> Earth5 (土五局)
//! Fire  -> Fire6  (火六局)
//! ```

use crate::core::model::ganzhi::StemBranch;
use crate::core::model::nayin::{NaYinElement, nayin_element};
use serde::{Deserialize, Serialize};

/// The five-element bureau (五行局) of a chart.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FiveElementBureau {
    /// Water 2 bureau (水二局).
    Water2,
    /// Wood 3 bureau (木三局).
    Wood3,
    /// Metal 4 bureau (金四局).
    Metal4,
    /// Earth 5 bureau (土五局).
    Earth5,
    /// Fire 6 bureau (火六局).
    Fire6,
}

impl FiveElementBureau {
    /// Returns the classical bureau number (`2..=6`).
    pub const fn number(self) -> u8 {
        match self {
            Self::Water2 => 2,
            Self::Wood3 => 3,
            Self::Metal4 => 4,
            Self::Earth5 => 5,
            Self::Fire6 => 6,
        }
    }

    /// Returns the NaYin element underlying this bureau.
    pub const fn element(self) -> NaYinElement {
        match self {
            Self::Water2 => NaYinElement::Water,
            Self::Wood3 => NaYinElement::Wood,
            Self::Metal4 => NaYinElement::Metal,
            Self::Earth5 => NaYinElement::Earth,
            Self::Fire6 => NaYinElement::Fire,
        }
    }

    /// Returns the bureau for a NaYin element.
    pub const fn from_element(element: NaYinElement) -> Self {
        match element {
            NaYinElement::Water => Self::Water2,
            NaYinElement::Wood => Self::Wood3,
            NaYinElement::Metal => Self::Metal4,
            NaYinElement::Earth => Self::Earth5,
            NaYinElement::Fire => Self::Fire6,
        }
    }
}

/// Calculates the five-element bureau from the Life Palace stem-branch pair.
///
/// `crate::core::model::ganzhi::StemBranch` is valid by construction, so this always succeeds.
pub fn five_element_bureau_from_life_palace(pair: StemBranch) -> FiveElementBureau {
    FiveElementBureau::from_element(nayin_element(pair))
}
