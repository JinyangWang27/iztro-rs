//! Verifies scoped flow-star (流耀) placement against upstream iztro@2.5.8
//! `getHoroscopeStar` fixtures, plus the temporal-layer invariants.

mod common;

use std::collections::HashMap;

use common::{fixture_value, flow_star_kind, parse_flow_base, parse_key};
use iztro::core::{
    ChartError, EarthlyBranch, FlowStarScope, HeavenlyStem, Scope, StarKind, StarName, StemBranch,
    TemporalContext, build_flow_star_layer, flow_star_name, try_flow_star_parts,
};
use serde_json::Value;

const FLOW_FIXTURE: &str = include_str!("../fixtures/iztro/flow_stars.json");

#[test]
fn flow_layers_match_upstream_fixtures() {
    let fixture: Value = fixture_value(FLOW_FIXTURE);
    let cases = fixture["cases"].as_array().expect("cases array");
    assert_eq!(
        cases.len(),
        60,
        "all 10 stems x 12 branches subset x 5 scopes"
    );

    for case in cases {
        let scope = parse_flow_scope(case["scope"].as_str().expect("scope"));
        let stem = parse_key::<HeavenlyStem>(case["stem"].as_str().expect("stem"));
        let branch = parse_key::<EarthlyBranch>(case["branch"].as_str().expect("branch"));
        let stem_branch = StemBranch::try_new(stem, branch).expect("valid sexagenary pair");
        let context = context_for(scope, stem_branch);

        let layer = build_flow_star_layer(context).expect("flow layer should build");

        // The layer scope matches the context, and every placement carries it.
        assert_eq!(layer.scope(), expected_scope(scope));
        for placement in layer.placements() {
            assert_eq!(
                placement.scope(),
                layer.scope(),
                "placement scope must match layer scope"
            );
        }

        let actual: HashMap<StarName, (EarthlyBranch, StarKind)> = layer
            .placements()
            .iter()
            .map(|p| (p.placement().name(), (p.branch(), p.placement().kind())))
            .collect();

        let matrix = case["matrix"].as_array().expect("matrix array");
        assert_eq!(matrix.len(), 10, "every scope emits the ten matrix stars");

        let mut expected_count = matrix.len();
        for entry in matrix {
            let base = parse_flow_base(entry["base"].as_str().expect("base"));
            let branch = parse_key::<EarthlyBranch>(entry["branch"].as_str().expect("branch"));
            let kind = flow_star_kind(entry["type"].as_str().expect("type"));
            let name = flow_star_name(scope, base);

            let (actual_branch, actual_kind) = actual
                .get(&name)
                .copied()
                .unwrap_or_else(|| panic!("{name:?} should be placed"));
            assert_eq!(actual_branch, branch, "{name:?} branch");
            assert_eq!(actual_kind, kind, "{name:?} kind");

            // Matrix flow stars round-trip through the identity primitives.
            assert_eq!(try_flow_star_parts(name), Some((scope, base)));
        }

        // 年解 is yearly-only and is not part of the FlowStarBase matrix.
        if scope == FlowStarScope::Yearly {
            expected_count += 1;
            let nian_jie_branch = parse_key::<EarthlyBranch>(
                case["nian_jie_branch"]
                    .as_str()
                    .expect("yearly case should record 年解 branch"),
            );
            let (actual_branch, actual_kind) = actual
                .get(&StarName::NianJieYearly)
                .copied()
                .expect("年解 should be placed in the yearly layer");
            assert_eq!(actual_branch, nian_jie_branch);
            assert_eq!(actual_kind, StarKind::Helper);
            assert_eq!(try_flow_star_parts(StarName::NianJieYearly), None);
        } else {
            assert!(
                !actual.contains_key(&StarName::NianJieYearly),
                "年解 is yearly-only"
            );
        }

        assert_eq!(
            actual.len(),
            expected_count,
            "layer should hold exactly the expected placements"
        );
    }
}

#[test]
fn flow_stars_are_unavailable_for_age_scope() {
    let stem_branch = StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Chen)
        .expect("valid sexagenary pair");
    let context = TemporalContext::Age {
        stem_branch,
        nominal_age: 37,
    };

    let error = build_flow_star_layer(context).expect_err("age flow stars should be unavailable");

    assert_eq!(
        error,
        ChartError::FlowStarsUnavailableForScope { scope: Scope::Age }
    );
}

fn context_for(scope: FlowStarScope, stem_branch: StemBranch) -> TemporalContext {
    match scope {
        FlowStarScope::Decadal => TemporalContext::Decadal {
            stem_branch,
            start_age: 6,
        },
        FlowStarScope::Yearly => TemporalContext::Yearly {
            stem_branch,
            lunar_year: 2020,
        },
        FlowStarScope::Monthly => TemporalContext::Monthly {
            stem_branch,
            lunar_month: 1,
        },
        FlowStarScope::Daily => TemporalContext::Daily {
            stem_branch,
            lunar_day: 1,
        },
        FlowStarScope::Hourly => TemporalContext::Hourly { stem_branch },
    }
}

fn expected_scope(scope: FlowStarScope) -> Scope {
    match scope {
        FlowStarScope::Decadal => Scope::Decadal,
        FlowStarScope::Yearly => Scope::Yearly,
        FlowStarScope::Monthly => Scope::Monthly,
        FlowStarScope::Daily => Scope::Daily,
        FlowStarScope::Hourly => Scope::Hourly,
    }
}

fn parse_flow_scope(value: &str) -> FlowStarScope {
    match value {
        "decadal" => FlowStarScope::Decadal,
        "yearly" => FlowStarScope::Yearly,
        "monthly" => FlowStarScope::Monthly,
        "daily" => FlowStarScope::Daily,
        "hourly" => FlowStarScope::Hourly,
        other => panic!("unsupported flow scope: {other}"),
    }
}
