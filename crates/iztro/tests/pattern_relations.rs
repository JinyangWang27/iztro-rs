use iztro::core::EarthlyBranch;
use iztro::core::EarthlyBranch::{Chen, Mao, Shen, Si, Wei, Yin, You, Zi};
use iztro::core::pattern::relation::{
    clamp_branches, is_in_san_fang_si_zheng, is_in_trine_group, opposite, san_fang_si_zheng,
    trine_branches,
};

#[test]
fn opposite_is_offset_six() {
    assert_eq!(opposite(Yin), Shen);
    assert_eq!(opposite(Zi), EarthlyBranch::Wu);
}

#[test]
fn trine_branches_are_self_plus_four_and_eight() {
    // 寅午戌 trine group, anchored on 寅.
    assert_eq!(
        trine_branches(Yin),
        [Yin, EarthlyBranch::Wu, EarthlyBranch::Xu]
    );
}

#[test]
fn san_fang_si_zheng_is_self_opposite_and_trine() {
    let set = san_fang_si_zheng(Yin);
    assert!(set.contains(&Yin));
    assert!(set.contains(&Shen)); // opposite
    assert!(set.contains(&EarthlyBranch::Wu)); // trine +4
    assert!(set.contains(&EarthlyBranch::Xu)); // trine +8
    assert_eq!(set.len(), 4);
}

#[test]
fn is_in_san_fang_si_zheng_membership() {
    assert!(is_in_san_fang_si_zheng(Yin, Yin));
    assert!(is_in_san_fang_si_zheng(Yin, Shen));
    assert!(is_in_san_fang_si_zheng(Yin, EarthlyBranch::Wu));
    assert!(!is_in_san_fang_si_zheng(Yin, Mao));
    // Sanity: a few unrelated branches are excluded.
    for b in [Chen, Si, Wei, You] {
        assert!(!is_in_san_fang_si_zheng(Yin, b));
    }
}

#[test]
fn is_in_trine_group_includes_anchor() {
    assert!(is_in_trine_group(Yin, Yin)); // self
    assert!(is_in_trine_group(Yin, EarthlyBranch::Wu)); // +4
    assert!(is_in_trine_group(Yin, EarthlyBranch::Xu)); // +8
    assert!(!is_in_trine_group(Yin, Shen)); // opposite is not a trine member
}

#[test]
fn clamp_branches_are_neighbours() {
    assert_eq!(clamp_branches(Mao), [Yin, Chen]);
}
