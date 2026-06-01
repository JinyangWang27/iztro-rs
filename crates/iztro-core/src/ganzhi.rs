use serde::{Deserialize, Serialize};

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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeavenlyStem {
    /// Jia stem.
    Jia,
    /// Yi stem.
    Yi,
    /// Bing stem.
    Bing,
    /// Ding stem.
    Ding,
    /// Wu stem.
    Wu,
    /// Ji stem.
    Ji,
    /// Geng stem.
    Geng,
    /// Xin stem.
    Xin,
    /// Ren stem.
    Ren,
    /// Gui stem.
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EarthlyBranch {
    /// Zi branch.
    Zi,
    /// Chou branch.
    Chou,
    /// Yin branch.
    Yin,
    /// Mao branch.
    Mao,
    /// Chen branch.
    Chen,
    /// Si branch.
    Si,
    /// Wu branch.
    Wu,
    /// Wei branch.
    Wei,
    /// Shen branch.
    Shen,
    /// You branch.
    You,
    /// Xu branch.
    Xu,
    /// Hai branch.
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
