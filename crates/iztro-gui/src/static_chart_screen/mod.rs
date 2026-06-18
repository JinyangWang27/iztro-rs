//! Iced rendering of one [`StaticChartViewSnapshot`] in a layout.
//!
//! The screen is a composed grid — a top row of four palaces, a middle band with
//! a left palace column, a center panel spanning the middle 2x2, and a right
//! palace column, then a bottom row of four palaces — placed by each palace's
//! fixed `grid_position`. A startup screen carries the solar birth-input bar and
//! the saved-charts list; the chart screen adds a 三方四正 highlight toggle, a
//! clickable temporal navigation panel, and 科权禄忌 badges. This module only
//! reads prepared snapshot view models; it performs no astrology placement,
//! 三方四正, mutagen, rule evaluation, or 成格 derivation.
//!
//! [`StaticChartViewSnapshot`]: iztro::core::StaticChartViewSnapshot

mod chart;
mod labels;
mod palace;
mod startup;
mod style;
mod temporal;

#[cfg(test)]
mod tests;

use iced::Element;

use crate::app::{Message, Screen, StaticChartApp};

/// Renders the active screen: the startup landing page or a generated chart.
pub fn view(app: &StaticChartApp) -> Element<'_, Message> {
    match (app.screen(), app.snapshot()) {
        (Screen::Chart, Some(snapshot)) => chart::chart_screen(app, snapshot),
        // Startup, or a defensive fallback if the chart screen has no snapshot.
        _ => startup::startup_screen(app),
    }
}
