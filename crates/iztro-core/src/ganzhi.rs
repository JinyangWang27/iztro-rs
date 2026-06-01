use serde::{Deserialize, Serialize};

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
