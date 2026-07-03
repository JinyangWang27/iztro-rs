use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length};
use iztro_i18n::I18n;

use crate::app::{
    BirthForm, FormError, InputMode, Message, SavedChart, StaticChartApp, birth_time_summary,
    utc_offset_choices,
};

use super::labels::{
    GenderChoice, InputModeChoice, TimeChoice, gender_choices, input_mode_choices, time_choices,
};
use super::style::{error_text_style, input_panel_style, section_title_style, subtle_text_style};
use super::theme::{GuiPalette, SPACING, TYPE};

/// The landing page: birth-input form plus the list of saved charts.
pub(super) fn startup_screen<'a>(
    app: &'a StaticChartApp,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let title = column![
        text(i18n.text("startup-title"))
            .size(TYPE.title)
            .style(section_title_style(palette)),
        text(i18n.text("startup-subtitle"))
            .size(TYPE.body)
            .style(subtle_text_style(palette)),
    ]
    .spacing(SPACING.sm);

    let editing = app.editing_saved_index().is_some();
    column![
        language_bar(app, i18n),
        title,
        input_bar(app.form(), app.error(), editing, palette, i18n),
        saved_charts_panel(app.saved(), palette, i18n),
    ]
    .spacing(SPACING.xl)
    .padding(SPACING.xxl)
    .into()
}

/// A compact language switcher placed above the page title.
fn language_bar<'a>(app: &'a StaticChartApp, i18n: &I18n) -> Element<'a, Message> {
    let choice = |label: &str, locale| {
        let selected = app.locale() == locale;
        let style = if selected {
            button::primary
        } else {
            button::secondary
        };
        button(text(i18n.text(label)).size(TYPE.label))
            .on_press(Message::SetLocale(locale))
            .style(style)
            .padding([SPACING.sm, SPACING.lg])
    };
    row![
        text(i18n.text("ui-language")).size(TYPE.label),
        choice("ui-english", iztro_i18n::Locale::EnUs),
        choice("ui-simplified-chinese", iztro_i18n::Locale::ZhHans),
    ]
    .spacing(SPACING.lg)
    .align_y(Alignment::Center)
    .into()
}

/// The saved-charts list shown on the startup page. Each row shows the saved
/// name prominently with birth metadata, plus open / edit / delete actions.
pub(super) fn saved_charts_panel<'a>(
    saved: &'a [SavedChart],
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let mut content =
        column![text(i18n.text("chart-saved-charts")).size(TYPE.heading)].spacing(SPACING.lg);
    if saved.is_empty() {
        content = content.push(
            text(i18n.text("saved-empty"))
                .size(TYPE.body)
                .style(subtle_text_style(palette)),
        );
    } else {
        let mut list = column![].spacing(SPACING.md);
        for (index, saved) in saved.iter().enumerate() {
            list = list.push(saved_chart_row(index, saved, palette, i18n));
        }
        content = content.push(list);
    }
    container(content)
        .style(input_panel_style(palette))
        .padding(SPACING.xl)
        .width(Length::Fill)
        .into()
}

/// One saved-chart row: the name (click to open) over its birth metadata, with
/// edit / delete actions.
fn saved_chart_row<'a>(
    index: usize,
    saved: &'a SavedChart,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let input = &saved.input;
    let meta = format!(
        "{}-{:02}-{:02} · {} · {}",
        input.year(),
        input.month(),
        input.day(),
        i18n.gender(input.gender()),
        birth_time_summary(input, i18n),
    );
    let info = column![
        text(saved.name.clone()).size(TYPE.heading),
        text(meta)
            .size(TYPE.label)
            .style(subtle_text_style(palette)),
    ]
    .spacing(SPACING.xs);

    row![
        button(info)
            .on_press(Message::SelectSaved(index))
            .style(button::secondary)
            .width(Length::Fill),
        button(text(i18n.text("button-edit")).size(TYPE.body))
            .on_press(Message::EditSaved(index))
            .style(button::secondary)
            .padding([SPACING.md, SPACING.xl]),
        button(text(i18n.text("button-delete")).size(TYPE.body))
            .on_press(Message::DeleteSaved(index))
            .style(button::danger)
            .padding([SPACING.md, SPACING.xl]),
    ]
    .spacing(SPACING.md)
    .align_y(Alignment::Center)
    .into()
}

