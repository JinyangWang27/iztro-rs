use iztro::core::{WesternZodiac, western_zodiac};

#[test]
fn boundary_dates_map_to_expected_signs() {
    assert_eq!(western_zodiac(5, 27), Some(WesternZodiac::Gemini));
    assert_eq!(western_zodiac(5, 20), Some(WesternZodiac::Taurus));
    assert_eq!(western_zodiac(5, 21), Some(WesternZodiac::Gemini));
    assert_eq!(western_zodiac(1, 1), Some(WesternZodiac::Capricorn));
    assert_eq!(western_zodiac(12, 25), Some(WesternZodiac::Capricorn));
}

#[test]
fn invalid_dates_return_none() {
    assert_eq!(western_zodiac(0, 1), None);
    assert_eq!(western_zodiac(13, 1), None);
    assert_eq!(western_zodiac(2, 30), None);
}
