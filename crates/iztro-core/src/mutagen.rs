use serde::{Deserialize, Serialize};

/// Four transformations, also known as mutagens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mutagen {
    /// Lu transformation (化禄).
    Lu,
    /// Quan transformation (化权).
    Quan,
    /// Ke transformation (化科).
    Ke,
    /// Ji transformation (化忌).
    Ji,
}

/// Time scope for a chart fact or transformation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Natal chart (本命盘).
    Natal,
    /// Decadal period (大限).
    Decadal,
    /// Yearly period (流年).
    Yearly,
    /// Monthly period (流月).
    Monthly,
    /// Daily period (流日).
    Daily,
    /// Hourly period (流时).
    Hourly,
}
