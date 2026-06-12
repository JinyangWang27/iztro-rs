// Dumps upstream iztro@2.5.8 full-horoscope reference fixtures.
//
// This is a tooling-only reference generator. It inspects the public iztro
// horoscope API (FunctionalAstrolabe#horoscope) and records a stable fixture
// contract that later iztro-rs horoscope-assembly PRs can target. It does NOT
// implement or claim Rust parity.
//
// Upstream API surface (discovered, iztro@2.5.8):
//   const chart = astro.byLunar(date, timeIndex, gender, fixLeap, fixLeap, lang);
//   const h = chart.horoscope(targetSolarDate, targetTimeIndex);
//   h => { lunarDate, solarDate, decadal, age, yearly, monthly, daily, hourly,
//          agePalace, palace, surroundPalaces, astrolabe, <boolean helpers> }
//   Each temporal scope (decadal/yearly/monthly/daily/hourly) =>
//     { index, name, heavenlyStem, earthlyBranch, palaceNames, mutagen, stars }
//   age => same minus `stars`, plus `nominalAge` (小限).
//   yearly => additionally exposes `yearlyDecStar` { suiqian12, jiangqian12 }.
//   `palaceNames` is the 12-palace layout rotated for the period, index 0 = 寅.
//   `mutagen` is the four-transform [禄, 权, 科, 忌] star names for the period.
//   `stars` is the per-palace flow-star (流耀) matrix, index 0 = 寅.
//
// Usage:
//   npm ci --prefix tools/iztro-reference
//   node tools/iztro-reference/scripts/dump-horoscope-fixtures.mjs [--write]
//
// Without --write the fixture is printed to stdout for inspection. With --write
// it is written to fixtures/iztro/horoscope.json.

import { astro } from "iztro";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import {
  BRANCH_KEYS,
  STEM_KEYS,
  PALACE_KEYS,
  MUTAGEN_KEYS,
  TYPED_STAR_KEYS,
  SUIQIAN_KEYS,
  JIANGQIAN_KEYS,
  requiredKey
} from "./lib/normalize.mjs";

const TARGET = "iztro@2.5.8";
const GENERATED_AT = "2026-06-12T00:00:00Z";
const GENERATION_COMMAND =
  "npm ci --prefix tools/iztro-reference && node tools/iztro-reference/scripts/dump-horoscope-fixtures.mjs --write";

// Period scopes that carry a flow-star (流耀) matrix.
const FLOW_SCOPES = ["decadal", "yearly", "monthly", "daily", "hourly"];
// All temporal scopes returned by horoscope(); `age` (小限) carries no matrix.
const SCOPES = ["decadal", "age", "yearly", "monthly", "daily", "hourly"];

// Scope-name prefix character on flow-star names (运/流/月/日/时); the remaining
// character is the base, which we normalize via FLOW_BASE_KEYS below.
const SCOPE_PREFIX = new Map([
  ["decadal", "运"],
  ["yearly", "流"],
  ["monthly", "月"],
  ["daily", "日"],
  ["hourly", "时"]
]);

// Flow-star base, taken from the second character of a flow-star name. Mirrors
// the suffix map in dump-runtime-star-families.mjs.
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

// Earthly branches in palace-index order; index 0 = 寅.
const BRANCH_BY_PALACE_INDEX = ["寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥", "子", "丑"];

// Mutagen array order is fixed by upstream as [禄, 权, 科, 忌].
const MUTAGEN_ORDER = ["lu", "quan", "ke", "ji"];

// Upstream keys we expect per scope. We fail loudly on anything unexpected so a
// future iztro bump that adds/renames horoscope fields cannot be silently
// dropped from the fixture.
const KNOWN_SCOPE_KEYS = {
  decadal: ["index", "name", "heavenlyStem", "earthlyBranch", "palaceNames", "mutagen", "stars"],
  age: ["index", "nominalAge", "name", "heavenlyStem", "earthlyBranch", "palaceNames", "mutagen"],
  yearly: ["index", "name", "heavenlyStem", "earthlyBranch", "palaceNames", "mutagen", "stars", "yearlyDecStar"],
  monthly: ["index", "name", "heavenlyStem", "earthlyBranch", "palaceNames", "mutagen", "stars"],
  daily: ["index", "name", "heavenlyStem", "earthlyBranch", "palaceNames", "mutagen", "stars"],
  hourly: ["index", "name", "heavenlyStem", "earthlyBranch", "palaceNames", "mutagen", "stars"]
};

