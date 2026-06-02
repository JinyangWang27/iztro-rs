use iztro_core::{
    ChartError, EarthlyBranch, FiveElementBureau, HeavenlyStem, NaYinElement, StemBranch,
    five_element_bureau_from_life_palace,
};

fn bureau(stem: HeavenlyStem, branch: EarthlyBranch) -> Result<FiveElementBureau, ChartError> {
    five_element_bureau_from_life_palace(StemBranch::new(stem, branch))
}

#[test]
fn bing_yin_life_palace_is_fire6() {
    assert_eq!(
        bureau(HeavenlyStem::Bing, EarthlyBranch::Yin),
        Ok(FiveElementBureau::Fire6)
    );
}

#[test]
fn wu_chen_life_palace_is_wood3() {
    assert_eq!(
        bureau(HeavenlyStem::Wu, EarthlyBranch::Chen),
        Ok(FiveElementBureau::Wood3)
    );
}

#[test]
fn jia_zi_life_palace_is_metal4() {
    assert_eq!(
        bureau(HeavenlyStem::Jia, EarthlyBranch::Zi),
        Ok(FiveElementBureau::Metal4)
    );
}

#[test]
fn bing_zi_life_palace_is_water2() {
    assert_eq!(
        bureau(HeavenlyStem::Bing, EarthlyBranch::Zi),
        Ok(FiveElementBureau::Water2)
    );
}

#[test]
fn geng_zi_life_palace_is_earth5() {
    assert_eq!(
        bureau(HeavenlyStem::Geng, EarthlyBranch::Zi),
        Ok(FiveElementBureau::Earth5)
    );
}

#[test]
fn ji_chou_life_palace_is_fire6() {
    // 1990 fixture Life Palace pair.
    assert_eq!(
        bureau(HeavenlyStem::Ji, EarthlyBranch::Chou),
        Ok(FiveElementBureau::Fire6)
    );
}

#[test]
fn invalid_life_palace_pair_is_rejected() {
    assert_eq!(
        bureau(HeavenlyStem::Jia, EarthlyBranch::Chou),
        Err(ChartError::InvalidStemBranchPair {
            stem: HeavenlyStem::Jia,
            branch: EarthlyBranch::Chou,
        })
    );
}

#[test]
fn bureau_numbers_match_classical_values() {
    assert_eq!(FiveElementBureau::Water2.number(), 2);
    assert_eq!(FiveElementBureau::Wood3.number(), 3);
    assert_eq!(FiveElementBureau::Metal4.number(), 4);
    assert_eq!(FiveElementBureau::Earth5.number(), 5);
    assert_eq!(FiveElementBureau::Fire6.number(), 6);
}

#[test]
fn bureau_elements_match_nayin_element() {
    assert_eq!(FiveElementBureau::Water2.element(), NaYinElement::Water);
    assert_eq!(FiveElementBureau::Wood3.element(), NaYinElement::Wood);
    assert_eq!(FiveElementBureau::Metal4.element(), NaYinElement::Metal);
    assert_eq!(FiveElementBureau::Earth5.element(), NaYinElement::Earth);
    assert_eq!(FiveElementBureau::Fire6.element(), NaYinElement::Fire);
}
