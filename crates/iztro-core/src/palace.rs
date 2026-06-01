use serde::{Deserialize, Serialize};

/// Canonical twelve-palace sequence used for cyclic palace arithmetic.
pub const PALACE_NAMES: [PalaceName; 12] = [
    PalaceName::Life,
    PalaceName::Siblings,
    PalaceName::Spouse,
    PalaceName::Children,
    PalaceName::Wealth,
    PalaceName::Health,
    PalaceName::Migration,
    PalaceName::Friends,
    PalaceName::Career,
    PalaceName::Property,
    PalaceName::Spirit,
    PalaceName::Parents,
];

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

impl PalaceName {
    /// Returns this palace's zero-based position in [`PALACE_NAMES`].
    pub const fn index(self) -> usize {
        match self {
            Self::Life => 0,
            Self::Siblings => 1,
            Self::Spouse => 2,
            Self::Children => 3,
            Self::Wealth => 4,
            Self::Health => 5,
            Self::Migration => 6,
            Self::Friends => 7,
            Self::Career => 8,
            Self::Property => 9,
            Self::Spirit => 10,
            Self::Parents => 11,
        }
    }

    /// Returns the palace at `index`, wrapping with modulo arithmetic.
    pub fn from_index(index: usize) -> Self {
        PALACE_NAMES[index % PALACE_NAMES.len()]
    }

    /// Returns the palace offset by `delta`, wrapping in both directions.
    pub fn offset(self, delta: isize) -> Self {
        let len = PALACE_NAMES.len() as isize;
        let index = (self.index() as isize + delta).rem_euclid(len) as usize;
        Self::from_index(index)
    }
}