// Birth input
pub(super) fn input_bar<'a>(
    form: &'a BirthForm,
    error: Option<&'a FormError>,
    editing: bool,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let locale = i18n.locale();

    // Identity + input-mode selector, then the shared solar date, then the
    // mode-specific birth-time controls, then gender + actions.
    let header = row![
        labeled(
            i18n.text("field-name"),
            text_input(&i18n.text("chart-name-placeholder"), &form.name)
                .on_input(Message::NameChanged)
                .width(150)
        ),
        labeled(
            i18n.text("field-input-mode"),
            pick_list(
                input_mode_choices(locale),
                Some(InputModeChoice {
                    mode: form.mode,
                    locale
                }),
                |choice| Message::InputModeSelected(choice.mode)
            )
            .width(180),
        ),
    ]
    .spacing(SPACING.xl)
    .align_y(Alignment::End);

    let date = row![
        labeled(
            i18n.text("field-year"),
            text_input("1990", &form.year)
                .on_input(Message::YearChanged)
                .width(82)
        ),
        labeled(
            i18n.text("field-month"),
            text_input("5", &form.month)
                .on_input(Message::MonthChanged)
                .width(58)
        ),
        labeled(
            i18n.text("field-day"),
            text_input("17", &form.day)
                .on_input(Message::DayChanged)
                .width(58)
        ),
    ]
    .spacing(SPACING.xl)
    .align_y(Alignment::End);

    let birth_time_row: Element<'a, Message> = match form.mode {
        InputMode::KnownTimeBranch => known_time_branch_fields(form, locale, i18n),
        InputMode::Clock => clock_time_fields(form, palette, i18n),
    };

    let mut actions = row![
        labeled(
            i18n.text("field-gender"),
            pick_list(
                gender_choices(locale),
                Some(GenderChoice {
                    gender: form.gender,
                    locale
                }),
                |choice| Message::GenderSelected(choice.gender)
            )
            .width(96),
        ),
        // In edit mode the primary action updates the chosen saved record.
        button(
            text(if editing {
                i18n.text("button-update")
            } else {
                i18n.text("button-generate")
            })
            .size(TYPE.heading)
        )
        .on_press(Message::Generate)
        .style(button::primary)
        .padding([SPACING.lg, SPACING.xxl]),
    ]
    .spacing(SPACING.xl)
    .align_y(Alignment::End);

    if editing {
        actions = actions.push(
            button(text(i18n.text("button-cancel")).size(TYPE.heading))
                .on_press(Message::CancelEditSaved)
                .style(button::secondary)
                .padding([SPACING.lg, SPACING.xxl]),
        );
    }

    let mut bar = column![header, date, birth_time_row, actions].spacing(SPACING.md);
    if let Some(error) = error {
        let message = format_error(error, i18n);
        let mut args = fluent_args();
        args.set("message", message);
        bar = bar.push(
            text(i18n.text_args("input-error", &args))
                .style(error_text_style(palette))
                .size(14),
        );
    }
    container(bar)
        .style(input_panel_style(palette))
        .padding(SPACING.lg)
        .width(Length::Fill)
        .into()
}

/// The birth-time controls for known-time-branch mode: the existing 时辰 picker.
fn known_time_branch_fields<'a>(
    form: &'a BirthForm,
    locale: iztro_i18n::Locale,
    i18n: &I18n,
) -> Element<'a, Message> {
    row![labeled(
        i18n.text("field-time"),
        pick_list(
            time_choices(locale),
            Some(TimeChoice {
                index: form.time_index,
                locale
            }),
            |choice| Message::TimeSelected(choice.index)
        )
        .width(150),
    )]
    .spacing(SPACING.xl)
    .align_y(Alignment::End)
    .into()
}

/// The birth-time controls for clock mode: hour/minute, birth-place UTC offset,
/// the apparent-solar-time toggle, and — only when it is enabled — a longitude
/// field. The GUI collects these and hands them to core; it never derives the
/// corrected time, time branch, or resolved date itself.
fn clock_time_fields<'a>(
    form: &'a BirthForm,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let mut fields = row![
        labeled(
            i18n.text("field-clock-hour"),
            text_input("12", &form.clock_hour)
                .on_input(Message::ClockHourChanged)
                .width(58)
        ),
        labeled(
            i18n.text("field-clock-minute"),
            text_input("00", &form.clock_minute)
                .on_input(Message::ClockMinuteChanged)
                .width(58)
        ),
        labeled(
            i18n.text("field-utc-offset"),
            pick_list(utc_offset_choices(), Some(form.utc_offset), |choice| {
                Message::UtcOffsetSelected(choice)
            })
            .width(120),
        ),
        labeled(
            i18n.text("field-apparent-solar-time"),
            checkbox("", form.apparent_solar_time).on_toggle(Message::ApparentSolarTimeToggled),
        ),
    ]
    .spacing(SPACING.xl)
    .align_y(Alignment::End);

    if form.apparent_solar_time {
        fields = fields.push(labeled(
            i18n.text("field-longitude"),
            column![
                text_input("120.0", &form.longitude)
                    .on_input(Message::LongitudeChanged)
                    .width(110),
                text(i18n.text("field-longitude-hint"))
                    .size(TYPE.label)
                    .style(subtle_text_style(palette)),
            ]
            .spacing(SPACING.xs),
        ));
    }

    fields.into()
}

/// Localizes a [`FormError`] into its display message. Every variant — including
/// chart-generation failures — resolves through a Fluent key, so no raw core
/// error string ever reaches the UI.
fn format_error(error: &FormError, i18n: &I18n) -> String {
    i18n.text(error.fluent_key())
}

fn fluent_args() -> iztro_i18n::FluentArgs<'static> {
    iztro_i18n::FluentArgs::new()
}

pub(super) fn labeled<'a>(
    label: String,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![text(label).size(TYPE.label), control.into()]
        .spacing(SPACING.xs)
        .into()
}
