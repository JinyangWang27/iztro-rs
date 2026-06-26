use iztro::core::labels::zh_cn::{
    brightness_zh, decorative_star_family_zh, earthly_branch_zh, five_element_bureau_zh,
    heavenly_stem_zh, mutagen_zh, palace_name_zh, scope_zh, star_category_zh, star_kind_zh,
    star_name_zh, zodiac_animal_zh,
};
use iztro::core::{
    Brightness, DecorativeStarFamily, EarthlyBranch, FiveElementBureau, HeavenlyStem, Mutagen,
    PalaceName, Scope, StarCategory, StarKind, StarName,
};

#[test]
fn heavenly_stem_labels_cover_all_ten_stems() {
    let pairs = [
        (HeavenlyStem::Jia, "甲"),
        (HeavenlyStem::Yi, "乙"),
        (HeavenlyStem::Bing, "丙"),
        (HeavenlyStem::Ding, "丁"),
        (HeavenlyStem::Wu, "戊"),
        (HeavenlyStem::Ji, "己"),
        (HeavenlyStem::Geng, "庚"),
        (HeavenlyStem::Xin, "辛"),
        (HeavenlyStem::Ren, "壬"),
        (HeavenlyStem::Gui, "癸"),
    ];
    for (stem, expected) in pairs {
        assert_eq!(heavenly_stem_zh(stem), expected);
    }
}

#[test]
fn earthly_branch_labels_cover_all_twelve_branches() {
    let pairs = [
        (EarthlyBranch::Zi, "子"),
        (EarthlyBranch::Chou, "丑"),
        (EarthlyBranch::Yin, "寅"),
        (EarthlyBranch::Mao, "卯"),
        (EarthlyBranch::Chen, "辰"),
        (EarthlyBranch::Si, "巳"),
        (EarthlyBranch::Wu, "午"),
        (EarthlyBranch::Wei, "未"),
        (EarthlyBranch::Shen, "申"),
        (EarthlyBranch::You, "酉"),
        (EarthlyBranch::Xu, "戌"),
        (EarthlyBranch::Hai, "亥"),
    ];
    for (branch, expected) in pairs {
        assert_eq!(earthly_branch_zh(branch), expected);
    }
}

#[test]
fn palace_name_labels_cover_all_twelve_palaces() {
    let pairs = [
        (PalaceName::Life, "命宫"),
        (PalaceName::Siblings, "兄弟"),
        (PalaceName::Spouse, "夫妻"),
        (PalaceName::Children, "子女"),
        (PalaceName::Wealth, "财帛"),
        (PalaceName::Health, "疾厄"),
        (PalaceName::Migration, "迁移"),
        (PalaceName::Friends, "仆役"),
        (PalaceName::Career, "官禄"),
        (PalaceName::Property, "田宅"),
        (PalaceName::Spirit, "福德"),
        (PalaceName::Parents, "父母"),
    ];
    for (name, expected) in pairs {
        assert_eq!(palace_name_zh(name), expected);
    }
}

#[test]
fn mutagen_labels_cover_all_four_transformations() {
    assert_eq!(mutagen_zh(Mutagen::Lu), "禄");
    assert_eq!(mutagen_zh(Mutagen::Quan), "权");
    assert_eq!(mutagen_zh(Mutagen::Ke), "科");
    assert_eq!(mutagen_zh(Mutagen::Ji), "忌");
}

#[test]
fn brightness_labels_cover_all_states() {
    assert_eq!(brightness_zh(Brightness::Temple), "庙");
    assert_eq!(brightness_zh(Brightness::Prosperous), "旺");
    assert_eq!(brightness_zh(Brightness::Advantage), "得");
    assert_eq!(brightness_zh(Brightness::Favourable), "利");
    assert_eq!(brightness_zh(Brightness::Flat), "平");
    assert_eq!(brightness_zh(Brightness::Weak), "不");
    assert_eq!(brightness_zh(Brightness::Trapped), "陷");
    assert_eq!(brightness_zh(Brightness::Unknown), "");
}

#[test]
fn star_category_labels_cover_all_categories() {
    assert_eq!(star_category_zh(StarCategory::Major), "主星");
    assert_eq!(star_category_zh(StarCategory::Minor), "辅星");
    assert_eq!(star_category_zh(StarCategory::Adjective), "杂曜");
}

#[test]
fn scope_labels_cover_all_scopes() {
    assert_eq!(scope_zh(Scope::Natal), "本命");
    assert_eq!(scope_zh(Scope::Decadal), "大限");
    assert_eq!(scope_zh(Scope::Age), "小限");
    assert_eq!(scope_zh(Scope::Yearly), "流年");
    assert_eq!(scope_zh(Scope::Monthly), "流月");
    assert_eq!(scope_zh(Scope::Daily), "流日");
    assert_eq!(scope_zh(Scope::Hourly), "流时");
}

