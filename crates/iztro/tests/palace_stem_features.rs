//! Tests for palace-stem role and mutagen-flow feature derivation.
//!
//! The primary chart is the iztro fixture (1990-05-17 chen female, lunar 四月廿三,
//! 庚午 year) built with the full supported-star inventory so every stem's four
//! 十干四化 targets are placed and resolvable.

use std::collections::HashMap;

use iztro::core::{
    BirthContext, CalendarDate, Chart, EARTHLY_BRANCHES, EarthlyBranch, Gender, HeavenlyStem,
    LunarDay, LunarMonth, MethodProfile, Mutagen, NatalChartWithSupportedStarsInput, PALACE_NAMES,
    Palace, PalaceName, StemBranch, build_natal_chart_with_supported_stars,
    palace_stems_from_year_stem, stem_mutagen_targets,
};
use iztro::features::{
    PalaceStemMutagenFlow, PalaceStemRole, birth_year_stem_origin_palaces,
    mutagen_flows_from_palace, mutagen_flows_landing_in_palace, palace_stem_mutagen_flows,
    palace_stem_role_assignments, self_transforming_flows,
};

/// Builds the 1990 fixture chart with all supported (major + minor) stars placed.
fn fixture_chart() -> Chart {
    build_natal_chart_with_supported_stars(NatalChartWithSupportedStarsInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("palace_stem_fixture"),
        LunarMonth::new(4).expect("month 4 should be valid"),
        LunarDay::new(23).expect("day 23 should be valid"),
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    ))
    .expect("supported natal chart should build for fixture input")
}

/// Builds a bare chart (no stars) whose palace stems follow `year_stem`.
///
/// Palace names are paired with branches in canonical order; only the palace
/// stems matter for role-assignment tests, and those come from the classical
/// 起五行寅例 derivation.
fn bare_chart_for_year(year_stem: HeavenlyStem, birth_year: StemBranch) -> Chart {
    let stems = palace_stems_from_year_stem(year_stem);
    let palaces: Vec<Palace> = PALACE_NAMES
        .iter()
        .copied()
        .zip(EARTHLY_BRANCHES.iter().copied())
        .zip(stems.iter().copied())
        .map(|((name, branch), stem)| Palace::new(name, branch, stem, Vec::new()))
        .collect();

    Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        birth_year,
        MethodProfile::placeholder("bare_palace_stem_fixture"),
        palaces,
        None,
        None,
    )
    .expect("bare chart should build")
}

#[test]
fn birth_year_stem_origin_is_unique_and_matches_year_stem() {
    let chart = fixture_chart();
    let origins = birth_year_stem_origin_palaces(&chart).expect("origins should derive");

    // 庚 is one of the stems that yields a single 来因宫.
    assert_eq!(origins.len(), 1);

    let origin = origins[0];
    assert_eq!(origin.role, PalaceStemRole::BirthYearStemOrigin);
    assert_eq!(origin.reference_stem, chart.birth_year().stem());
    assert_eq!(origin.palace_stem, chart.birth_year().stem());
    assert_eq!(origin.palace_stem, HeavenlyStem::Geng);

    // The named palace really carries that stem.
    let palace = chart
        .palace_by_name(origin.palace_name)
        .expect("origin palace should exist");
    assert_eq!(palace.stem(), HeavenlyStem::Geng);
    assert_eq!(palace.branch(), origin.branch);
}

#[test]
fn role_assignments_equal_birth_year_stem_origin_palaces() {
    let chart = fixture_chart();

    // The only role modeled today is BirthYearStemOrigin, so the two surfaces
    // return the same assignments.
    assert_eq!(
        palace_stem_role_assignments(&chart).expect("assignments"),
        birth_year_stem_origin_palaces(&chart).expect("origins"),
    );
}

#[test]
fn birth_year_stem_origin_count_matches_palace_stem_cycle_for_all_stems() {
    // The palace-stem cycle repeats two stems per chart. 辛 and 壬 birth years
    // make the birth-year stem one of those repeated stems, so they yield two
    // 来因宫; every other stem yields one. This is why the query is plural.
    let cases = [
        (HeavenlyStem::Jia, EarthlyBranch::Zi, 1),
        (HeavenlyStem::Yi, EarthlyBranch::Chou, 1),
        (HeavenlyStem::Bing, EarthlyBranch::Yin, 1),
        (HeavenlyStem::Ding, EarthlyBranch::Mao, 1),
        (HeavenlyStem::Wu, EarthlyBranch::Chen, 1),
        (HeavenlyStem::Ji, EarthlyBranch::Si, 1),
        (HeavenlyStem::Geng, EarthlyBranch::Wu, 1),
        (HeavenlyStem::Xin, EarthlyBranch::Wei, 2),
        (HeavenlyStem::Ren, EarthlyBranch::Shen, 2),
        (HeavenlyStem::Gui, EarthlyBranch::You, 1),
    ];

    for (stem, branch, expected_count) in cases {
        let birth_year = StemBranch::try_new(stem, branch)
            .expect("test case should use a valid stem-branch pair");
        let chart = bare_chart_for_year(stem, birth_year);
        let origins = birth_year_stem_origin_palaces(&chart).expect("origins should derive");

        assert_eq!(
            origins.len(),
            expected_count,
            "{stem:?} should produce {expected_count} birth-year-stem origin palace(s)",
        );
        assert!(
            origins
                .iter()
                .all(|origin| origin.role == PalaceStemRole::BirthYearStemOrigin),
            "{stem:?} origins should all carry BirthYearStemOrigin role",
        );
        assert!(
            origins.iter().all(|origin| origin.palace_stem == stem),
            "{stem:?} origins should all have matching palace stem",
        );
        assert!(
            origins.iter().all(|origin| origin.reference_stem == stem),
            "{stem:?} origins should all reference the birth-year stem",
        );
    }
}

