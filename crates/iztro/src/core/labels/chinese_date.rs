//! Conventional Chinese date, lunar-date, double-hour, and constellation
//! display labels.
//!
//! These are additive presentation helpers: they format already-computed
//! calendar facts into the conventional Chinese strings used by chart views
//! (for example `一九九三年四月初七`, `酉时(17:00~19:00)`, `双子座`). They never
//! perform calendar conversion themselves; callers supply the numeric facts.

use crate::core::model::zodiac::{WesternZodiac, western_zodiac};

/// Chinese digits used for digit-by-digit year rendering (`〇` for zero).
const YEAR_DIGITS: [&str; 10] = ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

/// Conventional lunar month labels (`正月`..`腊月`), one-based.
const LUNAR_MONTH_LABELS: [&str; 12] = [
    "正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "冬月", "腊月",
];

/// Conventional lunar day labels (`初一`..`三十`), one-based.
const LUNAR_DAY_LABELS: [&str; 30] = [
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二",
    "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四",
    "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
];

/// Renders a year digit-by-digit in Chinese numerals, such as `1993` → `一九九三`.
///
/// Negative years are rendered with a leading `公元前` marker; this never occurs
/// for real birth charts but keeps the helper total.
pub fn chinese_year_digits(year: i32) -> String {
    if year < 0 {
        return format!("公元前{}", chinese_year_digits(-year));
    }
    year.to_string()
        .bytes()
        .map(|byte| YEAR_DIGITS[(byte - b'0') as usize])
        .collect()
}

/// Returns the conventional Chinese label for a one-based lunar month.
///
/// Out-of-range months fall back to the bare number with a `月` suffix so the
/// helper stays total.
pub fn lunar_month_zh(month: u8) -> String {
    match month {
        1..=12 => LUNAR_MONTH_LABELS[(month - 1) as usize].to_owned(),
        other => format!("{other}月"),
    }
}

/// Returns the conventional Chinese label for a one-based lunar day.
///
/// Out-of-range days fall back to the bare number so the helper stays total.
pub fn lunar_day_zh(day: u8) -> String {
    match day {
        1..=30 => LUNAR_DAY_LABELS[(day - 1) as usize].to_owned(),
        other => other.to_string(),
    }
}

/// Formats a solar (Gregorian) date as padded `YYYY-MM-DD`, such as `1993-05-27`.
///
/// This is the birth-date display style used by the static chart center.
pub fn solar_date_label_padded(year: i32, month: u8, day: u8) -> String {
    format!("{year}-{month:02}-{day:02}")
}

/// Formats a solar (Gregorian) date as unpadded `YYYY-M-D`, such as `2008-2-10`.
///
/// This matches the iztro-style 运限阳历 display.
pub fn solar_date_label_unpadded(year: i32, month: u8, day: u8) -> String {
    format!("{year}-{month}-{day}")
}

/// Formats a lunar date as `<年>年<月><日>`, such as `一九九三年四月初七`.
///
/// Leap months are prefixed with `闰`, matching conventional almanac display.
pub fn lunar_date_label(year: i32, month: u8, day: u8, is_leap_month: bool) -> String {
    let leap = if is_leap_month { "闰" } else { "" };
    format!(
        "{}年{}{}{}",
        chinese_year_digits(year),
        leap,
        lunar_month_zh(month),
        lunar_day_zh(day)
    )
}

/// Returns the conventional double-hour (时辰) label with its clock range for an
/// upstream `iztro` `timeIndex` (`0..=12`), such as `酉时(17:00~19:00)`.
///
/// Early Zi (`0`) and late Zi (`12`) share the `子时` branch but carry the two
/// halves of the Zi double-hour range.
pub fn birth_time_label(time_index: u8) -> String {
    let (branch, range) = match time_index {
        0 => ("子", "00:00~01:00"),
        1 => ("丑", "01:00~03:00"),
        2 => ("寅", "03:00~05:00"),
        3 => ("卯", "05:00~07:00"),
        4 => ("辰", "07:00~09:00"),
        5 => ("巳", "09:00~11:00"),
        6 => ("午", "11:00~13:00"),
        7 => ("未", "13:00~15:00"),
        8 => ("申", "15:00~17:00"),
        9 => ("酉", "17:00~19:00"),
        10 => ("戌", "19:00~21:00"),
        11 => ("亥", "21:00~23:00"),
        12 => ("子", "23:00~24:00"),
        _ => ("未知", "--:--~--:--"),
    };
    format!("{branch}时({range})")
}

/// Returns the Western constellation (星座) for a solar month and day, such as
/// `双子座` for 27 May.
///
/// The sign boundaries live in the language-neutral
/// [`western_zodiac`](crate::core::model::zodiac::western_zodiac); this is the
/// Chinese display table for that enum.
pub fn constellation_zh(solar_month: u8, solar_day: u8) -> &'static str {
    match western_zodiac(solar_month, solar_day) {
        Some(sign) => western_zodiac_zh(sign),
        None => "未知",
    }
}

/// Returns the conventional Chinese label (星座) for a [`WesternZodiac`] sign.
pub const fn western_zodiac_zh(sign: WesternZodiac) -> &'static str {
    match sign {
        WesternZodiac::Aries => "白羊座",
        WesternZodiac::Taurus => "金牛座",
        WesternZodiac::Gemini => "双子座",
        WesternZodiac::Cancer => "巨蟹座",
        WesternZodiac::Leo => "狮子座",
        WesternZodiac::Virgo => "处女座",
        WesternZodiac::Libra => "天秤座",
        WesternZodiac::Scorpio => "天蝎座",
        WesternZodiac::Sagittarius => "射手座",
        WesternZodiac::Capricorn => "摩羯座",
        WesternZodiac::Aquarius => "水瓶座",
        WesternZodiac::Pisces => "双鱼座",
    }
}
