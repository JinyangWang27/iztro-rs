// Dumps a compact upstream iztro@2.5.8 byLunar fixture that characterizes
// leap-month behavior (isLeapMonth and fixLeap) for fields currently supported
// by iztro-rs, using real lunar dates from the 2020 leap year (闰四月).
//
// Usage:
//   npm ci --prefix tools/iztro-reference
//   node tools/iztro-reference/scripts/dump-leap-month-fixtures.mjs [--write]
//
// Without --write the fixture is printed to stdout. With --write it is written
// to fixtures/iztro/leap_month_by_lunar.json.

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
  "npm ci --prefix tools/iztro-reference && npm run dump:leap-month --prefix tools/iztro-reference -- --write";
const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

// 2020 has a leap fourth month (闰四月, 2020-05-23 .. 2020-06-20). Cases cover a
// date before the leap month, the regular month with the leap-month number, both
// halves of the leap month, and a date after the leap month, across the
// isLeapMonth and fixLeap toggles. The leap fourth month, day > 15, fixLeap pair
// (cases 4 and 5) is the discriminator: only fixLeap = true advances the
// effective month. All cases share the default algorithm.
const CASES = [
  {
    case: "2020_03_20_before_leap",
    lunarDate: "2020-3-20",
    lunarYear: 2020,
    lunarMonth: 3,
    lunarDay: 20,
    isLeapMonth: false,
    fixLeap: true,
    timeIndex: 2,
    birthTime: "yin",
    gender: "女",
    genderKey: "female"
  },
  {
    case: "2020_04_10_regular_month",
    lunarDate: "2020-4-10",
    lunarYear: 2020,
    lunarMonth: 4,
    lunarDay: 10,
    isLeapMonth: false,
    fixLeap: true,
    timeIndex: 6,
    birthTime: "wu",
    gender: "男",
    genderKey: "male"
  },
  {
    case: "2020_04_10_leap_first_half",
    lunarDate: "2020-4-10",
    lunarYear: 2020,
    lunarMonth: 4,
    lunarDay: 10,
    isLeapMonth: true,
    fixLeap: true,
    timeIndex: 6,
    birthTime: "wu",
    gender: "男",
    genderKey: "male"
  },
  {
    case: "2020_04_27_leap_second_half_fix",
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
    case: "2020_04_27_leap_second_half_nofix",
    lunarDate: "2020-4-27",
    lunarYear: 2020,
    lunarMonth: 4,
    lunarDay: 27,
    isLeapMonth: true,
    fixLeap: false,
    timeIndex: 6,
    birthTime: "wu",
    gender: "男",
    genderKey: "male"
  },
  {
    case: "2020_05_05_after_leap",
    lunarDate: "2020-5-5",
    lunarYear: 2020,
    lunarMonth: 5,
    lunarDay: 5,
    isLeapMonth: false,
    fixLeap: true,
    timeIndex: 10,
    birthTime: "xu",
    gender: "女",
    genderKey: "female"
  },
  // Invalid leap requests: isLeapMonth=true for a month that is NOT the year's
  // actual leap month. Upstream lunar2solar ignores the flag and resolves these
  // as ordinary lunar dates (resolved_lunar.is_leap_month=false). Day > 15 makes
  // the regression sharp: a wrongly-trusted leap flag would advance the month.
  {
    case: "2020_03_20_invalid_leap_before",
    lunarDate: "2020-3-20",
    lunarYear: 2020,
    lunarMonth: 3,
    lunarDay: 20,
    isLeapMonth: true,
    fixLeap: true,
    timeIndex: 2,
    birthTime: "yin",
    gender: "女",
    genderKey: "female"
  },
  {
    case: "2020_05_20_invalid_leap_after",
    lunarDate: "2020-5-20",
    lunarYear: 2020,
    lunarMonth: 5,
    lunarDay: 20,
    isLeapMonth: true,
    fixLeap: true,
    timeIndex: 6,
    birthTime: "wu",
    gender: "男",
    genderKey: "male"
  },
  {
    case: "2021_06_10_invalid_leap_no_leap_year",
    lunarDate: "2021-6-10",
    lunarYear: 2021,
    lunarMonth: 6,
    lunarDay: 10,
    isLeapMonth: true,
    fixLeap: true,
    timeIndex: 8,
    birthTime: "shen",
    gender: "女",
    genderKey: "female"
  }
];

const ALGORITHM = "default";

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
    // Birth-year ganzhi is derived by upstream and fed back to the Rust by_lunar
    // call; year-to-ganzhi derivation is otherwise deferred.
    birth_year_stem: requiredKey(STEM_KEYS, rawStem, "year stem"),
    raw_birth_year_stem: rawStem,
    birth_year_branch: requiredKey(BRANCH_KEYS, rawBranch, "year branch"),
    raw_birth_year_branch: rawBranch,
    language: "zh-CN"
  };
}

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
        "Supported-field-only byLunar leap-month fixture for iztro-rs. Cases use real 2020 闰四月 lunar dates and cover isLeapMonth and fixLeap toggles: before the leap month, the regular month with the leap-month number, both halves of the leap month, and a date after it. They also cover invalid leap requests where upstream ignores isLeapMonth because the requested month is not actually leap. The leap fourth month, day > 15 pair (fixLeap true vs false) is the discriminator. resolved_lunar records the lunar date upstream resolved to via lunar2solar. supported_fields normalizes life/body palace branches, five-element bureau, palace branch/stem/name facts, represented typed natal stars, and the four decorative runtime families. Birth-year ganzhi is taken from upstream and fed back to the Rust by_lunar call. Temporal flow stars, full facade serialization parity, rat-hour variants, horoscope palace-name derivation, temporal decorative arrays, features, rules, and narrative are excluded.",
      generation_command: GENERATION_COMMAND
    },
    cases: CASES.map(buildCase)
  };
}

const fixture = buildFixture();

if (process.argv.includes("--write")) {
  writeFileSync(
    join(REPO_ROOT, "fixtures", "iztro", "leap_month_by_lunar.json"),
    `${JSON.stringify(fixture, null, 2)}\n`
  );
  console.log("wrote fixtures/iztro/leap_month_by_lunar.json");
} else {
  console.log(JSON.stringify(fixture, null, 2));
}
