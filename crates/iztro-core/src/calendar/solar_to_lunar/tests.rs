use super::*;

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
        )
        .expect("conversion should succeed");

        let expected = lunar_lite::StemBranch::from_lunar_year(conversion.lunar_year());
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

#[test]
fn rejects_impossible_solar_date() {
    let err = solar_to_lunar(
        2021,
        SolarMonth::new(2).expect("valid solar month"),
        SolarDay::new(30).expect("valid solar day"),
    )
    .expect_err("30 February is not a real date");
    assert!(matches!(err, ChartError::InvalidSolarDate { .. }));
}
