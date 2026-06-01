use serde::{Deserialize, Serialize};

/// Stable identifiers for stars represented in chart facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarName {
    /// Zi Wei star.
    ZiWei,
    /// Tian Fu star.
    TianFu,
    /// Wu Qu star.
    WuQu,
}

/// Broad star category used by feature extractors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarCategory {
    /// Fourteen major stars.
    Major,
    /// Supportive or secondary stars.
    Minor,
    /// Malefic stars.
    Malefic,
    /// Auxiliary stars.
    Auxiliary,
    /// Miscellaneous symbolic markers.
    Adjective,
}

/// A star's brightness or strength state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Brightness {
    /// Temple brightness.
    Temple,
    /// Prosperous brightness.
    Prosperous,
    /// Advantageous brightness.
    Advantage,
    /// Flat brightness.
    Flat,
    /// Weak brightness.
    Weak,
    /// Trapped brightness.
    Trapped,
    /// Brightness has not been calculated.
    Unknown,
}
