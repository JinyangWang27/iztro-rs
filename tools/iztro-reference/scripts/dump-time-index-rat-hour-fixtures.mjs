// Dumps a compact upstream iztro@2.5.8 byLunar fixture for timeIndex 0..=12
// behavior, especially early Zi (0), late Zi (12), and the late-Zi leap-month
// guard.
//
// Usage:
//   npm ci --prefix tools/iztro-reference
//   npm run dump:time-index-rat-hour --prefix tools/iztro-reference [-- --write]
//
// Without --write the fixture is printed to stdout. With --write it is written
// to fixtures/iztro/time_index_rat_hour.json.

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
  "npm ci --prefix tools/iztro-reference && npm run dump:time-index-rat-hour --prefix tools/iztro-reference -- --write";
const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const ALGORITHM = "default";

const CASES = [
  {
    case: "1990_05_17_early_zi",
    lunarDate: "1990-5-17",
    lunarYear: 1990,
    lunarMonth: 5,
    lunarDay: 17,
    isLeapMonth: false,
    fixLeap: true,
    timeIndex: 0,
    birthTime: "zi",
    gender: "女",
    genderKey: "female"
  },
  {
    case: "1990_05_17_late_zi",
    lunarDate: "1990-5-17",
    lunarYear: 1990,
    lunarMonth: 5,
    lunarDay: 17,
    isLeapMonth: false,
    fixLeap: true,
    timeIndex: 12,
    birthTime: "zi",
    gender: "女",
    genderKey: "female"
  },
  {
    case: "1990_05_17_chen",
    lunarDate: "1990-5-17",
    lunarYear: 1990,
    lunarMonth: 5,
    lunarDay: 17,
    isLeapMonth: false,
    fixLeap: true,
    timeIndex: 4,
    birthTime: "chen",
    gender: "女",
    genderKey: "female"
  },
  {
    case: "2020_04_27_leap_second_half_wu",
    lunarDate: "2020-4-27",
    lunarYear: 2020,
    lunarMonth: 4,
    lunarDay: 27,
    isLeapMonth: true,
    fixLeap: true,
    timeIndex: 6,
    birthTime: "wu",
    gender: "男",
    genderKey: "male"
  },
  {
    case: "2020_04_27_leap_second_half_late_zi",
    lunarDate: "2020-4-27",
    lunarYear: 2020,
    lunarMonth: 4,
    lunarDay: 27,
    isLeapMonth: true,
    fixLeap: true,
    timeIndex: 12,
    birthTime: "zi",
    gender: "男",
    genderKey: "male"
  },
  {
    case: "1990_04_29_month_end_late_zi",
    lunarDate: "1990-4-29",
    lunarYear: 1990,
    lunarMonth: 4,
    lunarDay: 29,
    isLeapMonth: false,
    fixLeap: true,
    timeIndex: 12,
    birthTime: "zi",
    gender: "女",
    genderKey: "female"
  }
];

function upstreamYearly(input) {
  astro.config({ algorithm: ALGORITHM });
  const chart = astro.byLunar(
    input.lunarDate,
    input.timeIndex,
    input.gender,
    input.isLeapMonth,
    input.fixLeap,
    "zh-CN"
  );
  return chart.rawDates.chineseDate.yearly;
}

function normalizeInput(input) {
  const [rawStem, rawBranch] = upstreamYearly(input);
  return {
    lunar_date: input.lunarDate,
    lunar_year: input.lunarYear,
    lunar_month: input.lunarMonth,
    lunar_day: input.lunarDay,
    is_leap_month: input.isLeapMonth,
    fix_leap: input.fixLeap,
    birth_time: input.birthTime,
    iztro_time_index: input.timeIndex,
    gender: input.genderKey,
    iztro_gender: input.gender,
    birth_year_stem: requiredKey(STEM_KEYS, rawStem, "year stem"),
    raw_birth_year_stem: rawStem,
    birth_year_branch: requiredKey(BRANCH_KEYS, rawBranch, "year branch"),
    raw_birth_year_branch: rawBranch,
    language: "zh-CN"
  };
}

function normalizeResolvedLunar(chart) {
  const lunar = chart.rawDates.lunarDate;
  return {
    lunar_year: lunar.lunarYear,
    lunar_month: lunar.lunarMonth,
    lunar_day: lunar.lunarDay,
    is_leap_month: lunar.isLeap
  };
}

function buildCase(input) {
  astro.config({ algorithm: ALGORITHM });
  const chart = astro.byLunar(
    input.lunarDate,
    input.timeIndex,
    input.gender,
    input.isLeapMonth,
    input.fixLeap,
    "zh-CN"
  );

  return {
    case: input.case,
    algorithm: ALGORITHM,
    input: normalizeInput(input),
    resolved_lunar: normalizeResolvedLunar(chart),
    supported_fields: {
      birth_time: input.birthTime,
      iztro_time_index: input.timeIndex,
      gender: input.genderKey,
      is_leap_month: input.isLeapMonth,
      fix_leap: input.fixLeap,
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
        "Supported-field-only byLunar fixture for iztro-rs timeIndex behavior. Cases cover early Zi timeIndex 0, late Zi timeIndex 12, one normal non-Zi time, a real 2020 leap fourth-month second-half date with fixLeap=true for both normal time and late Zi, and a late-Zi case on the last day of a 29-day lunar month (1990-4-29) to exercise month-end wrap-around in major_lunar_day. The late-Zi leap case proves upstream does not advance the effective month when timeIndex is 12. supported_fields normalizes life/body palace branches, five-element bureau, palace branch/stem/name facts, represented typed natal stars, and the four decorative runtime families. Birth-year ganzhi is taken from upstream and fed back to the Rust by_lunar call. Full facade serialization parity, horoscope palace-name derivation, temporal decorative arrays, features, rules, and narrative are excluded.",
      generation_command: GENERATION_COMMAND
    },
    cases: CASES.map(buildCase)
  };
}

const fixture = buildFixture();

if (process.argv.includes("--write")) {
  writeFileSync(
    join(REPO_ROOT, "fixtures", "iztro", "time_index_rat_hour.json"),
    `${JSON.stringify(fixture, null, 2)}\n`
  );
  console.log("wrote fixtures/iztro/time_index_rat_hour.json");
} else {
  console.log(JSON.stringify(fixture, null, 2));
}
