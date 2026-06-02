use iztro_core::{
    ChartError, EarthlyBranch, LifeBodyPalaceIndices, LunarBirthContext, LunarMonth,
    calculate_life_body_palace_indices,
};

#[test]
fn month_one_zi_hour_places_life_and_body_at_yin() {
    let indices = calculate_life_body_palace_indices(LunarBirthContext::new(
        LunarMonth::new(1).expect("month 1 should be valid"),
        EarthlyBranch::Zi,
    ))
    .expect("valid lunar birth context should calculate palace indices");

    assert_eq!(
        indices,
        LifeBodyPalaceIndices::new(EarthlyBranch::Yin, EarthlyBranch::Yin)
    );
}

#[test]
fn month_one_chou_hour_places_life_at_chou_and_body_at_mao() {
    let indices = calculate_life_body_palace_indices(LunarBirthContext::new(
        LunarMonth::new(1).expect("month 1 should be valid"),
        EarthlyBranch::Chou,
    ))
    .expect("valid lunar birth context should calculate palace indices");

    assert_eq!(indices.life_palace_branch(), EarthlyBranch::Chou);
    assert_eq!(indices.body_palace_branch(), EarthlyBranch::Mao);
}

#[test]
fn month_one_yin_hour_places_life_at_zi_and_body_at_chen() {
    let indices = calculate_life_body_palace_indices(LunarBirthContext::new(
        LunarMonth::new(1).expect("month 1 should be valid"),
        EarthlyBranch::Yin,
    ))
    .expect("valid lunar birth context should calculate palace indices");

    assert_eq!(indices.life_palace_branch(), EarthlyBranch::Zi);
    assert_eq!(indices.body_palace_branch(), EarthlyBranch::Chen);
}

#[test]
fn calculation_wraps_across_month_and_hour_cycles() {
    let indices = calculate_life_body_palace_indices(LunarBirthContext::new(
        LunarMonth::new(12).expect("month 12 should be valid"),
        EarthlyBranch::Hai,
    ))
    .expect("valid lunar birth context should calculate palace indices");

    assert_eq!(indices.life_palace_branch(), EarthlyBranch::Yin);
    assert_eq!(indices.body_palace_branch(), EarthlyBranch::Zi);
}

#[test]
fn lunar_month_zero_is_rejected() {
    let error = LunarMonth::new(0).expect_err("month 0 should be invalid");

    assert_eq!(error, ChartError::InvalidLunarMonth { value: 0 });
}

#[test]
fn lunar_month_thirteen_is_rejected() {
    let error = LunarMonth::new(13).expect_err("month 13 should be invalid");

    assert_eq!(error, ChartError::InvalidLunarMonth { value: 13 });
}
