//! Factual natal four-pillar export tests for the facade snapshots.
//!
//! These assert that `NatalFacadeSnapshot` exposes the factual natal four
//! pillars retained on `Chart::four_pillars()` as optional structured facts with
//! additive zh-CN labels: present for `by_solar`-derived charts, absent for
//! `by_lunar`-derived charts. They also assert the horoscope facade carries the
//! natal four-pillar facts through its embedded `astrolabe`, and that the
//! optional field round-trips through JSON. No BaZi interpretation is exported.

use iztro::core::{
    Chart, EarthlyBranch, Gender, HeavenlyStem, HoroscopeFacadeSnapshot, HoroscopeStackInput,
    LunarChartRequest, LunarDay, LunarMonth, MethodProfile, NatalFacadeSnapshot, SolarChartRequest,
    SolarDay, SolarMonth, StemBranch, build_full_horoscope_chart, by_lunar, by_solar,
};

fn by_solar_chart() -> Chart {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("May should be valid"))
        .solar_day(SolarDay::new(17).expect("day 17 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::placeholder("natal_four_pillars_by_solar"))
        .build()
        .expect("solar request should build");

    by_solar(request).expect("by_solar should build chart")
}

fn by_lunar_chart() -> Chart {
    let request = LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
        .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .birth_year_stem(HeavenlyStem::Geng)
        .birth_year_branch(EarthlyBranch::Wu)
        .method_profile(MethodProfile::placeholder("natal_four_pillars_by_lunar"))
        .build()
        .expect("lunar request should build");

    by_lunar(request).expect("by_lunar should build chart")
}

#[test]
fn by_solar_natal_facade_exposes_some_four_pillars() {
    let chart = by_solar_chart();
    let snapshot = NatalFacadeSnapshot::from_chart(&chart);

    let pillars = snapshot
        .four_pillars()
        .expect("by_solar natal facade must expose four pillars");

    // The retained year pillar must equal the chart's factual birth-year stem-branch.
    assert_eq!(pillars.yearly(), chart.birth_year());
    assert_eq!(
        Some(pillars.yearly()),
        chart.four_pillars().map(|fact| fact.yearly)
    );
}

#[test]
fn by_solar_natal_facade_four_pillar_zh_labels_are_non_empty() {
    let snapshot = NatalFacadeSnapshot::from_chart(&by_solar_chart());
    let pillars = snapshot.four_pillars().expect("four pillars present");

    for label in [
        pillars.yearly_zh(),
        pillars.monthly_zh(),
        pillars.daily_zh(),
        pillars.hourly_zh(),
    ] {
        assert!(!label.is_empty(), "four-pillar zh label must be non-empty");
        // A 干支 label is exactly one stem char plus one branch char.
        assert_eq!(label.chars().count(), 2, "干支 label should be two chars");
    }
}

#[test]
fn by_lunar_natal_facade_has_no_four_pillars() {
    let chart = by_lunar_chart();
    assert!(
        chart.four_pillars().is_none(),
        "by_lunar charts do not derive full four pillars"
    );

    let snapshot = NatalFacadeSnapshot::from_chart(&chart);
    assert!(
        snapshot.four_pillars().is_none(),
        "by_lunar natal facade must stay honest with None"
    );
}

#[test]
fn horoscope_facade_astrolabe_preserves_natal_four_pillars() {
    let chart = by_solar_chart();
    let stack = HoroscopeStackInput::new(
        2020,
        SolarMonth::new(8).expect("August should be valid"),
        SolarDay::new(20).expect("day 20 should be valid"),
        chart.birth_context().birth_time_variant(),
    );
    let horoscope =
        build_full_horoscope_chart(chart, stack).expect("full horoscope stack should build");

    assert!(
        horoscope.natal().four_pillars().is_some(),
        "by_solar natal chart should retain four pillars"
    );

    let facade = HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope)
        .expect("facade snapshot should build");
    let natal = NatalFacadeSnapshot::from_chart(horoscope.natal());

    assert_eq!(
        facade.astrolabe().four_pillars(),
        natal.four_pillars(),
        "embedded astrolabe must carry the natal four-pillar facts"
    );
    assert_eq!(
        facade.astrolabe().four_pillars().map(|p| p.yearly()),
        Some(horoscope.natal().birth_year()),
        "embedded astrolabe year pillar matches the natal birth year"
    );
}

#[test]
fn natal_facade_four_pillars_round_trip_through_json() {
    // by_solar: optional field present and preserved.
    let solar = NatalFacadeSnapshot::from_chart(&by_solar_chart());
    let encoded = serde_json::to_value(&solar).expect("solar facade should serialize");
    assert!(
        encoded.get("four_pillars").is_some(),
        "by_solar facade should serialize four_pillars"
    );
    // The structured pillar is a machine-readable StemBranch object, while the
    // additive zh label sits beside it as a string.
    assert!(encoded["four_pillars"]["yearly"].is_object());
    assert!(encoded["four_pillars"]["yearly"]["stem"].is_string());
    assert!(encoded["four_pillars"]["yearly"]["branch"].is_string());
    assert!(encoded["four_pillars"]["yearly_zh"].is_string());
    let decoded: NatalFacadeSnapshot =
        serde_json::from_value(encoded).expect("solar facade should deserialize");
    assert_eq!(decoded, solar);

    // by_lunar: optional field omitted (additive) and round-trips as None.
    let lunar = NatalFacadeSnapshot::from_chart(&by_lunar_chart());
    let encoded = serde_json::to_value(&lunar).expect("lunar facade should serialize");
    assert!(
        encoded.get("four_pillars").is_none(),
        "by_lunar facade should omit the absent four_pillars field"
    );
    let decoded: NatalFacadeSnapshot =
        serde_json::from_value(encoded).expect("lunar facade should deserialize");
    assert_eq!(decoded, lunar);
    assert!(decoded.four_pillars().is_none());
}

#[test]
fn machine_readable_pillars_are_not_replaced_by_strings() {
    let snapshot = NatalFacadeSnapshot::from_chart(&by_solar_chart());
    let pillars = snapshot.four_pillars().expect("four pillars present");

    // Structured StemBranch facts sit beside, not instead of, the zh labels.
    let _yearly: StemBranch = pillars.yearly();
    let _monthly: StemBranch = pillars.monthly();
    let _daily: StemBranch = pillars.daily();
    let _hourly: StemBranch = pillars.hourly();
}
