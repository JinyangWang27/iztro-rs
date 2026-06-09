use std::collections::HashSet;

use iztro_core::{
    KnownStarFamily, StarKind, StarName, known_star_metadata, known_star_metadata_table,
    represented_star_metadata_table, try_known_star_metadata, try_star_metadata,
};

#[test]
fn represented_metadata_table_stays_strict() {
    assert_eq!(represented_star_metadata_table().len(), 66);
}

#[test]
fn known_metadata_table_covers_iztro_2_5_8_runtime_inventory() {
    // Pinned to upstream iztro@2.5.8 runtime chart, decorative, and horoscope star names.
    assert_eq!(known_star_metadata_table().len(), 170);
}

#[test]
fn every_represented_star_is_known() {
    let known_names: HashSet<StarName> = known_star_metadata_table()
        .iter()
        .map(|metadata| metadata.name())
        .collect();

    for represented in represented_star_metadata_table() {
        let known = known_star_metadata(represented.name());

        assert!(known_names.contains(&represented.name()));
        assert_eq!(known.key(), represented.key());
        assert_eq!(known.chinese_name(), represented.chinese_name());
        assert_eq!(known.kind(), Some(represented.kind()));
    }
}

#[test]
fn non_represented_runtime_stars_are_known_but_not_represented() {
    let long_de = known_star_metadata(StarName::LongDeAdj);
    assert_eq!(long_de.key(), "long_de_adj");
    assert_eq!(long_de.upstream_key(), "longde");
    assert_eq!(long_de.chinese_name(), "龙德");
    assert_eq!(long_de.family(), KnownStarFamily::ZhongzhouAdjective);
    assert_eq!(long_de.kind(), Some(StarKind::Adjective));
    assert!(try_star_metadata(StarName::LongDeAdj).is_none());

    let da_hao = known_star_metadata(StarName::DaHaoSuiqian);
    assert_eq!(da_hao.key(), "da_hao_suiqian");
    assert_eq!(da_hao.upstream_key(), "dahao");
    assert_eq!(da_hao.chinese_name(), "大耗");
    assert_eq!(da_hao.family(), KnownStarFamily::Suiqian12);
    assert_eq!(da_hao.kind(), None);
    assert!(try_star_metadata(StarName::DaHaoSuiqian).is_none());

    let yun_kui = known_star_metadata(StarName::YunKui);
    assert_eq!(yun_kui.key(), "yun_kui");
    assert_eq!(yun_kui.upstream_key(), "yunkui");
    assert_eq!(yun_kui.chinese_name(), "运魁");
    assert_eq!(yun_kui.family(), KnownStarFamily::DecadalFlow);
    assert_eq!(yun_kui.kind(), Some(StarKind::Soft));
    assert!(try_star_metadata(StarName::YunKui).is_none());

    let yearly_nian_jie = known_star_metadata(StarName::NianJieYearly);
    assert_eq!(yearly_nian_jie.key(), "nian_jie_yearly");
    assert_eq!(yearly_nian_jie.upstream_key(), "nianjie");
    assert_eq!(yearly_nian_jie.chinese_name(), "年解");
    assert_eq!(yearly_nian_jie.family(), KnownStarFamily::YearlyFlow);
    assert_eq!(yearly_nian_jie.kind(), Some(StarKind::Helper));
    assert!(try_star_metadata(StarName::NianJieYearly).is_none());
}

#[test]
fn known_metadata_keys_are_unique() {
    let mut keys = HashSet::new();

    for metadata in known_star_metadata_table() {
        assert!(
            keys.insert(metadata.key()),
            "duplicate key {}",
            metadata.key()
        );
    }
}

#[test]
fn duplicate_chinese_labels_are_intentional() {
    let count_label = |label: &str| {
        known_star_metadata_table()
            .iter()
            .filter(|metadata| metadata.chinese_name() == label)
            .count()
    };

    assert_eq!(count_label("大耗"), 3);
    assert_eq!(count_label("龙德"), 2);
    assert_eq!(count_label("年解"), 2);
    assert_eq!(count_label("华盖"), 2);
    assert_eq!(count_label("咸池"), 2);
    assert_eq!(count_label("天德"), 2);
    assert_eq!(count_label("小耗"), 2);
    assert_eq!(count_label("病符"), 2);
}

#[test]
fn xunzhong_is_absent_because_it_is_locale_only() {
    assert!(
        known_star_metadata_table()
            .iter()
            .all(|metadata| metadata.key() != "xun_zhong"
                && metadata.upstream_key() != "xunzhong"
                && metadata.chinese_name() != "旬中")
    );

    assert!(serde_json::from_str::<StarName>("\"xun_zhong\"").is_err());
}

#[test]
fn mutagens_are_not_star_name_variants() {
    for value in ["hua_lu", "hua_quan", "hua_ke", "hua_ji"] {
        assert!(serde_json::from_str::<StarName>(&format!("\"{value}\"")).is_err());
    }
}

#[test]
fn represented_star_name_serde_names_remain_stable() {
    assert_eq!(
        serde_json::to_string(&StarName::ZiWei).expect("serialize star"),
        "\"zi_wei\""
    );
    assert_eq!(
        serde_json::to_string(&StarName::TianFuAdj).expect("serialize star"),
        "\"tian_fu_adj\""
    );
    assert_eq!(
        serde_json::to_string(&StarName::TianYueAdj).expect("serialize star"),
        "\"tian_yue_adj\""
    );
    assert_eq!(
        serde_json::to_string(&StarName::XunKong).expect("serialize star"),
        "\"xun_kong\""
    );
}

#[test]
fn collision_prone_new_star_name_serde_keys_are_stable() {
    let cases = [
        (StarName::DaHaoAdj, "da_hao_adj"),
        (StarName::DaHaoBoshi, "da_hao_boshi"),
        (StarName::DaHaoSuiqian, "da_hao_suiqian"),
        (StarName::FayLianBoshi, "fay_lian_boshi"),
        (StarName::GuanFuBoshi, "guan_fu_boshi"),
        (StarName::GuanFuSuiqian, "guan_fu_suiqian"),
        (StarName::HuaGaiJiangqian, "hua_gai_jiangqian"),
        (StarName::NianJieYearly, "nian_jie_yearly"),
        (StarName::XianChiJiangqian, "xian_chi_jiangqian"),
    ];

    for (star, expected) in cases {
        assert_eq!(
            serde_json::to_string(&star).expect("serialize star"),
            format!("\"{expected}\"")
        );
        assert_eq!(
            serde_json::from_str::<StarName>(&format!("\"{expected}\"")).expect("deserialize star"),
            star
        );
    }
}

#[test]
fn try_known_star_metadata_resolves_known_inventory_names() {
    assert!(try_known_star_metadata(StarName::LongDeAdj).is_some());
}
