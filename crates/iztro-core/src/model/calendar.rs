use crate::error::ChartError;
use crate::model::ganzhi::EarthlyBranch;
use serde::{Deserialize, Serialize};

/// A validated solar (Gregorian) month (`1..=12`).
///
/// This is a coarse range check only. Whether the day-of-month is valid for the
/// month and year is enforced later during calendar conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SolarMonth(u8);

impl SolarMonth {
    /// Creates a validated solar month.
    pub const fn new(value: u8) -> Result<Self, ChartError> {
        if value == 0 || value > 12 {
            return Err(ChartError::InvalidSolarMonth { value });
        }

        Ok(Self(value))
    }

    /// Returns the one-based solar month value.
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// A validated solar (Gregorian) day of the month (`1..=31`).
///
/// This is a coarse range check only. Whether the day exists for the given month
/// and year (for example 31 April or 29 February) is enforced later during
/// calendar conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SolarDay(u8);

impl SolarDay {
    /// Creates a validated solar day.
    pub const fn new(value: u8) -> Result<Self, ChartError> {
        if value == 0 || value > 31 {
            return Err(ChartError::InvalidSolarDay { value });
        }

        Ok(Self(value))
    }

    /// Returns the one-based solar day value.
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// Calendar system used to express a birth date.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarKind {
    /// Gregorian solar date (公历).
    Solar,
    /// Lunar date placeholder (农历).
    Lunar, // TODO: perhaps rename to Lunisolar?
}

/// A birth date with a declared calendar system.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CalendarDate {
    kind: CalendarKind,
    year: i32,
    month: u8,
    day: u8,
}

impl CalendarDate {
    /// Creates a solar calendar date placeholder.
    pub const fn solar(year: i32, month: u8, day: u8) -> Self {
        Self {
            kind: CalendarKind::Solar,
            year,
            month,
            day,
        }
    }

    /// Creates a lunar calendar date placeholder.
    ///
    /// This records the provided lunar date as input facts only. It does not
    /// perform calendar conversion or leap-month normalization.
    pub const fn lunar(year: i32, month: u8, day: u8) -> Self {
        Self {
            kind: CalendarKind::Lunar,
            year,
            month,
            day,
        }
    }

    /// Returns the declared calendar kind.
    pub const fn kind(&self) -> CalendarKind {
        self.kind
    }

    /// Returns the year value.
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// Returns the month value.
    pub const fn month(&self) -> u8 {
        self.month
    }

    /// Returns the day value.
    pub const fn day(&self) -> u8 {
        self.day
    }
}

/// Gender marker used by chart-generation profiles.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    /// Female gender marker.
    Female,
    /// Male gender marker.
    Male,
}

/// Birth inputs retained as chart facts.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BirthContext {
    date: CalendarDate,
    birth_time: EarthlyBranch,
    gender: Gender,
}

impl BirthContext {
    /// Creates a birth context from typed calendar, time, and gender facts.
    pub const fn new(date: CalendarDate, birth_time: EarthlyBranch, gender: Gender) -> Self {
        Self {
            date,
            birth_time,
            gender,
        }
    }

    /// Returns the birth date.
    pub const fn date(&self) -> &CalendarDate {
        &self.date
    }

    /// Returns the birth time branch.
    pub const fn birth_time(&self) -> EarthlyBranch {
        self.birth_time
    }

    /// Returns the gender marker.
    pub const fn gender(&self) -> Gender {
        self.gender
    }
}
