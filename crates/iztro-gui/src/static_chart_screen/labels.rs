use std::fmt;

use iced::Element;
use iced::widget::text;
use iztro::core::{Gender, Scope, StaticChartCenterView, StaticTypedStarView};

use crate::app::Message;

use super::style::section_title_style;

pub(super) fn star_detail_label(star: &StaticTypedStarView) -> String {
    match (&star.brightness_zh.is_empty(), star.mutagen_zh.as_deref()) {
        (false, Some(mutagen)) => format!("{}{}{}", star.name_zh, star.brightness_zh, mutagen),
        (false, None) => format!("{}{}", star.name_zh, star.brightness_zh),
        (true, Some(mutagen)) => format!("{}{}", star.name_zh, mutagen),
        (true, None) => star.name_zh.clone(),
    }
}

pub(super) fn center_four_pillar_rows(
    center: &StaticChartCenterView,
) -> Vec<(&'static str, String)> {
    center
        .four_pillars
        .as_ref()
        .map(|pillars| {
            vec![
                ("年柱", pillars.yearly_zh.clone()),
                ("月柱", pillars.monthly_zh.clone()),
                ("日柱", pillars.daily_zh.clone()),
                ("时柱", pillars.hourly_zh.clone()),
            ]
        })
        .unwrap_or_default()
}

pub(super) fn bureau_label(center: &StaticChartCenterView) -> String {
    center
        .five_element_bureau
        .map(|bureau| format!("{bureau:?}"))
        .unwrap_or_else(|| "未提供".to_string())
}

pub(super) fn fact_row<'a>(label: &'a str, value: impl Into<String>) -> Element<'a, Message> {
    text(format!("{label}：{}", value.into())).size(13).into()
}

pub(super) fn section_title(label: &str) -> Element<'_, Message> {
    text(label).size(13).style(section_title_style).into()
}

pub(super) fn gender_zh(gender: Gender) -> &'static str {
    match gender {
        Gender::Female => "女",
        Gender::Male => "男",
    }
}

/// Chinese label for an `iztro` `timeIndex` double-hour (`0..=12`).
pub(super) fn hour_branch_zh(time_index: u8) -> &'static str {
    match time_index {
        0 => "早子时",
        1 => "丑时",
        2 => "寅时",
        3 => "卯时",
        4 => "辰时",
        5 => "巳时",
        6 => "午时",
        7 => "未时",
        8 => "申时",
        9 => "酉时",
        10 => "戌时",
        11 => "亥时",
        12 => "晚子时",
        _ => "未知",
    }
}

pub(super) fn scope_zh(scope: Scope) -> &'static str {
    match scope {
        Scope::Natal => "本命",
        Scope::Decadal => "大限",
        Scope::Age => "小限",
        Scope::Yearly => "流年",
        Scope::Monthly => "流月",
        Scope::Daily => "流日",
        Scope::Hourly => "流时",
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct TimeChoice(pub(super) u8);

pub(super) const TIME_CHOICES: &[TimeChoice] = &[
    TimeChoice(0),
    TimeChoice(1),
    TimeChoice(2),
    TimeChoice(3),
    TimeChoice(4),
    TimeChoice(5),
    TimeChoice(6),
    TimeChoice(7),
    TimeChoice(8),
    TimeChoice(9),
    TimeChoice(10),
    TimeChoice(11),
    TimeChoice(12),
];

impl fmt::Display for TimeChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self.0 {
            0 => "子(早)",
            1 => "丑",
            2 => "寅",
            3 => "卯",
            4 => "辰",
            5 => "巳",
            6 => "午",
            7 => "未",
            8 => "申",
            9 => "酉",
            10 => "戌",
            11 => "亥",
            12 => "子(晚)",
            _ => "?",
        };
        write!(f, "{label} ({})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct GenderChoice(pub(super) Gender);

pub(super) const GENDER_CHOICES: &[GenderChoice] =
    &[GenderChoice(Gender::Female), GenderChoice(Gender::Male)];

impl fmt::Display for GenderChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(gender_zh(self.0))
    }
}
