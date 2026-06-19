use std::fmt;

use iced::Element;
use iced::widget::text;
use iztro::core::{Gender, StaticChartCenterView};

use crate::app::Message;

use super::style::section_title_style;

/// The single-row 四柱 label, such as `癸酉 丁巳 戊申 辛酉`, joined from the
/// prepared pillar labels. `None` when the chart carries no four pillars.
pub(super) fn four_pillars_line(center: &StaticChartCenterView) -> Option<String> {
    center.four_pillars.as_ref().map(|pillars| {
        format!(
            "{} {} {} {}",
            pillars.yearly_zh, pillars.monthly_zh, pillars.daily_zh, pillars.hourly_zh
        )
    })
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

/// The gender symbol shown before `基本信息` (`♂` male / `♀` female).
pub(super) fn gender_symbol(gender: Gender) -> &'static str {
    match gender {
        Gender::Female => "♀",
        Gender::Male => "♂",
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
