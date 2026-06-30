//! Fixture-backed golden test for the renderer-neutral static chart view model.
//!
//! Unlike the upstream-reference fixtures, this fixture is a self-generated
//! golden: it captures the serialized [`StaticChartViewSnapshot`] for the
//! canonical natal case (lunar 1990-05-17, Chen hour, female). It carries no
//! top-level `input` block, so the fixture-case registry drift check skips it.
//!
//! Regenerate with:
//! `cargo test -p iztro --test static_chart_view -- --ignored regenerate_fixture`

use iztro::StaticChartViewSnapshot;
use iztro::core::{
    Chart, ChartAlgorithmKind, Gender, LunarChartRequest, LunarDay, LunarMonth, MethodProfile,
    StemBranch, by_lunar,
};
use serde_json::Value;

const FIXTURE: &str =
    include_str!("../fixtures/iztro/static_chart_view_1990_05_17_chen_female.json");

const CASE_ID: &str = "1990_05_17_chen_female";

/// Builds the canonical natal chart for the fixture case.
fn canonical_chart() -> Chart {
    let birth_year = StemBranch::from_lunar_year(1990);
    let method_profile = MethodProfile::new(
        CASE_ID,
        ChartAlgorithmKind::QuanShu,
        "static chart view fixture test",
    );
    let request = LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(5).expect("valid lunar month"))
        .lunar_day(LunarDay::new(17).expect("valid lunar day"))
        .iztro_time_index(4)
        .expect("valid time index")
        .gender(Gender::Female)
        .birth_year_stem(birth_year.stem())
        .birth_year_branch(birth_year.branch())
        .is_leap_month(false)
        .fix_leap(true)
        .method_profile(method_profile)
        .build()
        .expect("lunar chart request should build");
    by_lunar(request).expect("by_lunar should build the canonical chart")
}

#[test]
fn static_chart_view_matches_committed_fixture() {
    let snapshot = StaticChartViewSnapshot::from_chart(&canonical_chart());

    let fixture: Value =
        serde_json::from_str(FIXTURE).expect("fixture should be valid JSON; regenerate if stale");
    let expected: StaticChartViewSnapshot = serde_json::from_value(fixture["snapshot"].clone())
        .expect("fixture `snapshot` should deserialize; regenerate if stale");

    assert_eq!(snapshot, expected);
    // The committed golden never carries highlights for now.
    assert!(snapshot.highlights.is_empty());
}

/// Regenerates the committed fixture. Ignored by default; run explicitly after
/// an intentional change to the static chart view model.
#[test]
#[ignore]
fn regenerate_fixture() {
    let snapshot = StaticChartViewSnapshot::from_chart(&canonical_chart());
    let document = serde_json::json!({
        "case_id": CASE_ID,
        "view_model": "StaticChartViewSnapshot",
        "note": "Self-generated golden: serialized StaticChartViewSnapshot::from_chart for the canonical natal case. No upstream iztro data.",
        "snapshot": snapshot,
    });
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/iztro/static_chart_view_1990_05_17_chen_female.json");
    let mut text = serde_json::to_string_pretty(&document).expect("serialize fixture");
    text.push('\n');
    std::fs::write(&path, text).expect("write fixture");
}
