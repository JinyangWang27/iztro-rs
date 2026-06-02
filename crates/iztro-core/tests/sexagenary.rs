use iztro_core::{
    ChartError, EarthlyBranch, HeavenlyStem, NaYinElement, StemBranch, is_valid_sexagenary_pair,
    nayin_element,
};

fn pair(stem: HeavenlyStem, branch: EarthlyBranch) -> StemBranch {
    StemBranch::new(stem, branch)
}

#[test]
fn stem_branch_exposes_its_components() {
    let sb = StemBranch::new(HeavenlyStem::Jia, EarthlyBranch::Zi);

    assert_eq!(sb.stem(), HeavenlyStem::Jia);
    assert_eq!(sb.branch(), EarthlyBranch::Zi);
}

#[test]
fn jia_zi_is_a_valid_sexagenary_pair() {
    assert!(is_valid_sexagenary_pair(pair(
        HeavenlyStem::Jia,
        EarthlyBranch::Zi
    )));
}

#[test]
fn yi_chou_is_a_valid_sexagenary_pair() {
    assert!(is_valid_sexagenary_pair(pair(
        HeavenlyStem::Yi,
        EarthlyBranch::Chou
    )));
}

#[test]
fn jia_chou_is_an_invalid_sexagenary_pair() {
    assert!(!is_valid_sexagenary_pair(pair(
        HeavenlyStem::Jia,
        EarthlyBranch::Chou
    )));
}

#[test]
fn try_new_valid_accepts_matching_parity() {
    let sb = StemBranch::try_new_valid(HeavenlyStem::Yi, EarthlyBranch::Chou)
        .expect("yi-chou is a valid sexagenary pair");

    assert_eq!(sb.stem(), HeavenlyStem::Yi);
    assert_eq!(sb.branch(), EarthlyBranch::Chou);
}

#[test]
fn try_new_valid_rejects_mismatched_parity() {
    let error = StemBranch::try_new_valid(HeavenlyStem::Jia, EarthlyBranch::Chou)
        .expect_err("jia-chou must be rejected");

    assert_eq!(
        error,
        ChartError::InvalidStemBranchPair {
            stem: HeavenlyStem::Jia,
            branch: EarthlyBranch::Chou,
        }
    );
}

#[test]
fn nayin_element_jia_zi_is_metal() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Jia, EarthlyBranch::Zi)),
        Ok(NaYinElement::Metal)
    );
}

#[test]
fn nayin_element_yi_chou_is_metal() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Yi, EarthlyBranch::Chou)),
        Ok(NaYinElement::Metal)
    );
}

#[test]
fn nayin_element_bing_yin_is_fire() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Bing, EarthlyBranch::Yin)),
        Ok(NaYinElement::Fire)
    );
}

#[test]
fn nayin_element_ding_mao_is_fire() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Ding, EarthlyBranch::Mao)),
        Ok(NaYinElement::Fire)
    );
}

#[test]
fn nayin_element_wu_chen_is_wood() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Wu, EarthlyBranch::Chen)),
        Ok(NaYinElement::Wood)
    );
}

#[test]
fn nayin_element_ji_si_is_wood() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Ji, EarthlyBranch::Si)),
        Ok(NaYinElement::Wood)
    );
}

#[test]
fn nayin_element_bing_zi_is_water() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Bing, EarthlyBranch::Zi)),
        Ok(NaYinElement::Water)
    );
}

#[test]
fn nayin_element_geng_zi_is_earth() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Geng, EarthlyBranch::Zi)),
        Ok(NaYinElement::Earth)
    );
}

#[test]
fn nayin_element_jia_yin_is_water() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Jia, EarthlyBranch::Yin)),
        Ok(NaYinElement::Water)
    );
}

#[test]
fn nayin_element_ren_xu_is_water() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Ren, EarthlyBranch::Xu)),
        Ok(NaYinElement::Water)
    );
}

#[test]
fn nayin_element_ji_chou_is_fire() {
    // Life Palace pair for the 1990 fixture; drives the Fire6 bureau.
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Ji, EarthlyBranch::Chou)),
        Ok(NaYinElement::Fire)
    );
}

#[test]
fn nayin_element_rejects_invalid_parity_pair() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Jia, EarthlyBranch::Chou)),
        Err(ChartError::InvalidStemBranchPair {
            stem: HeavenlyStem::Jia,
            branch: EarthlyBranch::Chou,
        })
    );
}

#[test]
fn every_valid_pair_has_a_nayin_element() {
    // The sixty-pair sexagenary cycle: stem advances mod 10, branch mod 12.
    for n in 0..60usize {
        let stem = HeavenlyStem::from_index(n % 10);
        let branch = EarthlyBranch::from_index(n % 12);
        assert!(
            nayin_element(pair(stem, branch)).is_ok(),
            "cycle position {n} ({stem:?}-{branch:?}) must resolve a NaYin element"
        );
    }
}
