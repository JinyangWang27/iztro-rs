use super::*;
use crate::core::calculation::YearBoundary;

// Golden conversions captured from pinned upstream `iztro@2.5.8`:
//   node --input-type=module -e "import { astro } from 'iztro';
//     const a = astro.bySolar('YYYY-M-D', 4, '女', true, 'zh-CN');
//     console.log(a.rawDates.lunarDate, a.rawDates.chineseDate.yearly);"
// These exercise the adapter mapping independently of the chart E2E fixtures:
// before / on / after Chinese New Year, an ordinary mid-year date, a date
// that converts into a leap lunar month (both halves), and a date after a
// leap month. 1985-02-15 sits in the 立春/正月初一 window and still resolves
// to the prior lunar year, proving the adapter uses the lunar-new-year
// boundary like iztro's default `yearDivide: 'normal'`.
struct Case {
    year: i32,
    month: u8,
    day: u8,
    lunar_year: i32,
    lunar_month: u8,
    lunar_day: u8,
    is_leap: bool,
    stem: HeavenlyStem,
    branch: EarthlyBranch,
}

const CASES: &[Case] = &[
    // before Chinese New Year (in the 立春/CNY window) -> prior lunar year
    Case {
        year: 1985,
        month: 2,
        day: 15,
        lunar_year: 1984,
        lunar_month: 12,
        lunar_day: 26,
        is_leap: false,
        stem: HeavenlyStem::Jia,
        branch: EarthlyBranch::Zi,
    },
    // Chinese New Year day
    Case {
        year: 1986,
        month: 2,
        day: 9,
        lunar_year: 1986,
        lunar_month: 1,
        lunar_day: 1,
        is_leap: false,
        stem: HeavenlyStem::Bing,
        branch: EarthlyBranch::Yin,
    },
    // after Chinese New Year
    Case {
        year: 1986,
        month: 2,
        day: 25,
        lunar_year: 1986,
        lunar_month: 1,
        lunar_day: 17,
        is_leap: false,
        stem: HeavenlyStem::Bing,
        branch: EarthlyBranch::Yin,
    },
    // ordinary mid-year date
    Case {
        year: 1990,
        month: 5,
        day: 17,
        lunar_year: 1990,
        lunar_month: 4,
        lunar_day: 23,
        is_leap: false,
        stem: HeavenlyStem::Geng,
        branch: EarthlyBranch::Wu,
    },
    // before Chinese New Year of the next solar year -> prior lunar year
    Case {
        year: 2020,
        month: 1,
        day: 1,
        lunar_year: 2019,
        lunar_month: 12,
        lunar_day: 7,
        is_leap: false,
        stem: HeavenlyStem::Ji,
        branch: EarthlyBranch::Hai,
    },
    // converts into a leap lunar month, first half (day <= 15)
    Case {
        year: 2020,
        month: 6,
        day: 1,
        lunar_year: 2020,
        lunar_month: 4,
        lunar_day: 10,
        is_leap: true,
        stem: HeavenlyStem::Geng,
        branch: EarthlyBranch::Zi,
    },
    // converts into a leap lunar month, second half (day > 15)
    Case {
        year: 2020,
        month: 6,
        day: 18,
        lunar_year: 2020,
        lunar_month: 4,
        lunar_day: 27,
        is_leap: true,
        stem: HeavenlyStem::Geng,
        branch: EarthlyBranch::Zi,
    },
    // date after a leap month
    Case {
        year: 2020,
        month: 6,
        day: 25,
        lunar_year: 2020,
        lunar_month: 5,
        lunar_day: 5,
        is_leap: false,
        stem: HeavenlyStem::Geng,
        branch: EarthlyBranch::Zi,
    },
];

