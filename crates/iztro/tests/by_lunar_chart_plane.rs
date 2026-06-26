use iztro::core::{
    Chart, ChartAlgorithmKind, ChartError, ChartPlane, EarthlyBranch, Gender, HeavenlyStem,
    LunarChartRequest, LunarChartRequestBuilder, LunarDay, LunarMonth, MethodProfile, PalaceName,
    StemBranch, by_lunar, five_element_bureau_from_life_palace,
};

fn base_builder(profile: MethodProfile) -> LunarChartRequestBuilder {
    LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(5).expect("valid lunar month"))
        .lunar_day(LunarDay::new(17).expect("valid lunar day"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .birth_year_stem(HeavenlyStem::Geng)
        .birth_year_branch(EarthlyBranch::Wu)
        .method_profile(profile)
}

fn quanshu_profile() -> MethodProfile {
    MethodProfile::new("quanshu_test", ChartAlgorithmKind::QuanShu, "quanshu test")
}

fn zhongzhou_profile() -> MethodProfile {
    MethodProfile::new(
        "zhongzhou_test",
        ChartAlgorithmKind::Zhongzhou,
        "zhongzhou test",
    )
}

fn placeholder_profile() -> MethodProfile {
    MethodProfile::placeholder("placeholder_test")
}

fn zhongzhou_chart(plane: ChartPlane) -> Chart {
    by_lunar(
        base_builder(zhongzhou_profile())
            .chart_plane(plane)
            .build()
            .expect("request should build"),
    )
    .expect("zhongzhou chart should build")
}

fn life_palace_branch(chart: &Chart) -> EarthlyBranch {
    chart
        .life_palace()
        .expect("chart should have a Life Palace")
        .branch()
}

fn fortune_palace_branch(chart: &Chart) -> EarthlyBranch {
    chart
        .palace_by_name(PalaceName::Spirit)
        .expect("chart should have a Fortune Palace")
        .branch()
}

#[test]
fn chart_plane_defaults_to_heaven() {
    let request = base_builder(quanshu_profile())
        .build()
        .expect("request should build");

    assert_eq!(request.chart_plane(), ChartPlane::Heaven);
}

#[test]
fn explicit_chart_plane_is_preserved() {
    let request = base_builder(zhongzhou_profile())
        .chart_plane(ChartPlane::Earth)
        .build()
        .expect("request should build");

    assert_eq!(request.chart_plane(), ChartPlane::Earth);
}

#[test]
fn default_request_matches_explicit_heaven_request() {
    let default_chart = by_lunar(
        base_builder(quanshu_profile())
            .build()
            .expect("default request should build"),
    )
    .expect("default by_lunar should build");

    let heaven_chart = by_lunar(
        base_builder(quanshu_profile())
            .chart_plane(ChartPlane::Heaven)
            .build()
            .expect("heaven request should build"),
    )
    .expect("heaven by_lunar should build");

    assert_eq!(default_chart, heaven_chart);
}

#[test]
fn quanshu_earth_is_unsupported() {
    let request = base_builder(quanshu_profile())
        .chart_plane(ChartPlane::Earth)
        .build()
        .expect("request should build");

    assert_eq!(
        by_lunar(request),
        Err(ChartError::UnsupportedChartPlane {
            algorithm: ChartAlgorithmKind::QuanShu,
            plane: ChartPlane::Earth,
        }),
    );
}

#[test]
fn quanshu_human_is_unsupported() {
    let request = base_builder(quanshu_profile())
        .chart_plane(ChartPlane::Human)
        .build()
        .expect("request should build");

    assert_eq!(
        by_lunar(request),
        Err(ChartError::UnsupportedChartPlane {
            algorithm: ChartAlgorithmKind::QuanShu,
            plane: ChartPlane::Human,
        }),
    );
}

#[test]
fn zhongzhou_heaven_still_succeeds() {
    let request = base_builder(zhongzhou_profile())
        .chart_plane(ChartPlane::Heaven)
        .build()
        .expect("request should build");

    assert!(by_lunar(request).is_ok());
}

#[test]
fn zhongzhou_earth_succeeds() {
    let request = base_builder(zhongzhou_profile())
        .chart_plane(ChartPlane::Earth)
        .build()
        .expect("request should build");

    assert!(by_lunar(request).is_ok());
}

#[test]
fn zhongzhou_human_succeeds() {
    let request = base_builder(zhongzhou_profile())
        .chart_plane(ChartPlane::Human)
        .build()
        .expect("request should build");

    assert!(by_lunar(request).is_ok());
}

#[test]
fn zhongzhou_default_matches_explicit_heaven() {
    let default_chart = by_lunar(
        base_builder(zhongzhou_profile())
            .build()
            .expect("default request should build"),
    )
    .expect("default by_lunar should build");

    assert_eq!(default_chart, zhongzhou_chart(ChartPlane::Heaven));
}

#[test]
fn placeholder_earth_is_unsupported() {
    let request = base_builder(placeholder_profile())
        .chart_plane(ChartPlane::Earth)
        .build()
        .expect("request should build");

    assert_eq!(
        by_lunar(request),
        Err(ChartError::UnsupportedChartPlane {
            algorithm: ChartAlgorithmKind::Placeholder,
            plane: ChartPlane::Earth,
        }),
    );
}

#[test]
fn placeholder_human_is_unsupported() {
    let request = base_builder(placeholder_profile())
        .chart_plane(ChartPlane::Human)
        .build()
        .expect("request should build");

    assert_eq!(
        by_lunar(request),
        Err(ChartError::UnsupportedChartPlane {
            algorithm: ChartAlgorithmKind::Placeholder,
            plane: ChartPlane::Human,
        }),
    );
}

#[test]
fn zhongzhou_earth_life_palace_anchors_to_heaven_body_palace() {
    let heaven = zhongzhou_chart(ChartPlane::Heaven);
    let earth = zhongzhou_chart(ChartPlane::Earth);

    assert_eq!(
        life_palace_branch(&earth),
        heaven
            .body_palace_branch()
            .expect("heaven chart should have a Body Palace branch"),
    );
}

#[test]
fn zhongzhou_human_life_palace_anchors_to_heaven_fortune_palace() {
    let heaven = zhongzhou_chart(ChartPlane::Heaven);
    let human = zhongzhou_chart(ChartPlane::Human);

    assert_eq!(life_palace_branch(&human), fortune_palace_branch(&heaven));
}

#[test]
fn reanchored_bureau_follows_reanchored_life_palace() {
    for plane in [ChartPlane::Earth, ChartPlane::Human] {
        let chart = zhongzhou_chart(plane);
        let life = chart
            .life_palace()
            .expect("chart should have a Life Palace");
        let expected = five_element_bureau_from_life_palace(
            StemBranch::try_new(life.stem(), life.branch())
                .expect("life palace stem-branch should be valid"),
        );

        assert_eq!(chart.five_element_bureau(), Some(expected));
    }
}

#[test]
fn reanchored_planes_differ_from_heaven() {
    let heaven = zhongzhou_chart(ChartPlane::Heaven);

    // The fixture's Body and Fortune palaces are not the Life Palace, so
    // both re-anchored planes must differ from Heaven.
    assert_ne!(zhongzhou_chart(ChartPlane::Earth), heaven);
    assert_ne!(zhongzhou_chart(ChartPlane::Human), heaven);
}

#[test]
fn quanshu_heaven_chart_carries_heaven_plane() {
    let chart = by_lunar(
        base_builder(quanshu_profile())
            .chart_plane(ChartPlane::Heaven)
            .build()
            .expect("request should build"),
    )
    .expect("chart should build");

    assert_eq!(chart.chart_plane(), ChartPlane::Heaven);
    assert_eq!(
        chart.method_profile().algorithm_kind(),
        ChartAlgorithmKind::QuanShu,
    );
}

#[test]
fn default_request_chart_carries_heaven_plane() {
    let chart = by_lunar(
        base_builder(quanshu_profile())
            .build()
            .expect("request should build"),
    )
    .expect("chart should build");

    assert_eq!(chart.chart_plane(), ChartPlane::Heaven);
}

#[test]
fn zhongzhou_charts_carry_requested_plane() {
    for plane in [ChartPlane::Heaven, ChartPlane::Earth, ChartPlane::Human] {
        let chart = zhongzhou_chart(plane);

        assert_eq!(chart.chart_plane(), plane);
        assert_eq!(
            chart.method_profile().algorithm_kind(),
            ChartAlgorithmKind::Zhongzhou,
        );
    }
}

#[test]
fn reanchored_planes_still_place_stars() {
    for plane in [ChartPlane::Earth, ChartPlane::Human] {
        let chart = zhongzhou_chart(plane);
        assert!(
            chart
                .palaces()
                .iter()
                .any(|palace| !palace.stars().is_empty()),
            "{plane:?} chart should have placed stars",
        );
    }
}
