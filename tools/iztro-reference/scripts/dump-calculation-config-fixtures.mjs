// Dumps upstream iztro@2.5.8 supported-field fixture cases for calculation
// configuration switches: yearDivide, fixLeap, and ageDivide.
//
// Usage:
//   npm ci --prefix tools/iztro-reference
//   node tools/iztro-reference/scripts/dump-calculation-config-fixtures.mjs [--write]

import { astro } from "iztro";
import { lunar2solar } from "lunar-lite";
import { writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  BRANCH_KEYS,
  STEM_KEYS,
  PALACE_KEYS,
  MUTAGEN_KEYS,
  TYPED_STAR_KEYS,
  normalizeSupportedFields,
  requiredKey
} from "./lib/normalize.mjs";

const GENERATED_AT = "2026-06-24T00:00:00Z";
const GENERATION_COMMAND =
  "npm ci --prefix tools/iztro-reference && npm run dump:calculation-config --prefix tools/iztro-reference -- --write";
const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const LANGUAGE = "zh-CN";
const ALGORITHM = "default";
const MUTAGEN_ORDER = ["lu", "quan", "ke", "ji"];

function resetConfig(config = {}) {
  astro.config({
    algorithm: ALGORITHM,
    yearDivide: config.yearDivide ?? "normal",
    ageDivide: config.ageDivide ?? "normal",
    horoscopeDivide: "normal"
  });
}

function normalizeMutagenTransforms(mutagen) {
  const transforms = {};
  mutagen.forEach((star, i) => {
    transforms[MUTAGEN_ORDER[i]] = {
      transform: requiredKey(MUTAGEN_KEYS, ["禄", "权", "科", "忌"][i], "mutagen"),
      star: requiredKey(TYPED_STAR_KEYS, star, "mutagen star"),
      raw_star: star
    };
  });
  return transforms;
}

function normalizePalaceNames(names) {
  return names.map((name) => ({
    name: requiredKey(PALACE_KEYS, name, "palace"),
    raw_name: name
  }));
}