#[test]
fn matches_upstream_conversions() {
    for case in CASES {
        let conversion = solar_to_lunar(
            case.year,
            SolarMonth::new(case.month).expect("valid solar month"),
            SolarDay::new(case.day).expect("valid solar day"),
            0,
        )
        .unwrap_or_else(|err| {
            panic!(
                "{}-{}-{} should convert: {err}",
                case.year, case.month, case.day
            )
        });

        let label = format!("{}-{}-{}", case.year, case.month, case.day);
        assert_eq!(
            conversion.lunar_year(),
            case.lunar_year,
            "{label}: lunar year"
        );
        assert_eq!(
            conversion.lunar_month().value(),
            case.lunar_month,
            "{label}: lunar month"
        );
        assert_eq!(
            conversion.lunar_day().value(),
            case.lunar_day,
            "{label}: lunar day"
        );
        assert_eq!(
            conversion.is_leap_month(),
            case.is_leap,
            "{label}: leap flag"
        );
        assert_eq!(
            conversion.birth_year_stem(),
            case.stem,
            "{label}: year stem"
        );
        assert_eq!(
            conversion.birth_year_branch(),
            case.branch,
            "{label}: year branch"
        );
    }
}

#[test]
fn cyclic_year_matches_lunar_year_ganzhi() {
    // The four-pillar yearly result must equal the converted lunar-year
    // ganzhi when both use the normal lunar-new-year boundary.
    for case in CASES {
        let conversion = solar_to_lunar(
            case.year,
            SolarMonth::new(case.month).expect("valid solar month"),
            SolarDay::new(case.day).expect("valid solar day"),
            0,
        )
        .expect("conversion should succeed");

        let expected =
            crate::core::model::ganzhi::StemBranch::from_lunar_year(conversion.lunar_year());
        assert_eq!(
            conversion.birth_year_stem(),
            expected.stem(),
            "{} stem",
            case.year
        );
        assert_eq!(
            conversion.birth_year_branch(),
            expected.branch(),
            "{} branch",
            case.year
        );
    }
}

fn month(value: u8) -> SolarMonth {
    SolarMonth::new(value).expect("valid solar month")
}

fn day(value: u8) -> SolarDay {
    SolarDay::new(value).expect("valid solar day")
}

#[test]
fn default_year_boundary_matches_chinese_new_year_eve() {
    // The four-arg adapter must reproduce the explicit ChineseNewYearEve policy
    // so existing callers and fixtures keep the lunar-new-year boundary.
    for case in CASES {
        let default = solar_to_lunar(case.year, month(case.month), day(case.day), 0)
            .expect("default conversion should succeed");
        let explicit = solar_to_lunar_with_year_boundary(
            case.year,
            month(case.month),
            day(case.day),
            0,
            YearBoundary::ChineseNewYearEve,
        )
        .expect("explicit conversion should succeed");
        assert_eq!(
            default, explicit,
            "{}-{}-{}",
            case.year, case.month, case.day
        );
    }
}

#[test]
fn year_boundary_policies_differ_between_li_chun_and_chinese_new_year() {
    // Li Chun 2000 falls on 2000-02-04 in the evening (~20:40); Chinese New Year
    // 2000 is 2000-02-05. After the Li Chun instant on 2000-02-04 the two
    // policies disagree: the lunar-new-year boundary keeps the prior cyclic year
    // (1999 己卯) while the Li Chun boundary advances to 2000 庚辰. `time_index`
    // 11 synthesizes 21:30, after the Li Chun instant.
    let eve =
        resolve_effective_birth_year(2000, month(2), day(4), 11, YearBoundary::ChineseNewYearEve)
            .expect("eve year");
    let li_chun = resolve_effective_birth_year(2000, month(2), day(4), 11, YearBoundary::LiChun)
        .expect("li chun");

    assert_eq!(eve.stem(), HeavenlyStem::Ji);
    assert_eq!(eve.branch(), EarthlyBranch::Mao);
    assert_eq!(li_chun.stem(), HeavenlyStem::Geng);
    assert_eq!(li_chun.branch(), EarthlyBranch::Chen);
    assert_ne!(eve, li_chun);
}

#[test]
fn li_chun_boundary_is_datetime_level_on_the_li_chun_day() {
    // Datetime-level Li Chun: on the 立春 day the result depends on the time of
    // day. Before the ~20:40 instant the cyclic year is still 1999 己卯; after
    // it the cyclic year is 2000 庚辰. `time_index` 1 -> 01:30 (before),
    // `time_index` 11 -> 21:30 (after).
    let before = resolve_effective_birth_year(2000, month(2), day(4), 1, YearBoundary::LiChun)
        .expect("before instant");
    let after = resolve_effective_birth_year(2000, month(2), day(4), 11, YearBoundary::LiChun)
        .expect("after instant");

    assert_eq!(before.stem(), HeavenlyStem::Ji);
    assert_eq!(before.branch(), EarthlyBranch::Mao);
    assert_eq!(after.stem(), HeavenlyStem::Geng);
    assert_eq!(after.branch(), EarthlyBranch::Chen);
    assert_ne!(before, after);
}

