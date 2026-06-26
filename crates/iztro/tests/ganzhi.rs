use iztro::core::model::ganzhi::{lunar_year_branch, lunar_year_stem, lunar_year_stem_branch};
use iztro::core::{EarthlyBranch, HeavenlyStem, StemBranch};

#[test]
fn stem_branch_rejects_mismatched_parity() {
    assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Chou).is_err());
    assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi).is_ok());
}

#[test]
fn from_lunar_year_anchors_1984_jiazi() {
    let jiazi = StemBranch::from_lunar_year(1984);
    assert_eq!(jiazi.stem(), HeavenlyStem::Jia);
    assert_eq!(jiazi.branch(), EarthlyBranch::Zi);
    // 2024 is 甲辰 (JiaChen).
    let y2024 = StemBranch::from_lunar_year(2024);
    assert_eq!(y2024.stem(), HeavenlyStem::Jia);
    assert_eq!(y2024.branch(), EarthlyBranch::Chen);
    // 2023 is 癸卯 (GuiMao).
    let y2023 = StemBranch::from_lunar_year(2023);
    assert_eq!(y2023.stem(), HeavenlyStem::Gui);
    assert_eq!(y2023.branch(), EarthlyBranch::Mao);
}

#[test]
fn cycle_index_round_trips() {
    for index in 0..60 {
        assert_eq!(StemBranch::from_cycle_index(index).cycle_index(), index);
    }
}

#[test]
fn lunar_year_helpers_agree_with_pillar_accessors() {
    for year in [1850, 1984, 2000, 2023, 2150] {
        let pillar = StemBranch::from_lunar_year(year);
        assert_eq!(lunar_year_stem_branch(year), pillar);
        assert_eq!(lunar_year_stem(year), pillar.stem());
        assert_eq!(lunar_year_branch(year), pillar.branch());
    }
}
