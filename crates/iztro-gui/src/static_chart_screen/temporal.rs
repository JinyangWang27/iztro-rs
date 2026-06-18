use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element};
use iztro::core::{EarthlyBranch, StaticTemporalNavigationSelection, StaticTemporalOverlayView};

use crate::app::{LocalSolarMoment, Message, StepDirection, TemporalUnit};

use super::style::{period_badge_button_style, stepper_button_style, temporal_cell_style};

/// Renders the compact temporal badges for one palace overlay, iztro-style:
/// a single clickable `MAJOR_PURPLE` label such as `流年·丁`. Clicking it makes
/// this palace branch the active 三方四正 source.
///
/// `is_source` is `true` when this palace is the active selection, which gives
/// the badge the stronger filled styling.
pub(super) fn period_badge(
    overlay: &StaticTemporalOverlayView,
    branch: EarthlyBranch,
    is_source: bool,
) -> Element<'_, Message> {
    let label = overlay
        .period_label_zh
        .clone()
        .unwrap_or_else(|| overlay.temporal_palace_name_zh.clone().unwrap_or_default());
    button(text(label).size(10))
        .on_press(Message::SelectPalace(branch))
        .padding([1, 4])
        .style(move |_theme, _status| period_badge_button_style(is_source))
        .into()
}

/// The compact iztro-style temporal stepper row placed under 运限信息:
/// `◀限 ◀年 ◀月 ◀日 ◀时   今   时▶ 日▶ 月▶ 年▶ 限▶`.
///
/// Steps whose parent index is missing are rendered inert; the `今` control
/// carries the supplied current local moment so the update path can resolve it
/// in core (and tests can inject a fixed moment).
pub(super) fn temporal_controls<'a>(
    selection: StaticTemporalNavigationSelection,
    now: LocalSolarMoment,
) -> Element<'a, Message> {
    let backs = row![
        step_button("◀限", TemporalUnit::Decadal, StepDirection::Backward, true),
        step_button(
            "◀年",
            TemporalUnit::Year,
            StepDirection::Backward,
            selection.decadal_index().is_some()
        ),
        step_button(
            "◀月",
            TemporalUnit::Month,
            StepDirection::Backward,
            selection.year_index().is_some()
        ),
        step_button(
            "◀日",
            TemporalUnit::Day,
            StepDirection::Backward,
            selection.month_index().is_some()
        ),
        step_button(
            "◀时",
            TemporalUnit::Hour,
            StepDirection::Backward,
            selection.day_index().is_some()
        ),
    ]
    .spacing(3);

    let today = button(text("今").size(11))
        .on_press(Message::SelectToday(now))
        .padding([2, 8])
        .style(stepper_button_style);

    let forwards = row![
        step_button(
            "时▶",
            TemporalUnit::Hour,
            StepDirection::Forward,
            selection.day_index().is_some()
        ),
        step_button(
            "日▶",
            TemporalUnit::Day,
            StepDirection::Forward,
            selection.month_index().is_some()
        ),
        step_button(
            "月▶",
            TemporalUnit::Month,
            StepDirection::Forward,
            selection.year_index().is_some()
        ),
        step_button(
            "年▶",
            TemporalUnit::Year,
            StepDirection::Forward,
            selection.decadal_index().is_some()
        ),
        step_button("限▶", TemporalUnit::Decadal, StepDirection::Forward, true),
    ]
    .spacing(3);

    column![
        row![backs, today].spacing(8).align_y(Alignment::Center),
        forwards,
    ]
    .spacing(4)
    .into()
}

/// One stepper control. Enabled steps are clickable buttons; disabled steps stay
/// inert containers so an unavailable step can never change state.
fn step_button<'a>(
    label: &'a str,
    unit: TemporalUnit,
    direction: StepDirection,
    enabled: bool,
) -> Element<'a, Message> {
    let content = text(label).size(11);
    if enabled {
        button(content)
            .on_press(Message::StepTemporal(unit, direction))
            .padding([2, 5])
            .style(stepper_button_style)
            .into()
    } else {
        container(content)
            .style(|theme| temporal_cell_style(theme, false))
            .padding([2, 5])
            .into()
    }
}
