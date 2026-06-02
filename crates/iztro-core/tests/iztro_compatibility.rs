use iztro_core::{
    BirthContext, CalendarDate, EarthlyBranch, FiveElementBureau, Gender, HeavenlyStem, LunarMonth,
    MethodProfile, NatalChartInput, PalaceName, build_minimal_natal_chart,
};
use serde_json::Value;

const MINIMAL_NATAL_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/minimal_natal_1990_05_17_chen_female.json");

#[test]
fn minimal_natal_chart_matches_supported_iztro_fixture_fields() {
    let fixture: Value =
        serde_json::from_str(MINIMAL_NATAL_FIXTURE).expect("fixture should be valid JSON");
    let input = &fixture["input"];
    let expected = &fixture["supported_fields"];
    let lunar_month = LunarMonth::new(
        input["lunar_month"]
            .as_u64()
            .expect("fixture should include lunar_month") as u8,
    )
    .expect("fixture lunar month should be valid");
    let solar_date = parse_solar_date(
        input["solar_date"]
            .as_str()
            .expect("fixture should include solar_date"),
    );
    let birth_branch = parse_branch_key(
        input["birth_time"]
            .as_str()
            .expect("fixture should include birth_time"),
    );
    let gender = parse_gender_key(
        input["gender"]
            .as_str()
            .expect("fixture should include gender"),
    );
    let birth_year_stem = parse_stem_key(
        input["birth_year_stem"]
            .as_str()
            .expect("fixture should include birth_year_stem"),
    );
    let birth_context = BirthContext::new(solar_date, birth_branch, gender);
    let chart = build_minimal_natal_chart(NatalChartInput::new(
        birth_context.clone(),
        MethodProfile::placeholder("iztro_compatibility_fixture"),
        lunar_month,
        birth_year_stem,
    ))
    .expect("minimal natal chart should build for fixture input");
    let life_palace = chart
        .palaces()
        .iter()
        .find(|palace| palace.name() == PalaceName::Life)
        .expect("chart should contain a Life Palace");
    let expected_body_branch = parse_branch_key(
        expected["body_palace_branch"]
            .as_str()
            .expect("fixture should include body_palace_branch"),
    );

    assert_eq!(chart.body_palace_branch(), Some(expected_body_branch));
    assert_eq!(
        chart.body_palace().map(|palace| palace.branch()),
        Some(expected_body_branch)
    );
    assert!(chart.is_body_palace_branch(expected_body_branch));
    assert_eq!(
        fixture["metadata"]["target_version"]
            .as_str()
            .expect("fixture should record target version"),
        "2.5.8"
    );
    assert!(
        fixture["metadata"]["supported_fields_only"]
            .as_bool()
            .expect("fixture should mark supported_fields_only")
    );
    assert_eq!(expected["birth_time"].as_str(), Some("chen"));
    assert_eq!(expected["gender"].as_str(), Some("female"));
    assert_eq!(
        expected["life_palace_branch"].as_str(),
        Some(branch_key(life_palace.branch()))
    );

    // The five-element bureau is compared against iztro's fiveElementsClass
    // (火六局 -> fire6) only when the fixture records it.
    if let Some(expected_bureau) = expected.get("five_element_bureau").and_then(Value::as_str) {
        assert_eq!(
            chart.five_element_bureau().map(bureau_key),
            Some(expected_bureau)
        );
    }

    let palace_fields = expected["palaces"]
        .as_array()
        .expect("fixture should include supported palace fields");
    assert_eq!(palace_fields.len(), chart.palaces().len());

    for palace in chart.palaces() {
        let expected_palace = palace_fields
            .iter()
            .find(|expected_palace| {
                expected_palace["branch"].as_str() == Some(branch_key(palace.branch()))
            })
            .expect("fixture should contain every palace branch");

        assert_eq!(
            expected_palace["name"].as_str(),
            Some(palace_name_key(palace.name()))
        );

        // Palace stems are compared only when the fixture records them.
        if let Some(expected_stem) = expected_palace.get("stem").and_then(Value::as_str) {
            assert_eq!(stem_key(palace.stem()), expected_stem);
        }
    }
}

