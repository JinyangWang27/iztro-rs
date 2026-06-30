use crate::core::error::ChartError;
use lunar_lite::EarthlyBranch;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

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

/// A validated solar (Gregorian) calendar date used by the clock-time birth
/// input API.
///
/// The year is unconstrained here; the month and day are individually
/// range-checked through [`SolarMonth`] and [`SolarDay`]. Whether the day
/// actually exists for the month and year (for example 30 February) is
/// validated where the date is consumed (the calculation-policy resolver and
/// calendar conversion).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SolarDate {
    year: i32,
    month: SolarMonth,
    day: SolarDay,
}

impl SolarDate {
    /// Creates a solar date from raw year/month/day parts, range-checking the
    /// month and day.
    pub const fn new(year: i32, month: u8, day: u8) -> Result<Self, ChartError> {
        let month = match SolarMonth::new(month) {
            Ok(month) => month,
            Err(error) => return Err(error),
        };
        let day = match SolarDay::new(day) {
            Ok(day) => day,
            Err(error) => return Err(error),
        };
        Ok(Self::from_typed(year, month, day))
    }

    /// Creates a solar date from already-validated typed parts.
    pub const fn from_typed(year: i32, month: SolarMonth, day: SolarDay) -> Self {
        Self { year, month, day }
    }

    /// Returns the Gregorian year.
    pub const fn year(self) -> i32 {
        self.year
    }

    /// Returns the validated solar month.
    pub const fn month(self) -> SolarMonth {
        self.month
    }

    /// Returns the validated solar day.
    pub const fn day(self) -> SolarDay {
        self.day
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

/// Birth time as upstream `iztro` `timeIndex` values (`0..=12`).
///
/// `iztro` distinguishes early Zi (`0`) from late Zi (`12`). Both variants map
/// to the Zi Earthly Branch, but late Zi affects the facade's effective lunar
/// day and leap-month handling.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BirthTime {
    /// Early Zi hour (`timeIndex = 0`, 子时).
    EarlyZi,
    /// Chou hour (`timeIndex = 1`, 丑时).
    Chou,
    /// Yin hour (`timeIndex = 2`, 寅时).
    Yin,
    /// Mao hour (`timeIndex = 3`, 卯时).
    Mao,
    /// Chen hour (`timeIndex = 4`, 辰时).
    Chen,
    /// Si hour (`timeIndex = 5`, 巳时).
    Si,
    /// Wu hour (`timeIndex = 6`, 午时).
    Wu,
    /// Wei hour (`timeIndex = 7`, 未时).
    Wei,
    /// Shen hour (`timeIndex = 8`, 申时).
    Shen,
    /// You hour (`timeIndex = 9`, 酉时).
    You,
    /// Xu hour (`timeIndex = 10`, 戌时).
    Xu,
    /// Hai hour (`timeIndex = 11`, 亥时).
    Hai,
    /// Late Zi hour (`timeIndex = 12`, 晚子时).
    LateZi,
}

impl BirthTime {
    /// Converts an upstream `iztro` `timeIndex` into a typed birth time.
    pub const fn from_iztro_time_index(value: u8) -> Result<Self, ChartError> {
        match value {
            0 => Ok(Self::EarlyZi),
            1 => Ok(Self::Chou),
            2 => Ok(Self::Yin),
            3 => Ok(Self::Mao),
            4 => Ok(Self::Chen),
            5 => Ok(Self::Si),
            6 => Ok(Self::Wu),
            7 => Ok(Self::Wei),
            8 => Ok(Self::Shen),
            9 => Ok(Self::You),
            10 => Ok(Self::Xu),
            11 => Ok(Self::Hai),
            12 => Ok(Self::LateZi),
            value => Err(ChartError::InvalidBirthTimeIndex { value }),
        }
    }

    /// Returns the upstream `iztro` `timeIndex` value.
    pub const fn iztro_time_index(self) -> u8 {
        match self {
            Self::EarlyZi => 0,
            Self::Chou => 1,
            Self::Yin => 2,
            Self::Mao => 3,
            Self::Chen => 4,
            Self::Si => 5,
            Self::Wu => 6,
            Self::Wei => 7,
            Self::Shen => 8,
            Self::You => 9,
            Self::Xu => 10,
            Self::Hai => 11,
            Self::LateZi => 12,
        }
    }

    /// Returns the Earthly Branch projection of the birth time.
    pub const fn branch(self) -> EarthlyBranch {
        match self {
            Self::EarlyZi | Self::LateZi => EarthlyBranch::Zi,
            Self::Chou => EarthlyBranch::Chou,
            Self::Yin => EarthlyBranch::Yin,
            Self::Mao => EarthlyBranch::Mao,
            Self::Chen => EarthlyBranch::Chen,
            Self::Si => EarthlyBranch::Si,
            Self::Wu => EarthlyBranch::Wu,
            Self::Wei => EarthlyBranch::Wei,
            Self::Shen => EarthlyBranch::Shen,
            Self::You => EarthlyBranch::You,
            Self::Xu => EarthlyBranch::Xu,
            Self::Hai => EarthlyBranch::Hai,
        }
    }

