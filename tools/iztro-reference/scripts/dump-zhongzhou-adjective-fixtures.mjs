import { astro } from "iztro";
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

const STAR_KEYS = new Map([
  ["红鸾", "hong_luan"],
  ["天喜", "tian_xi"],
  ["天姚", "tian_yao"],
  ["天刑", "tian_xing"],
  ["台辅", "tai_fu"],
  ["封诰", "feng_gao"],
  ["三台", "san_tai"],
  ["八座", "ba_zuo"],
  ["龙池", "long_chi"],
  ["凤阁", "feng_ge"],
  ["天哭", "tian_ku"],
  ["天虚", "tian_xu"],
  ["恩光", "en_guang"],
  ["天贵", "tian_gui"],
  ["天巫", "tian_wu"],
  ["天月", "tian_yue_adj"],
  ["阴煞", "yin_sha"],
  ["解神", "jie_shen"],
  ["华盖", "hua_gai"],
  ["孤辰", "gu_chen"],
  ["寡宿", "gua_su"],
  ["蜚廉", "fei_lian"],
  ["破碎", "po_sui"],
  ["天德", "tian_de"],
  ["月德", "yue_de"],
  ["年解", "nian_jie"],
  ["咸池", "xian_chi"],
  ["天空", "tian_kong"],
  ["天官", "tian_guan"],
  ["天厨", "tian_chu"],
  ["天福", "tian_fu_adj"],
  ["天才", "tian_cai"],
  ["天寿", "tian_shou"],
  ["天伤", "tian_shang"],
  ["天使", "tian_shi"],
  ["截路", "jie_lu"],
  ["空亡", "kong_wang"],
  ["旬空", "xun_kong"],
  ["龙德", "long_de_adj"],
  ["截空", "jie_kong"],
  ["劫杀", "jie_sha_adj"],
  ["大耗", "da_hao_adj"]
]);

const CASES = [
  {
    fixtureName: "zhongzhou_adjective_stars_1990_05_17_chen_female.json",
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
    fixtureName: "zhongzhou_adjective_stars_1988_03_14_zi_male.json",
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
    fixtureName: "zhongzhou_adjective_stars_1991_08_09_hai_female.json",
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

function normalizeStar(star) {
  const key = STAR_KEYS.get(star.name);
  if (!key) {
    throw new Error(`Unsupported adjective star name: ${star.name}`);
  }

  return {
    name: key,
    type: star.type
  };
}

function normalizePalace(palace) {
  const branch = BRANCH_KEYS.get(palace.earthlyBranch);
  if (!branch) {
    throw new Error(`Unsupported branch name: ${palace.earthlyBranch}`);
  }

  return {
    branch,
    stars: palace.adjectiveStars.map(normalizeStar)
  };
}

function buildFixture(input) {
  astro.config({ algorithm: "zhongzhou" });

  const chart = astro.byLunar(input.lunarDate, input.timeIndex, input.gender, false, true, "zh-CN");
  const palaces = chart.palaces.map((palace) => ({
    earthlyBranch: palace.earthlyBranch,
    adjectiveStars: palace.adjectiveStars
  }));
  const adjectiveStarCount = palaces.reduce((count, palace) => count + palace.adjectiveStars.length, 0);

  return {
    metadata: {
      source: "iztro",
      target_package: "npm:iztro",
      target_version: "2.5.8",
      algorithm: "zhongzhou",
      generated_at: "2026-06-09T00:00:00Z",
      supported_fields_only: true,
      adjective_star_count: adjectiveStarCount,
      notes:
        "Zhongzhou natal adjective/helper star (杂曜) set from iztro@2.5.8 getAdjectiveStar with astro.config({ algorithm: 'zhongzhou' }). Zhongzhou replaces default 截路/空亡 with 龙德/截空/劫杀/大耗 and may swap 天伤/天使 according to getTianshiTianshangIndex. Flow stars, temporal scopes, horoscope placement, by_solar, calendar conversion, leap-month behavior, rat-hour variants, features, rules, and narrative are excluded.",
      generation_command:
        "npm ci --prefix tools/iztro-reference && node tools/iztro-reference/scripts/dump-zhongzhou-adjective-fixtures.mjs"
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
    iztro_output: {
      solarDate: chart.solarDate,
      lunarDate: chart.lunarDate,
      fiveElementsClass: chart.fiveElementsClass,
      palaces
    },
    supported_fields: {
      adjective_stars: chart.palaces.map(normalizePalace)
    }
  };
}

const fixtures = CASES.map((fixtureCase) => ({
  name: fixtureCase.fixtureName,
  fixture: buildFixture(fixtureCase)
}));

if (process.argv.includes("--write")) {
  for (const { name, fixture } of fixtures) {
    writeFileSync(
      join("fixtures", "iztro", name),
      `${JSON.stringify(fixture, null, 2)}\n`
    );
  }
} else {
  console.log(JSON.stringify(fixtures, null, 2));
}
