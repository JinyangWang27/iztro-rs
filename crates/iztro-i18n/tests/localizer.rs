use iztro_i18n::{I18n, Locale};

#[test]
fn english_lookup() {
    let i18n = I18n::new(Locale::EnUs);
    assert_eq!(i18n.text("button-save"), "Save");
}

#[test]
fn simplified_chinese_lookup() {
    let i18n = I18n::new(Locale::ZhHans);
    assert_eq!(i18n.text("button-save"), "保存");
}

#[test]
fn missing_key_returns_visible_placeholder() {
    let i18n = I18n::new(Locale::EnUs);
    assert_eq!(i18n.text("no.such.key"), "!no.such.key!");
}
