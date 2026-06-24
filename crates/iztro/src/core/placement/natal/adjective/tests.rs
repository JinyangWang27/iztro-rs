use super::formulas::{
    jie_lu_branch, kong_wang_branch, tian_chu_branch, tian_fu_adj_branch, tian_guan_branch,
    xian_chi_branch, xun_kong_branch,
};
use crate::core::model::ganzhi::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, HeavenlyStem};

// Tables transcribed from iztro 2.5.8 `getYearlyStarIndex` /
// `getHuagaiXianchiIndex` and cross-checked against `astro.byLunar` output.

/// 咸池, by birth year branch (子..亥). Same 三合 family as 华盖.
#[test]
fn xian_chi_matches_iztro_branch_table() {
    use EarthlyBranch::*;
    let expected = [You, Wu, Mao, Zi, You, Wu, Mao, Zi, You, Wu, Mao, Zi];
    for (index, branch) in EARTHLY_BRANCHES.into_iter().enumerate() {
        assert_eq!(
            xian_chi_branch(branch),
            expected[index],
            "咸池 for {branch:?}"
        );
    }
}

/// 天空 sits one branch forward from the birth year branch.
#[test]
fn tian_kong_is_one_branch_after_year_branch() {
    for branch in EARTHLY_BRANCHES {
        assert_eq!(
            branch.offset(1),
            EarthlyBranch::from_index(branch.index() + 1)
        );
    }
}

/// 天官, by birth year stem (甲..癸).
#[test]
fn tian_guan_matches_iztro_stem_table() {
    use EarthlyBranch::*;
    let expected = [Wei, Chen, Si, Yin, Mao, You, Hai, You, Xu, Wu];
    for (index, stem) in HEAVENLY_STEMS.into_iter().enumerate() {
        assert_eq!(tian_guan_branch(stem), expected[index], "天官 for {stem:?}");
    }
}

/// 天厨, by birth year stem (甲..癸).
#[test]
fn tian_chu_matches_iztro_stem_table() {
    use EarthlyBranch::*;
    let expected = [Si, Wu, Zi, Si, Wu, Shen, Yin, Wu, You, Hai];
    for (index, stem) in HEAVENLY_STEMS.into_iter().enumerate() {
        assert_eq!(tian_chu_branch(stem), expected[index], "天厨 for {stem:?}");
    }
}

/// 天福 adjective star, by birth year stem (甲..癸).
#[test]
fn tian_fu_adj_matches_iztro_stem_table() {
    use EarthlyBranch::*;
    let expected = [You, Shen, Zi, Hai, Mao, Yin, Wu, Si, Wu, Si];
    for (index, stem) in HEAVENLY_STEMS.into_iter().enumerate() {
        assert_eq!(
            tian_fu_adj_branch(stem),
            expected[index],
            "天福 for {stem:?}"
        );
    }
}

/// 截路 / 空亡, by birth year stem pair (stem index mod 5). 空亡 is one
/// branch forward from 截路.
#[test]
fn jie_lu_and_kong_wang_match_iztro_stem_table() {
    use EarthlyBranch::*;
    let jie_lu = [Shen, Wu, Chen, Yin, Zi, Shen, Wu, Chen, Yin, Zi];
    let kong_wang = [You, Wei, Si, Mao, Chou, You, Wei, Si, Mao, Chou];
    for (index, stem) in HEAVENLY_STEMS.into_iter().enumerate() {
        assert_eq!(jie_lu_branch(stem), jie_lu[index], "截路 for {stem:?}");
        assert_eq!(
            kong_wang_branch(stem),
            kong_wang[index],
            "空亡 for {stem:?}"
        );
        assert_eq!(
            kong_wang_branch(stem),
            jie_lu_branch(stem).offset(1),
            "空亡 should follow 截路 for {stem:?}"
        );
    }
}

/// 旬空 (旬中空亡) across the full sexagenary cycle: the void branch of the
/// year's 甲-旬 whose 阴阳 polarity matches the year branch.
#[test]
fn xun_kong_matches_iztro_over_full_sexagenary_cycle() {
    use EarthlyBranch::*;
    // i = sexagenary position; stem = i % 10, branch = i % 12.
    let expected = [
        Xu, Hai, Xu, Hai, Xu, Hai, Xu, Hai, Xu, Hai, // 甲子旬: void 戌亥
        Shen, You, Shen, You, Shen, You, Shen, You, Shen, You, // 甲戌旬: void 申酉
        Wu, Wei, Wu, Wei, Wu, Wei, Wu, Wei, Wu, Wei, // 甲申旬: void 午未
        Chen, Si, Chen, Si, Chen, Si, Chen, Si, Chen, Si, // 甲午旬: void 辰巳
        Yin, Mao, Yin, Mao, Yin, Mao, Yin, Mao, Yin, Mao, // 甲辰旬: void 寅卯
        Zi, Chou, Zi, Chou, Zi, Chou, Zi, Chou, Zi, Chou, // 甲寅旬: void 子丑
    ];
    for (i, &want) in expected.iter().enumerate() {
        let stem = HeavenlyStem::from_index(i % 10);
        let branch = EarthlyBranch::from_index(i % 12);
        assert_eq!(
            xun_kong_branch(stem, branch),
            want,
            "旬空 for sexagenary position {i} ({stem:?}{branch:?})"
        );
    }
}

/// The 旬空 result always shares the birth year branch's 阴阳 polarity.
#[test]
fn xun_kong_polarity_matches_year_branch() {
    for i in 0..60usize {
        let stem = HeavenlyStem::from_index(i % 10);
        let branch = EarthlyBranch::from_index(i % 12);
        let void = xun_kong_branch(stem, branch);
        assert_eq!(
            void.index() % 2,
            branch.index() % 2,
            "旬空 polarity mismatch for {stem:?}{branch:?}"
        );
    }
}
