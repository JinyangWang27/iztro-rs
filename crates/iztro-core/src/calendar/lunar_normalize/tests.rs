use super::*;

fn resolve(year: i32, month: u8, day: u8, leap: bool) -> ResolvedLunarDate {
    resolve_lunar_date(
        year,
        LunarMonth::new(month).expect("valid lunar month"),
        LunarDay::new(day).expect("valid lunar day"),
        leap,
    )
    .expect("resolution should succeed")
}

#[test]
fn honors_valid_leap_month() {
    // 2020 has a leap fourth month; the flag is kept.
    let resolved = resolve(2020, 4, 27, true);
    assert_eq!(resolved.lunar_year(), 2020);
    assert_eq!(resolved.lunar_month().value(), 4);
    assert_eq!(resolved.lunar_day().value(), 27);
    assert_eq!(resolved.month_days(), 29);
    assert!(resolved.is_leap_month());
}

#[test]
fn ignores_invalid_leap_month() {
    // 2020's leap month is the fourth, not the third; the flag is ignored.
    let resolved = resolve(2020, 3, 20, true);
    assert_eq!(resolved.lunar_month().value(), 3);
    assert_eq!(resolved.lunar_day().value(), 20);
    assert!(!resolved.is_leap_month());

    // Same year, fifth month is not leap either.
    assert!(!resolve(2020, 5, 20, true).is_leap_month());

    // 2021 has no leap month at all.
    assert!(!resolve(2021, 6, 10, true).is_leap_month());
}

#[test]
fn non_leap_request_stays_non_leap() {
    let resolved = resolve(2020, 4, 10, false);
    assert_eq!(resolved.lunar_month().value(), 4);
    assert!(!resolved.is_leap_month());
}
