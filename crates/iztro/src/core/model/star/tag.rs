//! Overlapping interpretive/domain tags for stars.
//!
//! These tags layer on top of the coarse, mutually exclusive grouping in
//! [`StarKind`]/[`super::StarCategory`] (`Major` / `Minor` / `Adjective`). Unlike
//! that grouping, tags are *not* mutually exclusive: a star may carry several
//! tags at once (for example 地空 is both 空劫 and 空曜). Tags express shared
//! interpretive families used by classical rules without copying chart facts.
//!
//! [`StarTag::VoidSymbol`] (空曜) is a broad interpretive taxonomy and is kept
//! deliberately distinct from the narrow 空亡-family modeled by
//! [`crate::rules::classical::void::VoidKind`].

use serde::{Deserialize, Serialize};

use crate::core::StarName;

/// Overlapping interpretive/domain tags for stars.
///
/// Unlike `StarKind` and the existing coarse `StarCategory`
/// (`Major` / `Minor` / `Adjective`), tags are not mutually exclusive.
/// A star may carry multiple tags; for example 地空 belongs to both
/// 空劫 and 空曜.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarTag {
    /// 刑曜：擎羊、天刑.
    Punishment,
    /// 空劫：地空、地劫.
    KongJie,
    /// 空曜：空劫、天空；截空、旬空 also count but weaker.
    VoidSymbol,
}

/// Strength of a star's membership in a tag.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarTagStrength {
    /// Primary / direct member of the tag.
    Primary,
    /// Secondary / weaker member of the tag.
    Secondary,
}

/// Returns the strength of `star`'s membership in `tag`, or `None` if `star`
/// does not carry `tag`.
pub const fn star_tag_strength(star: StarName, tag: StarTag) -> Option<StarTagStrength> {
    use StarTag::{KongJie, Punishment, VoidSymbol};
    use StarTagStrength::{Primary, Secondary};

    match tag {
        Punishment => match star {
            StarName::QingYang | StarName::TianXing => Some(Primary),
            _ => None,
        },
        KongJie => match star {
            StarName::DiKong | StarName::DiJie => Some(Primary),
            _ => None,
        },
        VoidSymbol => match star {
            StarName::DiKong | StarName::DiJie | StarName::TianKong => Some(Primary),
            StarName::JieKong | StarName::XunKong => Some(Secondary),
            _ => None,
        },
    }
}

/// Returns whether `star` carries `tag` at any strength.
pub const fn has_star_tag(star: StarName, tag: StarTag) -> bool {
    star_tag_strength(star, tag).is_some()
}
