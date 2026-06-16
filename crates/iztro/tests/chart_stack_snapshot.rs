use iztro::core::{
    Brightness, ChartAlgorithmKind, ChartLayerKind, ChartStackSnapshot, EarthlyBranch, Gender,
    HeavenlyStem, HoroscopeChart, MethodProfile, Mutagen, MutagenActivation, PalaceGridPosition,
    PalaceRoleKind, Scope, ScopedStarPlacement, SolarChartRequest, SolarDay, SolarMonth, StarKind,
    StarName, StarPlacement, StemBranch, TemporalContext, TemporalLayer, VISUAL_BRANCH_ORDER,
    by_solar, palace_grid_position,
};

fn solar_fixture_chart() -> iztro::core::Chart {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("May should be valid"))
        .solar_day(SolarDay::new(17).expect("day 17 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "chart_stack_snapshot_test",
            ChartAlgorithmKind::QuanShu,
            "chart stack snapshot test",
        ))
        .build()
        .expect("solar chart request should build");

    by_solar(request).expect("by_solar should build fixture chart")
}

#[test]
fn natal_chart_stack_snapshot_preserves_renderer_ready_natal_facts() {
    let chart = solar_fixture_chart();

    let snapshot = chart.stack_snapshot();

    assert_eq!(snapshot.birth_context(), chart.birth_context());
    assert_eq!(snapshot.method_profile(), chart.method_profile());
    assert_eq!(snapshot.birth_year(), chart.birth_year());
    assert_eq!(
        snapshot.life_palace_branch(),
        chart.life_palace().map(|palace| palace.branch())
    );
    assert_eq!(snapshot.body_palace_branch(), chart.body_palace_branch());
    assert_eq!(snapshot.five_element_bureau(), chart.five_element_bureau());

    let layers = snapshot.layers();
    assert_eq!(layers.len(), 1);

    let natal_layer = snapshot
        .layer(ChartLayerKind::Natal)
        .expect("natal snapshot layer should exist");
    assert_eq!(natal_layer.kind(), ChartLayerKind::Natal);
    assert_eq!(natal_layer.z_index(), 0);
    assert_eq!(natal_layer.context(), None);
    assert_eq!(natal_layer.cells().len(), 12);

    let branches: Vec<EarthlyBranch> = natal_layer
        .cells()
        .iter()
        .map(|cell| cell.branch())
        .collect();
    assert_eq!(branches, VISUAL_BRANCH_ORDER);

    assert_eq!(
        palace_grid_position(EarthlyBranch::Si),
        PalaceGridPosition::new(0, 0)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Wu),
        PalaceGridPosition::new(0, 1)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Wei),
        PalaceGridPosition::new(0, 2)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Shen),
        PalaceGridPosition::new(0, 3)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Chen),
        PalaceGridPosition::new(1, 0)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::You),
        PalaceGridPosition::new(1, 3)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Mao),
        PalaceGridPosition::new(2, 0)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Xu),
        PalaceGridPosition::new(2, 3)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Yin),
        PalaceGridPosition::new(3, 0)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Chou),
        PalaceGridPosition::new(3, 1)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Zi),
        PalaceGridPosition::new(3, 2)
    );
    assert_eq!(
        palace_grid_position(EarthlyBranch::Hai),
        PalaceGridPosition::new(3, 3)
    );
    for cell in natal_layer.cells() {
        assert_eq!(cell.grid_position(), palace_grid_position(cell.branch()));
        assert_eq!(
            cell.natal_palace_name(),
            chart
                .palaces()
                .iter()
                .find(|palace| palace.branch() == cell.branch())
                .map(|palace| palace.name())
        );
        assert_eq!(
            cell.natal_palace_stem(),
            chart
                .palaces()
                .iter()
                .find(|palace| palace.branch() == cell.branch())
                .map(|palace| palace.stem())
        );
        assert!(cell.scoped_stars().is_empty());
        assert!(cell.mutagen_activations().is_empty());
        assert!(
            cell.roles()
                .iter()
                .any(|role| matches!(role.kind(), PalaceRoleKind::NatalPalace(_)))
        );
    }

    let typed_star_count: usize = natal_layer
        .cells()
        .iter()
        .map(|cell| cell.typed_stars().len())
        .sum();
    let decorative_star_count: usize = natal_layer
        .cells()
        .iter()
        .map(|cell| cell.decorative_stars().len())
        .sum();
    assert_eq!(typed_star_count, chart.stars().len());
    assert_eq!(decorative_star_count, chart.decorative_stars().len());
    assert!(
        natal_layer
            .cells()
            .iter()
            .flat_map(|cell| cell.typed_stars())
            .all(|star| star.scope() == Scope::Natal)
    );
    assert!(
        natal_layer
            .cells()
            .iter()
            .flat_map(|cell| cell.decorative_stars())
            .all(|star| star.scope() == Scope::Natal)
    );

    let body_branch = chart
        .body_palace_branch()
        .expect("fixture chart should have a body palace branch");
    let body_cell = natal_layer
        .cells()
        .iter()
        .find(|cell| cell.branch() == body_branch)
        .expect("body palace branch should have a cell");
    assert!(
        body_cell
            .roles()
            .iter()
            .any(|role| role.kind() == PalaceRoleKind::NatalBodyPalace)
    );

    let encoded = serde_json::to_string(&snapshot).expect("snapshot should serialize");
    let decoded: ChartStackSnapshot =
        serde_json::from_str(&encoded).expect("snapshot should deserialize");

    assert_eq!(decoded, snapshot);
}

