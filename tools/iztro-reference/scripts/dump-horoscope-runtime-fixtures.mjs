// Dumps upstream iztro@2.5.8 horoscope runtime-helper reference fixtures.
//
// This fixture covers the model-level behavior behind FunctionalHoroscope
// helpers: agePalace, palace, surroundPalaces, hasHoroscopeStars,
// notHaveHoroscopeStars, hasOneOfHoroscopeStars, and hasHoroscopeMutagen.
//
// Usage:
//   npm ci --prefix tools/iztro-reference
//   node tools/iztro-reference/scripts/dump-horoscope-runtime-fixtures.mjs [--write]

import { astro } from "iztro";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import {
  BOSHI_KEYS,
  BRANCH_KEYS,
  CHANGSHENG_KEYS,
  JIANGQIAN_KEYS,
  MUTAGEN_KEYS,
  PALACE_KEYS,
  SUIQIAN_KEYS,
  TYPED_STAR_KEYS,
  STEM_KEYS,
  requiredKey
} from "./lib/normalize.mjs";

const TARGET = "iztro@2.5.8";
const GENERATED_AT = "2026-06-16T00:00:00Z";
const GENERATION_COMMAND =
  "npm ci --prefix tools/iztro-reference && node tools/iztro-reference/scripts/dump-horoscope-runtime-fixtures.mjs --write";

const SCOPES = ["origin", "age", "decadal", "yearly", "monthly", "daily", "hourly"];
const TEMPORAL_SCOPES = ["age", "decadal", "yearly", "monthly", "daily", "hourly"];
const FLOW_SCOPES = ["decadal", "yearly", "monthly", "daily", "hourly"];
const MUTAGENS = ["禄", "权", "科", "忌"];
const BRANCH_BY_PALACE_INDEX = ["寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥", "子", "丑"];

const SCOPE_PREFIX = new Map([
  ["decadal", "运"],
  ["yearly", "流"],
  ["monthly", "月"],
  ["daily", "日"],
  ["hourly", "时"]
]);

const FLOW_BASE_KEYS = new Map([
  ["魁", "kui"],
  ["钺", "yue"],
  ["昌", "chang"],
  ["曲", "qu"],
  ["禄", "lu"],
  ["羊", "yang"],
  ["陀", "tuo"],
  ["马", "ma"],
  ["鸾", "luan"],
  ["喜", "xi"]
]);

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

function branchKeyForPalaceIndex(index) {
  return requiredKey(BRANCH_KEYS, BRANCH_BY_PALACE_INDEX[index], "branch");
}

function palaceNameForScope(horoscope, scope, palaceIndex) {
  if (scope === "origin") {
    return requiredKey(PALACE_KEYS, horoscope.astrolabe.palace(palaceIndex).name, "palace");
  }
  return requiredKey(PALACE_KEYS, horoscope[scope].palaceNames[palaceIndex], "palace");
}

function normalizeRawStars(stars) {
  return stars.map((star) => requiredKey(TYPED_STAR_KEYS, star.name, "typed star"));
}

function normalizeDecorativeStars(palace) {
  return [
    requiredKey(CHANGSHENG_KEYS, palace.changsheng12, "changsheng12"),
    requiredKey(BOSHI_KEYS, palace.boshi12, "boshi12"),
    requiredKey(SUIQIAN_KEYS, palace.suiqian12, "suiqian12"),
    requiredKey(JIANGQIAN_KEYS, palace.jiangqian12, "jiangqian12")
  ];
}

function normalizeScopeStars(scope, starsByIndex, palaceIndex) {
  if (!FLOW_SCOPES.includes(scope)) {
    return [];
  }
  const prefix = SCOPE_PREFIX.get(scope);
  return starsByIndex[palaceIndex].map((star) => {
    if (scope === "yearly" && star.name === "年解") {
      return "nian_jie_yearly";
    }
    if (star.name[0] !== prefix) {
      throw new Error(`Unexpected ${scope} flow star ${star.name}`);
    }
    const base = FLOW_BASE_KEYS.get(star.name.slice(1));
    if (!base) {
      throw new Error(`Unsupported ${scope} flow star ${star.name}`);
    }
    return `${scope === "decadal" ? "yun" : scope === "yearly" ? "liu" : scope === "monthly" ? "yue" : scope === "daily" ? "ri" : "shi"}_${base}`;
  });
}

