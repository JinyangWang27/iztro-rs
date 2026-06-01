use serde::{Deserialize, Serialize};

/// Metadata describing the method profile used to build chart facts.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MethodProfile {
    id: String,
    description: String,
}

impl MethodProfile {
    /// Creates placeholder method-profile metadata for scaffolding.
    pub fn placeholder(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: "placeholder method profile; chart algorithms are not implemented"
                .to_owned(),
        }
    }

    /// Returns the stable profile identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the profile description.
    pub fn description(&self) -> &str {
        &self.description
    }
}