#[test]
fn horoscope_chart_stack_snapshot_groups_temporal_overlays_by_branch_without_duplicating_natal_stars()
 {
    let natal = solar_fixture_chart();
    let context = TemporalContext::Yearly {
        stem_branch: StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu)
            .expect("valid sexagenary pair"),
        lunar_year: 1990,
    };
    let scoped_star = ScopedStarPlacement::new(
        EarthlyBranch::Si,
        StarPlacement::new(
            StarName::NianJieYearly,
            StarKind::Helper,
            Brightness::Unknown,
            None,
            Scope::Yearly,
        ),
    );
    let activation = MutagenActivation::new(
        Scope::Yearly,
        StarName::TaiYang,
        EarthlyBranch::Wu,
        Mutagen::Lu,
    );
    let temporal_layer =
        TemporalLayer::try_new(Scope::Yearly, context, vec![scoped_star], vec![activation])
            .expect("temporal layer should build");
    let horoscope = HoroscopeChart::with_layers(natal.clone(), vec![temporal_layer]);

    let snapshot = ChartStackSnapshot::from_horoscope_chart(&horoscope);

    assert_eq!(snapshot.birth_context(), natal.birth_context());
    assert_eq!(snapshot.method_profile(), natal.method_profile());
    assert_eq!(snapshot.birth_year(), natal.birth_year());
    assert_eq!(snapshot.layers().len(), 2);
    assert_eq!(snapshot.layers()[0].kind(), ChartLayerKind::Natal);
    assert_eq!(snapshot.layers()[0].z_index(), 0);

    let temporal = &snapshot.layers()[1];
    assert_eq!(temporal.kind(), ChartLayerKind::Yearly);
    assert_eq!(temporal.z_index(), 1);
    assert_eq!(temporal.context(), Some(&context));

    assert!(
        temporal
            .cells()
            .iter()
            .all(|cell| cell.typed_stars().is_empty())
    );
    assert!(
        temporal
            .cells()
            .iter()
            .all(|cell| cell.decorative_stars().is_empty())
    );
    assert!(temporal.cells().iter().all(|cell| cell.roles().is_empty()));

    for cell in temporal.cells() {
        let natal_palace = natal
            .palaces()
            .iter()
            .find(|palace| palace.branch() == cell.branch())
            .expect("natal branch should have a palace");
        assert_eq!(cell.natal_palace_name(), Some(natal_palace.name()));
        assert_eq!(cell.natal_palace_stem(), Some(natal_palace.stem()));
    }

    let scoped_cell = temporal
        .cells()
        .iter()
        .find(|cell| cell.branch() == EarthlyBranch::Si)
        .expect("Si temporal cell should exist");
    assert_eq!(scoped_cell.scoped_stars().len(), 1);
    assert_eq!(
        scoped_cell.scoped_stars()[0].name(),
        StarName::NianJieYearly
    );
    assert_eq!(scoped_cell.scoped_stars()[0].scope(), Scope::Yearly);
    assert!(scoped_cell.mutagen_activations().is_empty());

    let activation_cell = temporal
        .cells()
        .iter()
        .find(|cell| cell.branch() == EarthlyBranch::Wu)
        .expect("Wu temporal cell should exist");
    assert_eq!(activation_cell.mutagen_activations().len(), 1);
    assert_eq!(
        activation_cell.mutagen_activations()[0].target_star(),
        StarName::TaiYang
    );
    assert_eq!(
        activation_cell.mutagen_activations()[0].target_branch(),
        EarthlyBranch::Wu
    );
    assert_eq!(
        activation_cell.mutagen_activations()[0].source_scope(),
        Scope::Yearly
    );
    assert_eq!(
        activation_cell.mutagen_activations()[0].mutagen(),
        Mutagen::Lu
    );
    assert!(activation_cell.scoped_stars().is_empty());
}