function assertExactKeys(obj, expected, label) {
  const actual = Object.keys(obj);
  const missing = expected.filter((key) => !(key in obj));
  const extra = actual.filter((key) => !expected.includes(key));
  if (missing.length || extra.length) {
    throw new Error(
      `Unexpected ${label} shape. missing=[${missing.join(", ")}] extra=[${extra.join(", ")}]`
    );
  }
}

function branchKeyForPalaceIndex(index) {
  const raw = BRANCH_BY_PALACE_INDEX[index];
  if (!raw) {
    throw new Error(`Palace index out of range: ${index}`);
  }
  return requiredKey(BRANCH_KEYS, raw, "branch");
}

function normalizePalaceNames(names) {
  return names.map((name) => ({
    name: requiredKey(PALACE_KEYS, name, "palace"),
    raw_name: name
  }));
}

// Four-transform stars: positional [禄, 权, 科, 忌].
function normalizeMutagenTransforms(mutagen) {
  if (!Array.isArray(mutagen) || mutagen.length !== MUTAGEN_ORDER.length) {
    throw new Error(`Unexpected mutagen array: ${JSON.stringify(mutagen)}`);
  }
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

// Per-palace flow-star matrix -> flat sorted [{ base, branch, type }]. The
// yearly scope additionally emits 年解 (a helper, not a flow base); it is
// pulled out into nian_jie_branch like dump-runtime-star-families.mjs.
function normalizeFlowStars(scope, palaceStars) {
  const prefix = SCOPE_PREFIX.get(scope);
  const matrix = [];
  let nianJieBranch = null;

  palaceStars.forEach((cell, index) => {
    const branch = branchKeyForPalaceIndex(index);
    for (const star of cell) {
      if (star.name === "年解") {
        nianJieBranch = branch;
        continue;
      }
      if (star.name[0] !== prefix) {
        throw new Error(`Flow-star ${star.name} does not match ${scope} prefix ${prefix}`);
      }
      const base = FLOW_BASE_KEYS.get(star.name.slice(1));
      if (!base) {
        throw new Error(`Unsupported flow-star name: ${star.name}`);
      }
      matrix.push({ base, branch, type: star.type, raw_name: star.name });
    }
  });

  matrix.sort((a, b) => a.base.localeCompare(b.base) || a.branch.localeCompare(b.branch));
  return { matrix, nianJieBranch };
}

function normalizeDecStarFamily(names, map, family) {
  if (!Array.isArray(names) || names.length !== BRANCH_BY_PALACE_INDEX.length) {
    throw new Error(`Unexpected ${family} array: ${JSON.stringify(names)}`);
  }
  return names.map((name, index) => ({
    name: requiredKey(map, name, family),
    raw_name: name,
    branch: branchKeyForPalaceIndex(index)
  }));
}

function normalizeScope(scope, data) {
  assertExactKeys(data, KNOWN_SCOPE_KEYS[scope], `${scope} scope`);

  const out = {
    index: data.index,
    name: scope,
    raw_name: data.name,
    heavenly_stem: requiredKey(STEM_KEYS, data.heavenlyStem, "stem"),
    raw_heavenly_stem: data.heavenlyStem,
    earthly_branch: requiredKey(BRANCH_KEYS, data.earthlyBranch, "branch"),
    raw_earthly_branch: data.earthlyBranch,
    palace_names: normalizePalaceNames(data.palaceNames),
    mutagen: normalizeMutagenTransforms(data.mutagen)
  };

  if (scope === "age") {
    out.nominal_age = data.nominalAge;
  }

  if (FLOW_SCOPES.includes(scope)) {
    const { matrix, nianJieBranch } = normalizeFlowStars(scope, data.stars);
    out.flow_stars = matrix;
    if (scope === "yearly") {
      out.nian_jie_branch = nianJieBranch;
      out.yearly_dec_stars = {
        suiqian12: normalizeDecStarFamily(data.yearlyDecStar.suiqian12, SUIQIAN_KEYS, "suiqian12"),
        jiangqian12: normalizeDecStarFamily(data.yearlyDecStar.jiangqian12, JIANGQIAN_KEYS, "jiangqian12")
      };
    }
  }

  return out;
}

const TOP_LEVEL_KNOWN = [
  "agePalace",
  "palace",
  "surroundPalaces",
  "hasHoroscopeStars",
  "notHaveHoroscopeStars",
  "hasOneOfHoroscopeStars",
  "hasHoroscopeMutagen",
  "lunarDate",
  "solarDate",
  "decadal",
  "age",
  "yearly",
  "monthly",
  "daily",
  "hourly",
  "astrolabe"
];

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

  if (typeof chart.horoscope !== "function") {
    throw new Error("Installed iztro chart does not expose horoscope()");
  }

  const h = chart.horoscope(def.targetSolarDate, def.targetTimeIndex);

  // Fail loudly if upstream grows/renames a top-level horoscope field.
  const extraTop = Object.keys(h).filter((key) => !TOP_LEVEL_KNOWN.includes(key));
  if (extraTop.length) {
    throw new Error(`Unexpected top-level horoscope keys: ${extraTop.join(", ")}`);
  }
  for (const scope of SCOPES) {
    if (!h[scope]) {
      throw new Error(`Missing horoscope scope: ${scope}`);
    }
  }

  const supported = {
    lunar_date: h.lunarDate,
    solar_date: h.solarDate
  };
  for (const scope of SCOPES) {
    supported[scope] = normalizeScope(scope, h[scope]);
  }

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
    raw_keys: {
      horoscope: TOP_LEVEL_KNOWN,
      ...Object.fromEntries(SCOPES.map((scope) => [scope, Object.keys(h[scope])]))
    },
    supported_fields: supported
  };
}

