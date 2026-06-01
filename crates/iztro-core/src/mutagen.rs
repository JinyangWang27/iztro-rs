use serde::{Deserialize, Serialize};

/// Four transformations, also known as mutagens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mutagen {
    /// Lu transformation.
    Lu,
    /// Quan transformation.
    Quan,
    /// Ke transformation.
    Ke,
    /// Ji transformation.
    Ji,
}

/// Time scope for a chart fact or transformation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Natal chart scope.
    Natal,
    /// Decadal period scope.
    Decadal,
    /// Yearly period scope.
    Yearly,
    /// Monthly period scope.
    Monthly,
    /// Daily period scope.
    Daily,
    /// Hourly period scope.
    Hourly,
}