function normalizeYearlyDecStars(horoscope, palaceIndex) {
  const yearly = horoscope.yearly.yearlyDecStar;
  return [
    requiredKey(SUIQIAN_KEYS, yearly.suiqian12[palaceIndex], "suiqian12"),
    requiredKey(JIANGQIAN_KEYS, yearly.jiangqian12[palaceIndex], "jiangqian12")
  ];
}

function mutagenActivationsForProjection(horoscope, scope, palace) {
  if (scope === "origin") {
    return [];
  }
  const mutagens = horoscope[scope].mutagen;
  return mutagens.flatMap((starName, index) => {
    const star = horoscope.astrolabe.star(starName);
    if (star.palace().earthlyBranch !== palace.earthlyBranch) {
      return [];
    }
    return [{
      target_star: requiredKey(TYPED_STAR_KEYS, starName, "mutagen star"),
      mutagen: requiredKey(MUTAGEN_KEYS, MUTAGENS[index], "mutagen")
    }];
  });
}

function projectionFromPalace(horoscope, scope, palaceName, palace) {
  const palaceIndex = palace.index;
  return {
    scope,
    requested_palace_name: requiredKey(PALACE_KEYS, palaceName, "palace"),
    branch: requiredKey(BRANCH_KEYS, palace.earthlyBranch, "branch"),
    natal_palace_name: requiredKey(PALACE_KEYS, palace.name, "palace"),
    temporal_palace_name: scope === "origin" ? null : palaceNameForScope(horoscope, scope, palaceIndex),
    natal_palace_stem: requiredKey(STEM_KEYS, palace.heavenlyStem, "stem"),
    natal_typed_stars: normalizeRawStars([
      ...palace.majorStars,
      ...palace.minorStars,
      ...palace.adjectiveStars
    ]),
    natal_decorative_stars: normalizeDecorativeStars(palace),
    temporal_stars: normalizeScopeStars(scope, horoscope[scope]?.stars ?? [], palaceIndex),
    temporal_decorative_stars: scope === "yearly" ? normalizeYearlyDecStars(horoscope, palaceIndex) : [],
    temporal_mutagen_activations: mutagenActivationsForProjection(horoscope, scope, palace)
  };
}

function projection(horoscope, scope, palaceName) {
  const palace = scope === "age"
    ? horoscope.agePalace()
    : horoscope.palace(palaceName, scope === "origin" ? "origin" : scope);
  return projectionFromPalace(horoscope, scope, palaceName, palace);
}

function surroundProjection(horoscope, scope, palaceName) {
  const surrounded = horoscope.surroundPalaces(palaceName, scope === "origin" ? "origin" : scope);
  return {
    scope,
    requested_palace_name: requiredKey(PALACE_KEYS, palaceName, "palace"),
    target: projectionFromPalace(horoscope, scope, palaceName, surrounded.target),
    opposite: projectionFromPalace(horoscope, scope, palaceName, surrounded.opposite),
    wealth: projectionFromPalace(horoscope, scope, palaceName, surrounded.wealth),
    career: projectionFromPalace(horoscope, scope, palaceName, surrounded.career)
  };
}

function mergedDecadalYearlyStarsByIndex(horoscope, palaceIndex) {
  return [
    ...horoscope.decadal.stars[palaceIndex],
    ...horoscope.yearly.stars[palaceIndex]
  ].map((star) => star.name);
}

function firstAbsentStar(present) {
  for (const candidate of ["运禄", "流禄", "年解", "月禄", "岁建"]) {
    if (!present.includes(candidate)) {
      return candidate;
    }
  }
  throw new Error("expected at least one absent probe star");
}

function normalizeQueryStar(star) {
  if (star === "岁建") {
    return "sui_jian";
  }
  if (star === "年解") {
    return "nian_jie_yearly";
  }
  return normalizeScopeStars(
    star[0] === "运" ? "decadal" : star[0] === "流" ? "yearly" : "monthly",
    [[{ name: star }]],
    0
  )[0];
}

function starQuery(horoscope, helper, scope, palaceName, stars) {
  const fn = horoscope[helper].bind(horoscope);
  return {
    helper,
    scope,
    palace_name: requiredKey(PALACE_KEYS, palaceName, "palace"),
    stars: stars.map(normalizeQueryStar),
    expected: fn(palaceName, scope, stars)
  };
}

function mutagenQuery(horoscope, scope, palaceName, mutagen) {
  return {
    scope,
    palace_name: requiredKey(PALACE_KEYS, palaceName, "palace"),
    mutagen: requiredKey(MUTAGEN_KEYS, mutagen, "mutagen"),
    expected: horoscope.hasHoroscopeMutagen(palaceName, scope, mutagen)
  };
}

