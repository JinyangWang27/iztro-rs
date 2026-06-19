use iced::widget::{button, container, row, text};
use iced::{Alignment, Element};
use iztro::core::{EarthlyBranch, StaticTemporalNavigationSelection};

use crate::app::{Message, StepDirection, TemporalUnit};

use super::style::{period_badge_button_style, stepper_button_style, temporal_cell_style};

/// Renders one compact temporal period badge, iztro-style: a single clickable
/// `MAJOR_PURPLE` label such as `流年·丁`. Clicking it makes this palace branch
/// the active 三方四正 source.
///
/// The caller passes the prepared `label` only for overlays core marked as the
/// period's anchor palace (`period_label_zh.is_some()`); the GUI never derives a
/// label from temporal palace-name metadata or branch arithmetic.
///
/// `is_source` is `true` when this palace is the active selection, which gives
/// the badge the stronger filled styling.
pub(super) fn period_badge(
    label: &str,
    branch: EarthlyBranch,
    is_source: bool,
) -> Element<'_, Message> {
    button(text(label.to_owned()).size(10))
        .on_press(Message::SelectPalace(branch))
        .padding([1, 4])
        .style(move |_theme, _status| period_badge_button_style(is_source))
        .into()
}

/// The compact iztro-style temporal stepper row placed under 运限信息:
/// `◀限 ◀年 ◀月 ◀日 ◀时   今   时▶ 日▶ 月▶ 年▶ 限▶`.
///
/// Steps whose parent index is missing are rendered inert; the `今` control
/// emits a clock-free message so the Iced update boundary reads the click-time
/// local moment.
pub(super) fn temporal_controls(
    selection: StaticTemporalNavigationSelection,
) -> Element<'static, Message> {
    let today = button(text("今").size(11))
        .on_press(Message::TodayPressed)
        .padding([2, 8])
        .style(stepper_button_style);

    // One horizontal line: `◀限 ◀年 ◀月 ◀日 ◀时 今 时▶ 日▶ 月▶ 年▶ 限▶`. The tight
    // spacing keeps all eleven controls inside the center panel on one row.
    row![
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
        today,
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
    .spacing(3)
    .align_y(Alignment::Center)
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
