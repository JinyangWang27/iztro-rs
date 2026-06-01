use iztro_core::{
    BirthContext, Brightness, CalendarDate, Chart, ChartAlgorithmKind, ChartError,
    EARTHLY_BRANCHES, EarthlyBranch, Gender, HEAVENLY_STEMS, HeavenlyStem, MethodProfile, Mutagen,
    PALACE_NAMES, Palace, PalaceName, Scope, StarCategory, StarName, StarPlacement,
};

#[test]
fn chart_scaffold_can_be_constructed_and_serialized() {
    let chart = Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("quan_shu_placeholder"),
        twelve_palaces(),
    )
    .expect("twelve-palace scaffold chart should serialize");

    assert_eq!(chart.birth_context().birth_time(), EarthlyBranch::Chen);
    assert_eq!(chart.palaces().len(), 12);

    let encoded = serde_json::to_string(&chart).expect("chart should serialize");
    let decoded: Chart = serde_json::from_str(&encoded).expect("chart should deserialize");

    assert_eq!(decoded.method_profile().id(), "quan_shu_placeholder");
    assert_eq!(decoded.palaces()[0].stars()[0].name(), StarName::ZiWei);
}

#[test]
fn heavenly_stems_have_canonical_cyclic_ordering() {
    for (index, stem) in HEAVENLY_STEMS.iter().copied().enumerate() {
        assert_eq!(stem.index(), index);
        assert_eq!(HeavenlyStem::from_index(index), stem);
    }

    assert_eq!(HeavenlyStem::from_index(10), HeavenlyStem::Jia);
    assert_eq!(HeavenlyStem::Jia.offset(1), HeavenlyStem::Yi);
    assert_eq!(HeavenlyStem::Gui.offset(1), HeavenlyStem::Jia);
    assert_eq!(HeavenlyStem::Jia.offset(-1), HeavenlyStem::Gui);
}

#[test]
fn earthly_branches_have_canonical_cyclic_ordering() {
    for (index, branch) in EARTHLY_BRANCHES.iter().copied().enumerate() {
        assert_eq!(branch.index(), index);
        assert_eq!(EarthlyBranch::from_index(index), branch);
    }

    assert_eq!(EarthlyBranch::from_index(12), EarthlyBranch::Zi);
    assert_eq!(EarthlyBranch::Zi.offset(1), EarthlyBranch::Chou);
    assert_eq!(EarthlyBranch::Zi.offset(-1), EarthlyBranch::Hai);
    assert_eq!(EarthlyBranch::Hai.offset(1), EarthlyBranch::Zi);
}

#[test]
fn palaces_have_canonical_cyclic_ordering() {
    let expected = [
        PalaceName::Life,
        PalaceName::Siblings,
        PalaceName::Spouse,
        PalaceName::Children,
        PalaceName::Wealth,
        PalaceName::Health,
        PalaceName::Migration,
        PalaceName::Friends,
        PalaceName::Career,
        PalaceName::Property,
        PalaceName::Spirit,
        PalaceName::Parents,
    ];

    assert_eq!(PALACE_NAMES, expected);
    for (index, palace) in PALACE_NAMES.iter().copied().enumerate() {
        assert_eq!(palace.index(), index);
        assert_eq!(PalaceName::from_index(index), palace);
    }

    assert_eq!(PalaceName::from_index(12), PalaceName::Life);
    assert_eq!(PalaceName::Life.offset(1), PalaceName::Siblings);
    assert_eq!(PalaceName::Life.offset(-1), PalaceName::Parents);
    assert_eq!(PalaceName::Parents.offset(1), PalaceName::Life);
}

#[test]
fn fourteen_major_star_names_round_trip_through_json() {
    let stars = [
        StarName::ZiWei,
        StarName::TianJi,
        StarName::TaiYang,
        StarName::WuQu,
        StarName::TianTong,
        StarName::LianZhen,
        StarName::TianFu,
        StarName::TaiYin,
        StarName::TanLang,
        StarName::JuMen,
        StarName::TianXiang,
        StarName::TianLiang,
        StarName::QiSha,
        StarName::PoJun,
    ];

    let encoded = serde_json::to_string(&stars).expect("star names should serialize");
    assert!(encoded.contains("zi_wei"));
    assert!(encoded.contains("po_jun"));

    let decoded: [StarName; 14] =
        serde_json::from_str(&encoded).expect("star names should deserialize");
    assert_eq!(decoded, stars);
}

#[test]
fn placeholder_method_profile_has_typed_algorithm_kind() {
    let profile = MethodProfile::placeholder("placeholder_profile");

    assert_eq!(profile.id(), "placeholder_profile");
    assert_eq!(profile.algorithm_kind(), ChartAlgorithmKind::Placeholder);
}

#[test]
fn chart_try_new_accepts_exactly_twelve_palaces() {
    let chart = Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("valid_chart"),
        twelve_palaces(),
    )
    .expect("twelve palaces should satisfy the core invariant");

    assert_eq!(chart.palaces().len(), 12);
}

#[test]
fn chart_try_new_rejects_non_twelve_palace_counts() {
    let error = Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("invalid_chart"),
        Vec::new(),
    )
    .expect_err("empty palace list should fail");

    assert_eq!(
        error,
        ChartError::InvalidPalaceCount {
            expected: 12,
            actual: 0
        }
    );
}

fn twelve_palaces() -> Vec<Palace> {
    PALACE_NAMES
        .iter()
        .copied()
        .enumerate()
        .map(|(index, palace)| {
            Palace::new(
                palace,
                EarthlyBranch::from_index(index),
                HeavenlyStem::from_index(index),
                if palace == PalaceName::Life {
                    vec![StarPlacement::new(
                        StarName::ZiWei,
                        StarCategory::Major,
                        Brightness::Temple,
                        Some(Mutagen::Lu),
                        Scope::Natal,
                    )]
                } else {
                    Vec::new()
                },
            )
        })
        .collect()
}
