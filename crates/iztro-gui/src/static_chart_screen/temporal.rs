use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};
use iztro::core::{StaticNavigationCellView, StaticTemporalOverlayView, StaticTemporalPanelView};

use crate::app::{Message, TemporalCell};

use super::labels::{scope_zh, star_detail_label};
use super::style::{
    subtle_text_style, temporal_cell_button_style, temporal_cell_style, temporal_panel_style,
};

/// Tone for the remaining grouped badges (temporal overlays only).
#[derive(Clone, Copy)]
enum StarGroupTone {
    Decorative,
    Temporal,
}

pub(super) fn overlay_badges(overlay: &StaticTemporalOverlayView) -> Element<'_, Message> {
    let mut content = column![text(scope_zh(overlay.scope)).size(11)].spacing(2);
    if let Some(name) = overlay.temporal_palace_name_zh.as_deref() {
        content = content.push(text(name).size(11).style(subtle_text_style));
    }
    if !overlay.typed_stars.is_empty() {
        content = content.push(star_group(
            "流曜",
            overlay.typed_stars.iter().map(star_detail_label).collect(),
            StarGroupTone::Temporal,
        ));
    }
    if !overlay.decorative_stars.is_empty() {
        content = content.push(star_group(
            "流神",
            overlay
                .decorative_stars
                .iter()
                .map(|star| star.name_zh.clone())
                .collect(),
            StarGroupTone::Decorative,
        ));
    }
    if !overlay.mutagens.is_empty() {
        let labels = overlay
            .mutagens
            .iter()
            .map(|mutagen| format!("{}{}", mutagen.star_zh, mutagen.mutagen_zh))
            .collect::<Vec<_>>();
        content = content.push(star_group("四化", labels, StarGroupTone::Temporal));
    }
    content.into()
}

pub(super) fn temporal_navigation_panel<'a>(
    panel: &'a StaticTemporalPanelView,
    natal_selected: bool,
) -> Element<'a, Message> {
    // First row: 本命 (natal) and 限前 (pre-decadal) lead the 大限 cells inline.
    let mut decadal_cells = vec![
        temporal_cell(
            TemporalCell::Natal,
            Some("本命"),
            None,
            true,
            natal_selected,
        ),
        temporal_cell(
            TemporalCell::PreDecadal,
            Some(panel.pre_decadal_cell.label_zh.as_str()),
            panel.pre_decadal_cell.age_range_zh.as_deref(),
            panel.pre_decadal_cell.enabled,
            panel.pre_decadal_cell.selected,
        ),
    ];
    decadal_cells.extend(panel.decadal_cells.iter().enumerate().map(|(i, cell)| {
        temporal_cell(
            TemporalCell::Decadal(i),
            cell.age_range_zh.as_deref(),
            cell.limit_label_zh.as_deref(),
            cell.enabled,
            cell.selected,
        )
    }));
    let decadal = temporal_row("本命/限前/大限", decadal_cells);
    let yearly = temporal_row(
        "流年/小限",
        panel
            .yearly_age_cells
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                temporal_cell(
                    TemporalCell::YearlyAge(i),
                    cell.year_label.as_deref(),
                    cell.stem_branch_age_zh.as_deref(),
                    cell.enabled,
                    cell.selected,
                )
            })
            .collect(),
    );
    let month = temporal_row("流月", nav_cells(&panel.month_cells, TemporalCell::Month));

    let mut rows = column![decadal, yearly, month].spacing(4);
    for (r, days) in panel.day_rows.iter().enumerate() {
        let widgets = days
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                temporal_cell(
                    TemporalCell::Day(r, i),
                    Some(cell.label_zh.as_str()),
                    None,
                    cell.enabled,
                    cell.selected,
                )
            })
            .collect();
        rows = rows.push(temporal_row("流日", widgets));
    }
    rows = rows.push(temporal_row(
        "流时",
        nav_cells(&panel.hour_cells, TemporalCell::Hour),
    ));

    container(rows)
        .style(temporal_panel_style)
        .padding(8)
        .width(Length::Fill)
        .into()
}

/// Builds the clickable cell widgets for a simple navigation row.
pub(super) fn nav_cells<'a>(
    cells: &'a [StaticNavigationCellView],
    id_for: impl Fn(usize) -> TemporalCell,
) -> Vec<Element<'a, Message>> {
    cells
        .iter()
        .enumerate()
        .map(|(i, cell)| {
            temporal_cell(
                id_for(i),
                Some(cell.label_zh.as_str()),
                None,
                cell.enabled,
                cell.selected,
            )
        })
        .collect()
}

pub(super) fn temporal_row<'a>(
    label: &'static str,
    cells: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut content = row![container(text(label).size(11)).width(72)]
        .spacing(3)
        .align_y(Alignment::Center);
    for cell in cells {
        content = content.push(cell);
    }
    content.into()
}

/// Renders one temporal cell. Enabled cells are clickable buttons that emit a
/// [`Message::SelectTemporalCell`]; disabled cells stay inert containers and can
/// never become an active selection.
pub(super) fn temporal_cell<'a>(
    id: TemporalCell,
    primary: Option<&'a str>,
    secondary: Option<&'a str>,
    enabled: bool,
    selected: bool,
) -> Element<'a, Message> {
    let primary_text = text(primary.unwrap_or("—")).size(10);
    let primary_text = if enabled {
        primary_text
    } else {
        primary_text.style(subtle_text_style)
    };
    let mut content = column![primary_text].spacing(1).align_x(Alignment::Center);
    if let Some(secondary) = secondary {
        content = content.push(text(secondary).size(9));
    }

    if enabled {
        button(content)
            .on_press(Message::SelectTemporalCell(id))
            .padding([3, 2])
            .width(Length::FillPortion(1))
            .style(move |theme, _status| temporal_cell_button_style(theme, selected))
            .into()
    } else {
        container(content)
            .style(move |theme| temporal_cell_style(theme, false))
            .padding([3, 2])
            .width(Length::FillPortion(1))
            .into()
    }
}

fn star_group(
    label: &'static str,
    labels: Vec<String>,
    tone: StarGroupTone,
) -> Element<'static, Message> {
    row![
        star_badge(label.to_owned(), tone),
        text(labels.join(" ")).size(11).width(Length::Fill),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .into()
}

fn star_badge(label: String, tone: StarGroupTone) -> Element<'static, Message> {
    container(text(label).size(11))
        .style(star_badge_style(tone))
        .padding([2, 5])
        .into()
}

fn star_badge_style(tone: StarGroupTone) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {
        let (background, text_color) = star_badge_colors(tone);
        container::Style {
            background: Some(background.into()),
            text_color: Some(text_color),
            border: iced::Border {
                color: background,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..container::Style::default()
        }
    }
}

fn star_badge_colors(tone: StarGroupTone) -> (Color, Color) {
    match tone {
        StarGroupTone::Decorative => (Color::from_rgb8(126, 87, 48), Color::WHITE),
        StarGroupTone::Temporal => (Color::from_rgb8(45, 102, 63), Color::WHITE),
    }
}
