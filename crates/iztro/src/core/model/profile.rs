use serde::{Deserialize, Serialize};

/// Chart algorithm family associated with a method profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartAlgorithmKind {
    /// Quan Shu chart algorithm family (全书).
    QuanShu,
    /// Zhongzhou chart algorithm family (中州).
    Zhongzhou,
    /// Placeholder algorithm marker used before chart generation is implemented.
    Placeholder,
}

/// Metadata describing the method profile used to build chart facts.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodProfile {
    id: String,
    algorithm_kind: ChartAlgorithmKind,
    description: String,
}

impl MethodProfile {
    /// Creates method-profile metadata from a stable identifier and algorithm kind.
    pub fn new(
        id: impl Into<String>,
        algorithm_kind: ChartAlgorithmKind,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            algorithm_kind,
            description: description.into(),
        }
    }

    /// Creates placeholder method-profile metadata for scaffolding.
    pub fn placeholder(id: impl Into<String>) -> Self {
        Self::new(
            id,
            ChartAlgorithmKind::Placeholder,
            "placeholder method profile; chart algorithms are not implemented",
        )
    }

    /// Returns the stable profile identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the typed chart algorithm kind.
    pub const fn algorithm_kind(&self) -> ChartAlgorithmKind {
        self.algorithm_kind
    }

    /// Returns the profile description.
    pub fn description(&self) -> &str {
        &self.description
    }
}
