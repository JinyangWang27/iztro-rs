use serde::{Deserialize, Serialize};

/// Placeholder condition shape for future rule matching.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    /// Condition that always matches in tests or scaffolding.
    Always,
    /// Condition that never matches in tests or scaffolding.
    Never,
}
