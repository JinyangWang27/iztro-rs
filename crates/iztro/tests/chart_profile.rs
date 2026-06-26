use iztro::core::{
    ChartAlgorithmKind, ChartPlane, ChartProfile, MethodProfile, is_valid_chart_algorithm_plane,
};

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

#[test]
fn chart_profile_preserves_method_profile_and_plane() {
    let method_profile =
        MethodProfile::new("zhongzhou_test", ChartAlgorithmKind::Zhongzhou, "zhongzhou");
    let profile = ChartProfile::new(method_profile.clone(), ChartPlane::Earth);

    assert_eq!(profile.method_profile(), &method_profile);
    assert_eq!(profile.chart_plane(), ChartPlane::Earth);
    assert_eq!(profile.algorithm_kind(), ChartAlgorithmKind::Zhongzhou);
}

#[test]
fn chart_profile_defaults_are_independent_axes() {
    let method_profile = MethodProfile::new("quanshu_test", ChartAlgorithmKind::QuanShu, "quanshu");
    let profile = ChartProfile::new(method_profile, ChartPlane::Heaven);

    assert_eq!(profile.chart_plane(), ChartPlane::Heaven);
    assert_eq!(profile.algorithm_kind(), ChartAlgorithmKind::QuanShu);
}