#[test]
fn every_palace_produces_four_flows_in_lu_quan_ke_ji_order() {
    let chart = fixture_chart();
    let flows = palace_stem_mutagen_flows(&chart).expect("flows should derive");

    // Twelve palaces, four flows each.
    assert_eq!(flows.len(), PALACE_NAMES.len() * 4);

    let mut by_source: HashMap<PalaceName, Vec<Mutagen>> = HashMap::new();
    for flow in &flows {
        by_source
            .entry(flow.source.palace_name)
            .or_default()
            .push(flow.target.mutagen);
    }

    assert_eq!(by_source.len(), PALACE_NAMES.len());
    for mutagens in by_source.values() {
        assert_eq!(
            mutagens,
            &[Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji],
        );
    }
}

#[test]
fn flows_are_deterministic_and_ordered_by_palace_then_mutagen() {
    let chart = fixture_chart();
    let first = palace_stem_mutagen_flows(&chart).expect("flows");
    let second = palace_stem_mutagen_flows(&chart).expect("flows");
    assert_eq!(first, second, "derivation must be deterministic");

    // Sources appear in chart-palace order, each block in 禄/权/科/忌 order.
    let source_order: Vec<PalaceName> = first
        .chunks(4)
        .map(|chunk| chunk[0].source.palace_name)
        .collect();
    let expected: Vec<PalaceName> = chart.palaces().iter().map(Palace::name).collect();
    assert_eq!(source_order, expected);

    for chunk in first.chunks(4) {
        let mutagens: Vec<Mutagen> = chunk.iter().map(|flow| flow.target.mutagen).collect();
        assert_eq!(
            mutagens,
            vec![Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji],
        );
    }
}

#[test]
fn every_flow_target_resolves_to_a_real_placement() {
    let chart = fixture_chart();
    let flows = palace_stem_mutagen_flows(&chart).expect("flows");

    for flow in &flows {
        // The source stem really drives this (mutagen, star) pairing.
        let expected_targets = stem_mutagen_targets(flow.source.stem);
        assert!(
            expected_targets.contains(&(flow.target.mutagen, flow.target.star)),
            "flow {flow:?} must match the 十干四化 table",
        );

        // The target resolves to an actual natal placement, and the recorded
        // branch/palace match where that star actually sits.
        let placement = chart
            .star(flow.target.star)
            .expect("target star should be placed natally");
        assert_eq!(placement.palace().branch(), flow.target.branch);
        assert_eq!(placement.palace().name(), flow.target.palace_name);
    }
}

#[test]
fn mutagen_flows_from_palace_filters_by_source() {
    let chart = fixture_chart();
    let all = palace_stem_mutagen_flows(&chart).expect("flows");

    let from_life = mutagen_flows_from_palace(&chart, PalaceName::Life).expect("filtered flows");
    let expected: Vec<PalaceStemMutagenFlow> = all
        .iter()
        .copied()
        .filter(|flow| flow.source.palace_name == PalaceName::Life)
        .collect();

    assert_eq!(from_life, expected);
    assert_eq!(from_life.len(), 4);
    for flow in &from_life {
        assert_eq!(flow.source.palace_name, PalaceName::Life);
    }
}

#[test]
fn mutagen_flows_landing_in_palace_filters_by_target() {
    let chart = fixture_chart();
    let all = palace_stem_mutagen_flows(&chart).expect("flows");

    let landing =
        mutagen_flows_landing_in_palace(&chart, PalaceName::Life).expect("filtered flows");
    let expected: Vec<PalaceStemMutagenFlow> = all
        .iter()
        .copied()
        .filter(|flow| flow.target.palace_name == PalaceName::Life)
        .collect();

    assert_eq!(landing, expected);
    for flow in &landing {
        assert_eq!(flow.target.palace_name, PalaceName::Life);
    }
}

#[test]
fn self_transforming_flows_equal_predicate_filtering() {
    let chart = fixture_chart();
    let all = palace_stem_mutagen_flows(&chart).expect("flows");

    let self_flows = self_transforming_flows(&chart).expect("self-transform flows");
    let expected: Vec<PalaceStemMutagenFlow> = all
        .iter()
        .copied()
        .filter(PalaceStemMutagenFlow::is_self_transform)
        .collect();

    assert_eq!(self_flows, expected);
    for flow in &self_flows {
        assert!(flow.is_self_transform());
        assert_eq!(flow.source.branch, flow.target.branch);
    }
}

#[test]
fn flows_round_trip_through_json() {
    let chart = fixture_chart();
    let flows = palace_stem_mutagen_flows(&chart).expect("flows");

    let serialized = serde_json::to_string(&flows).expect("flows should serialize");
    let round_tripped: Vec<PalaceStemMutagenFlow> =
        serde_json::from_str(&serialized).expect("flows should deserialize");
    assert_eq!(round_tripped, flows);
}
