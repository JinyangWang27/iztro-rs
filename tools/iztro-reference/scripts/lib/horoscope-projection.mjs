// Shared upstream iztro@2.5.8 horoscope palace-projection helpers.
//
// These build the normalized branch-based palace projections (agePalace, palace,
// surroundPalaces) the runtime-helper and facade fixtures both record. They
// preserve natal palace identity by branch while layering the period's temporal
// palace name, flow stars, decorative stars, and mutagen activations on top.
//
// Factored out of dump-horoscope-runtime-fixtures.mjs so the facade fixture can
// reuse the exact same projection shape without duplicating the logic.

import {
  BRANCH_KEYS,
  BOSHI_KEYS,
  CHANGSHENG_KEYS,
  JIANGQIAN_KEYS,
  MUTAGEN_KEYS,
  PALACE_KEYS,
  SUIQIAN_KEYS,
  TYPED_STAR_KEYS,
  STEM_KEYS,
  requiredKey
} from "./normalize.mjs";

// Period scopes that carry a flow-star (流耀) matrix.
export const FLOW_SCOPES = ["decadal", "yearly", "monthly", "daily", "hourly"];

// Four-transform array order is fixed by upstream as [禄, 权, 科, 忌].
export const MUTAGENS = ["禄", "权", "科", "忌"];

// Earthly branches in palace-index order; index 0 = 寅.
export const BRANCH_BY_PALACE_INDEX = ["寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥", "子", "丑"];

// Scope-name prefix character on flow-star names (运/流/月/日/时).
export const SCOPE_PREFIX = new Map([
  ["decadal", "运"],
  ["yearly", "流"],
  ["monthly", "月"],
  ["daily", "日"],
  ["hourly", "时"]
]);

// Flow-star base, taken from the second character of a flow-star name.
export const FLOW_BASE_KEYS = new Map([
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

export function branchKeyForPalaceIndex(index) {
  return requiredKey(BRANCH_KEYS, BRANCH_BY_PALACE_INDEX[index], "branch");
}

export function palaceNameForScope(horoscope, scope, palaceIndex) {
  if (scope === "origin") {
    return requiredKey(PALACE_KEYS, horoscope.astrolabe.palace(palaceIndex).name, "palace");
  }
  return requiredKey(PALACE_KEYS, horoscope[scope].palaceNames[palaceIndex], "palace");
}

export function normalizeRawStars(stars) {
  return stars.map((star) => requiredKey(TYPED_STAR_KEYS, star.name, "typed star"));
}

export function normalizeDecorativeStars(palace) {
  return [
    requiredKey(CHANGSHENG_KEYS, palace.changsheng12, "changsheng12"),
    requiredKey(BOSHI_KEYS, palace.boshi12, "boshi12"),
    requiredKey(SUIQIAN_KEYS, palace.suiqian12, "suiqian12"),
    requiredKey(JIANGQIAN_KEYS, palace.jiangqian12, "jiangqian12")
  ];
}

export function normalizeScopeStars(scope, starsByIndex, palaceIndex) {
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

export function normalizeYearlyDecStars(horoscope, palaceIndex) {
  const yearly = horoscope.yearly.yearlyDecStar;
  return [
    requiredKey(SUIQIAN_KEYS, yearly.suiqian12[palaceIndex], "suiqian12"),
    requiredKey(JIANGQIAN_KEYS, yearly.jiangqian12[palaceIndex], "jiangqian12")
  ];
}

export function mutagenActivationsForProjection(horoscope, scope, palace) {
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

export function projectionFromPalace(horoscope, scope, palaceName, palace) {
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

export function projection(horoscope, scope, palaceName) {
  const palace = scope === "age"
    ? horoscope.agePalace()
    : horoscope.palace(palaceName, scope === "origin" ? "origin" : scope);
  return projectionFromPalace(horoscope, scope, palaceName, palace);
}

export function surroundProjection(horoscope, scope, palaceName) {
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
