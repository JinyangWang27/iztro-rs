//! Regression tests pinning the deterministic facade/export ordering of palace
//! star arrays.
//!
//! Core engine placement is order-independent — the compatibility tests compare
//! star *sets* because Rust and upstream TS `iztro` do not necessarily emit
//! stars in the same `Vec` order. The facade/export layer, by contrast, must not
//! depend on that incidental order: [`NatalFacadePalaceSnapshot`] applies a
//! stable Rust-side canonical ordering. These tests make that policy explicit:
//!
//! * typed stars are ordered by `(kind, name, brightness, mutagen)`;
//! * decorative stars are ordered by `(family, name)`;
//! * repeated construction yields a byte-identical star array order.
//!
//! This is not a claim of upstream TS palace-star array-order parity, which
//! remains deferred.

mod common;

use common::{build_chart_from_horoscope_fixture_case, horoscope_facade_fixture_cases};
use iztro::core::{
    Brightness, DecorativeStarFamily, HoroscopeFacadeSnapshot, HoroscopeStackInput, Mutagen,
    NatalFacadeDecorativeStarSnapshot, NatalFacadeTypedStarSnapshot, SolarDay, SolarMonth,
    StarKind, StarName, build_full_horoscope_chart,
};
use serde_json::Value;

use common::{target_solar_date, target_time};

fn typed_key(
    star: &NatalFacadeTypedStarSnapshot,
) -> (StarKind, StarName, Brightness, Option<Mutagen>) {
    (star.kind(), star.name(), star.brightness(), star.mutagen())
}

fn decorative_key(star: &NatalFacadeDecorativeStarSnapshot) -> (DecorativeStarFamily, StarName) {
    (star.family(), star.name())
}

fn build_facade_snapshot(case: &Value) -> HoroscopeFacadeSnapshot {
    let chart = build_chart_from_horoscope_fixture_case(case);
    let (year, month, day) = target_solar_date(case);
    let input = HoroscopeStackInput::new(
        year,
        SolarMonth::new(month).expect("target solar month should be valid"),
        SolarDay::new(day).expect("target solar day should be valid"),
        target_time(case),
    );
    let horoscope =
        build_full_horoscope_chart(chart, input).expect("full horoscope stack should build");
    HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope).expect("facade snapshot should build")
}

#[test]
fn facade_typed_star_arrays_follow_canonical_order() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let snapshot = build_facade_snapshot(&case);

        for palace in snapshot.astrolabe().palaces() {
            let actual: Vec<_> = palace.typed_stars().iter().map(typed_key).collect();
            let mut expected = actual.clone();
            expected.sort();
            assert_eq!(
                actual,
                expected,
                "{case_id}: typed stars in {:?} must be ordered by (kind, name, brightness, mutagen)",
                palace.branch()
            );
        }
    }
}

#[test]
fn facade_decorative_star_arrays_follow_canonical_order() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let snapshot = build_facade_snapshot(&case);

        for palace in snapshot.astrolabe().palaces() {
            let actual: Vec<_> = palace
                .decorative_stars()
                .iter()
                .map(decorative_key)
                .collect();
            let mut expected = actual.clone();
            expected.sort();
            assert_eq!(
                actual,
                expected,
                "{case_id}: decorative stars in {:?} must be ordered by (family, name)",
                palace.branch()
            );
        }
    }
}

#[test]
fn facade_star_order_is_stable_across_repeated_construction() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");

        let first = build_facade_snapshot(&case);
        let second = build_facade_snapshot(&case);

        // Whole-snapshot equality already pins the serialized order, but assert
        // the per-palace star arrays explicitly so a regression points straight
        // at the ordering helper rather than some unrelated facade field.
        for (left, right) in first
            .astrolabe()
            .palaces()
            .iter()
            .zip(second.astrolabe().palaces())
        {
            let left_typed: Vec<_> = left.typed_stars().iter().map(typed_key).collect();
            let right_typed: Vec<_> = right.typed_stars().iter().map(typed_key).collect();
            assert_eq!(
                left_typed,
                right_typed,
                "{case_id}: repeated construction must yield identical typed star order in {:?}",
                left.branch()
            );

            let left_decorative: Vec<_> =
                left.decorative_stars().iter().map(decorative_key).collect();
            let right_decorative: Vec<_> = right
                .decorative_stars()
                .iter()
                .map(decorative_key)
                .collect();
            assert_eq!(
                left_decorative,
                right_decorative,
                "{case_id}: repeated construction must yield identical decorative star order in {:?}",
                left.branch()
            );
        }

        let first_json = serde_json::to_value(&first).expect("facade should serialize");
        let second_json = serde_json::to_value(&second).expect("facade should serialize");
        assert_eq!(
            first_json, second_json,
            "{case_id}: repeated facade construction must serialize identically"
        );
    }
}
