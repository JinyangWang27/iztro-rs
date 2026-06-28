//! Inputs for the supported natal adjective-star placement.

use crate::core::model::calendar::BirthTime;
use lunar_lite::{EarthlyBranch, HeavenlyStem};
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};

/// Inputs required to place the supported natal adjective-star set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AdjectiveStarPlacementInput {
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    daily_star_offset: u8,
    birth_time: BirthTime,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
}

impl AdjectiveStarPlacementInput {
    /// Creates adjective-star placement input from explicit lunar and ganzhi facts.
    pub const fn new(
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        birth_time: EarthlyBranch,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self::new_with_daily_star_offset(
            lunar_month,
            lunar_day,
            lunar_day.value() - 1,
            BirthTime::from_branch(birth_time),
            birth_year_stem,
            birth_year_branch,
        )
    }

    /// Creates adjective-star placement input with explicit daily-star offset.
    pub const fn new_with_daily_star_offset(
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
        daily_star_offset: u8,
        birth_time: BirthTime,
        birth_year_stem: HeavenlyStem,
        birth_year_branch: EarthlyBranch,
    ) -> Self {
        Self {
            lunar_month,
            lunar_day,
            daily_star_offset,
            birth_time,
            birth_year_stem,
            birth_year_branch,
        }
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the validated lunar day.
    pub const fn lunar_day(self) -> LunarDay {
        self.lunar_day
    }

    /// Returns the day offset used by daily adjective-star formulas.
    pub const fn daily_star_offset(self) -> u8 {
        self.daily_star_offset
    }

    /// Returns the birth time branch.
    pub const fn birth_time(self) -> EarthlyBranch {
        self.birth_time.branch()
    }

    /// Returns the full birth-time variant.
    pub const fn birth_time_variant(self) -> BirthTime {
        self.birth_time
    }

    /// Returns the birth year Heavenly Stem used for stem-based stars.
    pub const fn birth_year_stem(self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth year Earthly Branch used for branch-based stars.
    pub const fn birth_year_branch(self) -> EarthlyBranch {
        self.birth_year_branch
    }
}
