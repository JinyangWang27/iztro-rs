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
  name: palace.name,
  heavenlyStem: palace.heavenlyStem,
  earthlyBranch: palace.earthlyBranch,
  majorStars: palace.majorStars,
  minorStars: palace.minorStars,
  adjectiveStars: palace.adjectiveStars
}));

console.log(
  JSON.stringify(
    {
      target: "iztro@2.5.8",
      input: CANONICAL_CASE,
      chart: {
        solarDate: chart.solarDate,
        lunarDate: chart.lunarDate,
        time: chart.time,
        timeRange: chart.timeRange,
        gender: chart.gender,
        fiveElementsClass: chart.fiveElementsClass,
        earthlyBranchOfSoulPalace: chart.earthlyBranchOfSoulPalace,
        earthlyBranchOfBodyPalace: chart.earthlyBranchOfBodyPalace,
        palaces
      }
    },
    null,
    2
  )
);
