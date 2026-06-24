//! iztro 2.5.8 lookup tables for adjective-star placement.
//!
//! `*_BY_MONTH` tables come from iztro `getMonthlyStarIndex`, indexed by the
//! zero-based lunar month (正月 = 0); the year-branch tables reproduce the
//! birth-year-branch subset of `getYearlyStarIndex`. Each entry is the target
//! Earthly Branch.

use crate::core::model::ganzhi::EarthlyBranch;

pub(super) const TIAN_WU_BY_MONTH: [EarthlyBranch; 4] = [
    EarthlyBranch::Si,
    EarthlyBranch::Shen,
    EarthlyBranch::Yin,
    EarthlyBranch::Hai,
];
pub(super) const TIAN_YUE_BY_MONTH: [EarthlyBranch; 12] = [
    EarthlyBranch::Xu,
    EarthlyBranch::Si,
    EarthlyBranch::Chen,
    EarthlyBranch::Yin,
    EarthlyBranch::Wei,
    EarthlyBranch::Mao,
    EarthlyBranch::Hai,
    EarthlyBranch::Wei,
    EarthlyBranch::Yin,
    EarthlyBranch::Wu,
    EarthlyBranch::Xu,
    EarthlyBranch::Yin,
];
pub(super) const YIN_SHA_BY_MONTH: [EarthlyBranch; 6] = [
    EarthlyBranch::Yin,
    EarthlyBranch::Zi,
    EarthlyBranch::Xu,
    EarthlyBranch::Shen,
    EarthlyBranch::Wu,
    EarthlyBranch::Chen,
];
pub(super) const JIE_SHEN_BY_HALF_MONTH: [EarthlyBranch; 6] = [
    EarthlyBranch::Shen,
    EarthlyBranch::Xu,
    EarthlyBranch::Zi,
    EarthlyBranch::Yin,
    EarthlyBranch::Chen,
    EarthlyBranch::Wu,
];
pub(super) const FEI_LIAN_BY_YEAR_BRANCH: [EarthlyBranch; 12] = [
    EarthlyBranch::Shen,
    EarthlyBranch::You,
    EarthlyBranch::Xu,
    EarthlyBranch::Si,
    EarthlyBranch::Wu,
    EarthlyBranch::Wei,
    EarthlyBranch::Yin,
    EarthlyBranch::Mao,
    EarthlyBranch::Chen,
    EarthlyBranch::Hai,
    EarthlyBranch::Zi,
    EarthlyBranch::Chou,
];
pub(super) const NIAN_JIE_BY_YEAR_BRANCH: [EarthlyBranch; 12] = [
    EarthlyBranch::Xu,
    EarthlyBranch::You,
    EarthlyBranch::Shen,
    EarthlyBranch::Wei,
    EarthlyBranch::Wu,
    EarthlyBranch::Si,
    EarthlyBranch::Chen,
    EarthlyBranch::Mao,
    EarthlyBranch::Yin,
    EarthlyBranch::Chou,
    EarthlyBranch::Zi,
    EarthlyBranch::Hai,
];
pub(super) const DA_HAO_ADJ_BY_YEAR_BRANCH: [EarthlyBranch; 12] = [
    EarthlyBranch::Wei,
    EarthlyBranch::Wu,
    EarthlyBranch::You,
    EarthlyBranch::Shen,
    EarthlyBranch::Hai,
    EarthlyBranch::Xu,
    EarthlyBranch::Chou,
    EarthlyBranch::Zi,
    EarthlyBranch::Mao,
    EarthlyBranch::Yin,
    EarthlyBranch::Si,
    EarthlyBranch::Chen,
];
