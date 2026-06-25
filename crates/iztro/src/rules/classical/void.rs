//! 空亡 (void) modeling for classical rules.
//!
//! Classical 空亡 is a small, specific family of stars — **not** every star whose
//! name contains 空. [`VoidKind`] enumerates exactly the modeled 空亡-family stars
//! and deliberately **excludes** 天空 (TianKong), 地空 (DiKong), and 地劫 (DiJie),
//! which are different stars with different meanings. A [`VoidPolicy`] makes the
//! set a rule consults explicit, leaving room for future school-specific tuning
//! without scattering `match` arms across the codebase.

use serde::{Deserialize, Serialize};

use crate::core::StarName;

/// A modeled 空亡-family star kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoidKind {
    /// 旬空 (旬中空亡).
    XunKong,
    /// 空亡.
    KongWang,
    /// 截路.
    JieLu,
    /// 截空 (Zhongzhou algorithm).
    JieKong,
}

impl VoidKind {
    /// All modeled void kinds, in a stable order.
    pub const ALL: [VoidKind; 4] = [
        VoidKind::XunKong,
        VoidKind::KongWang,
        VoidKind::JieLu,
        VoidKind::JieKong,
    ];

    /// Maps a [`StarName`] to its [`VoidKind`], if it is a modeled 空亡-family star.
    ///
    /// Returns `None` for every non-void star, including 天空/地空/地劫, so callers
    /// never mistake those for 空亡.
    pub const fn from_star(name: StarName) -> Option<Self> {
        match name {
            StarName::XunKong => Some(Self::XunKong),
            StarName::KongWang => Some(Self::KongWang),
            StarName::JieLu => Some(Self::JieLu),
            StarName::JieKong => Some(Self::JieKong),
            _ => None,
        }
    }

    /// The star name backing this void kind.
    pub const fn star(self) -> StarName {
        match self {
            Self::XunKong => StarName::XunKong,
            Self::KongWang => StarName::KongWang,
            Self::JieLu => StarName::JieLu,
            Self::JieKong => StarName::JieKong,
        }
    }
}

/// The set of [`VoidKind`]s a rule treats as 空亡.
///
/// The default policy includes the whole modeled family. A narrower policy can be
/// supplied later (for example, a school that only counts 旬空) without changing
/// rule code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VoidPolicy {
    kinds: &'static [VoidKind],
}

impl VoidPolicy {
    /// The default policy: every modeled 空亡-family star counts.
    pub const DEFAULT: VoidPolicy = VoidPolicy {
        kinds: &VoidKind::ALL,
    };

    /// Returns whether `kind` is treated as 空亡 under this policy.
    pub fn includes(&self, kind: VoidKind) -> bool {
        self.kinds.contains(&kind)
    }
}

impl Default for VoidPolicy {
    fn default() -> Self {
        Self::DEFAULT
    }
}
