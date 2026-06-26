use iztro::core::labels::chinese_date::{
    birth_time_label, chinese_year_digits, constellation_zh, lunar_date_label,
    solar_date_label_padded, solar_date_label_unpadded,
};

#[test]
fn year_renders_digit_by_digit() {
    assert_eq!(chinese_year_digits(1993), "一九九三");
    assert_eq!(chinese_year_digits(2008), "二〇〇八");
    assert_eq!(chinese_year_digits(2020), "二〇二〇");
}

#[test]
fn padded_solar_date_label_keeps_birth_date_style() {
    assert_eq!(solar_date_label_padded(1993, 5, 27), "1993-05-27");
    assert_eq!(solar_date_label_padded(2008, 2, 10), "2008-02-10");
}

#[test]
fn unpadded_solar_date_label_matches_temporal_style() {
    assert_eq!(solar_date_label_unpadded(2008, 2, 10), "2008-2-10");
    assert_eq!(solar_date_label_unpadded(2026, 11, 28), "2026-11-28");
}

#[test]
fn lunar_date_label_matches_almanac_form() {
    assert_eq!(lunar_date_label(1993, 4, 7, false), "一九九三年四月初七");
    assert_eq!(lunar_date_label(2008, 1, 4, false), "二〇〇八年正月初四");
    assert_eq!(lunar_date_label(2020, 4, 15, true), "二〇二〇年闰四月十五");
}

#[test]
fn birth_time_label_includes_hour_range() {
    assert_eq!(birth_time_label(9), "酉时(17:00~19:00)");
    assert_eq!(birth_time_label(0), "子时(00:00~01:00)");
    assert_eq!(birth_time_label(12), "子时(23:00~24:00)");
}

#[test]
fn constellation_matches_western_zodiac() {
    assert_eq!(constellation_zh(5, 27), "双子座");
    assert_eq!(constellation_zh(5, 20), "金牛座");
    assert_eq!(constellation_zh(5, 21), "双子座");
    assert_eq!(constellation_zh(1, 1), "摩羯座");
    assert_eq!(constellation_zh(12, 25), "摩羯座");
}
