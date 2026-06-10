// Shared normalization maps and helpers for the supported-field fixture
// generators (by_solar E2E and leap-month by_lunar). They map upstream
// iztro@2.5.8 Chinese labels onto the snake_case keys iztro-rs serde uses, and
// preserve the raw upstream labels beside the normalized keys for diagnosis.
//
// These mirror the maps used by dump-e2e-supported-fixtures.mjs; they are
// factored out here so the by_solar and leap-month generators stay in sync.

export const BRANCH_KEYS = new Map([
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

export const STEM_KEYS = new Map([
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

export const PALACE_KEYS = new Map([
  ["命宫", "life"],
  ["命", "life"],
  ["兄弟", "siblings"],
  ["夫妻", "spouse"],
  ["子女", "children"],
  ["财帛", "wealth"],
  ["疾厄", "health"],
  ["迁移", "migration"],
  ["仆役", "friends"],
  ["官禄", "career"],
  ["田宅", "property"],
  ["福德", "spirit"],
  ["父母", "parents"]
]);

export const BUREAU_KEYS = new Map([
  ["水二局", "water2"],
  ["木三局", "wood3"],
  ["金四局", "metal4"],
  ["土五局", "earth5"],
  ["火六局", "fire6"]
]);

export const BRIGHTNESS_KEYS = new Map([
  ["", "unknown"],
  ["庙", "temple"],
  ["旺", "prosperous"],
  ["得", "advantage"],
  ["利", "favourable"],
  ["平", "flat"],
  ["不", "weak"],
  ["陷", "trapped"]
]);

export const MUTAGEN_KEYS = new Map([
  ["禄", "lu"],
  ["权", "quan"],
  ["科", "ke"],
  ["忌", "ji"]
]);

export const TYPED_STAR_KEYS = new Map([
  ["紫微", "zi_wei"],
  ["天机", "tian_ji"],
  ["太阳", "tai_yang"],
  ["武曲", "wu_qu"],
  ["天同", "tian_tong"],
  ["廉贞", "lian_zhen"],
  ["天府", "tian_fu"],
  ["太阴", "tai_yin"],
  ["贪狼", "tan_lang"],
  ["巨门", "ju_men"],
  ["天相", "tian_xiang"],
  ["天梁", "tian_liang"],
  ["七杀", "qi_sha"],
  ["破军", "po_jun"],
  ["左辅", "zuo_fu"],
  ["右弼", "you_bi"],
  ["文昌", "wen_chang"],
  ["文曲", "wen_qu"],
  ["天魁", "tian_kui"],
  ["天钺", "tian_yue"],
  ["禄存", "lu_cun"],
  ["天马", "tian_ma"],
  ["擎羊", "qing_yang"],
  ["陀罗", "tuo_luo"],
  ["火星", "huo_xing"],
  ["铃星", "ling_xing"],
  ["地空", "di_kong"],
  ["地劫", "di_jie"],
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

export const CHANGSHENG_KEYS = new Map([
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

export const BOSHI_KEYS = new Map([
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

export const SUIQIAN_KEYS = new Map([
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

export const JIANGQIAN_KEYS = new Map([
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

export function reverseMap(map) {
  return new Map([...map.entries()].map(([raw, key]) => [key, raw]));
}

export function requiredKey(map, name, label) {
  const key = map.get(name);
  if (key === undefined) {
    throw new Error(`Unsupported ${label}: ${name}`);
  }
  return key;
}

export function normalizeMutagen(mutagen) {
  if (!mutagen) {
    return null;
  }
  return requiredKey(MUTAGEN_KEYS, mutagen, "mutagen");
}

export function normalizeBrightness(brightness) {
  return requiredKey(BRIGHTNESS_KEYS, brightness ?? "", "brightness");
}

export function normalizeTypedStar(star) {
  return {
    name: requiredKey(TYPED_STAR_KEYS, star.name, "typed star"),
    raw_name: star.name,
    kind: star.type,
    raw_kind: star.type,
    brightness: normalizeBrightness(star.brightness),
    raw_brightness: star.brightness || null,
    mutagen: normalizeMutagen(star.mutagen),
    raw_mutagen: star.mutagen || null
  };
}

export function normalizePalaceFacts(palace) {
  return {
    branch: requiredKey(BRANCH_KEYS, palace.earthlyBranch, "branch"),
    raw_branch: palace.earthlyBranch,
    name: requiredKey(PALACE_KEYS, palace.name, "palace"),
    raw_name: palace.name,
    stem: requiredKey(STEM_KEYS, palace.heavenlyStem, "stem"),
    raw_stem: palace.heavenlyStem
  };
}

export function normalizeTypedStars(palace) {
  return {
    branch: requiredKey(BRANCH_KEYS, palace.earthlyBranch, "branch"),
    raw_branch: palace.earthlyBranch,
    stars: [
      ...palace.majorStars.map(normalizeTypedStar),
      ...palace.minorStars.map(normalizeTypedStar),
      ...palace.adjectiveStars.map(normalizeTypedStar)
    ]
  };
}

export function normalizeDecorativeStars(palace) {
  return {
    branch: requiredKey(BRANCH_KEYS, palace.earthlyBranch, "branch"),
    raw_branch: palace.earthlyBranch,
    changsheng12: requiredKey(CHANGSHENG_KEYS, palace.changsheng12, "changsheng12"),
    raw_changsheng12: palace.changsheng12,
    boshi12: requiredKey(BOSHI_KEYS, palace.boshi12, "boshi12"),
    raw_boshi12: palace.boshi12,
    suiqian12: requiredKey(SUIQIAN_KEYS, palace.suiqian12, "suiqian12"),
    raw_suiqian12: palace.suiqian12,
    jiangqian12: requiredKey(JIANGQIAN_KEYS, palace.jiangqian12, "jiangqian12"),
    raw_jiangqian12: palace.jiangqian12
  };
}

// Builds the normalized supported_fields block shared by both generators.
export function normalizeSupportedFields(chart) {
  const typedStarCount = chart.palaces.reduce(
    (count, palace) =>
      count + palace.majorStars.length + palace.minorStars.length + palace.adjectiveStars.length,
    0
  );

  return {
    life_palace_branch: requiredKey(BRANCH_KEYS, chart.earthlyBranchOfSoulPalace, "life palace branch"),
    body_palace_branch: requiredKey(BRANCH_KEYS, chart.earthlyBranchOfBodyPalace, "body palace branch"),
    five_element_bureau: requiredKey(BUREAU_KEYS, chart.fiveElementsClass, "five element bureau"),
    palaces: chart.palaces.map(normalizePalaceFacts),
    typed_natal_stars: chart.palaces.map(normalizeTypedStars),
    decorative_stars: chart.palaces.map(normalizeDecorativeStars),
    typed_natal_star_count: typedStarCount,
    decorative_runtime_star_count: chart.palaces.length * 4
  };
}
