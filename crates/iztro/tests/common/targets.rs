//! Target/scope parsing helpers for the temporal-layer (運限) horoscope tests.
//!
//! These read a case's target solar date / time / year and resolve a per-scope
//! `supported` block to typed expectations (palace names, mutagens, flow stars,
//! yearly decorative stars).

use std::collections::HashMap;

use iztro::core::{
    BirthTime, Chart, DecorativeStarFamily, EarthlyBranch, FlowStarScope, HeavenlyStem, Mutagen,
    PalaceName, StarKind, StarName, StemBranch, flow_star_name,
};
use serde_json::Value;

use super::normalize::{flow_star_kind, parse_flow_base, parse_key};

/// Returns the `(year, month, day)` of a case's target solar date.
pub fn target_solar_date(case: &Value) -> (i32, u8, u8) {
    let raw = case["input"]["target"]["solar_date"]
        .as_str()
        .expect("target solar date");
    let parts: Vec<_> = raw.split('-').collect();
    assert_eq!(parts.len(), 3);
    (
        parts[0].parse().expect("target solar year"),
        parts[1].parse().expect("target solar month"),
        parts[2].parse().expect("target solar day"),
    )
}

/// Returns a case's target `timeIndex`.
pub fn target_time_index(case: &Value) -> u8 {
    case["input"]["target"]["time_index"]
        .as_u64()
        .expect("target time index") as u8
}

/// Returns a case's target birth time.
pub fn target_time(case: &Value) -> BirthTime {
    BirthTime::from_iztro_time_index(target_time_index(case))
        .expect("target time index should be valid")
}

/// Returns a case's declared target lunar year.
pub fn target_year(case: &Value) -> i32 {
    case["input"]["target"]["year"]
        .as_i64()
        .expect("fixture target year") as i32
}

/// Returns the stem-branch declared on a per-scope supported block.
pub fn scope_stem_branch(scope: &Value) -> StemBranch {
    StemBranch::try_new(
        parse_key::<HeavenlyStem>(scope["heavenly_stem"].as_str().expect("scope stem")),
        parse_key::<EarthlyBranch>(scope["earthly_branch"].as_str().expect("scope branch")),
    )
    .expect("fixture scope stem-branch should be valid")
}

/// Maps a per-scope `palace_names` array (Yin-first) to branch-keyed names.
pub fn expected_palace_names_by_branch(scope: &Value) -> HashMap<EarthlyBranch, PalaceName> {
    scope["palace_names"]
        .as_array()
        .expect("scope palace names")
        .iter()
        .enumerate()
        .map(|(index, palace)| {
            (
                EarthlyBranch::Yin.offset(index as isize),
                parse_key::<PalaceName>(palace["name"].as_str().expect("palace name")),
            )
        })
        .collect()
}

/// Resolves a per-scope `mutagen` block to `(star, natal branch) -> transform`.
pub fn expected_scope_mutagens(
    scope: &Value,
    chart: &Chart,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    scope["mutagen"]
        .as_object()
        .expect("scope mutagen map")
        .iter()
        .filter_map(|(transform, entry)| {
            let star = parse_key::<StarName>(entry["star"].as_str().expect("mutagen star"));
            let branch = chart.star(star).map(|fact| fact.palace().branch())?;
            Some(((star, branch), parse_key::<Mutagen>(transform)))
        })
        .collect()
}

/// Resolves a per-scope `flow_stars` array to `star -> (branch, kind)`, adding
/// the yearly-only 年解 (`NianJieYearly`) when `nian_jie_branch` is present.
pub fn expected_scope_flow_stars(
    scope: &Value,
    flow_scope: FlowStarScope,
) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    let mut expected: HashMap<StarName, (EarthlyBranch, StarKind)> = scope["flow_stars"]
        .as_array()
        .expect("scope flow stars")
        .iter()
        .map(|entry| {
            let base = parse_flow_base(entry["base"].as_str().expect("flow star base"));
            (
                flow_star_name(flow_scope, base),
                (
                    parse_key::<EarthlyBranch>(entry["branch"].as_str().expect("branch")),
                    flow_star_kind(entry["type"].as_str().expect("type")),
                ),
            )
        })
        .collect();

    if let Some(branch) = scope["nian_jie_branch"].as_str() {
        expected.insert(
            StarName::NianJieYearly,
            (parse_key::<EarthlyBranch>(branch), StarKind::Helper),
        );
    }

    expected
}

/// Resolves the yearly `yearly_dec_stars` block to `(branch, family) -> name`.
///
/// Reads both `suiqian12` and `jiangqian12` families; each entry's normalized
/// `name`/`branch` keys are parsed to their typed values. The `(branch, family)`
/// key is unique even when both families occupy the same branch.
pub fn expected_yearly_dec_stars(
    yearly: &Value,
) -> HashMap<(EarthlyBranch, DecorativeStarFamily), StarName> {
    let block = &yearly["yearly_dec_stars"];
    let mut expected = HashMap::new();
    for (key, family) in [
        ("suiqian12", DecorativeStarFamily::Suiqian12),
        ("jiangqian12", DecorativeStarFamily::Jiangqian12),
    ] {
        for entry in block[key]
            .as_array()
            .unwrap_or_else(|| panic!("yearly_dec_stars.{key} array"))
        {
            let branch = parse_key::<EarthlyBranch>(
                entry["branch"].as_str().expect("yearly dec star branch"),
            );
            let name = parse_key::<StarName>(entry["name"].as_str().expect("yearly dec star name"));
            assert!(
                expected.insert((branch, family), name).is_none(),
                "duplicate yearly dec star at {branch:?}/{family:?}"
            );
        }
    }
    expected
}
