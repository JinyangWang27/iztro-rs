use iztro::core::{EarthlyBranch, HeavenlyStem, StemBranch};
use iztro::core::model::nayin::{NaYinElement, nayin_element};

fn pair(stem: HeavenlyStem, branch: EarthlyBranch) -> StemBranch {
    StemBranch::try_new(stem, branch).expect("valid sexagenary pair")
}

#[test]
fn nayin_element_jia_zi_is_metal() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Jia, EarthlyBranch::Zi)),
        NaYinElement::Metal
    );
}

#[test]
fn nayin_element_yi_chou_is_metal() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Yi, EarthlyBranch::Chou)),
        NaYinElement::Metal
    );
}

#[test]
fn nayin_element_bing_yin_is_fire() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Bing, EarthlyBranch::Yin)),
        NaYinElement::Fire
    );
}

#[test]
fn nayin_element_ding_mao_is_fire() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Ding, EarthlyBranch::Mao)),
        NaYinElement::Fire
    );
}

#[test]
fn nayin_element_wu_chen_is_wood() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Wu, EarthlyBranch::Chen)),
        NaYinElement::Wood
    );
}

#[test]
fn nayin_element_ji_si_is_wood() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Ji, EarthlyBranch::Si)),
        NaYinElement::Wood
    );
}

#[test]
fn nayin_element_geng_wu_is_earth() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Geng, EarthlyBranch::Wu)),
        NaYinElement::Earth
    );
}

#[test]
fn nayin_element_xin_wei_is_earth() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Xin, EarthlyBranch::Wei)),
        NaYinElement::Earth
    );
}

#[test]
fn nayin_element_ren_shen_is_metal() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Ren, EarthlyBranch::Shen)),
        NaYinElement::Metal
    );
}

#[test]
fn nayin_element_gui_you_is_metal() {
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Gui, EarthlyBranch::You)),
        NaYinElement::Metal
    );
}

#[test]
fn nayin_element_ji_chou_is_fire() {
    // Life Palace pair for the 1990 fixture; drives the Fire6 bureau.
    assert_eq!(
        nayin_element(pair(HeavenlyStem::Ji, EarthlyBranch::Chou)),
        NaYinElement::Fire
    );
}

#[test]
fn nayin_element_indexes_by_cycle_position() {
    // Every position in the sixty-pair cycle resolves a NaYin element, and
    // adjacent positions (2g, 2g+1) share one element.
    for n in 0..60usize {
        let element = nayin_element(StemBranch::from_cycle_index(n));
        let group_start = nayin_element(StemBranch::from_cycle_index(n - (n % 2)));
        assert_eq!(
            element, group_start,
            "cycle position {n} must share its NaYin with its group start"
        );
    }
}