#[test]
fn star_kind_labels_cover_all_kinds() {
    assert_eq!(star_kind_zh(StarKind::Major), "主星");
    assert_eq!(star_kind_zh(StarKind::Soft), "吉星");
    assert_eq!(star_kind_zh(StarKind::Tough), "煞星");
    assert_eq!(star_kind_zh(StarKind::LuCun), "禄存");
    assert_eq!(star_kind_zh(StarKind::TianMa), "天马");
    assert_eq!(star_kind_zh(StarKind::Adjective), "杂曜");
    assert_eq!(star_kind_zh(StarKind::Flower), "桃花星");
    assert_eq!(star_kind_zh(StarKind::Helper), "助星");
}

#[test]
fn decorative_star_family_labels_cover_all_families() {
    assert_eq!(
        decorative_star_family_zh(DecorativeStarFamily::Changsheng12),
        "长生十二神"
    );
    assert_eq!(
        decorative_star_family_zh(DecorativeStarFamily::Boshi12),
        "博士十二神"
    );
    assert_eq!(
        decorative_star_family_zh(DecorativeStarFamily::Suiqian12),
        "岁前十二神"
    );
    assert_eq!(
        decorative_star_family_zh(DecorativeStarFamily::Jiangqian12),
        "将前十二神"
    );
}

#[test]
fn star_name_labels_cover_representative_stars() {
    // Major stars (主星).
    assert_eq!(star_name_zh(StarName::ZiWei), "紫微");
    assert_eq!(star_name_zh(StarName::PoJun), "破军");
    // Minor / soft / tough stars (辅星).
    assert_eq!(star_name_zh(StarName::ZuoFu), "左辅");
    assert_eq!(star_name_zh(StarName::QingYang), "擎羊");
    assert_eq!(star_name_zh(StarName::LuCun), "禄存");
    // Adjective stars (杂曜), including the disambiguated homophones.
    assert_eq!(star_name_zh(StarName::HuaGai), "华盖");
    assert_eq!(star_name_zh(StarName::TianYueAdj), "天月");
    assert_eq!(star_name_zh(StarName::TianFuAdj), "天福");
    // Decorative "twelve gods" entries.
    assert_eq!(star_name_zh(StarName::ChangSheng), "长生");
    assert_eq!(star_name_zh(StarName::BoShi), "博士");
    assert_eq!(star_name_zh(StarName::JiangXing), "将星");
    // Flow stars across scopes.
    assert_eq!(star_name_zh(StarName::YunKui), "运魁");
    assert_eq!(star_name_zh(StarName::ShiXi), "时喜");
}

#[test]
fn homophone_stars_keep_distinct_labels_from_their_namesakes() {
    // Disambiguated romanizations still carry their conventional characters.
    assert_eq!(star_name_zh(StarName::TianYue), "天钺");
    assert_eq!(star_name_zh(StarName::TianYueAdj), "天月");
    assert_eq!(star_name_zh(StarName::TianFu), "天府");
    assert_eq!(star_name_zh(StarName::TianFuAdj), "天福");
}

#[test]
fn five_element_bureau_labels_are_chinese() {
    assert_eq!(five_element_bureau_zh(FiveElementBureau::Water2), "水二局");
    assert_eq!(five_element_bureau_zh(FiveElementBureau::Wood3), "木三局");
    assert_eq!(five_element_bureau_zh(FiveElementBureau::Metal4), "金四局");
    assert_eq!(five_element_bureau_zh(FiveElementBureau::Earth5), "土五局");
    assert_eq!(five_element_bureau_zh(FiveElementBureau::Fire6), "火六局");
}

#[test]
fn zodiac_animal_labels_cover_all_twelve_branches() {
    let pairs = [
        (EarthlyBranch::Zi, "鼠"),
        (EarthlyBranch::Chou, "牛"),
        (EarthlyBranch::Yin, "虎"),
        (EarthlyBranch::Mao, "兔"),
        (EarthlyBranch::Chen, "龙"),
        (EarthlyBranch::Si, "蛇"),
        (EarthlyBranch::Wu, "马"),
        (EarthlyBranch::Wei, "羊"),
        (EarthlyBranch::Shen, "猴"),
        (EarthlyBranch::You, "鸡"),
        (EarthlyBranch::Xu, "狗"),
        (EarthlyBranch::Hai, "猪"),
    ];
    for (branch, expected) in pairs {
        assert_eq!(zodiac_animal_zh(branch), expected);
    }
}
