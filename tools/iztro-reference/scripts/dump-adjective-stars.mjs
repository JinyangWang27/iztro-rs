import { astro } from "iztro";

const CANONICAL_CASE = {
  lunarDate: "1990-5-17",
  timeIndex: 4,
  gender: "女",
  isLeapMonth: false,
  fixLeap: true,
  language: "zh-CN"
};

const chart = astro.byLunar(
  CANONICAL_CASE.lunarDate,
  CANONICAL_CASE.timeIndex,
  CANONICAL_CASE.gender,
  CANONICAL_CASE.isLeapMonth,
  CANONICAL_CASE.fixLeap,
  CANONICAL_CASE.language
);

const palaces = chart.palaces.map((palace) => ({
  earthlyBranch: palace.earthlyBranch,
  adjectiveStars: palace.adjectiveStars
}));

const adjectiveStarCount = palaces.reduce((count, palace) => count + palace.adjectiveStars.length, 0);

console.log(
  JSON.stringify(
    {
      target: "iztro@2.5.8",
      input: CANONICAL_CASE,
      adjectiveStarCount,
      palaces
    },
    null,
    2
  )
);
