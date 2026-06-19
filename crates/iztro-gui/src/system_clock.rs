//! OS-clock adapter for the Iced application boundary.

use chrono::{Datelike, Local, Timelike};

use crate::app::LocalSolarMoment;

/// Reads the current local wall-clock moment as plain solar date/time facts.
///
/// Core owns all calendar and temporal-selection derivation.
pub(crate) fn local_solar_moment() -> LocalSolarMoment {
    let now = Local::now();
    LocalSolarMoment {
        year: now.year(),
        month: now.month() as u8,
        day: now.day() as u8,
        hour: now.hour() as u8,
        minute: now.minute() as u8,
    }
}