    /// Returns whether this is the late Zi (`timeIndex = 12`) variant.
    pub const fn is_late_zi(self) -> bool {
        matches!(self, Self::LateZi)
    }

    /// Converts a branch-based API input into a birth-time variant.
    ///
    /// Zi maps to early Zi for backward compatibility.
    pub const fn from_branch(value: EarthlyBranch) -> Self {
        match value {
            EarthlyBranch::Zi => Self::EarlyZi,
            EarthlyBranch::Chou => Self::Chou,
            EarthlyBranch::Yin => Self::Yin,
            EarthlyBranch::Mao => Self::Mao,
            EarthlyBranch::Chen => Self::Chen,
            EarthlyBranch::Si => Self::Si,
            EarthlyBranch::Wu => Self::Wu,
            EarthlyBranch::Wei => Self::Wei,
            EarthlyBranch::Shen => Self::Shen,
            EarthlyBranch::You => Self::You,
            EarthlyBranch::Xu => Self::Xu,
            EarthlyBranch::Hai => Self::Hai,
        }
    }

    const fn key(self) -> &'static str {
        match self {
            Self::EarlyZi => "early_zi",
            Self::Chou => "chou",
            Self::Yin => "yin",
            Self::Mao => "mao",
            Self::Chen => "chen",
            Self::Si => "si",
            Self::Wu => "wu",
            Self::Wei => "wei",
            Self::Shen => "shen",
            Self::You => "you",
            Self::Xu => "xu",
            Self::Hai => "hai",
            Self::LateZi => "late_zi",
        }
    }
}

/// Maps a clock hour (`0..=23`) to the upstream `iztro` `timeIndex` (`0..=12`).
///
/// `0` is early Zi (子) and `23` is late Zi (晚子时, `timeIndex = 12`); every other
/// hour rounds up to its 时辰 index. This is a pure time transformation owned by
/// `core`, used by both the calculation layer and the projection facade.
pub(crate) const fn time_index_for_hour(hour: u8) -> u8 {
    match hour {
        0 => 0,
        23 => 12,
        h => h.div_ceil(2),
    }
}

impl Serialize for BirthTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.key())
    }
}

impl<'de> Deserialize<'de> for BirthTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BirthTimeVisitor;

        impl serde::de::Visitor<'_> for BirthTimeVisitor {
            type Value = BirthTime;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a birth time key")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "early_zi" | "zi" => Ok(BirthTime::EarlyZi),
                    "chou" => Ok(BirthTime::Chou),
                    "yin" => Ok(BirthTime::Yin),
                    "mao" => Ok(BirthTime::Mao),
                    "chen" => Ok(BirthTime::Chen),
                    "si" => Ok(BirthTime::Si),
                    "wu" => Ok(BirthTime::Wu),
                    "wei" => Ok(BirthTime::Wei),
                    "shen" => Ok(BirthTime::Shen),
                    "you" => Ok(BirthTime::You),
                    "xu" => Ok(BirthTime::Xu),
                    "hai" => Ok(BirthTime::Hai),
                    "late_zi" => Ok(BirthTime::LateZi),
                    other => Err(E::unknown_variant(
                        other,
                        &[
                            "early_zi", "zi", "chou", "yin", "mao", "chen", "si", "wu", "wei",
                            "shen", "you", "xu", "hai", "late_zi",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_str(BirthTimeVisitor)
    }
}

/// Birth inputs retained as chart facts.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BirthContext {
    date: CalendarDate,
    birth_time: BirthTime,
    gender: Gender,
}

impl BirthContext {
    /// Creates a birth context from typed calendar, time, and gender facts.
    pub const fn new(date: CalendarDate, birth_time: EarthlyBranch, gender: Gender) -> Self {
        Self::new_with_birth_time_variant(date, BirthTime::from_branch(birth_time), gender)
    }

    /// Creates a birth context from a full iztro time-index birth-time variant.
    pub const fn new_with_birth_time_variant(
        date: CalendarDate,
        birth_time: BirthTime,
        gender: Gender,
    ) -> Self {
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
        self.birth_time.branch()
    }

    /// Returns the full birth-time variant.
    pub const fn birth_time_variant(&self) -> BirthTime {
        self.birth_time
    }

    /// Returns the gender marker.
    pub const fn gender(&self) -> Gender {
        self.gender
    }
}
