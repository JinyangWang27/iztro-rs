// Dumps upstream iztro@2.5.8 horoscope facade reference fixtures.
//
// This fixture captures the serializable shape iztro-rs exposes through
// HoroscopeFacadeSnapshot: the modeled full-horoscope surface combined into one
// payload that moves toward the upstream FunctionalAstrolabe#horoscope shape.
//
// It records only the in-scope, already-modeled facts:
//   * context: the numeric target solar date, numeric target lunar date, target
//     leap-month flag, and target timeIndex retained by full stack assembly.
//     Localized upstream lunarDate/solarDate strings remain deferred.
//   * age_palace / palace_projections / surround_palaces: the runtime palace
//     projections for the Life palace (命宫) across each modeled scope, reusing
//     the shared projection helpers that back the runtime fixture.
//
// The decadal/age/yearly/monthly/daily/hourly supported-field blocks are NOT
// duplicated here: HoroscopeFacadeSnapshot reuses HoroscopeSupportedFieldsSnapshot
// for them, and the facade test asserts that reuse against horoscope.json.
//
// Usage:
//   npm ci --prefix tools/iztro-reference
//   node tools/iztro-reference/scripts/dump-horoscope-facade-fixtures.mjs [--write]

import { astro } from "iztro";
import { solar2lunar } from "lunar-lite";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { projection, surroundProjection } from "./lib/horoscope-projection.mjs";

const TARGET = "iztro@2.5.8";
const GENERATED_AT = "2026-06-17T00:00:00Z";
const GENERATION_COMMAND =
  "npm ci --prefix tools/iztro-reference && node tools/iztro-reference/scripts/dump-horoscope-facade-fixtures.mjs --write";

// The Life palace is the canonical anchor; the facade projects it in every
// modeled scope. `age` is projected separately through agePalace(), mirroring the
// upstream split between agePalace() and palace(name, scope).
const LIFE_PALACE = "命宫";
const PROJECTION_SCOPES = ["origin", "decadal", "yearly", "monthly", "daily", "hourly"];

const CASE_DEFS = [
  {
    id: "canonical_female_default_2026",
    lunarDate: "1990-5-17",
    year: 1990,
    month: 5,
    day: 17,
    timeIndex: 4,
    gender: "女",
    genderKey: "female",
    isLeapMonth: false,
    fixLeap: true,
    language: "zh-CN",
    algorithm: "default",
    targetSolarDate: "2026-7-1",
    targetYear: 2026,
    targetTimeIndex: 2
  },
  {
    id: "canonical_female_default_2034_decade_boundary",
    lunarDate: "1990-5-17",
    year: 1990,
    month: 5,
    day: 17,
    timeIndex: 4,
    gender: "女",
    genderKey: "female",
    isLeapMonth: false,
    fixLeap: true,
    language: "zh-CN",
    algorithm: "default",
    targetSolarDate: "2034-7-1",
    targetYear: 2034,
    targetTimeIndex: 2
  },
  {
    id: "canonical_female_zhongzhou_2026",
    lunarDate: "1990-5-17",
    year: 1990,
    month: 5,
    day: 17,
    timeIndex: 4,
    gender: "女",
    genderKey: "female",
    isLeapMonth: false,
    fixLeap: true,
    language: "zh-CN",
    algorithm: "zhongzhou",
    targetSolarDate: "2026-7-1",
    targetYear: 2026,
    targetTimeIndex: 2
  },
  {
    id: "male_1988_default_2026",
    lunarDate: "1988-3-14",
    year: 1988,
    month: 3,
    day: 14,
    timeIndex: 0,
    gender: "男",
    genderKey: "male",
    isLeapMonth: false,
    fixLeap: true,
    language: "zh-CN",
    algorithm: "default",
    targetSolarDate: "2026-7-1",
    targetYear: 2026,
    targetTimeIndex: 2
  }
];

function parseSolarDate(targetSolarDate) {
  const [year, month, day] = targetSolarDate.split("-").map((part) => Number.parseInt(part, 10));
  return { year, month, day };
}

function targetContext(def) {
  const lunar = solar2lunar(def.targetSolarDate);
  return {
    solar_date: parseSolarDate(def.targetSolarDate),
    lunar_date: {
      year: lunar.lunarYear,
      month: lunar.lunarMonth,
      day: lunar.lunarDay,
      is_leap_month: lunar.isLeap
    },
    time_index: def.targetTimeIndex
  };
}

function buildCase(def) {
  astro.config({ algorithm: def.algorithm });
  const chart = astro.byLunar(
    def.lunarDate,
    def.timeIndex,
    def.gender,
    def.isLeapMonth,
    def.fixLeap,
    def.language
  );
  const horoscope = chart.horoscope(def.targetSolarDate, def.targetTimeIndex);

  return {
    id: def.id,
    input: {
      calendar: "lunar",
      year: def.year,
      month: def.month,
      day: def.day,
      time_index: def.timeIndex,
      gender: def.genderKey,
      is_leap_month: def.isLeapMonth,
      fix_leap: def.fixLeap,
      language: def.language,
      algorithm: def.algorithm,
      target: {
        solar_date: def.targetSolarDate,
        year: def.targetYear,
        time_index: def.targetTimeIndex
      }
    },
    facade: {
      context: targetContext(def),
      age_palace: projection(horoscope, "age", LIFE_PALACE),
      palace_projections: PROJECTION_SCOPES.map((scope) => projection(horoscope, scope, LIFE_PALACE)),
      surround_palaces: PROJECTION_SCOPES.map((scope) => surroundProjection(horoscope, scope, LIFE_PALACE))
    }
  };
}

const fixture = {
  target: TARGET,
  description:
    "Upstream iztro@2.5.8 horoscope facade reference. Captures the serializable " +
    "HoroscopeFacadeSnapshot surface: the numeric target solar/lunar/time context " +
    "retained by the modeled full horoscope stack, plus the Life-palace (命宫) runtime projections " +
    "(agePalace, palace, surroundPalaces) across each modeled scope. The " +
    "decadal/age/yearly/monthly/daily/hourly supported-field blocks are reused " +
    "from horoscope.json and are not duplicated here. The localized lunarDate " +
    "and solarDate strings, plus the re-embedded " +
    "natal astrolabe remain deferred.",
  generated_at: GENERATED_AT,
  generation_command: GENERATION_COMMAND,
  deferred: [
    "lunarDate (localized string) / solarDate (localized/string facade field)",
    "astrolabe (the full natal chart re-embedded in the horoscope result)",
    "hasHoroscopeStars / notHaveHoroscopeStars / hasOneOfHoroscopeStars / hasHoroscopeMutagen (query helpers)"
  ],
  cases: CASE_DEFS.map(buildCase)
};

if (process.argv.includes("--write")) {
  const outPath = fileURLToPath(new URL("../../../crates/iztro/fixtures/iztro/horoscope_facade.json", import.meta.url));
  writeFileSync(outPath, `${JSON.stringify(fixture, null, 2)}\n`);
  console.log("wrote crates/iztro/fixtures/iztro/horoscope_facade.json");
} else {
  console.log(JSON.stringify(fixture, null, 2));
}
