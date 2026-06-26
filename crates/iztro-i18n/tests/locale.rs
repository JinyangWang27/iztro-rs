use iztro_i18n::Locale;

#[test]
fn default_locale_is_english() {
    assert_eq!(Locale::default(), Locale::EnUs);
}

#[test]
fn parse_and_display_round_trip() {
    for locale in Locale::ALL {
        assert_eq!(locale.to_string().parse::<Locale>().unwrap(), locale);
        assert_eq!(Locale::parse_or_default(locale.as_bcp47()), locale);
    }
}

#[test]
fn unknown_tag_falls_back_to_default() {
    assert!("fr-FR".parse::<Locale>().is_err());
    assert_eq!(Locale::parse_or_default("fr-FR"), Locale::EnUs);
}
