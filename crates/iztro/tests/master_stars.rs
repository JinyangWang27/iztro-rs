use iztro::core::{EarthlyBranch, StarName, body_master, soul_master};

#[test]
fn soul_master_matches_life_palace_branch_table() {
    let pairs = [
        (EarthlyBranch::Zi, StarName::TanLang),
        (EarthlyBranch::Chou, StarName::JuMen),
        (EarthlyBranch::Hai, StarName::JuMen),
        (EarthlyBranch::Yin, StarName::LuCun),
        (EarthlyBranch::Xu, StarName::LuCun),
        (EarthlyBranch::Mao, StarName::WenQu),
        (EarthlyBranch::You, StarName::WenQu),
        (EarthlyBranch::Chen, StarName::LianZhen),
        (EarthlyBranch::Shen, StarName::LianZhen),
        (EarthlyBranch::Si, StarName::WuQu),
        (EarthlyBranch::Wei, StarName::WuQu),
        (EarthlyBranch::Wu, StarName::PoJun),
    ];
    for (branch, expected) in pairs {
        assert_eq!(soul_master(branch), expected);
    }
}

#[test]
fn body_master_matches_birth_year_branch_table() {
    let pairs = [
        (EarthlyBranch::Zi, StarName::HuoXing),
        (EarthlyBranch::Wu, StarName::HuoXing),
        (EarthlyBranch::Chou, StarName::TianXiang),
        (EarthlyBranch::Wei, StarName::TianXiang),
        (EarthlyBranch::Yin, StarName::TianLiang),
        (EarthlyBranch::Shen, StarName::TianLiang),
        (EarthlyBranch::Mao, StarName::TianTong),
        (EarthlyBranch::You, StarName::TianTong),
        (EarthlyBranch::Chen, StarName::WenChang),
        (EarthlyBranch::Xu, StarName::WenChang),
        (EarthlyBranch::Si, StarName::TianJi),
        (EarthlyBranch::Hai, StarName::TianJi),
    ];
    for (branch, expected) in pairs {
        assert_eq!(body_master(branch), expected);
    }
}
