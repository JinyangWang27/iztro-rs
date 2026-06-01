use serde::{Deserialize, Serialize};

/// A named Zi Wei Dou Shu palace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PalaceName {
    /// Life Palace.
    Life,
    /// Siblings Palace.
    Siblings,
    /// Spouse Palace.
    Spouse,
    /// Children Palace.
    Children,
    /// Wealth Palace.
    Wealth,
    /// Health Palace.
    Health,
    /// Migration Palace.
    Migration,
    /// Friends Palace.
    Friends,
    /// Career Palace.
    Career,
    /// Property Palace.
    Property,
    /// Spirit Palace.
    Spirit,
    /// Parents Palace.
    Parents,
}