function normalizeAgeScope(age) {
  return {
    index: age.index,
    name: "age",
    raw_name: age.name,
    heavenly_stem: requiredKey(STEM_KEYS, age.heavenlyStem, "age stem"),
    raw_heavenly_stem: age.heavenlyStem,
    earthly_branch: requiredKey(BRANCH_KEYS, age.earthlyBranch, "age branch"),
    raw_earthly_branch: age.earthlyBranch,
    palace_names: normalizePalaceNames(age.palaceNames),
    mutagen: normalizeMutagenTransforms(age.mutagen),
    nominal_age: age.nominalAge
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

function normalizeNatalStructureFields(chart) {
  const full = normalizeSupportedFields(chart);
  return {
    life_palace_branch: full.life_palace_branch,
    body_palace_branch: full.body_palace_branch,
    five_element_bureau: full.five_element_bureau,
    palaces: full.palaces
  };
}

function solarInput(def) {
  return {
    calendar: "solar",
    solar_date: def.solarDate,
    solar_year: def.solarYear,
    solar_month: def.solarMonth,
    solar_day: def.solarDay,
    clock_hour: def.clockHour,
    iztro_time_index: def.timeIndex,
    gender: def.genderKey,
    iztro_gender: def.gender,
    fix_leap: def.fixLeap ?? true,
    language: LANGUAGE,
    calculation_config: {
      year_divide: def.yearDivide ?? "normal",
      fix_leap: def.fixLeap ?? true,
      age_divide: def.ageDivide ?? "normal"
    }
  };
}

function lunarInput(def) {
  return {
    calendar: "lunar",
    lunar_date: def.lunarDate,
    lunar_year: def.lunarYear,
    lunar_month: def.lunarMonth,
    lunar_day: def.lunarDay,
    is_leap_month: def.isLeapMonth,
    clock_hour: def.clockHour,
    iztro_time_index: def.timeIndex,
    gender: def.genderKey,
    iztro_gender: def.gender,
    fix_leap: def.fixLeap ?? true,
    language: LANGUAGE,
    calculation_config: {
      year_divide: def.yearDivide ?? "normal",
      fix_leap: def.fixLeap ?? true,
      age_divide: def.ageDivide ?? "normal"
    }
  };
}

const YEAR_CASES = [
  {
    case: "year_divide_normal_2000_02_04",
    solarDate: "2000-2-4",
    solarYear: 2000,
    solarMonth: 2,
    solarDay: 4,
    clockHour: 8,
    timeIndex: 4,
    gender: "女",
    genderKey: "female",
    yearDivide: "normal"
  },
  {
    case: "year_divide_exact_2000_02_04",
    solarDate: "2000-2-4",
    solarYear: 2000,
    solarMonth: 2,
    solarDay: 4,
    clockHour: 8,
    timeIndex: 4,
    gender: "女",
    genderKey: "female",
    yearDivide: "exact"
  }
];

const LEAP_CASES = [
  ["leap_day_1_mid_month", 1, true],
  ["leap_day_1_as_previous", 1, false],
  ["leap_day_15_mid_month", 15, true],
  ["leap_day_15_as_previous", 15, false],
  ["leap_day_16_mid_month", 16, true],
  ["leap_day_16_as_previous", 16, false],
  ["leap_final_day_mid_month", 29, true],
  ["leap_final_day_as_previous", 29, false],
  ["regular_month_mid_month", 16, true, false],
  ["regular_month_as_previous", 16, false, false]
].map(([label, day, fixLeap, isLeapMonth = true]) => ({
  case: label,
  lunarDate: `2020-4-${day}`,
  lunarYear: 2020,
  lunarMonth: 4,
  lunarDay: day,
  isLeapMonth,
  fixLeap,
  clockHour: 12,
  timeIndex: 6,
  gender: "男",
  genderKey: "male"
}));

function targetFromLunar(date, isLeapMonth = false) {
  return lunar2solar(date, isLeapMonth).toString();
}

const AGE_BIRTH = {
  lunarDate: "1990-5-17",
  lunarYear: 1990,
  lunarMonth: 5,
  lunarDay: 17,
  isLeapMonth: false,
  fixLeap: true,
  clockHour: 8,
  timeIndex: 4,
  gender: "女",
  genderKey: "female"
};

const AGE_CASES = [
  {
    case: "age_divide_birthday_before_birthday",
    ageDivide: "birthday",
    targetLunarDate: "2026-5-16"
  },
  {
    case: "age_divide_birthday_on_birthday",
    ageDivide: "birthday",
    targetLunarDate: "2026-5-17"
  },
  {
    case: "age_divide_birthday_same_month_after_birthday",
    ageDivide: "birthday",
    targetLunarDate: "2026-5-18"
  },
  {
    case: "age_divide_birthday_later_year_same_month_after_birthday",
    ageDivide: "birthday",
    targetLunarDate: "2027-5-18"
  },
  {
    case: "age_divide_birthday_later_month_after_birthday",
    ageDivide: "birthday",
    targetLunarDate: "2026-6-1"
  },
  {
    case: "age_divide_normal_before_birthday",
    ageDivide: "normal",
    targetLunarDate: "2026-5-16"
  }
].map((def) => ({
  ...AGE_BIRTH,
  ...def,
  targetSolarDate: targetFromLunar(def.targetLunarDate),
  targetTimeIndex: 2
}));

function buildChartCase(def) {
  resetConfig(def);
  const chart =
    def.calendar === "solar"
      ? astro.bySolar(def.solarDate, def.timeIndex, def.gender, def.fixLeap ?? true, LANGUAGE)
      : astro.byLunar(def.lunarDate, def.timeIndex, def.gender, def.isLeapMonth, def.fixLeap ?? true, LANGUAGE);

  return {
    case: def.case,
    kind: def.kind,
    algorithm: ALGORITHM,
    input: def.calendar === "solar" ? solarInput(def) : lunarInput(def),
    converted_lunar: def.calendar === "solar" ? normalizeConvertedLunar(chart) : undefined,
    supported_fields:
      def.kind === "year_divide"
        ? normalizeNatalStructureFields(chart)
        : {
            gender: def.genderKey,
            ...normalizeSupportedFields(chart)
          }
  };
}

function buildAgeCase(def) {
  resetConfig(def);
  const chart = astro.byLunar(
    def.lunarDate,
    def.timeIndex,
    def.gender,
    def.isLeapMonth,
    def.fixLeap,
    LANGUAGE
  );
  const horoscope = chart.horoscope(def.targetSolarDate, def.targetTimeIndex);

  return {
    case: def.case,
    kind: "age_divide",
    algorithm: ALGORITHM,
    input: {
      ...lunarInput(def),
      calculation_config: {
        year_divide: "normal",
        fix_leap: def.fixLeap,
        age_divide: def.ageDivide
      },
      target: {
        lunar_date: def.targetLunarDate,
        solar_date: def.targetSolarDate,
        time_index: def.targetTimeIndex
      }
    },
    supported_fields: {
      age: normalizeAgeScope(horoscope.age)
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
        "Supported-field-only calculation configuration fixture for iztro-rs. Cases cover yearDivide normal/exact for converted birth-year and palace facts, fixLeap false/true for leap-month chart facts, and ageDivide normal/birthday for horoscope nominal-age resolution. It is not a full upstream semantic clone fixture.",
      generation_command: GENERATION_COMMAND
    },
    cases: [
      ...YEAR_CASES.map((def) => buildChartCase({ ...def, kind: "year_divide", calendar: "solar" })),
      ...LEAP_CASES.map((def) => buildChartCase({ ...def, kind: "fix_leap", calendar: "lunar" })),
      ...AGE_CASES.map(buildAgeCase)
    ]
  };
}

const fixture = buildFixture();

if (process.argv.includes("--write")) {
  writeFileSync(
    join(REPO_ROOT, "crates", "iztro", "fixtures", "iztro", "calculation_config.json"),
    `${JSON.stringify(fixture, null, 2)}\n`
  );
  console.log("wrote crates/iztro/fixtures/iztro/calculation_config.json");
} else {
  console.log(JSON.stringify(fixture, null, 2));
}
