use iced::widget::{button, container, row, text};
use iced::{Alignment, Element};
use iztro::core::{EarthlyBranch, Scope, StaticTemporalNavigationSelection};
use iztro_i18n::I18n;

use crate::app::{Message, StepDirection, TemporalUnit};

use super::style::{period_badge_button_style, stepper_button_style, temporal_cell_style};
use super::theme::GuiPalette;

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
    palette: GuiPalette,
    label: &str,
    branch: EarthlyBranch,
    is_source: bool,
) -> Element<'static, Message> {
    button(text(label.to_owned()).size(10))
        .on_press(Message::SelectPalace(branch))
        .padding([1, 4])
        .style(move |_theme, _status| period_badge_button_style(palette, is_source))
        .into()
}

/// The compact iztro-style temporal stepper row placed under 运限信息:
/// `◀限 ◀年 ◀月 ◀日 ◀时   今   时▶ 日▶ 月▶ 年▶ 限▶`.
///
/// Steps whose parent index is missing are rendered inert; the `今` control
/// emits a clock-free message so the Iced update boundary reads the click-time
/// local moment.
pub(super) fn temporal_controls<'a>(
    palette: GuiPalette,
    selection: StaticTemporalNavigationSelection,
    i18n: &I18n,
) -> Element<'a, Message> {
    let today = button(text(i18n.text("temporal-today")).size(11))
        .on_press(Message::TodayPressed)
        .padding([2, 8])
        .style(stepper_button_style(palette));

    // `◀` + short label for backward steps, short label + `▶` for forward steps.
    let back = |scope: Scope| format!("◀{}", i18n.temporal_short(scope));
    let fwd = |scope: Scope| format!("{}▶", i18n.temporal_short(scope));

    // One horizontal line: `◀限 ◀年 ◀月 ◀日 ◀时 今 时▶ 日▶ 月▶ 年▶ 限▶` (or the
    // English equivalent). The tight spacing keeps all eleven controls on one row.
    row![
        step_button(
            palette,
            back(Scope::Decadal),
            TemporalUnit::Decadal,
            StepDirection::Backward,
            true
        ),
        step_button(
            palette,
            back(Scope::Yearly),
            TemporalUnit::Year,
            StepDirection::Backward,
            selection.decadal_index().is_some()
        ),
        step_button(
            palette,
            back(Scope::Monthly),
            TemporalUnit::Month,
            StepDirection::Backward,
            selection.year_index().is_some()
        ),
        step_button(
            palette,
            back(Scope::Daily),
            TemporalUnit::Day,
            StepDirection::Backward,
            selection.month_index().is_some()
        ),
        step_button(
            palette,
            back(Scope::Hourly),
            TemporalUnit::Hour,
            StepDirection::Backward,
            selection.day_index().is_some()
        ),
        today,
        step_button(
            palette,
            fwd(Scope::Hourly),
            TemporalUnit::Hour,
            StepDirection::Forward,
            selection.day_index().is_some()
        ),
        step_button(
            palette,
            fwd(Scope::Daily),
            TemporalUnit::Day,
            StepDirection::Forward,
            selection.month_index().is_some()
        ),
        step_button(
            palette,
            fwd(Scope::Monthly),
            TemporalUnit::Month,
            StepDirection::Forward,
            selection.year_index().is_some()
        ),
        step_button(
            palette,
            fwd(Scope::Yearly),
            TemporalUnit::Year,
            StepDirection::Forward,
            selection.decadal_index().is_some()
        ),
        step_button(
            palette,
            fwd(Scope::Decadal),
            TemporalUnit::Decadal,
            StepDirection::Forward,
            true
        ),
    ]
    .spacing(3)
    .align_y(Alignment::Center)
    .into()
}

/// One stepper control. Enabled steps are clickable buttons; disabled steps stay
/// inert containers so an unavailable step can never change state.
fn step_button<'a>(
    palette: GuiPalette,
    label: String,
    unit: TemporalUnit,
    direction: StepDirection,
    enabled: bool,
) -> Element<'a, Message> {
    let content = text(label).size(11);
    if enabled {
        button(content)
            .on_press(Message::StepTemporal(unit, direction))
            .padding([2, 5])
            .style(stepper_button_style(palette))
            .into()
    } else {
        container(content)
            .style(move |_theme| temporal_cell_style(palette, false))
            .padding([2, 5])
            .into()
    }
}
