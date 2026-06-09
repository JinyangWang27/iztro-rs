// Dumps upstream iztro@2.5.8 runtime star-family placement fixtures:
//   * decorative natal families (长生/博士/岁前/将前十二神), per palace, for the
//     default and Zhongzhou algorithms (Zhongzhou renames the 7th 岁前 entry
//     大耗 to 岁破);
//   * scoped flow stars (流耀) from getHoroscopeStar for every scope, covering
//     all ten Heavenly Stems and twelve Earthly Branches, plus yearly 年解.
//
// Usage:
//   npm ci --prefix tools/iztro-reference
//   node tools/iztro-reference/scripts/dump-runtime-star-families.mjs [--write]
//
// Without --write the fixtures are printed to stdout for inspection. With
// --write they are written under fixtures/iztro/.

import { astro } from "iztro";
import { getHoroscopeStar } from "iztro/lib/star/horoscopeStar.js";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const BRANCH_KEYS = new Map([
  ["子", "zi"],
  ["丑", "chou"],
  ["寅", "yin"],
  ["卯", "mao"],
  ["辰", "chen"],
  ["巳", "si"],
  ["午", "wu"],
  ["未", "wei"],
  ["申", "shen"],
  ["酉", "you"],
  ["戌", "xu"],
  ["亥", "hai"]
]);

const STEMS = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const STEM_KEYS = new Map([
  ["甲", "jia"],
  ["乙", "yi"],
  ["丙", "bing"],
  ["丁", "ding"],
  ["戊", "wu"],
  ["己", "ji"],
  ["庚", "geng"],
  ["辛", "xin"],
  ["壬", "ren"],
  ["癸", "gui"]
]);
const BRANCHES = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

// 长生/博士/岁前/将前十二神 names collide across families (e.g. 小耗, 大耗, 病符),
// so each family has its own Chinese -> StarName-key map.
const CHANGSHENG_KEYS = new Map([
  ["长生", "chang_sheng"],
  ["沐浴", "mu_yu"],
  ["冠带", "guan_dai"],
  ["临官", "lin_guan"],
  ["帝旺", "di_wang"],
  ["衰", "shuai"],
  ["病", "bing"],
  ["死", "si"],
  ["墓", "mu"],
  ["绝", "jue"],
  ["胎", "tai"],
  ["养", "yang"]
]);
const BOSHI_KEYS = new Map([
  ["博士", "bo_shi"],
  ["力士", "li_shi"],
  ["青龙", "qing_long"],
  ["小耗", "xiao_hao_boshi"],
  ["将军", "jiang_jun"],
  ["奏书", "zhou_shu"],
  ["飞廉", "fay_lian_boshi"],
  ["喜神", "xi_shen_boshi"],
  ["病符", "bing_fu_boshi"],
  ["大耗", "da_hao_boshi"],
  ["伏兵", "fu_bing"],
  ["官府", "guan_fu_boshi"]
]);
const SUIQIAN_KEYS = new Map([
  ["岁建", "sui_jian"],
  ["晦气", "hui_qi"],
  ["丧门", "sang_men"],
  ["贯索", "guan_suo"],
  ["官符", "guan_fu_suiqian"],
  ["小耗", "xiao_hao_suiqian"],
  ["大耗", "da_hao_suiqian"],
  ["岁破", "sui_po"],
  ["龙德", "long_de_suiqian"],
  ["白虎", "bai_hu"],
  ["天德", "tian_de_suiqian"],
  ["吊客", "diao_ke"],
  ["病符", "bing_fu_suiqian"]
]);
const JIANGQIAN_KEYS = new Map([
  ["将星", "jiang_xing"],
  ["攀鞍", "pan_an"],
  ["岁驿", "sui_yi"],
  ["息神", "xi_shen_jiangqian"],
  ["华盖", "hua_gai_jiangqian"],
  ["劫煞", "jie_sha"],
  ["灾煞", "zai_sha"],
  ["天煞", "tian_sha"],
  ["指背", "zhi_bei"],
  ["咸池", "xian_chi_jiangqian"],
  ["月煞", "yue_sha"],
  ["亡神", "wang_shen"]
]);