function runtimeQueries(horoscope) {
  const queries = [];
  const palaceNames = horoscope.decadal.palaceNames;
  const positiveIndex = palaceNames.findIndex((_, index) => mergedDecadalYearlyStarsByIndex(horoscope, index).length > 0);
  const positivePalaceName = palaceNames[positiveIndex];
  const present = mergedDecadalYearlyStarsByIndex(horoscope, positiveIndex);
  const absent = firstAbsentStar(present);

  queries.push(starQuery(horoscope, "hasHoroscopeStars", "decadal", positivePalaceName, [present[0]]));
  queries.push(starQuery(horoscope, "hasHoroscopeStars", "decadal", positivePalaceName, [present[0], absent]));
  queries.push(starQuery(horoscope, "notHaveHoroscopeStars", "decadal", positivePalaceName, [absent]));
  queries.push(starQuery(horoscope, "notHaveHoroscopeStars", "decadal", positivePalaceName, [present[0], absent]));
  queries.push(starQuery(horoscope, "hasOneOfHoroscopeStars", "decadal", positivePalaceName, [present[0], absent]));
  queries.push(starQuery(horoscope, "hasOneOfHoroscopeStars", "decadal", positivePalaceName, [absent]));
  queries.push(starQuery(horoscope, "hasHoroscopeStars", "decadal", positivePalaceName, []));
  queries.push(starQuery(horoscope, "notHaveHoroscopeStars", "decadal", positivePalaceName, []));
  queries.push(starQuery(horoscope, "hasOneOfHoroscopeStars", "decadal", positivePalaceName, []));
  queries.push(starQuery(horoscope, "hasHoroscopeStars", "monthly", horoscope.monthly.palaceNames[0], ["月禄"]));
  queries.push(starQuery(horoscope, "hasHoroscopeStars", "yearly", horoscope.yearly.palaceNames[0], ["岁建"]));

  return queries;
}

function runtimeMutagenQueries(horoscope) {
  const out = [mutagenQuery(horoscope, "origin", "命宫", "禄")];
  for (const scope of TEMPORAL_SCOPES) {
    for (const palaceName of horoscope[scope].palaceNames) {
      for (const mutagen of MUTAGENS) {
        const query = mutagenQuery(horoscope, scope, palaceName, mutagen);
        if (query.expected && !out.some((entry) => entry.expected)) {
          out.push(query);
        }
        if (!query.expected && out.filter((entry) => entry.scope === scope && !entry.expected).length === 0) {
          out.push(query);
        }
      }
    }
  }
  return out;
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
    runtime: {
      age_palace: projection(horoscope, "age", horoscope.age.palaceNames[horoscope.age.index]),
      palace_projections: [
        projection(horoscope, "origin", "命宫"),
        projection(horoscope, "decadal", "命宫"),
        projection(horoscope, "yearly", "官禄"),
        projection(horoscope, "monthly", "命宫"),
        projection(horoscope, "daily", "财帛"),
        projection(horoscope, "hourly", "父母")
      ],
      surround_palaces: [
        surroundProjection(horoscope, "origin", "命宫"),
        surroundProjection(horoscope, "decadal", "命宫"),
        surroundProjection(horoscope, "yearly", "官禄")
      ],
      star_queries: runtimeQueries(horoscope),
      mutagen_queries: runtimeMutagenQueries(horoscope)
    }
  };
}

const fixture = {
  target: TARGET,
  description:
    "Upstream iztro@2.5.8 horoscope runtime-helper fixture. Projections preserve branch-based natal palace identity while adding temporal labels and already-modeled temporal facts. Query helper cases encode upstream behavior: star helpers search only merged decadal+yearly flow-star matrices, while hasHoroscopeMutagen checks the selected scope's mutagen target against natal major/minor stars in the projected palace.",
  generated_at: GENERATED_AT,
  generation_command: GENERATION_COMMAND,
  cases: CASE_DEFS.map(buildCase)
};

if (process.argv.includes("--write")) {
  const outPath = fileURLToPath(new URL("../../../crates/iztro/fixtures/iztro/horoscope_runtime.json", import.meta.url));
  writeFileSync(outPath, `${JSON.stringify(fixture, null, 2)}\n`);
  console.log("wrote crates/iztro/fixtures/iztro/horoscope_runtime.json");
} else {
  console.log(JSON.stringify(fixture, null, 2));
}
