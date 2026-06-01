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
