//! Western (tropical) zodiac sign as a language-neutral domain value.
//!
//! Zi Wei Dou Shu charts conventionally display the native's Western zodiac
//! sign (星座) alongside the Chinese zodiac animal. The sign itself is a neutral
//! enum; localized display strings live in the presentation layer. The Chinese
//! label table stays in [`crate::core::labels::chinese_date`].

use serde::{Deserialize, Serialize};

/// A Western (tropical) zodiac sign derived from a solar (Gregorian) date.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WesternZodiac {
    /// Aries (白羊座).
    Aries,
    /// Taurus (金牛座).
    Taurus,
    /// Gemini (双子座).
    Gemini,
    /// Cancer (巨蟹座).
    Cancer,
    /// Leo (狮子座).
    Leo,
    /// Virgo (处女座).
    Virgo,
    /// Libra (天秤座).
    Libra,
    /// Scorpio (天蝎座).
    Scorpio,
    /// Sagittarius (射手座).
    Sagittarius,
    /// Capricorn (摩羯座).
    Capricorn,
    /// Aquarius (水瓶座).
    Aquarius,
    /// Pisces (双鱼座).
    Pisces,
}

/// Returns the Western zodiac sign for a solar (Gregorian) month and day.
///
/// Returns `None` for an out-of-range month/day so the helper stays total; real
/// birth charts always carry a valid solar date.
pub const fn western_zodiac(solar_month: u8, solar_day: u8) -> Option<WesternZodiac> {
    let sign = match (solar_month, solar_day) {
        (3, 21..=31) | (4, 1..=19) => WesternZodiac::Aries,
        (4, 20..=30) | (5, 1..=20) => WesternZodiac::Taurus,
        (5, 21..=31) | (6, 1..=21) => WesternZodiac::Gemini,
        (6, 22..=30) | (7, 1..=22) => WesternZodiac::Cancer,
        (7, 23..=31) | (8, 1..=22) => WesternZodiac::Leo,
        (8, 23..=31) | (9, 1..=22) => WesternZodiac::Virgo,
        (9, 23..=30) | (10, 1..=23) => WesternZodiac::Libra,
        (10, 24..=31) | (11, 1..=22) => WesternZodiac::Scorpio,
        (11, 23..=30) | (12, 1..=21) => WesternZodiac::Sagittarius,
        (12, 22..=31) | (1, 1..=19) => WesternZodiac::Capricorn,
        (1, 20..=31) | (2, 1..=18) => WesternZodiac::Aquarius,
        (2, 19..=29) | (3, 1..=20) => WesternZodiac::Pisces,
        _ => return None,
    };
    Some(sign)
}
