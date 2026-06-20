use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length};
use iztro_i18n::I18n;

use crate::app::{BirthForm, FormError, Message, SavedChart, StaticChartApp};

use super::labels::{GenderChoice, TimeChoice, gender_choices, time_choices};
use super::style::{error_text_style, input_panel_style, subtle_text_style};

/// The landing page: birth-input form plus the list of saved charts.
pub(super) fn startup_screen<'a>(app: &'a StaticChartApp, i18n: &I18n) -> Element<'a, Message> {
    let title = column![
        text(i18n.text("startup-title")).size(24),
        text(i18n.text("startup-subtitle"))
            .size(13)
            .style(subtle_text_style),
    ]
    .spacing(4);

    let editing = app.editing_saved_index().is_some();
    column![
        language_bar(app, i18n),
        title,
        input_bar(app.form(), app.error(), editing, i18n),
        saved_charts_panel(app.saved(), i18n),
    ]
    .spacing(12)
    .padding(16)
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
        button(text(i18n.text(label)).size(12))
            .on_press(Message::SetLocale(locale))
            .style(style)
            .padding([4, 10])
    };
    row![
        text(i18n.text("ui-language")).size(12),
        choice("ui-english", iztro_i18n::Locale::EnUs),
        choice("ui-simplified-chinese", iztro_i18n::Locale::ZhHans),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

/// The saved-charts list shown on the startup page. Each row shows the saved
/// name prominently with birth metadata, plus open / edit / delete actions.
pub(super) fn saved_charts_panel<'a>(saved: &'a [SavedChart], i18n: &I18n) -> Element<'a, Message> {
    let mut content = column![text(i18n.text("chart-saved-charts")).size(15)].spacing(8);
    if saved.is_empty() {
        content = content.push(
            text(i18n.text("saved-empty"))
                .size(13)
                .style(subtle_text_style),
        );
    } else {
        let mut list = column![].spacing(6);
        for (index, saved) in saved.iter().enumerate() {
            list = list.push(saved_chart_row(index, saved, i18n));
        }
        content = content.push(list);
    }
    container(content)
        .style(input_panel_style)
        .padding(12)
        .width(Length::Fill)
        .into()
}

/// One saved-chart row: the name (click to open) over its birth metadata, with
/// edit / delete actions.
fn saved_chart_row<'a>(index: usize, saved: &'a SavedChart, i18n: &I18n) -> Element<'a, Message> {
    let input = &saved.input;
    let meta = format!(
        "{}-{:02}-{:02} · {} · {}",
        input.year,
        input.month,
        input.day,
        i18n.gender(input.gender),
        i18n.hour_branch(input.time_index),
    );
    let info = column![
        text(saved.name.clone()).size(15),
        text(meta).size(12).style(subtle_text_style),
    ]
    .spacing(2);

    row![
        button(info)
            .on_press(Message::SelectSaved(index))
            .style(button::secondary)
            .width(Length::Fill),
        button(text(i18n.text("button-edit")).size(13))
            .on_press(Message::EditSaved(index))
            .style(button::secondary)
            .padding([6, 12]),
        button(text(i18n.text("button-delete")).size(13))
            .on_press(Message::DeleteSaved(index))
            .style(button::danger)
            .padding([6, 12]),
    ]
    .spacing(6)
    .align_y(Alignment::Center)
    .into()
}

// Birth input
pub(super) fn input_bar<'a>(
    form: &'a BirthForm,
    error: Option<&'a FormError>,
    editing: bool,
    i18n: &I18n,
) -> Element<'a, Message> {
    let locale = i18n.locale();
    let mut fields = row![
        labeled(
            i18n.text("field-name"),
            text_input(&i18n.text("chart-name-placeholder"), &form.name)
                .on_input(Message::NameChanged)
                .width(150)
        ),
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
        labeled(
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
        ),
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
            .size(15)
        )
        .on_press(Message::Generate)
        .style(button::primary)
        .padding([8, 16]),
    ]
    .spacing(12)
    .align_y(Alignment::End);

    if editing {
        fields = fields.push(
            button(text(i18n.text("button-cancel")).size(15))
                .on_press(Message::CancelEditSaved)
                .style(button::secondary)
                .padding([8, 16]),
        );
    }

    let mut bar = column![fields].spacing(6);
    if let Some(error) = error {
        let message = format_error(error, i18n);
        let mut args = fluent_args();
        args.set("message", message);
        bar = bar.push(
            text(i18n.text_args("input-error", &args))
                .style(error_text_style)
                .size(14),
        );
    }
    container(bar)
        .style(input_panel_style)
        .padding(10)
        .width(Length::Fill)
        .into()
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
    column![text(label).size(12), control.into()]
        .spacing(2)
        .into()
}