// Small, representative matrix (not exhaustive):
//   * canonical lunar female chart, default algorithm, mid-decade target 2026;
//   * same chart at target 2034, the start of the next 大限 (decade boundary);
//   * same chart under the Zhongzhou algorithm at 2026 (algorithm selection);
//   * a male chart, default algorithm, target 2026.
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

const fixture = {
  target: TARGET,
  description:
    "Upstream iztro@2.5.8 full-horoscope reference. Captures the decadal (大限), " +
    "age (小限), yearly (流年), monthly (流月), daily (流日) and hourly (流时) " +
    "frames returned by FunctionalAstrolabe#horoscope: each frame's index, period " +
    "stem/branch, 12-palace layout, four-transform (四化) stars and flow-star (流耀) " +
    "matrix, plus the yearly 年解 and yearlyDecStar (岁前/将前) families. Raw upstream " +
    "labels are preserved beside normalized snake_case keys. Tooling-only fixture " +
    "contract; no Rust parity is claimed. Narrative text and the re-embedded natal " +
    "astrolabe are intentionally excluded.",
  generated_at: GENERATED_AT,
  generation_command: GENERATION_COMMAND,
  not_normalized_yet: [
    "agePalace / palace / surroundPalaces (runtime palace projections)",
    "astrolabe (the full natal chart re-embedded in the horoscope result)",
    "hasHoroscopeStars / notHaveHoroscopeStars / hasOneOfHoroscopeStars / hasHoroscopeMutagen (query helpers)"
  ],
  cases: CASE_DEFS.map(buildCase)
};

if (process.argv.includes("--write")) {
  // Resolve relative to this script so the write target is the same whether the
  // dumper runs from the repo root (`node tools/.../dump-horoscope-fixtures.mjs`)
  // or under `npm run --prefix` (cwd = tools/iztro-reference).
  const outPath = fileURLToPath(new URL("../../../fixtures/iztro/horoscope.json", import.meta.url));
  writeFileSync(outPath, `${JSON.stringify(fixture, null, 2)}\n`);
  console.log("wrote fixtures/iztro/horoscope.json");
} else {
  console.log(JSON.stringify(fixture, null, 2));
}
