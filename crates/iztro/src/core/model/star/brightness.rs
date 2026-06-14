use serde::{Deserialize, Serialize};

/// A star's brightness or strength state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Brightness {
    /// Temple brightness (庙).
    Temple,
    /// Prosperous brightness (旺).
    Prosperous,
    /// Advantageous brightness (得).
    Advantage,
    /// Favourable brightness (利).
    Favourable,
    /// Flat brightness (平).
    Flat,
    /// Weak brightness (不).
    Weak,
    /// Trapped brightness (陷).
    Trapped,
    /// Brightness has not been calculated.
    Unknown,
}