fn branch_key(branch: EarthlyBranch) -> &'static str {
    match branch {
        EarthlyBranch::Zi => "zi",
        EarthlyBranch::Chou => "chou",
        EarthlyBranch::Yin => "yin",
        EarthlyBranch::Mao => "mao",
        EarthlyBranch::Chen => "chen",
        EarthlyBranch::Si => "si",
        EarthlyBranch::Wu => "wu",
        EarthlyBranch::Wei => "wei",
        EarthlyBranch::Shen => "shen",
        EarthlyBranch::You => "you",
        EarthlyBranch::Xu => "xu",
        EarthlyBranch::Hai => "hai",
    }
}

fn stem_key(stem: HeavenlyStem) -> &'static str {
    match stem {
        HeavenlyStem::Jia => "jia",
        HeavenlyStem::Yi => "yi",
        HeavenlyStem::Bing => "bing",
        HeavenlyStem::Ding => "ding",
        HeavenlyStem::Wu => "wu",
        HeavenlyStem::Ji => "ji",
        HeavenlyStem::Geng => "geng",
        HeavenlyStem::Xin => "xin",
        HeavenlyStem::Ren => "ren",
        HeavenlyStem::Gui => "gui",
    }
}

fn bureau_key(bureau: FiveElementBureau) -> &'static str {
    match bureau {
        FiveElementBureau::Water2 => "water2",
        FiveElementBureau::Wood3 => "wood3",
        FiveElementBureau::Metal4 => "metal4",
        FiveElementBureau::Earth5 => "earth5",
        FiveElementBureau::Fire6 => "fire6",
    }
}

fn parse_solar_date(value: &str) -> CalendarDate {
    let mut parts = value.split('-');
    let year = parts
        .next()
        .and_then(|part| part.parse::<i32>().ok())
        .unwrap_or_else(|| panic!("unsupported solar_date in fixture: {value}"));
    let month = parts
        .next()
        .and_then(|part| part.parse::<u8>().ok())
        .unwrap_or_else(|| panic!("unsupported solar_date in fixture: {value}"));
    let day = parts
        .next()
        .and_then(|part| part.parse::<u8>().ok())
        .unwrap_or_else(|| panic!("unsupported solar_date in fixture: {value}"));
    if parts.next().is_some() {
        panic!("unsupported solar_date in fixture: {value}");
    }

    CalendarDate::solar(year, month, day)
}

fn parse_branch_key(value: &str) -> EarthlyBranch {
    match value {
        "zi" => EarthlyBranch::Zi,
        "chou" => EarthlyBranch::Chou,
        "yin" => EarthlyBranch::Yin,
        "mao" => EarthlyBranch::Mao,
        "chen" => EarthlyBranch::Chen,
        "si" => EarthlyBranch::Si,
        "wu" => EarthlyBranch::Wu,
        "wei" => EarthlyBranch::Wei,
        "shen" => EarthlyBranch::Shen,
        "you" => EarthlyBranch::You,
        "xu" => EarthlyBranch::Xu,
        "hai" => EarthlyBranch::Hai,
        other => panic!("unsupported branch key in fixture: {other}"),
    }
}

fn parse_stem_key(value: &str) -> HeavenlyStem {
    match value {
        "jia" => HeavenlyStem::Jia,
        "yi" => HeavenlyStem::Yi,
        "bing" => HeavenlyStem::Bing,
        "ding" => HeavenlyStem::Ding,
        "wu" => HeavenlyStem::Wu,
        "ji" => HeavenlyStem::Ji,
        "geng" => HeavenlyStem::Geng,
        "xin" => HeavenlyStem::Xin,
        "ren" => HeavenlyStem::Ren,
        "gui" => HeavenlyStem::Gui,
        other => panic!("unsupported stem key in fixture: {other}"),
    }
}

fn parse_gender_key(value: &str) -> Gender {
    match value {
        "male" => Gender::Male,
        "female" => Gender::Female,
        other => panic!("unsupported gender key in fixture: {other}"),
    }
}

fn palace_name_key(name: PalaceName) -> &'static str {
    match name {
        PalaceName::Life => "life",
        PalaceName::Siblings => "siblings",
        PalaceName::Spouse => "spouse",
        PalaceName::Children => "children",
        PalaceName::Wealth => "wealth",
        PalaceName::Health => "health",
        PalaceName::Migration => "migration",
        PalaceName::Friends => "friends",
        PalaceName::Career => "career",
        PalaceName::Property => "property",
        PalaceName::Spirit => "spirit",
        PalaceName::Parents => "parents",
    }
}