#[test]
fn li_chun_2024_before_exact_instant_uses_previous_ganzhi_year() {
    // Regression for the known date-level mismatch class. Li Chun 2024 is
    // 2024-02-04 ~16:27. A birth earlier that day (`time_index` 8 -> 15:30) under
    // the Li Chun boundary belongs to the previous Ganzhi year 癸卯 (GuiMao), not
    // 甲辰 (JiaChen). The old `lunar-lite` date-level boundary returned 甲辰 here.
    let before = resolve_effective_birth_year(2024, month(2), day(4), 8, YearBoundary::LiChun)
        .expect("before instant");
    assert_eq!(before.stem(), HeavenlyStem::Gui);
    assert_eq!(before.branch(), EarthlyBranch::Mao);

    // Later that day (`time_index` 9 -> 17:30), after the instant, it is 甲辰.
    let after = resolve_effective_birth_year(2024, month(2), day(4), 9, YearBoundary::LiChun)
        .expect("after instant");
    assert_eq!(after.stem(), HeavenlyStem::Jia);
    assert_eq!(after.branch(), EarthlyBranch::Chen);
}

#[test]
fn li_chun_2024_exact_clock_minute_splits_same_time_branch() {
    // Both times are in the same 申时 branch. The exact clock minute must still
    // decide the LiChun boundary: 2024-02-04 16:10 is before the ~16:27 instant,
    // while 16:40 is after it.
    let before = solar_to_lunar_with_resolved_datetime(
        2024,
        month(2),
        day(4),
        16,
        10,
        0,
        YearBoundary::LiChun,
    )
    .expect("before instant");
    let after = solar_to_lunar_with_resolved_datetime(
        2024,
        month(2),
        day(4),
        16,
        40,
        0,
        YearBoundary::LiChun,
    )
    .expect("after instant");

    assert_eq!(before.birth_year_stem(), HeavenlyStem::Gui);
    assert_eq!(before.birth_year_branch(), EarthlyBranch::Mao);
    assert_eq!(after.birth_year_stem(), HeavenlyStem::Jia);
    assert_eq!(after.birth_year_branch(), EarthlyBranch::Chen);
}

#[test]
fn year_boundary_after_chinese_new_year_before_li_chun_differs() {
    // Chinese New Year 2001 is 2001-01-24; Li Chun 2001 is 2001-02-03. On
    // 2001-01-28 the date is after the lunar new year (so the normal boundary
    // already uses 2001 辛巳) but the whole day is before Li Chun (so the exact
    // boundary keeps 2000 庚辰), regardless of the time of day.
    let eve =
        resolve_effective_birth_year(2001, month(1), day(28), 6, YearBoundary::ChineseNewYearEve)
            .expect("eve year");
    let li_chun = resolve_effective_birth_year(2001, month(1), day(28), 6, YearBoundary::LiChun)
        .expect("li chun");

    assert_eq!(eve.stem(), HeavenlyStem::Xin);
    assert_eq!(eve.branch(), EarthlyBranch::Si);
    assert_eq!(li_chun.stem(), HeavenlyStem::Geng);
    assert_eq!(li_chun.branch(), EarthlyBranch::Chen);
    assert_ne!(eve, li_chun);
}

#[test]
fn year_boundary_policies_agree_on_ordinary_date() {
    let eve =
        resolve_effective_birth_year(1990, month(6), day(15), 6, YearBoundary::ChineseNewYearEve)
            .expect("eve year");
    let li_chun = resolve_effective_birth_year(1990, month(6), day(15), 6, YearBoundary::LiChun)
        .expect("li chun");
    assert_eq!(eve, li_chun);
}

#[test]
fn rejects_impossible_solar_date() {
    let err = solar_to_lunar(
        2021,
        SolarMonth::new(2).expect("valid solar month"),
        SolarDay::new(30).expect("valid solar day"),
        0,
    )
    .expect_err("30 February is not a real date");
    assert!(matches!(err, ChartError::InvalidSolarDate { .. }));
}
