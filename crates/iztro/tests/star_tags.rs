use iztro::core::{StarName, StarTag, StarTagStrength, has_star_tag, star_tag_strength};

#[test]
fn qing_yang_and_tian_xing_are_punishment_primary() {
    assert_eq!(
        star_tag_strength(StarName::QingYang, StarTag::Punishment),
        Some(StarTagStrength::Primary)
    );
    assert_eq!(
        star_tag_strength(StarName::TianXing, StarTag::Punishment),
        Some(StarTagStrength::Primary)
    );
    assert!(has_star_tag(StarName::QingYang, StarTag::Punishment));
    assert!(has_star_tag(StarName::TianXing, StarTag::Punishment));
}

#[test]
fn di_kong_and_di_jie_are_kong_jie_and_void_symbol_primary() {
    for star in [StarName::DiKong, StarName::DiJie] {
        assert_eq!(
            star_tag_strength(star, StarTag::KongJie),
            Some(StarTagStrength::Primary)
        );
        assert_eq!(
            star_tag_strength(star, StarTag::VoidSymbol),
            Some(StarTagStrength::Primary)
        );
    }
}

#[test]
fn tian_kong_is_void_symbol_primary_but_not_kong_jie() {
    assert_eq!(
        star_tag_strength(StarName::TianKong, StarTag::VoidSymbol),
        Some(StarTagStrength::Primary)
    );
    assert!(!has_star_tag(StarName::TianKong, StarTag::KongJie));
}

#[test]
fn jie_kong_and_xun_kong_are_void_symbol_secondary() {
    assert_eq!(
        star_tag_strength(StarName::JieKong, StarTag::VoidSymbol),
        Some(StarTagStrength::Secondary)
    );
    assert_eq!(
        star_tag_strength(StarName::XunKong, StarTag::VoidSymbol),
        Some(StarTagStrength::Secondary)
    );
}

#[test]
fn kong_wang_and_jie_lu_are_not_void_symbol() {
    assert!(!has_star_tag(StarName::KongWang, StarTag::VoidSymbol));
    assert!(!has_star_tag(StarName::JieLu, StarTag::VoidSymbol));
}

#[test]
fn tan_lang_has_no_tags() {
    assert!(!has_star_tag(StarName::TanLang, StarTag::Punishment));
    assert!(!has_star_tag(StarName::TanLang, StarTag::KongJie));
    assert!(!has_star_tag(StarName::TanLang, StarTag::VoidSymbol));
}
