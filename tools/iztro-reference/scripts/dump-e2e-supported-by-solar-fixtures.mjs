// Dumps one compact upstream iztro@2.5.8 bySolar E2E fixture for fields
// currently supported by iztro-rs, including the converted lunar facts used to
// diagnose Gregorian-to-Chinese-lunisolar calendar mismatches.
//
// Usage:
//   npm ci --prefix tools/iztro-reference
//   node tools/iztro-reference/scripts/dump-e2e-supported-by-solar-fixtures.mjs [--write]
//
// Without --write the fixture is printed to stdout. With --write it is written
// to fixtures/iztro/e2e_supported_by_solar.json.

import { astro } from "iztro";
import { writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  BRANCH_KEYS,
  STEM_KEYS,
  normalizeSupportedFields,
  requiredKey
} from "./lib/normalize.mjs";

const GENERATED_AT = "2026-06-10T00:00:00Z";
const GENERATION_COMMAND =
  "npm ci --prefix tools/iztro-reference && npm run dump:e2e-supported-by-solar --prefix tools/iztro-reference -- --write";
const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

// Diagnostic case set: multiple years/months/birth times, both genders, dates
// around Chinese New Year boundaries, ordinary non-leap dates, a date that
// converts into a leap lunar month (second half), the same leap second-half
// solar date with fixLeap=false, and a date after a leap month.
const CASES = [
  {
    case: "1985_02_15_yin_male", // before Chinese New Year (1985 CNY = Feb 20)
    solarDate: "1985-2-15",
    solarYear: 1985,
    solarMonth: 2,
    solarDay: 15,
    timeIndex: 2,
    birthTime: "yin",
    gender: "男",
    genderKey: "male"
  },
  {
    case: "1986_02_09_chen_female", // Chinese New Year day (1986 CNY = Feb 9)
    solarDate: "1986-2-9",
    solarYear: 1986,
    solarMonth: 2,
    solarDay: 9,
    timeIndex: 4,
    birthTime: "chen",
    gender: "女",
    genderKey: "female"
  },
  {
    case: "1986_02_25_wu_male", // after Chinese New Year
    solarDate: "1986-2-25",
    solarYear: 1986,
    solarMonth: 2,
    solarDay: 25,
    timeIndex: 6,
    birthTime: "wu",
    gender: "男",
    genderKey: "male"
  },
  {
    case: "1990_05_17_chen_female", // ordinary mid-year date
    solarDate: "1990-5-17",
    solarYear: 1990,
    solarMonth: 5,
    solarDay: 17,
    timeIndex: 4,
    birthTime: "chen",
    gender: "女",
    genderKey: "female"
  },
  {
    case: "2020_06_18_shen_female", // converts to leap 四月, second half (day > 15)
    solarDate: "2020-6-18",
    solarYear: 2020,
    solarMonth: 6,
    solarDay: 18,
    timeIndex: 8,
    birthTime: "shen",
    gender: "女",
    genderKey: "female",
    fixLeap: true
  },
  {
    // Same leap second-half date with fix_leap=false: the effective month does
    // not advance, so month-based placement differs from the fix_leap=true case.
    case: "2020_06_18_shen_female_nofix",
    solarDate: "2020-6-18",
    solarYear: 2020,
    solarMonth: 6,
    solarDay: 18,
    timeIndex: 8,
    birthTime: "shen",
    gender: "女",
    genderKey: "female",
    fixLeap: false
  },
  {
    case: "2020_06_25_xu_male", // after the leap month
    solarDate: "2020-6-25",
    solarYear: 2020,
    solarMonth: 6,
    solarDay: 25,
    timeIndex: 10,
    birthTime: "xu",
    gender: "男",
    genderKey: "male"
  }
];

const ALGORITHMS = ["default", "zhongzhou"];

function normalizeInput(input) {
  return {
    solar_date: input.solarDate,
    solar_year: input.solarYear,
    solar_month: input.solarMonth,
    solar_day: input.solarDay,
    birth_time: input.birthTime,
    iztro_time_index: input.timeIndex,
    gender: input.genderKey,
    iztro_gender: input.gender,
    fix_leap: input.fixLeap ?? true,
    language: "zh-CN"
  };
}

function normalizeConvertedLunar(chart) {
  const lunar = chart.rawDates.lunarDate;
  const [rawStem, rawBranch] = chart.rawDates.chineseDate.yearly;
  return {
    lunar_year: lunar.lunarYear,
    lunar_month: lunar.lunarMonth,
    lunar_day: lunar.lunarDay,
    is_leap_month: lunar.isLeap,
    birth_year_stem: requiredKey(STEM_KEYS, rawStem, "year stem"),
    raw_birth_year_stem: rawStem,
    birth_year_branch: requiredKey(BRANCH_KEYS, rawBranch, "year branch"),
    raw_birth_year_branch: rawBranch
  };
}

function buildCase(input, algorithm) {
  astro.config({ algorithm });
  const chart = astro.bySolar(
    input.solarDate,
    input.timeIndex,
    input.gender,
    input.fixLeap ?? true,
    "zh-CN"
  );

  return {
    case: input.case,
    algorithm,
    input: normalizeInput(input),
    converted_lunar: normalizeConvertedLunar(chart),
    supported_fields: {
      birth_time: input.birthTime,
      gender: input.genderKey,
      ...normalizeSupportedFields(chart)
    }
  };
}

function buildFixture() {
  return {
    metadata: {
      source: "iztro",
      target_package: "npm:iztro",
      target_version: "2.5.8",
      generated_at: GENERATED_AT,
      supported_fields_only: true,
      notes:
        "Supported-field-only bySolar E2E fixture for iztro-rs. Cases cover Chinese New Year boundaries, ordinary non-leap dates, a date converting into a leap lunar month, the same leap second-half solar date with fixLeap=true and fixLeap=false, and a date after a leap month, under default and Zhongzhou algorithms. converted_lunar records the ICU-comparable lunar year/month/day, leap flag, and birth-year ganzhi derived by upstream (rawDates.lunarDate + rawDates.chineseDate.yearly). supported_fields normalizes life/body palace branches, five-element bureau, palace branch/stem/name facts, represented typed natal stars, and the four decorative runtime families. Raw upstream labels are preserved beside normalized keys. Temporal flow stars, full facade serialization parity, rat-hour variants, horoscope palace-name derivation, temporal decorative arrays, features, rules, and narrative are excluded.",
      generation_command: GENERATION_COMMAND
    },
    cases: CASES.flatMap((input) => ALGORITHMS.map((algorithm) => buildCase(input, algorithm)))
  };
}

const fixture = buildFixture();

if (process.argv.includes("--write")) {
  writeFileSync(
    join(REPO_ROOT, "fixtures", "iztro", "e2e_supported_by_solar.json"),
    `${JSON.stringify(fixture, null, 2)}\n`
  );
  console.log("wrote fixtures/iztro/e2e_supported_by_solar.json");
} else {
  console.log(JSON.stringify(fixture, null, 2));
}
