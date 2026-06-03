//! Life and body palace index calculation from lunar month and birth hour.

use crate::{error::ChartError, ganzhi::EarthlyBranch};

/// A validated non-leap lunar birth month.
///
/// Leap-month handling is intentionally deferred. Current callers must pass the
/// effective lunar month in the supported `1..=12` range.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LunarMonth(u8);

impl LunarMonth {
    /// Creates a validated lunar month.
    pub const fn new(value: u8) -> Result<Self, ChartError> {
        if value == 0 || value > 12 {
            return Err(ChartError::InvalidLunarMonth { value });
        }

        Ok(Self(value))
    }

    /// Returns the one-based lunar month value.
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// A validated lunar day of the month (初一 = 1 through 30).
///
/// Lunar months span at most thirty days, so the supported range is `1..=30`.
/// Full calendar conversion is deferred, so callers supply the lunar day
/// directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LunarDay(u8);

impl LunarDay {
    /// Creates a validated lunar day.
    pub const fn new(value: u8) -> Result<Self, ChartError> {
        if value == 0 || value > 30 {
            return Err(ChartError::InvalidLunarDay { value });
        }

        Ok(Self(value))
    }

    /// Returns the one-based lunar day value.
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// Lunar birth facts needed for life and body palace calculation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LunarBirthContext {
    lunar_month: LunarMonth,
    birth_time: EarthlyBranch,
}

impl LunarBirthContext {
    /// Creates the lunar birth context needed by the palace-index rule.
    pub const fn new(lunar_month: LunarMonth, birth_time: EarthlyBranch) -> Self {
        Self {
            lunar_month,
            birth_time,
        }
    }

    /// Returns the validated lunar birth month.
    pub const fn lunar_month(self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the birth time branch.
    pub const fn birth_time(self) -> EarthlyBranch {
        self.birth_time
    }
}

/// Calculated life and body palace branches.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LifeBodyPalaceIndices {
    life_palace_branch: EarthlyBranch,
    body_palace_branch: EarthlyBranch,
}

impl LifeBodyPalaceIndices {
    /// Creates calculated life and body palace branch indices.
    pub const fn new(life_palace_branch: EarthlyBranch, body_palace_branch: EarthlyBranch) -> Self {
        Self {
            life_palace_branch,
            body_palace_branch,
        }
    }

    /// Returns the branch containing the Life Palace.
    pub const fn life_palace_branch(self) -> EarthlyBranch {
        self.life_palace_branch
    }

    /// Returns the branch containing the Body Palace.
    pub const fn body_palace_branch(self) -> EarthlyBranch {
        self.body_palace_branch
    }
}

/// Calculates life and body palace branches from lunar birth month and hour.
///
/// Classical rule implemented here:
///
/// - start from Yin as the first lunar month;
/// - count forward to the birth lunar month;
/// - from that month position, count backward from Zi hour to place Life;
/// - from that month position, count forward from Zi hour to place Body.
///
/// Leap-month behavior is intentionally not implemented yet.
pub fn calculate_life_body_palace_indices(
    context: LunarBirthContext,
) -> Result<LifeBodyPalaceIndices, ChartError> {
    let month_offset = isize::from(context.lunar_month().value() - 1);
    let hour_offset = context.birth_time().index() as isize - EarthlyBranch::Zi.index() as isize;
    let month_anchor = EarthlyBranch::Yin.offset(month_offset);

    Ok(LifeBodyPalaceIndices::new(
        month_anchor.offset(-hour_offset),
        month_anchor.offset(hour_offset),
    ))
}