// Flow-star base from the second character of a flow-star name (the first char
// is the scope prefix 运/流/月/日/时).
const FLOW_BASE_BY_SUFFIX = new Map([
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

const SCOPES = ["decadal", "yearly", "monthly", "daily", "hourly"];

const DECORATIVE_CASES = [
  {
    case: "1990_05_17_chen_female",
    lunarDate: "1990-5-17",
    lunarYear: 1990,
    lunarMonth: 5,
    lunarDay: 17,
    timeIndex: 4,
    birthTime: "chen",
    gender: "女",
    genderKey: "female",
    birthYearStem: "geng",
    birthYearBranch: "wu"
  },
  {
    case: "1988_03_14_zi_male",
    lunarDate: "1988-3-14",
    lunarYear: 1988,
    lunarMonth: 3,
    lunarDay: 14,
    timeIndex: 0,
    birthTime: "zi",
    gender: "男",
    genderKey: "male",
    birthYearStem: "wu",
    birthYearBranch: "chen"
  },
  {
    case: "1991_08_09_hai_female",
    lunarDate: "1991-8-9",
    lunarYear: 1991,
    lunarMonth: 8,
    lunarDay: 9,
    timeIndex: 11,
    birthTime: "hai",
    gender: "女",
    genderKey: "female",
    birthYearStem: "xin",
    birthYearBranch: "wei"
  }
];

const GENERATED_AT = "2026-06-09T00:00:00Z";
const GENERATION_COMMAND =
  "npm ci --prefix tools/iztro-reference && node tools/iztro-reference/scripts/dump-runtime-star-families.mjs --write";

function branchKey(name) {
  const key = BRANCH_KEYS.get(name);
  if (!key) {
    throw new Error(`Unsupported branch name: ${name}`);
  }
  return key;
}

function familyKey(map, name, family) {
  const key = map.get(name);
  if (!key) {
    throw new Error(`Unsupported ${family} name: ${name}`);
  }
  return key;
}

function decorativeFixture(input, algorithm) {
  astro.config({ algorithm });
  const chart = astro.byLunar(input.lunarDate, input.timeIndex, input.gender, false, true, "zh-CN");

  const palaces = chart.palaces.map((palace) => ({
    branch: branchKey(palace.earthlyBranch),
    changsheng12: familyKey(CHANGSHENG_KEYS, palace.changsheng12, "changsheng12"),
    boshi12: familyKey(BOSHI_KEYS, palace.boshi12, "boshi12"),
    suiqian12: familyKey(SUIQIAN_KEYS, palace.suiqian12, "suiqian12"),
    jiangqian12: familyKey(JIANGQIAN_KEYS, palace.jiangqian12, "jiangqian12")
  }));

  return {
    metadata: {
      source: "iztro",
      target_package: "npm:iztro",
      target_version: "2.5.8",
      algorithm,
      generated_at: GENERATED_AT,
      supported_fields_only: true,
      notes:
        "Decorative runtime star families (长生/博士/岁前/将前十二神) from iztro@2.5.8 byLunar per-palace output. Upstream emits these as bare names with no StarKind. Under the Zhongzhou algorithm the seventh 岁前 entry 大耗 is renamed 岁破. Flow stars, by_solar, calendar conversion, leap-month behavior, rat-hour variants, features, rules, and narrative are excluded.",
      generation_command: GENERATION_COMMAND
    },
    input: {
      lunar_date: input.lunarDate,
      lunar_year: input.lunarYear,
      lunar_month: input.lunarMonth,
      lunar_day: input.lunarDay,
      birth_time: input.birthTime,
      iztro_time_index: input.timeIndex,
      gender: input.genderKey,
      iztro_gender: input.gender,
      birth_year_stem: input.birthYearStem,
      birth_year_branch: input.birthYearBranch,
      is_leap_month: false,
      fix_leap: true,
      language: "zh-CN"
    },
    supported_fields: { decorative_stars: palaces }
  };
}

function flowCase(stemName, branchName, scope) {
  const stars = getHoroscopeStar(stemName, branchName, scope);
  const matrix = [];
  let nianJieBranch = null;

  stars.forEach((cell, index) => {
    // Palace index -> branch is fixed (寅 = index 0); recover it from the order
    // of EARTHLY_BRANCHES rotated to start at 寅.
    const branch = branchKey(BRANCHES[(index + 2) % 12]);
    for (const star of cell) {
      if (star.name === "年解") {
        nianJieBranch = branch;
        continue;
      }
      const base = FLOW_BASE_BY_SUFFIX.get(star.name.slice(1));
      if (!base) {
        throw new Error(`Unsupported flow-star name: ${star.name}`);
      }
      matrix.push({ base, branch, type: star.type });
    }
  });

  matrix.sort((a, b) => a.base.localeCompare(b.base));

  const entry = {
    scope,
    stem: STEM_KEYS.get(stemName),
    branch: branchKey(branchName),
    matrix
  };
  if (scope === "yearly") {
    entry.nian_jie_branch = nianJieBranch;
  }
  return entry;
}

function flowFixture() {
  const cases = [];
  for (let i = 0; i < BRANCHES.length; i++) {
    const stemName = STEMS[i % STEMS.length];
    const branchName = BRANCHES[i];
    for (const scope of SCOPES) {
      cases.push(flowCase(stemName, branchName, scope));
    }
  }

  return {
    metadata: {
      source: "iztro",
      target_package: "npm:iztro",
      target_version: "2.5.8",
      generated_at: GENERATED_AT,
      supported_fields_only: true,
      notes:
        "Scoped flow stars (流耀) from iztro@2.5.8 getHoroscopeStar(stem, branch, scope). Each case lists the ten matrix stars (魁钺昌曲禄羊陀马鸾喜) by base and branch; the yearly scope also records 年解. Placement depends only on the period stem-branch. Cases cover all ten stems and twelve branches across every scope. Palace index 0 is 寅.",
      generation_command: GENERATION_COMMAND
    },
    cases
  };
}

const outputs = [];
for (const input of DECORATIVE_CASES) {
  outputs.push({
    name: `runtime_decorative_default_${input.case}.json`,
    fixture: decorativeFixture(input, "default")
  });
  outputs.push({
    name: `runtime_decorative_zhongzhou_${input.case}.json`,
    fixture: decorativeFixture(input, "zhongzhou")
  });
}
outputs.push({ name: "flow_stars.json", fixture: flowFixture() });

if (process.argv.includes("--write")) {
  for (const { name, fixture } of outputs) {
    writeFileSync(join("fixtures", "iztro", name), `${JSON.stringify(fixture, null, 2)}\n`);
  }
  console.log(`wrote ${outputs.length} fixtures`);
} else {
  console.log(JSON.stringify(outputs, null, 2));
}
