use iztro::core::{
    Chart, ChartAlgorithmKind, ChartError, ChartPlane, EarthlyBranch, Gender, MethodProfile,
    PalaceName, SolarChartRequest, SolarChartRequestBuilder, SolarDay, SolarMonth, by_solar,
};

fn base_builder(profile: MethodProfile) -> SolarChartRequestBuilder {
    SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(6).expect("valid solar month"))
        .solar_day(SolarDay::new(15).expect("valid solar day"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
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
fn chart_plane_propagates_into_lunar_request_path() {
    // A Zhongzhou + Earth solar request must produce a chart that differs
    // from the Heaven plane, proving the chart plane is propagated
    // downstream rather than dropped.
    let earth = by_solar(
        base_builder(zhongzhou_profile())
            .chart_plane(ChartPlane::Earth)
            .build()
            .expect("earth request should build"),
    )
    .expect("zhongzhou earth solar request should build");

    let heaven = by_solar(
        base_builder(zhongzhou_profile())
            .chart_plane(ChartPlane::Heaven)
            .build()
            .expect("heaven request should build"),
    )
    .expect("zhongzhou heaven solar request should build");

    assert_ne!(earth, heaven);
}

#[test]
fn zhongzhou_earth_solar_request_anchors_to_heaven_body_palace() {
    let solar_chart = |plane: ChartPlane| {
        by_solar(
            base_builder(zhongzhou_profile())
                .chart_plane(plane)
                .build()
                .expect("request should build"),
        )
        .expect("zhongzhou solar chart should build")
    };

    let heaven = solar_chart(ChartPlane::Heaven);
    let earth = solar_chart(ChartPlane::Earth);
    let human = solar_chart(ChartPlane::Human);

    let life_branch = |chart: &Chart| {
        chart
            .life_palace()
            .expect("chart should have a Life Palace")
            .branch()
    };
    let fortune_branch = |chart: &Chart| {
        chart
            .palace_by_name(PalaceName::Spirit)
            .expect("chart should have a Fortune Palace")
            .branch()
    };

    assert_eq!(
        life_branch(&earth),
        heaven
            .body_palace_branch()
            .expect("heaven chart should have a Body Palace branch"),
    );
    assert_eq!(life_branch(&human), fortune_branch(&heaven));
}

#[test]
fn solar_request_chart_carries_requested_plane() {
    // by_solar reconstructs the chart after delegating to by_lunar; this
    // guards against accidentally resetting the plane to Heaven during that
    // final reconstruction.
    let solar_chart = |plane: ChartPlane| {
        by_solar(
            base_builder(zhongzhou_profile())
                .chart_plane(plane)
                .build()
                .expect("request should build"),
        )
        .expect("zhongzhou solar chart should build")
    };

    for plane in [ChartPlane::Heaven, ChartPlane::Earth, ChartPlane::Human] {
        let chart = solar_chart(plane);
        assert_eq!(chart.chart_plane(), plane);
        assert_eq!(
            chart.method_profile().algorithm_kind(),
            ChartAlgorithmKind::Zhongzhou,
        );
    }
}

#[test]
fn quanshu_earth_solar_request_is_unsupported() {
    let request = base_builder(quanshu_profile())
        .chart_plane(ChartPlane::Earth)
        .build()
        .expect("request should build");

    assert_eq!(
        by_solar(request),
        Err(ChartError::UnsupportedChartPlane {
            algorithm: ChartAlgorithmKind::QuanShu,
            plane: ChartPlane::Earth,
        }),
    );
}
