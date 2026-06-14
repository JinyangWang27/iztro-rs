use iztro::core::{
    Chart, ChartAlgorithmKind, DecadalDirection, DecadalFrame, EarthlyBranch, Gender, HeavenlyStem,
    LunarChartRequest, LunarDay, LunarMonth, MethodProfile, StemBranch, build_decadal_frame,
    by_lunar,
};

fn chart_with_birth_year(stem: HeavenlyStem, branch: EarthlyBranch, gender: Gender) -> Chart {
    let request = LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
        .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(gender)
        .birth_year_stem(stem)
        .birth_year_branch(branch)
        .method_profile(MethodProfile::new(
            "decadal_frame_test",
            ChartAlgorithmKind::QuanShu,
            "decadal frame test",
        ))
        .build()
        .expect("lunar request should build");

    by_lunar(request).expect("by_lunar should build fixture chart")
}

fn geng_wu_female_chart() -> Chart {
    chart_with_birth_year(HeavenlyStem::Geng, EarthlyBranch::Wu, Gender::Female)
}

fn geng_wu_male_chart() -> Chart {
    chart_with_birth_year(HeavenlyStem::Geng, EarthlyBranch::Wu, Gender::Male)
}

fn xin_you_female_chart() -> Chart {
    chart_with_birth_year(HeavenlyStem::Xin, EarthlyBranch::You, Gender::Female)
}

fn xin_you_male_chart() -> Chart {
    chart_with_birth_year(HeavenlyStem::Xin, EarthlyBranch::You, Gender::Male)
}

#[test]
fn decadal_frame_starts_from_life_palace_and_bureau_age() {
    let chart = geng_wu_female_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");
    let periods = frame.periods();
    let life_palace = chart
        .life_palace()
        .expect("fixture should have Life Palace");
    let start_age = chart
        .five_element_bureau()
        .expect("fixture should have five-element bureau")
        .number();

    assert_eq!(periods.len(), 12);
    assert_eq!(frame.direction(), DecadalDirection::Reverse);
    assert_eq!(periods[0].palace_branch(), life_palace.branch());
    assert_eq!(periods[0].palace_name(), life_palace.name());
    assert_eq!(periods[0].palace_stem(), life_palace.stem());
    assert_eq!(periods[0].start_age(), start_age);
    assert_eq!(periods[0].end_age(), start_age + 9);
}

#[test]
fn decadal_frame_advances_period_ages_by_ten_years() {
    let chart = geng_wu_female_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");
    let start_age = frame.periods()[0].start_age();

    for (index, period) in frame.periods().iter().enumerate() {
        let expected_start_age = start_age + index as u8 * 10;

        assert_eq!(period.start_age(), expected_start_age);
        assert_eq!(period.end_age(), expected_start_age + 9);
    }
}

#[test]
fn decadal_direction_follows_yang_male_and_yin_female_forward_matrix() {
    let cases = [
        (geng_wu_male_chart(), DecadalDirection::Forward),
        (xin_you_female_chart(), DecadalDirection::Forward),
        (geng_wu_female_chart(), DecadalDirection::Reverse),
        (xin_you_male_chart(), DecadalDirection::Reverse),
    ];

    for (chart, expected) in cases {
        let frame = build_decadal_frame(&chart).expect("decadal frame should build");

        assert_eq!(frame.direction(), expected);
        assert!(
            frame
                .periods()
                .iter()
                .all(|period| period.direction() == expected)
        );
    }
}

#[test]
fn forward_and_reverse_frames_follow_branch_order_from_life_palace() {
    let forward_chart = geng_wu_male_chart();
    let reverse_chart = geng_wu_female_chart();
    let forward = build_decadal_frame(&forward_chart).expect("forward frame should build");
    let reverse = build_decadal_frame(&reverse_chart).expect("reverse frame should build");
    let life_branch = forward_chart
        .life_palace()
        .expect("fixture should have Life Palace")
        .branch();

    let forward_branches: Vec<EarthlyBranch> = forward
        .periods()
        .iter()
        .map(|period| period.palace_branch())
        .collect();
    let reverse_branches: Vec<EarthlyBranch> = reverse
        .periods()
        .iter()
        .map(|period| period.palace_branch())
        .collect();

    assert_eq!(
        forward_branches,
        (0..12)
            .map(|offset| life_branch.offset(offset))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        reverse_branches,
        (0..12)
            .map(|offset| life_branch.offset(-offset))
            .collect::<Vec<_>>()
    );
}

#[test]
fn decadal_period_stem_branch_matches_matching_natal_palace() {
    let chart = geng_wu_male_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");

    for period in frame.periods() {
        let palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == period.palace_branch())
            .expect("period branch should refer to a natal palace");
        let expected = StemBranch::try_new(palace.stem(), palace.branch())
            .expect("palace pair should be valid");

        assert_eq!(period.palace_name(), palace.name());
        assert_eq!(period.palace_stem(), palace.stem());
        assert_eq!(period.stem_branch(), expected);
    }
}

#[test]
fn decadal_frame_round_trips_through_json() {
    let chart = geng_wu_female_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");
    let encoded = serde_json::to_string(&frame).expect("frame should serialize");
    let decoded: DecadalFrame = serde_json::from_str(&encoded).expect("frame should deserialize");

    assert_eq!(decoded, frame);
}
