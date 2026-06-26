use serde::{Deserialize, Serialize};

/// Chart algorithm family (school) associated with a method profile.
///
/// This is the *algorithm family* axis — it names the school of rules used to
/// generate a chart. It is independent of [`ChartPlane`], which names the
/// *plane variant* (天盘 / 地盘 / 人盘) within that school.
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

/// Requested chart plane (天盘 / 地盘 / 人盘) for a Zi Wei Dou Shu reading.
///
/// `ChartAlgorithmKind` names the algorithm *family* (全书, 中州, …).
/// `ChartPlane` names the *plane variant* within that family.
/// They are independent axes; do not conflate them.
///
/// `Heaven` is the default and reproduces existing chart-generation behaviour.
/// `Earth` and `Human` are meaningful for Zhongzhou (中州) but are not yet
/// implemented.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartPlane {
    /// 天盘 — the Heaven chart; the default plane, preserving existing behaviour.
    #[default]
    Heaven,
    /// 地盘 — the Earth chart; valid for Zhongzhou, implemented by re-anchoring
    /// the Life Palace to the Heaven chart's Body Palace (身宫) branch.
    Earth,
    /// 人盘 — the Human chart; valid for Zhongzhou, implemented by re-anchoring
    /// the Life Palace to the Heaven chart's Spirit / 福德宫 branch.
    Human,
}

/// Returns `true` if `plane` is a domain-valid chart plane for `algorithm`.
///
/// This checks semantic validity only. It does not guarantee that chart
/// generation for that combination is implemented.
///
/// Valid combinations:
/// - `QuanShu + Heaven`
/// - `Zhongzhou + Heaven`, `Zhongzhou + Earth`, `Zhongzhou + Human`
/// - `Placeholder + Heaven` (backward-compatible fallback path)
///
/// This predicate is about domain validity, not dispatch. It does not select a
/// chart plane's anchor or strategy; that resolution lives at the facade
/// boundary.
pub const fn is_valid_chart_algorithm_plane(
    algorithm: ChartAlgorithmKind,
    plane: ChartPlane,
) -> bool {
    match (algorithm, plane) {
        (
            ChartAlgorithmKind::Zhongzhou,
            ChartPlane::Heaven | ChartPlane::Earth | ChartPlane::Human,
        ) => true,
        (ChartAlgorithmKind::QuanShu | ChartAlgorithmKind::Placeholder, ChartPlane::Heaven) => true,
        (ChartAlgorithmKind::QuanShu | ChartAlgorithmKind::Placeholder, _) => false,
    }
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

/// Chart-generation profile metadata combining the method profile (algorithm
/// family) with the chart plane (天盘 / 地盘 / 人盘).
///
/// This pairs the two independent chart-generation axes so that a generated
/// [`Chart`](crate::core::model::chart::Chart) is self-describing: it records
/// both the algorithm family used to build it and which plane variant it
/// represents, without depending on request-side context.
///
/// The serde representation is intentionally flat: this type is flattened into
/// [`Chart`](crate::core::model::chart::Chart) (and any other container) so that
/// `method_profile` and `chart_plane` appear as sibling top-level keys rather
/// than nested under a `chart_profile` object. `chart_plane` defaults to
/// [`ChartPlane::Heaven`], so charts serialized before the plane axis existed
/// still deserialize unchanged.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ChartProfile {
    method_profile: MethodProfile,
    #[serde(default)]
    chart_plane: ChartPlane,
}

impl ChartProfile {
    /// Creates chart-profile metadata from a method profile and chart plane.
    pub const fn new(method_profile: MethodProfile, chart_plane: ChartPlane) -> Self {
        Self {
            method_profile,
            chart_plane,
        }
    }

    /// Returns the method profile (algorithm family) metadata.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }

    /// Returns the chart plane (天盘 / 地盘 / 人盘) this profile describes.
    pub const fn chart_plane(&self) -> ChartPlane {
        self.chart_plane
    }

    /// Returns the typed chart algorithm kind, delegating to the method profile.
    pub const fn algorithm_kind(&self) -> ChartAlgorithmKind {
        self.method_profile.algorithm_kind()
    }
}
