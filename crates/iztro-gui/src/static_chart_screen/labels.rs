use std::fmt;

use iced::Element;
use iced::widget::text;
use iztro::StaticChartCenterProjection;
use iztro::core::Gender;
use iztro_i18n::{I18n, Locale};

use crate::app::Message;

use super::style::section_title_style;
use super::theme::GuiPalette;

/// The single-row 四柱 label, such as `癸酉 丁巳 戊申 辛酉` / `Gui You Ding Si …`,
/// built from the typed pillar stem-branches. `None` when the chart carries no
/// four pillars.
pub(super) fn four_pillars_line(
    center: &StaticChartCenterProjection,
    i18n: &I18n,
) -> Option<String> {
    center.four_pillars.as_ref().map(|pillars| {
        let pillar = |sb| i18n.stem_branch_value(sb);
        format!(
            "{} {} {} {}",
            pillar(pillars.yearly),
            pillar(pillars.monthly),
            pillar(pillars.daily),
            pillar(pillars.hourly)
        )
    })
}

/// One labeled fact row, such as `五行局：木三局` / `Bureau: Wood 3`. The label is
/// already localized; the separator follows the locale (full-width colon for
/// Simplified Chinese, ASCII colon for English).
pub(super) fn fact_row(
    i18n: &I18n,
    label: &str,
    value: impl Into<String>,
) -> Element<'static, Message> {
    let sep = match i18n.locale() {
        Locale::ZhHans => "：",
        Locale::EnUs => ": ",
    };
    text(format!("{label}{sep}{}", value.into()))
        .size(13)
        .into()
}

pub(super) fn section_title(palette: GuiPalette, label: &str) -> Element<'static, Message> {
    text(label.to_owned())
        .size(13)
        .style(section_title_style(palette))
        .into()
}

/// The gender symbol shown before the basic-info header (`♂` male / `♀` female).
/// Symbols are locale-neutral.
pub(super) fn gender_symbol(gender: Gender) -> &'static str {
    match gender {
        Gender::Female => "♀",
        Gender::Male => "♂",
    }
}

/// A locale-aware birth-time pick-list option for the double-hour (时辰) `0..=12`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct TimeChoice {
    pub(super) index: u8,
    pub(super) locale: Locale,
}

/// Builds the thirteen birth-time options for `locale`.
pub(super) fn time_choices(locale: Locale) -> Vec<TimeChoice> {
    (0..=12).map(|index| TimeChoice { index, locale }).collect()
}

impl fmt::Display for TimeChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = I18n::new(self.locale).hour_branch(self.index);
        write!(f, "{label} ({})", self.index)
    }
}

/// A locale-aware gender pick-list option.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct GenderChoice {
    pub(super) gender: Gender,
    pub(super) locale: Locale,
}

/// Builds the gender options for `locale`.
pub(super) fn gender_choices(locale: Locale) -> Vec<GenderChoice> {
    [Gender::Female, Gender::Male]
        .into_iter()
        .map(|gender| GenderChoice { gender, locale })
        .collect()
}

impl fmt::Display for GenderChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&I18n::new(self.locale).gender(self.gender))
    }
}
