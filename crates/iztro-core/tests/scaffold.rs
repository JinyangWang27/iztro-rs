use iztro_core::{
    BirthContext, Brightness, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem,
    MethodProfile, Mutagen, Palace, PalaceName, Scope, StarCategory, StarName, StarPlacement,
};

#[test]
fn chart_scaffold_can_be_constructed_and_serialized() {
    let chart = Chart::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("quan_shu_placeholder"),
        vec![Palace::new(
            PalaceName::Life,
            EarthlyBranch::Zi,
            HeavenlyStem::Jia,
            vec![StarPlacement::new(
                StarName::ZiWei,
                StarCategory::Major,
                Brightness::Temple,
                Some(Mutagen::Lu),
                Scope::Natal,
            )],
        )],
    );

    assert_eq!(chart.birth_context().birth_time(), EarthlyBranch::Chen);
    assert_eq!(chart.palaces().len(), 1);

    let encoded = serde_json::to_string(&chart).expect("chart should serialize");
    let decoded: Chart = serde_json::from_str(&encoded).expect("chart should deserialize");

    assert_eq!(decoded.method_profile().id(), "quan_shu_placeholder");
    assert_eq!(decoded.palaces()[0].stars()[0].name(), StarName::ZiWei);
}
