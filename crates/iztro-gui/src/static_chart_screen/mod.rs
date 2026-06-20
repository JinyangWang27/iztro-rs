//! Iced rendering of one [`StaticChartViewSnapshot`] in a layout.
//!
//! The screen is a composed grid — a top row of four palaces, a middle band with
//! a left palace column, a center panel spanning the middle 2x2, and a right
//! palace column, then a bottom row of four palaces — placed by each palace's
//! fixed `grid_position`. A startup screen carries the solar birth-input bar and
//! the saved-charts list. The chart screen follows the original iztro layout: an
//! iztro-style center information block with a compact temporal stepper, 大限/小限
//! limits and 流年/流月/流日/流时 badges in each palace, and a transparent canvas
//! overlay drawing the 三方四正 connecting lines. This module only reads prepared
//! snapshot view models; it performs no astrology placement, 三方四正, mutagen,
//! rule evaluation, or 成格 derivation.
//!
//! [`StaticChartViewSnapshot`]: iztro::core::StaticChartViewSnapshot

mod chart;
mod labels;
mod lines;
mod palace;
mod startup;
mod style;
mod temporal;

#[cfg(test)]
mod tests;

use iced::Element;
use iztro_i18n::I18n;

use crate::app::{Message, Screen, StaticChartApp};

/// Renders the active screen: the startup landing page or a generated chart.
///
/// The localizer is built once per frame from the app's current locale and
/// threaded into the render functions, so all user-facing strings resolve at
/// this presentation boundary.
pub fn view(app: &StaticChartApp) -> Element<'_, Message> {
    let i18n = I18n::new(app.locale());
    match (app.screen(), app.snapshot()) {
        (Screen::Chart, Some(snapshot)) => chart::chart_screen(app, snapshot, &i18n),
        // Startup, or a defensive fallback if the chart screen has no snapshot.
        _ => startup::startup_screen(app, &i18n),
    }
}
