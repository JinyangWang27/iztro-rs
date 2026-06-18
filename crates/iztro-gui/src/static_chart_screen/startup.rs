use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::app::{BirthForm, BirthInput, Message, StaticChartApp};

use super::labels::{
    GENDER_CHOICES, GenderChoice, TIME_CHOICES, TimeChoice, gender_zh, hour_branch_zh,
};
use super::style::{error_text_style, input_panel_style, subtle_text_style};

/// The landing page: birth-input form plus the list of saved charts.
pub(super) fn startup_screen(app: &StaticChartApp) -> Element<'_, Message> {
    let title = column![
        text("紫微斗数 · 静态命盘").size(24),
        text("输入出生信息生成命盘，或打开已保存的命盘。")
            .size(13)
            .style(subtle_text_style),
    ]
    .spacing(4);

    column![
        title,
        input_bar(app.form(), app.error()),
        saved_charts_panel(app.saved()),
    ]
    .spacing(12)
    .padding(16)
    .into()
}

/// The saved-charts list shown on the startup page.
pub(super) fn saved_charts_panel(saved: &[BirthInput]) -> Element<'_, Message> {
    let mut content = column![text("已保存命盘").size(15)].spacing(8);
    if saved.is_empty() {
        content = content.push(
            text("暂无保存的命盘。生成命盘后会自动保存到本地。")
                .size(13)
                .style(subtle_text_style),
        );
    } else {
        let mut list = column![].spacing(6);
        for (index, input) in saved.iter().enumerate() {
            let label = format!(
                "{}-{:02}-{:02} · {} · {}",
                input.year,
                input.month,
                input.day,
                gender_zh(input.gender),
                hour_branch_zh(input.time_index),
            );
            list = list.push(
                button(text(label).size(14))
                    .on_press(Message::SelectSaved(index))
                    .style(button::secondary)
                    .width(Length::Fill),
            );
        }
        content = content.push(list);
    }
    container(content)
        .style(input_panel_style)
        .padding(12)
        .width(Length::Fill)
        .into()
}

// Birth input
pub(super) fn input_bar<'a>(form: &BirthForm, error: Option<&'a str>) -> Element<'a, Message> {
    let fields = row![
        labeled(
            "年",
            text_input("1990", &form.year)
                .on_input(Message::YearChanged)
                .width(82)
        ),
        labeled(
            "月",
            text_input("5", &form.month)
                .on_input(Message::MonthChanged)
                .width(58)
        ),
        labeled(
            "日",
            text_input("17", &form.day)
                .on_input(Message::DayChanged)
                .width(58)
        ),
        labeled(
            "时",
            pick_list(TIME_CHOICES, Some(TimeChoice(form.time_index)), |choice| {
                Message::TimeSelected(choice.0)
            })
            .width(126),
        ),
        labeled(
            "性别",
            pick_list(GENDER_CHOICES, Some(GenderChoice(form.gender)), |choice| {
                Message::GenderSelected(choice.0)
            })
            .width(82),
        ),
        button(text("生成命盘").size(15))
            .on_press(Message::Generate)
            .style(button::primary)
            .padding([8, 16]),
    ]
    .spacing(12)
    .align_y(Alignment::End);

    let mut bar = column![fields].spacing(6);
    if let Some(message) = error {
        bar = bar.push(
            text(format!("输入错误：{message}"))
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

pub(super) fn labeled<'a>(
    label: &'a str,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![text(label).size(12), control.into()]
        .spacing(2)
        .into()
}
