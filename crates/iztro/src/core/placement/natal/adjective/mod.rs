//! Deterministic adjective-star (杂曜) placement for the natal chart.
//!
//! Reproduces the full supported natal 杂曜 set of `iztro` 2.5.8
//! `adjectiveStars` (`getAdjectiveStar` plus the `getLuanXiIndex`,
//! `getMonthlyStarIndex`, `getTimelyStarIndex`, `getDailyStarIndex`,
//! `getHuagaiXianchiIndex`, and `getYearlyStarIndex` helpers in `src/star`,
//! MIT licensed). The default (non-Zhongzhou) algorithm places 38 natal-origin
//! stars. Zhongzhou keeps the common natal stars, replaces 截路/空亡 with
//! 龙德/截空/劫煞/大耗, and may swap 天伤/天使 by year-branch/gender polarity.
//! The common stars are grouped by placement basis:
//!
//! - 红鸾 (HongLuan) / 天喜 (TianXi): from the birth year branch;
//! - 天姚 (TianYao) / 天刑 (TianXing): from the lunar month;
//! - 台辅 (TaiFu) / 封诰 (FengGao): from the birth time branch;
//! - 三台 (SanTai) / 八座 (BaZuo): from the placed 左辅/右弼 and lunar day;
//! - 龙池 (LongChi) / 凤阁 (FengGe): from the birth year branch;
//! - 天哭 (TianKu) / 天虚 (TianXu): from the birth year branch;
//! - 恩光 (EnGuang) / 天贵 (TianGui): from the placed 文昌/文曲 and lunar day;
//! - 天巫 (TianWu) / 天月 (TianYueAdj) / 阴煞 (YinSha) / 解神 (JieShen): fixed
//!   per-lunar-month branch lookups;
//! - 华盖 (HuaGai) / 咸池 (XianChi) / 孤辰 (GuChen) / 寡宿 (GuaSu) /
//!   蜚廉 (FeiLian) / 破碎 (PoSui) / 天德 (TianDe) / 月德 (YueDe) /
//!   年解 (NianJie): from the birth year branch;
//! - 天空 (TianKong): one branch forward from the birth year branch;
//! - 天官 (TianGuan) / 天厨 (TianChu) / 天福 (TianFuAdj): from the birth year
//!   stem;
//! - 天才 (TianCai) / 天寿 (TianShou): Life/Body-palace anchored, counted by the
//!   birth year branch;
//! - 天伤 (TianShang) / 天使 (TianShi): Life-palace anchored (仆役/疾厄 under the
//!   default algorithm; no阴阳 swap);
//! - 截路 (JieLu) / 空亡 (KongWang): from the birth year stem;
//! - 旬空 (XunKong): the 旬中空亡 void branch whose阴阳 polarity matches the birth
//!   year branch.
//!
//! 神煞 beyond this supported natal slice, adjective-star brightness, temporal
//! scopes, horoscope placement, and leap-month behavior stay out of scope.
//! 四化 remain `mutagen: Option<Mutagen>` facts on placements, never independent stars.

mod formulas;
mod input;
mod metadata;
mod placer;
mod tables;

#[cfg(test)]
mod tests;

pub use input::AdjectiveStarPlacementInput;
pub use metadata::{
    adjective_star_metadata, adjective_star_metadata_table, try_adjective_star_metadata,
};
pub use placer::{AdjectiveStarPlacer, DeterministicAdjectiveStarPlacer};
