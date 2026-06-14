use crate::core::PalaceName;
use serde::{Deserialize, Serialize};

/// Interpretation domain identified by extracted features.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    /// Identity and life-direction features.
    Identity,
    /// Career and professional-role features.
    Career,
    /// Wealth and resource-flow features.
    Wealth,
    /// Relationship features.
    Relationship,
    /// Health and vulnerability features.
    Health,
}

/// Returns the semantic [`Domain`] for a palace, if one is supported.
///
/// This first feature-extraction slice maps only the five palaces with a direct
/// domain analogue. Every other palace returns [`None`]; broader coverage is
/// intentionally deferred to keep the mapping deterministic and reviewable.
pub const fn domain_for_palace(palace: PalaceName) -> Option<Domain> {
    match palace {
        PalaceName::Life => Some(Domain::Identity),
        PalaceName::Career => Some(Domain::Career),
        PalaceName::Wealth => Some(Domain::Wealth),
        PalaceName::Spouse => Some(Domain::Relationship),
        PalaceName::Health => Some(Domain::Health),
        _ => None,
    }
}
