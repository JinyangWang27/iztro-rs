use iztro_core::{
    EARTHLY_BRANCHES, EarthlyBranch, HeavenlyStem, palace_stem_for_branch,
    palace_stems_from_year_stem,
};

#[test]
fn jia_year_places_bing_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Jia, EarthlyBranch::Yin),
        HeavenlyStem::Bing
    );
}

#[test]
fn ji_year_places_bing_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Ji, EarthlyBranch::Yin),
        HeavenlyStem::Bing
    );
}

#[test]
fn yi_year_places_wu_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Yi, EarthlyBranch::Yin),
        HeavenlyStem::Wu
    );
}

#[test]
fn geng_year_places_wu_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Geng, EarthlyBranch::Yin),
        HeavenlyStem::Wu
    );
}

#[test]
fn bing_year_places_geng_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Bing, EarthlyBranch::Yin),
        HeavenlyStem::Geng
    );
}

#[test]
fn xin_year_places_geng_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Xin, EarthlyBranch::Yin),
        HeavenlyStem::Geng
    );
}

#[test]
fn ding_year_places_ren_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Ding, EarthlyBranch::Yin),
        HeavenlyStem::Ren
    );
}

#[test]
fn ren_year_places_ren_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Ren, EarthlyBranch::Yin),
        HeavenlyStem::Ren
    );
}

#[test]
fn wu_year_places_jia_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Wu, EarthlyBranch::Yin),
        HeavenlyStem::Jia
    );
}

#[test]
fn gui_year_places_jia_at_yin() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Gui, EarthlyBranch::Yin),
        HeavenlyStem::Jia
    );
}

#[test]
fn jia_year_forward_sequence_from_yin_is_bing_ding_wu() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Jia, EarthlyBranch::Yin),
        HeavenlyStem::Bing
    );
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Jia, EarthlyBranch::Mao),
        HeavenlyStem::Ding
    );
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Jia, EarthlyBranch::Chen),
        HeavenlyStem::Wu
    );
}

#[test]
fn jia_year_wraps_from_hai_yi_to_zi_bing() {
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Jia, EarthlyBranch::Hai),
        HeavenlyStem::Yi
    );
    assert_eq!(
        palace_stem_for_branch(HeavenlyStem::Jia, EarthlyBranch::Zi),
        HeavenlyStem::Bing
    );
}

#[test]
fn array_is_in_zi_to_hai_order_and_has_twelve_entries() {
    let stems = palace_stems_from_year_stem(HeavenlyStem::Jia);

    assert_eq!(stems.len(), 12);
    for (index, branch) in EARTHLY_BRANCHES.iter().enumerate() {
        assert_eq!(
            stems[index],
            palace_stem_for_branch(HeavenlyStem::Jia, *branch),
            "array entry must match per-branch helper for {branch:?}"
        );
    }
}

#[test]
fn jia_year_full_zi_to_hai_array_matches_classical_sequence() {
    use HeavenlyStem::*;

    // Zi, Chou, Yin, Mao, Chen, Si, Wu, Wei, Shen, You, Xu, Hai
    let expected = [Bing, Ding, Bing, Ding, Wu, Ji, Geng, Xin, Ren, Gui, Jia, Yi];
    assert_eq!(palace_stems_from_year_stem(HeavenlyStem::Jia), expected);
}
