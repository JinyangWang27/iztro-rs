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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart_plane_default_is_heaven() {
        assert_eq!(ChartPlane::default(), ChartPlane::Heaven);
    }

    #[test]
    fn quanshu_heaven_is_valid() {
        assert!(is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
        ));
    }

    #[test]
    fn quanshu_earth_is_not_valid() {
        assert!(!is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Earth,
        ));
    }

    #[test]
    fn quanshu_human_is_not_valid() {
        assert!(!is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Human,
        ));
    }

    #[test]
    fn zhongzhou_heaven_is_valid() {
        assert!(is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::Zhongzhou,
            ChartPlane::Heaven,
        ));
    }

    #[test]
    fn zhongzhou_earth_is_valid() {
        assert!(is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::Zhongzhou,
            ChartPlane::Earth,
        ));
    }

    #[test]
    fn zhongzhou_human_is_valid() {
        assert!(is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::Zhongzhou,
            ChartPlane::Human,
        ));
    }

    #[test]
    fn placeholder_heaven_is_valid() {
        assert!(is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::Placeholder,
            ChartPlane::Heaven,
        ));
    }

    #[test]
    fn placeholder_earth_is_not_valid() {
        assert!(!is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::Placeholder,
            ChartPlane::Earth,
        ));
    }

    #[test]
    fn placeholder_human_is_not_valid() {
        assert!(!is_valid_chart_algorithm_plane(
            ChartAlgorithmKind::Placeholder,
            ChartPlane::Human,
        ));
    }
}
